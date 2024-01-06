use std::{rc::Rc, cell::RefCell};

use nalgebra_glm::DVec2;

use crate::storage::entity_allocator::Entity;

use self::{orbit::Orbit, burn::Burn};

pub mod burn;
pub mod orbit;

#[derive(Clone)]
pub enum Segment {
    Burn(Rc<RefCell<Burn>>),
    Orbit(Rc<RefCell<Orbit>>)
}

impl Segment {
    pub fn update(&mut self, delta_time: f64) {
        match self {
            Segment::Burn(burn) => burn.borrow_mut().update(delta_time),
            Segment::Orbit(orbit) => orbit.borrow_mut().update(delta_time),
        }
    }

    pub fn predict(&mut self, delta_time: f64) {
        match self {
            Segment::Burn(_) => panic!("Attempt to update a burn segment for prediction"),
            Segment::Orbit(orbit) => orbit.borrow_mut().predict(delta_time),
        }
    }

    pub fn is_finished(&self) -> bool {
        match self {
            Segment::Burn(burn) => burn.borrow().is_finished(),
            Segment::Orbit(orbit) => orbit.borrow().is_finished(),
        }
    }

    pub fn get_overshot_time(&self, time: f64) -> f64 {
        match self {
            Segment::Burn(burn) => burn.borrow().get_overshot_time(time),
            Segment::Orbit(orbit) => orbit.borrow().get_overshot_time(time),
        }
    }

    pub fn get_start_time(&self) -> f64 {
        match self {
            Segment::Burn(burn) => burn.borrow().get_start_point().get_time(),
            Segment::Orbit(orbit) => orbit.borrow().get_start_point().get_time(),
        }
    }

    pub fn get_current_position(&self) -> DVec2 {
        match self {
            Segment::Burn(burn) => burn.borrow().get_current_point().get_position(),
            Segment::Orbit(orbit) => orbit.borrow().get_current_point().get_position(),
        }
    }

    pub fn get_current_velocity(&self) -> DVec2 {
        match self {
            Segment::Burn(burn) => burn.borrow().get_current_point().get_velocity(),
            Segment::Orbit(orbit) => orbit.borrow().get_current_point().get_velocity(),
        }
    }

    pub fn get_end_position(&self) -> DVec2 {
        match self {
            Segment::Burn(burn) => burn.borrow().get_end_point().get_position(),
            Segment::Orbit(orbit) => orbit.borrow().get_end_point().get_position(),
        }
    }

    pub fn get_end_velocity(&self) -> DVec2 {
        match self {
            Segment::Burn(burn) => burn.borrow().get_end_point().get_velocity(),
            Segment::Orbit(orbit) => orbit.borrow().get_end_point().get_velocity(),
        }
    }

    pub fn get_end_time(&self) -> f64 {
        match self {
            Segment::Burn(burn) => burn.borrow().get_end_point().get_time(),
            Segment::Orbit(orbit) => orbit.borrow().get_end_point().get_time(),
        }
    }

    pub fn get_parent(&self) -> Entity {
        match self {
            Segment::Burn(burn) => burn.borrow().get_parent(),
            Segment::Orbit(orbit) => orbit.borrow().get_parent(),
        }
    }

    pub fn as_orbit(&self) -> &Rc<RefCell<Orbit>> {
        match self {
            Segment::Burn(_) => panic!("Attempted to get non-orbit segment as orbit"),
            Segment::Orbit(orbit) => orbit,
        }
    }

    pub fn get_position_at_time(&self, time: f64) -> DVec2 {
        match self {
            Segment::Burn(burn) => burn.borrow().get_point_at_time(time).get_position(),
            Segment::Orbit(orbit) => {
                let theta = orbit.borrow().get_theta_from_time(time);
                orbit.borrow().get_position_from_theta(theta)
            }
        }
    }

    pub fn get_velocity_at_time(&self, time: f64) -> DVec2 {
        match self {
            Segment::Burn(burn) => burn.borrow().get_point_at_time(time).get_velocity(),
            Segment::Orbit(orbit) => {
                let theta = orbit.borrow().get_theta_from_time(time);
                orbit.borrow().get_velocity_from_theta(theta)
            }
        }
    }

    pub fn reset(&self) {
        match self {
            Segment::Burn(burn) => burn.borrow_mut().reset(),
            Segment::Orbit(orbit) => orbit.borrow_mut().reset(),
        }
    }

    pub fn equals(&self, other: &Self) -> bool {
        match self {
            Segment::Burn(burn) => match other {
                Segment::Burn(other_burn) => burn.as_ptr() == other_burn.as_ptr(),
                Segment::Orbit(_) => false,
            },
            Segment::Orbit(orbit) => match other {
                Segment::Burn(_) => false,
                Segment::Orbit(other_orbit) => orbit.as_ptr() == other_orbit.as_ptr(),
            },
        }
    }
}