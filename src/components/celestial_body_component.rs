use eframe::epaint::Rgba;

use crate::storage::entity_allocator::Entity;

pub struct CelestialBodyComponent {
    radius: f64,
    color: Rgba,
    children: Vec<Entity>,
}

impl CelestialBodyComponent {
    pub fn new(radius: f64, color: Rgba) -> Self {
        let children = Vec::new();
        Self { radius, color, children }
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn get_color(&self) -> Rgba {
        self.color
    }

    pub fn get_children(&self) -> Vec<Entity> {
        self.children
    }

    pub fn add_child(&mut self, child: Entity) {
        self.children.push(child);
    }
}