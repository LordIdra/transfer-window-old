use std::f32::consts::PI;

use eframe::epaint::{Rgba, vec2, Vec2};

pub struct Object {
    location: Vec2,
    mass: f32,
    radius: f32,
    color: Rgba,
}

impl Object {
    pub fn new(location: Vec2, mass: f32, radius: f32, color: Rgba) -> Self {
        Object { location, mass, radius, color }
    }

    pub fn get_vertices(&self) -> Vec<f32> {
        let mut vertices = vec![];
        let sides = 100; // TODO make this depend on zoom level
        let mut previous_location = self.location + vec2(self.radius, 0.0);
        for i in 1..=sides { // 1..=sides to make sure we fill in the gap between the last location and first location, wrapping back round
            let angle = (i as f32 / sides as f32) * 2.0 * PI; // both i and sides must be cast to f32 to prevent integer division problems
            let new_location = self.location + vec2(self.radius * f32::cos(angle), self.radius * f32::sin(angle));
            vertices.append(&mut vec![self.location.x, self.location.y,         self.color.r(), self.color.g(), self.color.b(), self.color.a()]);
            vertices.append(&mut vec![previous_location.x, previous_location.y, self.color.r(), self.color.g(), self.color.b(), self.color.a()]);
            vertices.append(&mut vec![new_location.x, new_location.y,           self.color.r(), self.color.g(), self.color.b(), self.color.a()]);
            previous_location = new_location;
        }
        vertices
    }
}
