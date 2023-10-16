use std::sync::Arc;

use nalgebra_glm::DVec2;

use super::{conic::Conic, Object, orbit_direction::GRAVITATIONAL_CONSTANT};

// For an object x, returns all objects which have an SOI that x is within
fn get_within_sphere_of_influence(object: &Object, significant_mass_objects: &[Arc<Object>]) -> Vec<Arc<Object>> {
    let mut objects = vec![];
    for other_object in significant_mass_objects.iter() {
        if other_object.name == object.name {
            continue;
        }
        let distance_squared = (object.get_absolute_position() - other_object.get_absolute_position()).magnitude_squared();
        // We can unwrap because we're only dealing with significant mass bodies
        if distance_squared < other_object.sphere_of_influence_squared.unwrap() {
            objects.push(other_object.clone());
        }
    }
    objects
}

// For an object x, returns the object inducing the greatest acceleration on x
fn get_object_inducing_highest_acceleration(object: &Object, within_soi: Vec<Arc<Object>>) -> Option<Arc<Object>> {
    let mut highest_acceleration = 0.0;
    let mut parent_with_highest_acceleration = None;
    for other_object in within_soi {
        let distance_squared = (other_object.get_absolute_position() - object.get_absolute_position()).magnitude_squared();
        let acceleration = GRAVITATIONAL_CONSTANT * other_object.mass / distance_squared;
        if acceleration > highest_acceleration {
            highest_acceleration = acceleration;
            parent_with_highest_acceleration = Some(other_object);
        }
    }
    parent_with_highest_acceleration
}

#[derive(Debug, Clone)]
pub struct OrbitPoint {
    angle_since_periapsis: f64,
    time_since_periapsis: f64,
    position: DVec2,
    velocity: DVec2,
}

impl OrbitPoint {
    pub fn new(conic: &dyn Conic, position: DVec2) -> Self {
        let mut angle_since_periapsis = conic.get_true_anomaly_from_position(position);
        // Insane correction for floating point errors
        // If the true anomaly is slightly negative due to floating point errors, the object is considered as being BEHIND the periapsis which is... problematic
        angle_since_periapsis += 1.0e-6;
        let time_since_periapsis = conic.get_time_since_periapsis(angle_since_periapsis);
        let velocity = conic.get_velocity(position, angle_since_periapsis);
        Self { angle_since_periapsis, time_since_periapsis, position, velocity }
    }

    pub fn next(&self, conic: &dyn Conic, delta_time: f64) -> Self {
        let time_since_periapsis = self.time_since_periapsis + delta_time;
        let angle_since_periapsis = conic.get_true_anomaly_from_time_since_periapsis(time_since_periapsis);
        let position = conic.get_position(angle_since_periapsis);
        let velocity = conic.get_velocity(position, angle_since_periapsis);
        Self { angle_since_periapsis, time_since_periapsis, position, velocity }
    }

    pub fn get_angle_since_periapsis(&self) -> f64 {
        self.angle_since_periapsis
    }

    pub fn get_unscaled_position(&self) -> DVec2 {
        self.position
    }

    pub fn get_velocity(&self) -> DVec2 {
        self.velocity
    }

    pub fn compute_new_parent(&self, object: &Object, significant_mass_objects: &[Arc<Object>]) -> Option<Arc<Object>> {
        let within_soi = get_within_sphere_of_influence(object, significant_mass_objects);
        if within_soi.is_empty() {
            None
        } else if within_soi.len() == 1 {
            Some(within_soi.first().unwrap().clone())
        } else {
            get_object_inducing_highest_acceleration(object, within_soi)
        }
    }

    pub fn debug(&self) {
        println!("asp {}", self.angle_since_periapsis);
        println!("tsp {}", self.time_since_periapsis);
        println!("pos {:?}", self.position);
        println!("vel {:?}", self.velocity);
    }
}