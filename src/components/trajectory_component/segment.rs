use std::{rc::Rc, cell::RefCell};

use self::{orbit::Orbit, burn::Burn};

pub mod burn;
pub mod orbit;

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
            Segment::Burn(_) => todo!(),
            Segment::Orbit(orbit) => orbit.borrow().get_overshot_time(time),
        }
    }

    pub fn as_orbit(&self) -> &Rc<RefCell<Orbit>> {
        match self {
            Segment::Burn(_) => panic!("Attempted to get non-orbit segment as orbit"),
            Segment::Orbit(orbit) => orbit,
        }
    }
}