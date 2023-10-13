use std::f32::consts::PI;

use nalgebra_glm::{Vec2, vec2};

use super::orbit_direction::{OrbitDirection, GRAVITATIONAL_CONSTANT};

// https://phys.libretexts.org/Bookshelves/Astronomy__Cosmology/Celestial_Mechanics_(Tatum)/09%3A_The_Two_Body_Problem_in_Two_Dimensions/9.08%3A_Orbital_Elements_and_Velocity_Vector#mjx-eqn-9.5.31
// https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/elliptical-orbits.html

pub fn transverse_velocity(position: Vec2, velocity: Vec2) -> f32 {
    // Component of velocity perpendicular to the displacement
    let perpendicular_to_displacement = vec2(position.y, position.x).normalize();
    let cos = perpendicular_to_displacement.dot(&velocity) / (perpendicular_to_displacement.magnitude() * velocity.magnitude());
    velocity.magnitude() * cos
}


fn semi_major_axis(displacement: Vec2, velocity: Vec2, reduced_mass: f32) -> f32 {
    ((2.0 / displacement.magnitude()) - (velocity.magnitude().powi(2) / reduced_mass)).powi(-1)
}

fn eccentricity(position: Vec2, velocity: Vec2, reduced_mass: f32, semi_major_axis: f32) -> f32 {
    (1.0 - ((position.magnitude_squared() * transverse_velocity(position, velocity).powi(2)) / (reduced_mass * semi_major_axis))).sqrt()
}

fn period(reduced_mass: f32, semi_major_axis: f32) -> Option<f32> {
    let period = 2.0 * PI * f32::sqrt(semi_major_axis.powi(3) / reduced_mass);
    if period.is_nan() {
        None // Hyperbola (eccentricity > 1)
    } else {
        Some(period)
    }
}

fn argument_of_periapsis(position: Vec2, velocity: Vec2, reduced_mass: f32, eccentricity: f32) -> f32 {
    let mut x = ((position.magnitude() * transverse_velocity(position, velocity).powi(2) / reduced_mass) - 1.0) / eccentricity;
    // Make sure x is between -1 and 1; sometimes it will go slightly out of bounds due to floating point errors
    x = f32::min(x, 1.0);
    x = f32::max(x, -1.0);
    f32::atan2(position.y, position.x) - x.acos()
}

fn solve_kepler_equation(eccentricity: f32, mean_anomaly: f32) -> f32 {
    let mut eccentric_anomaly = mean_anomaly;
    for _ in 0..100 {
        eccentric_anomaly = mean_anomaly + eccentricity * f32::sin(eccentric_anomaly);
    }
    eccentric_anomaly
}

pub trait ConicDescription {
    fn new(mass: f32, position: Vec2, velocity: Vec2) -> Box<dyn ConicDescription> {
        let reduced_mass = GRAVITATIONAL_CONSTANT * mass;
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
    }
}

// Describes all the static parmeters of an elliptic orbit, but says nothing about the current state of the object in the orbit
#[derive(Debug)]
pub struct EllipseDescription {
    reduced_mass: f32,
    semi_major_axis: f32,
    eccentricity: f32,
    period: Option<f32>,
    argument_of_periapsis: f32,
    direction: OrbitDirection,
}

impl EllipseDescription {
    pub fn new(mass: f32, position: Vec2, velocity: Vec2) -> Self {
        let reduced_mass = GRAVITATIONAL_CONSTANT * mass;
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let period = period(reduced_mass, semi_major_axis);
        let argument_of_periapsis = argument_of_periapsis(position, velocity, reduced_mass, eccentricity);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        EllipseDescription { reduced_mass, semi_major_axis, eccentricity, period, argument_of_periapsis, direction }
    }

    pub fn get_angle_since_periapsis(&self, position: Vec2) -> f32 {
        let mut angle_since_periapsis = f32::atan2(position.y, position.x) - self.argument_of_periapsis;
        if let OrbitDirection::Anticlockwise = self.direction {
            angle_since_periapsis = -angle_since_periapsis
        }
        angle_since_periapsis
    }

    pub fn get_angle_since_periapsis_from_time_since_periapsis(&self, time: f32) -> f32 {
        let mean_anomaly = (2.0 * PI * time) / self.period;
        let eccentric_anomaly = solve_kepler_equation(self.eccentricity, mean_anomaly);
        let mut theta = 2.0 * f32::atan(f32::sqrt((1.0 + self.eccentricity) / (1.0 - self.eccentricity)) * f32::tan(eccentric_anomaly / 2.0));
        if let OrbitDirection::Anticlockwise = self.direction {
            theta = -theta;
        }
        theta
    }

    pub fn get_time_since_periapsis_from_angle_since_periapsis(&self, theta: f32) -> f32 {
        let new_theta = theta % (2.0 * PI);
        let mut time = self.period * (theta - new_theta) / (2.0 * PI);
        let mut eccentric_anomaly = 2.0 * f32::atan(f32::sqrt((1.0 - self.eccentricity) / (1.0 + self.eccentricity)) * f32::tan(theta / 2.0));
        // stop time from being negative
        if eccentric_anomaly < 0.0 {
            eccentric_anomaly += 2.0 * PI;
        }

        let mean_anomaly = eccentric_anomaly - self.eccentricity * f32::sin(eccentric_anomaly);
        time += mean_anomaly * self.period / (2.0 * PI);
        time
    }

    pub fn get_position(&self, angle_since_periapsis: f32) -> Vec2 {
        let angle = angle_since_periapsis + self.argument_of_periapsis;
        let radius = (self.semi_major_axis * (1.0 - self.eccentricity.powi(2))) / (1.0 + self.eccentricity * angle_since_periapsis.cos());
        vec2(radius * angle.cos(), radius * angle.sin())
    }
    
    pub fn get_velocity(&self, position: Vec2, angle_since_periapsis: f32) -> Vec2 {
        let speed = position.magnitude() * f32::sqrt(self.reduced_mass / (self.semi_major_axis.powi(3) * (1.0 - self.eccentricity.powi(2)).powi(3))) * (1.0 + (self.eccentricity * f32::cos(angle_since_periapsis))).powi(2);
        let mut velocity_unit = vec2(-position.y, position.x).normalize();
        if let OrbitDirection::Anticlockwise = self.direction {
            velocity_unit *= -1.0;
        }
        speed * velocity_unit
    }

    pub fn get_sphere_of_influence(&self, mass: f32, parent_mass: f32) -> f32 {
        self.semi_major_axis * (mass / parent_mass).powf(2.0 / 5.0)
    }

    pub fn get_direction(&self) -> OrbitDirection {
        self.direction
    }
}