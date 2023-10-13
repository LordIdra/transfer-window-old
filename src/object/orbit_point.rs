use nalgebra_glm::Vec2;

use super::orbit_description::EllipseDescription;

#[derive(Clone)]
pub struct OrbitPoint {
    angle_since_periapsis: f32,
    time_since_periapsis: f32,
    position: Vec2,
    velocity: Vec2,
}

impl OrbitPoint {
    pub fn new(orbit_description: &EllipseDescription, position: Vec2) -> Self {
        let angle_since_periapsis = orbit_description.get_angle_since_periapsis(position);
        let time_since_periapsis = orbit_description.get_time_since_periapsis_from_angle_since_periapsis(angle_since_periapsis);
        let velocity = orbit_description.get_velocity(position, angle_since_periapsis);
        Self { angle_since_periapsis, time_since_periapsis, position, velocity }
    }

    pub fn next(&self, orbit_description: &EllipseDescription, delta_time: f32) -> Self {
        let time_since_periapsis = self.time_since_periapsis + delta_time;
        let angle_since_periapsis = orbit_description.get_angle_since_periapsis_from_time_since_periapsis(time_since_periapsis);
        let position = orbit_description.get_position(angle_since_periapsis);
        let velocity = orbit_description.get_velocity(position, angle_since_periapsis);
        Self { angle_since_periapsis, time_since_periapsis, position, velocity }
    }

    pub fn get_angle_since_periapsis(&self) -> f32 {
        self.angle_since_periapsis
    }

    pub fn get_unscaled_position(&self) -> Vec2 {
        self.position
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.velocity
    }
}