use std::{rc::Rc, cell::RefCell};

use eframe::epaint::{Vec2, vec2};
use nalgebra_glm::vec3;

use super::{conic::Conic, Object};

pub struct Path {
    conics: Vec<Conic>,
}

impl Path {
    pub fn new(parent: Option<Rc<RefCell<Object>>>, position: Vec2, velocity: Vec2) -> Self {
        let mut conics = vec![];
        if let Some(parent) = parent {
            conics.push(Conic::new(parent, vec3(0.0, 0.6, 1.0), position, velocity))
        }
        Self { conics }
    }

    pub fn get_current_conic(&self) -> Option<&Conic> {
        self.conics.first()
    }

    pub fn get_current_absolute_parent_position(&self) -> Vec2 {
        match self.get_current_conic() {
            Some(conic) => conic.get_absolute_parent_position(),
            None => vec2(0.0, 0.0),
        }
    }

    pub fn get_orbit_vertices(&self, zoom: f32) -> Vec<f32> {
        let mut vertices = vec![];
        for conic in &self.conics {
            vertices.extend(conic.get_orbit_vertices(zoom));
        }
        vertices
    }
}