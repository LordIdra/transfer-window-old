use nalgebra_glm::DVec2;

use super::conic::transverse_velocity;

pub const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11;

#[derive(Debug, Clone, Copy)]
pub enum OrbitDirection {
    Clockwise,
    Anticlockwise,
}

impl OrbitDirection {
    pub fn from_position_and_velocity(position: DVec2, velocity: DVec2) -> Self {
        if transverse_velocity(position, velocity) > 0.0 {
            Self::Clockwise
        } else {
            Self::Anticlockwise
        }
    }
}
