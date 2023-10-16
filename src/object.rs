use std::{sync::{Arc, Mutex}, f64::consts::PI};

use eframe::epaint::Rgba;
use nalgebra_glm::{vec2, DVec2};

use crate::util::add_triangle;

use self::trajectory::Trajectory;

mod orbit;
mod conic;
pub mod trajectory_integrator;
mod orbit_point;
mod visual_orbit_point;
mod trajectory;
mod orbit_direction;

const SIGNIFICANT_MASS_THRESHOLD: f64 = 1.0e8; // Objects above this mass are modelled as having an SOI
const SCALE_FACTOR: f64 = 1.0 / 100000.0;

#[derive(Debug)]
pub struct Object {
    name: String,
    trajectory: Mutex<Trajectory>,
    parent: Mutex<Option<Arc<Object>>>,
    position: Mutex<DVec2>,
    velocity: Mutex<DVec2>,
    mass: f64,
    radius: f64,
    color: Rgba,
    sphere_of_influence_squared: Option<f64>,
}

impl Object {
    pub fn new(name: String, parent: Option<Arc<Object>>, position: DVec2, velocity: DVec2, mass: f64, radius: f64, color: Rgba) -> Arc<Self> {
        let trajectory = Mutex::new(Trajectory::new(parent, position, velocity));
        let sphere_of_influence_squared = if mass > SIGNIFICANT_MASS_THRESHOLD {
            Some(trajectory.lock().unwrap().get_sphere_of_influence(mass).powi(2))
        } else {
            None
        };
        let parent = Mutex::new(trajectory.lock().unwrap().get_parent());
        let position = Mutex::new(position);
        let velocity = Mutex::new(velocity);
        Arc::new(Self { name, trajectory, parent, position, velocity, mass, radius, color, sphere_of_influence_squared })
    }

    pub fn get_absolute_parent_position(&self) -> DVec2 {
        if let Some(parent) = &*self.parent.lock().unwrap() {
            parent.get_absolute_position()
        } else {
            vec2(0.0, 0.0)
        }
    }

    pub fn get_absolute_position(&self) -> DVec2 {
        self.get_absolute_parent_position() + *self.position.lock().unwrap()
    }

    pub fn get_absolute_parent_velocity(&self, time: f64) -> DVec2 {
        if let Some(parent) = &*self.parent.lock().unwrap() {
            parent.get_absolute_velocity(time)
        } else {
            vec2(0.0, 0.0)
        }
    }

    pub fn get_absolute_velocity(&self, time: f64) -> DVec2 {
        self.get_absolute_parent_velocity(time) + *self.velocity.lock().unwrap()
    }

    pub fn get_absolute_scaled_position(&self) -> DVec2 {
        self.get_absolute_position() * SCALE_FACTOR
    }

    pub fn get_object_vertices(&self) -> Vec<f32> {
        let scaled_radius = self.radius * SCALE_FACTOR;
        let absolute_scaled_position = (self.get_absolute_parent_position() + *self.position.lock().unwrap()) * SCALE_FACTOR;
        let mut vertices = vec![];
        let sides = 100; // TODO make this depend on something else
        let mut previous_location = absolute_scaled_position + vec2(scaled_radius, 0.0);
        for i in 1..=sides { // 1..=sides to make sure we fill in the gap between the last location and first location, wrapping back round
            let angle = (i as f64 / sides as f64) * 2.0 * PI; // both i and sides must be cast to prevent integer division problems
            let new_location = absolute_scaled_position + vec2(scaled_radius * f64::cos(angle), scaled_radius * f64::sin(angle));
            add_triangle(&mut vertices, absolute_scaled_position, previous_location, new_location, self.color);
            previous_location = new_location;
        }
        vertices
    }

    pub fn get_orbit_vertices(&self, zoom: f64) -> Vec<f32> {
        self.trajectory.lock().unwrap().get_orbit_vertices(zoom)
    }

    pub fn update(&self, delta_time: f64) {
        self.trajectory.lock().unwrap().update(delta_time);
        if let Some(position) = self.trajectory.lock().unwrap().get_current_unscaled_position() {
            *self.position.lock().unwrap() = position;
        }
        if let Some(velocity) = self.trajectory.lock().unwrap().get_current_velocity() {
            *self.velocity.lock().unwrap() = velocity;
        }
    }

    pub fn update_for_trajectory_integration(&self, significant_mass_objects: &[Arc<Object>], delta_time: f64, time: f64) {
        self.trajectory.lock().unwrap().update_for_trajectory_integration(self, significant_mass_objects, delta_time, time);
        if let Some(position) = self.trajectory.lock().unwrap().get_final_unscaled_position() {
            *self.position.lock().unwrap() = position;
        }
        if let Some(velocity) = self.trajectory.lock().unwrap().get_final_velocity() {
            *self.velocity.lock().unwrap() = velocity;
        }
    }

    pub fn reset_all_conics(&self) {
        self.trajectory.lock().unwrap().reset()
    }
}
