use std::{collections::VecDeque, rc::Rc, cell::RefCell};

use nalgebra_glm::DVec2;

use crate::storage::entity_allocator::Entity;

use self::orbit::Orbit;

use super::Components;

mod conic;
pub mod orbit_direction;
mod orbit_point;
pub mod orbit;

pub struct TrajectoryComponent {
    orbits: VecDeque<Rc<RefCell<Orbit>>>,
}

impl TrajectoryComponent {
    pub fn new(components: &Components, parent: Entity, position: DVec2, velocity: DVec2, time: f64) -> Self {
        let mut orbits = VecDeque::new();
        orbits.push_back(Rc::new(RefCell::new(Orbit::new(components, parent, position, velocity, time))));
        Self { orbits }
    }

    pub fn get_orbits(&self) -> &VecDeque<Rc<RefCell<Orbit>>> {
        &self.orbits
    }

    pub fn get_current_orbit(&self) -> Rc<RefCell<Orbit>> {
        self.orbits.front().unwrap().clone()
    }

    pub fn get_final_orbit(&self) -> Rc<RefCell<Orbit>> {
        self.orbits.back().unwrap().clone()
    }

    pub fn add_orbit(&mut self, orbit: Orbit) {
        self.orbits.push_back(Rc::new(RefCell::new(orbit)));
    }

    pub fn predict(&mut self, delta_time: f64) {
        if let Some(orbit) = self.orbits.back_mut() {
            orbit.borrow_mut().predict(delta_time);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        if let Some(orbit) = self.orbits.front_mut() { 
            orbit.borrow_mut().update(delta_time);
            if orbit.borrow().is_finished() {
                self.orbits.pop_front();
            }
        }
    }
}