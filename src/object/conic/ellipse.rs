use std::f64::consts::PI;

use nalgebra_glm::{vec2, DVec2};

use crate::object::orbit_direction::OrbitDirection;

use super::{period, argument_of_periapsis, Conic, solve_kepler_equation_for_ellipse, specific_angular_momentum};

#[derive(Debug)]
pub struct Ellipse {
    standard_gravitational_parameter: f64,
    semi_major_axis: f64,
    eccentricity: f64,
    direction: OrbitDirection,
    period: f64,
    argument_of_periapsis: f64,
    specific_angular_momentum: f64,
}

impl Ellipse {
    pub(in super) fn new(position: DVec2, velocity: DVec2, standard_gravitational_parameter: f64, semi_major_axis: f64, eccentricity: f64, direction: OrbitDirection) -> Self {
        let period = period(standard_gravitational_parameter, semi_major_axis);
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter, eccentricity, direction);
        let specific_angular_momentum = specific_angular_momentum(position, velocity);
        Ellipse { standard_gravitational_parameter, semi_major_axis, eccentricity, period, argument_of_periapsis, direction, specific_angular_momentum }
    }
}

impl Conic for Ellipse {
    fn get_true_anomaly_from_position(&self, position: DVec2) -> f64 {
        let mut true_anomaly = f64::atan2(position.y, position.x) - self.argument_of_periapsis;
        if let OrbitDirection::Clockwise = self.direction {
            true_anomaly = -true_anomaly
        }
        true_anomaly
    }

    fn get_true_anomaly_from_time_since_periapsis(&self, time: f64) -> f64 {
        let mean_anomaly = (2.0 * PI * time) / self.period;
        let eccentric_anomaly = solve_kepler_equation_for_ellipse(self.eccentricity, mean_anomaly);
        let mut true_anomaly = 2.0 * f64::atan(f64::sqrt((1.0 + self.eccentricity) / (1.0 - self.eccentricity)) * f64::tan(eccentric_anomaly / 2.0));
        // The reason we add 0.5 here is kind of weird; the sign of atan flips halfway through the orbit
        // So we need to add 2pi halfway through the orbit to keep things consistent
        true_anomaly += (time / self.period + 0.5).floor() * 2.0 * PI;
        if let OrbitDirection::Clockwise = self.direction {
            true_anomaly = -true_anomaly;
        }
        true_anomaly
    }

    fn get_time_since_periapsis(&self, true_anomaly: f64) -> f64 {
        let adjusted_true_anomaly = true_anomaly % (2.0 * PI);
        let time = self.period * (true_anomaly - adjusted_true_anomaly) / (2.0 * PI);
        let eccentric_anomaly = 2.0 * f64::atan(f64::sqrt((1.0 - self.eccentricity) / (1.0 + self.eccentricity)) * f64::tan(true_anomaly / 2.0));
        let mean_anomaly = eccentric_anomaly - self.eccentricity * f64::sin(eccentric_anomaly);
        time + mean_anomaly * self.period / (2.0 * PI)
    }

    fn get_position(&self, true_anomaly: f64) -> DVec2 {
        let mean_anomaly = true_anomaly + self.argument_of_periapsis;
        let radius = (self.semi_major_axis * (1.0 - self.eccentricity.powi(2))) / (1.0 + self.eccentricity * mean_anomaly.cos());
        vec2(radius * true_anomaly.cos(), radius * true_anomaly.sin())
    }
    
    fn get_velocity(&self, position: DVec2, true_anomaly: f64) -> DVec2 {
        // // todo this is wrong
        // let speed = position.magnitude() * f64::sqrt(self.standard_gravitational_parameter / (self.semi_major_axis.powi(3) * (1.0 - self.eccentricity.powi(2)).powi(3))) * (1.0 + (self.eccentricity * f64::cos(true_anomaly))).powi(2);
        // let mut velocity_unit = vec2(-position.y, position.x).normalize();
        // if let OrbitDirection::Clockwise = self.direction {
        //     velocity_unit *= -1.0;
        // }
        // speed * velocity_unit
        // let speed = f64::sqrt(self.standard_gravitational_parameter * ((2.0 / position.magnitude()) - (1.0 / self.semi_major_axis)));
        // let eccentric_anomaly = eccentric_anomaly(self.eccentricity, true_anomaly);
        // let intermediate_value = f64::sqrt(1.0 + self.eccentricity.powi(2) + 2.0 * self.eccentricity * true_anomaly.cos());
        // let velocity_unit = vec2(-f64::sin(true_anomaly) / intermediate_value, (self.eccentricity + f64::cos(true_anomaly)) / intermediate_value);
        // speed * vec2(f64::cos(eccentric_anomaly), f64::sin(eccentric_anomaly))
        let mean_anomaly = true_anomaly - self.argument_of_periapsis;
        let radial_speed = (self.standard_gravitational_parameter / self.specific_angular_momentum) * self.eccentricity * mean_anomaly.sin();
        let normal_speed = self.specific_angular_momentum / position.magnitude();
        let radial_direction = position.normalize();
        let mut normal_direction = vec2(-radial_direction.y, radial_direction.x);
        if let OrbitDirection::Clockwise = self.direction {
            normal_direction = -normal_direction;
        }
        (radial_speed * radial_direction) + (normal_speed * normal_direction)
    }

    fn get_sphere_of_influence(&self, mass: f64, parent_mass: f64) -> f64 {
        self.semi_major_axis * (mass / parent_mass).powf(2.0 / 5.0)
    }

    fn get_direction(&self) -> OrbitDirection {
        self.direction
    }

    fn debug(&self) {
        println!("rm {}", self.standard_gravitational_parameter);
        println!("sma {}", self.semi_major_axis);
        println!("ecc {}", self.eccentricity);
        println!("dir {:?}", self.direction);
        println!("aop {}", self.period);
        println!("aop {}", self.argument_of_periapsis);
    }
}

#[cfg(test)]
mod tests {
    use crate::object::{orbit_direction::GRAVITATIONAL_CONSTANT, conic::{semi_major_axis, eccentricity}};

    use super::*;
    
    #[test]
    fn test_time_from_true_anomaly() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/elliptical-orbits.html
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = 1.53000e7;
        let eccentricity = 0.3725;
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let true_anomaly = f64::to_radians(120.0);
        let time = ellipse.get_time_since_periapsis(true_anomaly);
        let expected_time = 1.13 * 60.0 * 60.0;
        assert!((time - expected_time).abs() < 30.0);
    }

    #[test]
    fn true_anomaly_from_time() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/elliptical-orbits.html
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = 1.53000e7;
        let eccentricity = 0.3725;
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let time = 3.0 * 3600.0;
        let true_anomaly = ellipse.get_true_anomaly_from_time_since_periapsis(time);
        let expected_true_anomaly = f64::to_radians(193.16);
        assert!((true_anomaly - expected_true_anomaly).abs() < 30.0);
    }

    #[test]
    fn test_position_from_true_anomaly() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/elliptical-orbits.html
        let position = vec2(1.52100e11,  0.0);
        let velocity = vec2(0.0, 2.929e4);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.988500e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let true_anomaly = PI;
        let new_position = ellipse.get_position(true_anomaly);
        let expected_position = vec2(-1.470834e11, 0.0);
        let position_difference = new_position - expected_position;
        assert!(position_difference.x < 10000.0);
        assert!(position_difference.y < 10000.0);
    }

    #[test]
    fn test_velocity_from_true_anomaly() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/elliptical-orbits.html
        let position = vec2(1.52100e11,  0.0);
        let velocity = vec2(0.0, 2.929e4);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.988500e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let true_anomaly = PI;
        let new_position = ellipse.get_position(true_anomaly);
        let new_velocity = ellipse.get_velocity(new_position, true_anomaly);
        let expected_velocity = vec2(0.0, -3.029e4);
        let velocity_difference = new_velocity - expected_velocity;
        assert!(velocity_difference.x < 10.0);
        assert!(velocity_difference.y < 10.0);
    }
}