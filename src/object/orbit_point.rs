use std::{cell::RefCell, rc::Rc};

use nalgebra_glm::Vec2;

use super::orbit_description::EllipseDescription;

#[derive(Clone)]
pub struct OrbitPoint {
    angle_since_periapsis: f32,
    time_since_periapsis: f32,
    position: Vec2,
    velocity: Vec2,
}

impl OrbitPoint {
    pub fn new(orbit_description: &EllipseDescription, position: Vec2) -> Self {
        let angle_since_periapsis = orbit_description.get_angle_since_periapsis(position);
        let time_since_periapsis = orbit_description.get_time_since_periapsis_from_angle_since_periapsis(angle_since_periapsis);
        let velocity = orbit_description.get_velocity(position, angle_since_periapsis);
        Self { angle_since_periapsis, time_since_periapsis, position, velocity }
    }

    pub fn next(&self, orbit_description: &EllipseDescription, delta_time: f32) -> Self {
        let time_since_periapsis = self.time_since_periapsis + delta_time;
        let angle_since_periapsis = orbit_description.get_angle_since_periapsis_from_time_since_periapsis(time_since_periapsis);
        let position = orbit_description.get_position(angle_since_periapsis);
        let velocity = orbit_description.get_velocity(position, angle_since_periapsis);
        Self { angle_since_periapsis, time_since_periapsis, position, velocity }
    }

    pub fn get_angle_since_periapsis(&self) -> f32 {
        self.angle_since_periapsis
    }

    pub fn get_unscaled_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn compute_new_parent(&self, object: Rc<RefCell<Object>>, significant_mass_objects: &Vec<Rc<RefCell<Object>>>) -> Option<Rc<RefCell<Object>>> {
        let mut new_parents = vec![];
        for other_object in significant_mass_objects.iter() {
            if other_object.as_ptr() == object.as_ptr() {
                continue;
            }
            let distance_squared = (object.borrow().get_absolute_position() - other_object.borrow().get_absolute_position()).magnitude_squared();
            // We can unwrap because we're only dealing with significant mass bodies
            if distance_squared < object.borrow().sphere_of_influence_squared.unwrap() {
                new_parents.push(object.clone());
            }
        }

        if new_parents.is_empty() {
            None
        } else if new_parents.len() == 1 {
            Some(new_parents.first().unwrap().clone())
        } else {
            let mut highest_acceleration = 0.0;
            let mut parent_with_highest_acceleration = None;
            for parent in new_parents {
                let distance_squared = (parent.borrow().get_absolute_position() - object.borrow().get_absolute_position()).magnitude_squared();
                let acceleration = GRAVITATIONAL_CONSTANT * parent.borrow().mass / distance_squared;
                if acceleration > highest_acceleration {
                    highest_acceleration = acceleration;
                    parent_with_highest_acceleration = Some(parent);
                }
            }
            parent_with_highest_acceleration
        }
    }
}