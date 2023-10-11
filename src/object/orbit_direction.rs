use nalgebra_glm::Vec2;

use super::orbit_description::transverse_velocity;

pub const GRAVITATIONAL_CONSTANT: f32 = 6.674e-11;

#[derive(Debug, Clone, Copy)]
pub enum OrbitDirection {
    Clockwise,
    Anticlockwise,
}

impl OrbitDirection {
    pub fn from_position_and_velocity(position: Vec2, velocity: Vec2) -> Self {
        if transverse_velocity(position, velocity) > 0.0 {
            Self::Clockwise
        } else {
            Self::Anticlockwise
        }
    }
}
