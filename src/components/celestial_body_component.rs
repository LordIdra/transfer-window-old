use std::collections::HashSet;

use eframe::epaint::Rgba;

use crate::storage::entity_allocator::Entity;

pub struct CelestialBodyComponent {
    radius: f64,
    color: Rgba,
    children: HashSet<Entity>,
}

impl CelestialBodyComponent {
    pub fn new(radius: f64, color: Rgba) -> Self {
        let children = HashSet::new();
        Self { radius, color, children }
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }

    pub fn get_color(&self) -> Rgba {
        self.color
    }

    pub fn get_children(&self) -> &HashSet<Entity> {
        &self.children
    }

    pub fn add_child(&mut self, child_to_add: Entity) {
        self.children.insert(child_to_add);
    }

    pub fn remove_child(&mut self, child_to_remove: Entity) {
        self.children.remove(&child_to_remove);
    }
}