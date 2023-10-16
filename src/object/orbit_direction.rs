use nalgebra_glm::DVec2;

use super::conic::transverse_velocity;

pub const GRAVITATIONAL_CONSTANT: f64 = 6.674e-11;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OrbitDirection {
    AntiClockwise,
    Clockwise,
}

impl OrbitDirection {
    pub fn from_position_and_velocity(position: DVec2, velocity: DVec2) -> Self {
        if transverse_velocity(position, velocity) > 0.0 {
            Self::AntiClockwise
        } else {
            Self::Clockwise
        }
    }
}

#[test]
fn test() {
    use nalgebra_glm::vec2;

    // https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html
    assert_eq!(OrbitDirection::from_position_and_velocity(vec2(1.0, 0.0), vec2(0.0, 1.0)), OrbitDirection::AntiClockwise);
    assert_eq!(OrbitDirection::from_position_and_velocity(vec2(1.0, 0.0), vec2(0.0, -1.0)), OrbitDirection::Clockwise);
    assert_eq!(OrbitDirection::from_position_and_velocity(vec2(1.0, 1.0), vec2(0.0, 1.0)), OrbitDirection::AntiClockwise);
    assert_eq!(OrbitDirection::from_position_and_velocity(vec2(1.0, 1.0), vec2(0.0, -1.0)), OrbitDirection::Clockwise);
    assert_eq!(OrbitDirection::from_position_and_velocity(vec2(-1.0, 1.0), vec2(0.0, -1.0)), OrbitDirection::AntiClockwise);
    assert_eq!(OrbitDirection::from_position_and_velocity(vec2(-1.0, 1.0), vec2(0.0, 1.0)), OrbitDirection::Clockwise);
    assert_eq!(OrbitDirection::from_position_and_velocity(vec2(-0.2, 1.0), vec2(1.0, 0.0)), OrbitDirection::Clockwise);
    assert_eq!(OrbitDirection::from_position_and_velocity(vec2(-1.0, 1.0), vec2(-1.0, 0.0)), OrbitDirection::AntiClockwise);
}