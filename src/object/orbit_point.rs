use nalgebra_glm::Vec2;

use super::scary_maths::{self, OrbitDirection};

impl OrbitDirection {
    pub fn from_position_and_velocity(position: Vec2, velocity: Vec2) -> Self {
        if scary_maths::transverse_velocity(position, velocity) > 0.0 {
            Self::Clockwise
        } else {
            Self::Anticlockwise
        }
    }
}

pub struct OrbitPoint {
    angle_since_periapsis: f32,
    time_since_periapsis: f32,
    position: Vec2,
}

impl OrbitPoint {
    pub fn new(semi_major_axis: f32, eccentricity: f32, period: f32, argument_of_periapsis: f32, direction: OrbitDirection, position: Vec2) -> Self {
        let angle_since_periapsis = scary_maths::angle_since_periapsis(position, direction, argument_of_periapsis);
        let time_since_periapsis = scary_maths::time_since_periapsis_from_angle_since_periapsis(eccentricity, period, angle_since_periapsis);
        let position = scary_maths::position(argument_of_periapsis, semi_major_axis, eccentricity, angle_since_periapsis);
        Self { angle_since_periapsis, time_since_periapsis, position }
    }

    pub fn next(&self, semi_major_axis: f32, eccentricity: f32, argument_of_periapsis: f32, period: f32, direction: OrbitDirection, delta_time: f32) -> Self {
        let time_since_periapsis = self.time_since_periapsis + delta_time;
        let angle_since_periapsis = scary_maths::angle_since_periapsis_from_time_since_periapsis(eccentricity, period, direction, time_since_periapsis);
        let position = scary_maths::position(argument_of_periapsis, semi_major_axis, eccentricity, angle_since_periapsis);
        Self { angle_since_periapsis, time_since_periapsis, position }
    }

    pub fn get_angle_since_periapsis(&self) -> f32 {
        self.angle_since_periapsis
    }

    pub fn get_unscaled_position(&self) -> Vec2 {
        self.position
    }
}