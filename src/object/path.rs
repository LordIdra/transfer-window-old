use std::{rc::Rc, cell::RefCell};

use eframe::epaint::{Vec2, vec2};

use super::{conic::Conic, Object};

pub struct Path {
    conics: Vec<Conic>,
}

impl Path {
    pub fn new(parent: Option<Rc<RefCell<Object>>>, position: Vec2, velocity: Vec2) -> Self {
        let mut conics = vec![];
        if let Some(parent) = parent {
            conics.push(Conic::new(parent, position, velocity))
        }
        Self { conics }
    }

    pub fn get_current_absolute_parent_position(&self) -> Vec2 {
        match &self.conics.first() {
            Some(conic) => conic.get_absolute_parent_position(),
            None => vec2(0.0, 0.0),
        }
    }
}