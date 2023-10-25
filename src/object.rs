use std::{f64::consts::PI, collections::HashSet, cell::RefCell};

use eframe::epaint::Rgba;
use nalgebra_glm::{vec2, DVec2};

use crate::{util::add_triangle, app::ObjectId, storage::Storage};

use self::{trajectory::Trajectory, orbit_direction::GRAVITATIONAL_CONSTANT};

mod orbit;
mod conic;
mod orbit_point;
mod visual_orbit_point;
mod trajectory;
mod orbit_direction;

const SIGNIFICANT_MASS_THRESHOLD: f64 = 1.0e8; // Objects above this mass are modelled as having an SOI
pub const SCALE_FACTOR: f64 = 1.0 / 1000.0;

pub struct Object {
    id: String,
    trajectory: RefCell<Trajectory>,
    parent: Option<ObjectId>,
    children: HashSet<ObjectId>,
    position: DVec2,
    velocity: DVec2,
    mass: f64,
    radius: f64,
    color: Rgba,
    sphere_of_influence_squared: Option<f64>,
}

impl Object {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(storage: &mut Storage, id: String, parent: Option<ObjectId>, position: DVec2, velocity: DVec2, mass: f64, radius: f64, color: Rgba, time: f64) -> ObjectId {
        let trajectory = RefCell::new(Trajectory::new(storage, parent.clone(), position, velocity, time));
        let sphere_of_influence_squared = if mass > SIGNIFICANT_MASS_THRESHOLD {
            trajectory.borrow().get_sphere_of_influence_squared(storage, mass)
        } else {
            None
        };
        let children = HashSet::new();
        let object = Self { id: id.clone(), trajectory, parent, children, position, velocity, mass, radius, color, sphere_of_influence_squared };
        storage.add_object(object);
        id
    }

    pub fn add_child(&mut self, child: ObjectId) {
        self.children.insert(child);
    }

    pub fn get_id(&self) -> ObjectId {
        self.id.clone()
    }

    pub fn get_parent(&self) -> Option<ObjectId> {
        self.parent.clone()
    }

    pub fn get_children(&self) -> HashSet<ObjectId> {
        self.children.clone()
    }

    pub fn get_absolute_parent_position(&self, storage: &Storage) -> DVec2 {
        if let Some(parent) = &self.parent {
            storage.get(parent).get_absolute_position(storage)
        } else {
            vec2(0.0, 0.0)
        }
    }

    pub fn get_absolute_position(&self, storage: &Storage) -> DVec2 {
        self.get_absolute_parent_position(storage) + self.position
    }

    pub fn get_absolute_parent_velocity(&self, storage: &Storage, time: f64) -> DVec2 {
        if let Some(parent) = &self.parent {
            storage.get(parent).get_absolute_velocity(storage, time)
        } else {
            vec2(0.0, 0.0)
        }
    }

    pub fn get_absolute_velocity(&self, storage: &Storage, time: f64) -> DVec2 {
        self.get_absolute_parent_velocity(storage, time) + self.velocity
    }

    pub fn get_absolute_scaled_position(&self, storage: &Storage) -> DVec2 {
        self.get_absolute_position(storage) * SCALE_FACTOR
    }

    pub fn get_object_vertices(&self, storage: &Storage) -> Vec<f32> {
        let scaled_radius = self.radius * SCALE_FACTOR;
        let absolute_scaled_position = (self.get_absolute_parent_position(storage) + self.position) * SCALE_FACTOR;
        let mut vertices = vec![];
        let sides = 100; // TODO make this depend on something else ie zoom/translation
        let mut previous_location = absolute_scaled_position + vec2(scaled_radius, 0.0);
        for i in 1..=sides { // 1..=sides to make sure we fill in the gap between the last location and first location, wrapping back round
            let angle = (i as f64 / sides as f64) * 2.0 * PI; // both i and sides must be cast to prevent integer division problems
            let new_location = absolute_scaled_position + vec2(scaled_radius * f64::cos(angle), scaled_radius * f64::sin(angle));
            add_triangle(&mut vertices, absolute_scaled_position, previous_location, new_location, self.color);
            previous_location = new_location;
        }
        vertices
    }

    pub fn get_orbit_vertices(&self, storage: &Storage, zoom: f64) -> Vec<f32> {
        self.trajectory.borrow().get_orbit_vertices(storage, zoom)
    }

    fn update_position_and_velocity(&mut self, new_position: Option<DVec2>, new_velocity: Option<DVec2>) {
        if let Some(position) = new_position {
            self.position = position;
        }
        if let Some(velocity) = new_velocity {
            self.velocity = velocity;
        }
    }

    fn compute_new_parent_upper(&self, storage: &Storage, parent: &ObjectId) -> Option<ObjectId> {
        // Check if we've left the SOI of our current parent
        let Some(parent_sphere_of_influence_squared) = storage.get(parent).sphere_of_influence_squared else {
            return None;
        };
        if self.position.magnitude_squared() < parent_sphere_of_influence_squared {
            return None;
        }
        // We can unwrap since any object with an SOI must also have a parent
        Some(storage.get(parent).parent.clone().unwrap())
    }

    fn object_causing_highest_acceleration(&self, storage: &Storage, objects: Vec<ObjectId>) -> Option<ObjectId> {
        let highest_acceleration = 0.0;
        let mut object_causing_highest_acceleration = None;
        for object in objects {
            let acceleration = storage.get(&object).mass * GRAVITATIONAL_CONSTANT / (self.position - storage.get(&object).position).magnitude_squared();
            if acceleration > highest_acceleration {
                object_causing_highest_acceleration = Some(object);
            }
        }
        object_causing_highest_acceleration
    }

    fn compute_new_parent_lower(&self, storage: &Storage, parent: &ObjectId) -> Option<ObjectId> {
        // Check if we've entered the SOI of any objects with the same parent
        let mut potential_children = vec![];
        for child in &storage.get(parent).children {
            if *child == self.id { // Prevents deadlocks
                continue;
            }
            let Some(parent_sphere_of_influence_squared) = storage.get(child).sphere_of_influence_squared else {
                continue
            };
            if (self.position - storage.get(child).position).magnitude_squared() < parent_sphere_of_influence_squared {
                potential_children.push(child.clone());
            }
        }
        self.object_causing_highest_acceleration(storage, potential_children)
    }

    fn update_parent_for_prediction(&mut self, storage: &Storage, time: f64) {
        if let Some(parent) = &self.parent {
            if let Some(new_parent) = self.compute_new_parent_upper(storage, parent) {
                self.trajectory.borrow_mut().change_parent(storage, self, new_parent.clone(), time);
                self.parent = Some(new_parent);
            } else if let Some(new_parent) = self.compute_new_parent_lower(storage, parent) {
                self.trajectory.borrow_mut().change_parent(storage, self, new_parent.clone(), time);
                self.parent = Some(new_parent);
            }
        }
    }

    fn sync_to_trajectory(&mut self) {
        let new_position = self.trajectory.borrow().get_current_unscaled_position();
        let new_velocity = self.trajectory.borrow().get_current_velocity();
        self.update_position_and_velocity(new_position, new_velocity);
        self.parent = self.trajectory.borrow().get_current_parent();
    }

    pub fn update(&mut self, delta_time: f64) {
        self.trajectory.borrow_mut().update(delta_time);
        self.sync_to_trajectory();
    }

    pub fn update_for_prediction(&mut self, storage: &Storage, delta_time: f64, time: f64) {
        self.trajectory.borrow_mut().update_for_prediction(delta_time);
        let new_position = self.trajectory.borrow().get_final_unscaled_position();
        let new_velocity = self.trajectory.borrow().get_final_velocity();
        self.update_position_and_velocity(new_position, new_velocity);
        self.update_parent_for_prediction(storage, time);
    }

    pub fn reset(&mut self) {
        self.sync_to_trajectory();
    }
}
