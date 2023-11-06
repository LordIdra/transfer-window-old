use std::collections::VecDeque;

use nalgebra_glm::{DVec2, vec3};

use crate::storage::{entity_allocator::Entity, components::Components};

use self::orbit::Orbit;

mod conic;
mod orbit_direction;
mod orbit_point;
pub mod orbit;

pub struct TrajectoryComponent {
    orbits: VecDeque<Orbit>,
}

impl TrajectoryComponent {
    pub fn new(components: &Components, parent: Option<Entity>, position: DVec2, velocity: DVec2, time: f64) -> Self {
        let mut orbits = VecDeque::new();
        if let Some(parent) = parent {
            orbits.push_back(Orbit::new(components, parent, vec3(0.0, 0.6, 1.0), position, velocity, time));
        }
        Self { orbits }
    }

    pub fn get_orbits(&self) -> VecDeque<Orbit> {
        self.orbits
    }

    pub fn get_current_unscaled_position(&self) -> Option<DVec2> {
        self.orbits.front().map(|orbit| orbit.get_current_unscaled_position())
    }

    pub fn get_current_velocity(&self) -> Option<DVec2> {
        self.orbits.front().map(|orbit| orbit.get_current_velocity())
    }

    pub fn get_final_unscaled_position(&self) -> Option<DVec2> {
        self.orbits.back().map(|orbit| orbit.get_end_unscaled_position())
    }

    pub fn get_final_velocity(&self) -> Option<DVec2> {
        self.orbits.back().map(|orbit| orbit.get_end_velocity())
    }

    pub fn get_current_parent(&self) -> Option<Entity> {
        self.orbits.front().map(|orbit| orbit.get_parent())
    }
}