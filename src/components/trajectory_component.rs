use std::collections::VecDeque;

use nalgebra_glm::DVec2;

use crate::storage::entity_allocator::Entity;

use self::orbit::Orbit;

use super::Components;

mod conic;
pub mod orbit_direction;
mod orbit_point;
pub mod orbit;

pub struct TrajectoryComponent {
    orbits: VecDeque<Orbit>,
}

impl TrajectoryComponent {
    pub fn new(components: &Components, parent: Entity, position: DVec2, velocity: DVec2, time: f64) -> Self {
        let mut orbits = VecDeque::new();
        orbits.push_back(Orbit::new(components, parent, position, velocity, time));
        Self { orbits }
    }

    pub fn get_orbits(&self) -> &VecDeque<Orbit> {
        &self.orbits
    }

    pub fn get_current_orbit(&self) -> &Orbit {
        self.orbits.front().unwrap()
    }

    pub fn get_final_orbit(&self) -> &Orbit {
        self.orbits.back().unwrap()
    }

    pub fn add_orbit(&mut self, orbit: Orbit) {
        self.orbits.push_back(orbit);
    }

    pub fn predict(&mut self, delta_time: f64) {
        if let Some(orbit) = self.orbits.back_mut() {
            orbit.predict(delta_time);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        if let Some(orbit) = self.orbits.front_mut() { 
            orbit.update(delta_time);
            if orbit.is_finished() {
                self.orbits.pop_front();
            }
        }
    }
}