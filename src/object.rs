use std::{f32::consts::PI, cell::RefCell, rc::Rc};

use eframe::epaint::Rgba;
use nalgebra_glm::{Vec2, vec2};

use self::trajectory::Trajectory;

mod conic;
mod orbit_description;
mod trajectory_integrator;
mod orbit_point;
mod visual_orbit_point;
mod trajectory;
mod orbit_direction;

const SIGNIFICANT_MASS_THRESHOLD:f32 = 1.0e8; // Objects above this mass are modelled as having an SOI
const SCALE_FACTOR: f32 = 1.0 / 100000.0;

pub struct Object {
    trajectory: Trajectory,
    position: Vec2,
    velocity: Vec2,
    mass: f32,
    radius: f32,
    color: Rgba,
    sphere_of_influence_squared: Option<f32>,
}

impl Object {
    pub fn new(parent: Option<Rc<RefCell<Object>>>, position: Vec2, velocity: Vec2, mass: f32, radius: f32, color: Rgba) -> Rc<RefCell<Self>> {
        let path = Trajectory::new(parent, position, velocity);
        let sphere_of_influence_squared = if mass > SIGNIFICANT_MASS_THRESHOLD {
            Some(Self::calculate_sphere_of_influence(&path, mass).powi(2))
        } else {
            None
        };
        Rc::new(RefCell::new(Self { trajectory: path, position, velocity, mass, radius, color, sphere_of_influence_squared }))
    }

    fn add_triangle(vertices: &mut Vec<f32>, v1: Vec2, v2: Vec2, v3: Vec2, color: Rgba) {
        vertices.append(&mut vec![v1.x, v1.y, color.r(), color.g(), color.b(), color.a()]);
        vertices.append(&mut vec![v2.x, v2.y, color.r(), color.g(), color.b(), color.a()]);
        vertices.append(&mut vec![v3.x, v3.y, color.r(), color.g(), color.b(), color.a()]);
    }

    fn calculate_sphere_of_influence(path: &Trajectory, mass: f32) -> f32 {
        // https://en.wikipedia.org/wiki/Sphere_of_influence_(astrodynamics)
        if let Some(conic) = path.get_current_conic() {
            conic.get_sphere_of_influence(mass)
        } else {
            0.0
        }
    }

    pub fn get_absolute_position(&self) -> Vec2 {
        self.trajectory.get_current_absolute_parent_position() + self.position
    }

    pub fn get_object_vertices(&self) -> Vec<f32> {
        let scaled_radius = self.radius * SCALE_FACTOR;
        let absolute_scaled_position = (self.trajectory.get_current_absolute_parent_position() + self.position) * SCALE_FACTOR;
        let mut vertices = vec![];
        let sides = 100; // TODO make this depend on something else
        let mut previous_location = absolute_scaled_position + vec2(scaled_radius, 0.0);
        for i in 1..=sides { // 1..=sides to make sure we fill in the gap between the last location and first location, wrapping back round
            let angle = (i as f32 / sides as f32) * 2.0 * PI; // both i and sides must be cast to f32 to prevent integer division problems
            let new_location = absolute_scaled_position + vec2(scaled_radius * f32::cos(angle), scaled_radius * f32::sin(angle));
            Self::add_triangle(&mut vertices, absolute_scaled_position, previous_location, new_location, self.color);
            previous_location = new_location;
        }
        vertices
    }

    pub fn get_orbit_vertices(&self, zoom: f32) -> Vec<f32> {
        self.trajectory.get_orbit_vertices(zoom)
    }

    pub fn update(&mut self, delta_time: f32) {
        self.trajectory.update(delta_time);
        if let Some(position) = self.trajectory.get_unscaled_position() {
            self.position = position;
        }
        if let Some(velocity) = self.trajectory.get_velocity() {
            self.velocity = velocity;
        }
    }

    pub fn update_for_trajectory_integration(&mut self, object: Rc<RefCell<Object>>, significant_mass_objects: &Vec<Rc<RefCell<Object>>>, delta_time: f32) {
        self.trajectory.update_for_trajectory_integration(object, significant_mass_objects, delta_time);
        if let Some(position) = self.trajectory.get_unscaled_position() {
            self.position = position;
        }
        if let Some(velocity) = self.trajectory.get_velocity() {
            self.velocity = velocity;
        }
    }

    pub fn reset_all_conics(&mut self) {
        self.trajectory.reset()
    }
}
