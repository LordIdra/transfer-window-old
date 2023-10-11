use std::f32::consts::PI;

use nalgebra_glm::{Vec2, vec2};

// https://phys.libretexts.org/Bookshelves/Astronomy__Cosmology/Celestial_Mechanics_(Tatum)/09%3A_The_Two_Body_Problem_in_Two_Dimensions/9.08%3A_Orbital_Elements_and_Velocity_Vector#mjx-eqn-9.5.31
// Lots of the formulae come from here ^

#[derive(Clone, Copy)]
pub enum OrbitDirection {
    Clockwise,
    Anticlockwise,
}

pub fn semi_major_axis(displacement: Vec2, velocity: Vec2, reduced_mass: f32) -> f32 {
    ((2.0 / displacement.magnitude()) - (velocity.magnitude().powi(2) / reduced_mass)).powi(-1)
}

pub fn eccentricity(position: Vec2, velocity: Vec2, reduced_mass: f32, semi_major_axis: f32) -> f32 {
    (1.0 - ((position.magnitude_squared() * transverse_velocity(position, velocity).powi(2)) / (reduced_mass * semi_major_axis))).sqrt()
}

pub fn argument_of_periapsis(position: Vec2, velocity: Vec2, reduced_mass: f32, eccentricity: f32) -> f32 {
    f32::atan2(position.y, position.x) - (((position.magnitude() * transverse_velocity(position, velocity).powi(2) / reduced_mass) - 1.0) / eccentricity).acos()
}

pub fn period(reduced_mass: f32, semi_major_axis: f32) -> f32 {
    2.0 * PI * f32::sqrt(semi_major_axis.powi(3) / reduced_mass)
}

pub fn position(argument_of_periapsis: f32, semi_major_axis: f32, eccentricity: f32, angle_since_periapsis: f32) -> Vec2 {
    let angle = angle_since_periapsis + argument_of_periapsis;
    let radius = (semi_major_axis * (1.0 - eccentricity.powi(2))) / (1.0 + eccentricity * angle_since_periapsis.cos());
    vec2(radius * angle.cos(), radius * angle.sin())
}

pub fn transverse_velocity(position: Vec2, velocity: Vec2) -> f32 {
    // Component of velocity perpendicular to the displacement
    let perpendicular_to_displacement = vec2(position.y, position.x).normalize();
    let cos = perpendicular_to_displacement.dot(&velocity) / (perpendicular_to_displacement.magnitude() * velocity.magnitude());
    velocity.magnitude() * cos
}

pub fn angle_since_periapsis(position: Vec2, direction: OrbitDirection, argument_of_periapsis: f32) -> f32 {
    let mut angle_since_periapsis = f32::atan2(position.y, position.x) - argument_of_periapsis;
    if let OrbitDirection::Anticlockwise = direction {
        angle_since_periapsis = -angle_since_periapsis
    }
    angle_since_periapsis
}

pub fn time_since_periapsis_from_angle_since_periapsis(eccentricity: f32, period: f32, theta: f32) -> f32 {
    let new_theta = theta % (2.0 * PI);
    let mut time = period * (theta - new_theta) / (2.0 * PI);
    let mut eccentric_anomaly = 2.0 * f32::atan(f32::sqrt((1.0 - eccentricity) / (1.0 + eccentricity)) * f32::tan(theta / 2.0));
    // stop time from being negative
    if eccentric_anomaly < 0.0 {
        eccentric_anomaly += 2.0 * PI;
    }

    let mean_anomaly = eccentric_anomaly - eccentricity * f32::sin(eccentric_anomaly);
    time += mean_anomaly * period / (2.0 * PI);
    time
}

fn solve_kepler_equation(eccentricity: f32, mean_anomaly: f32) -> f32 {
    let mut eccentric_anomaly = mean_anomaly;
    for _ in 0..100 {
        eccentric_anomaly = mean_anomaly + eccentricity * f32::sin(eccentric_anomaly);
    }
    eccentric_anomaly
}

pub fn angle_since_periapsis_from_time_since_periapsis(eccentricity: f32, period: f32, direction: OrbitDirection, time: f32) -> f32 {
    let mean_anomaly = (2.0 * PI * time) / period;
    let eccentric_anomaly = solve_kepler_equation(eccentricity, mean_anomaly);
    let mut theta = 2.0 * f32::atan(f32::sqrt((1.0 + eccentricity) / (1.0 - eccentricity)) * f32::tan(eccentric_anomaly / 2.0));
    if let OrbitDirection::Anticlockwise = direction {
        theta = -theta;
    }
    theta
}