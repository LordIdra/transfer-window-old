use std::f64::consts::PI;

use nalgebra_glm::{vec2, DVec2};

use crate::object::orbit_direction::OrbitDirection;

use super::{argument_of_periapsis, Conic, specific_angular_momentum};

fn solve_kepler_equation(eccentricity: f64, mean_anomaly: f64) -> f64 {
    let mut eccentric_anomaly = mean_anomaly;
    for _ in 0..1000 {
        eccentric_anomaly = eccentric_anomaly - (eccentricity * f64::sinh(eccentric_anomaly) - eccentric_anomaly - mean_anomaly) / (eccentricity * f64::cosh(eccentric_anomaly) - 1.0);
    }
    eccentric_anomaly
}

#[derive(Debug)]
pub struct Hyperbola {
    standard_gravitational_parameter: f64,
    semi_major_axis: f64,
    eccentricity: f64,
    direction: OrbitDirection,
    argument_of_periapsis: f64,
    specific_angular_momentum: f64,
}

impl Hyperbola {
    pub(in super) fn new(position: DVec2, velocity: DVec2, standard_gravitational_parameter: f64, semi_major_axis: f64, eccentricity: f64, direction: OrbitDirection) -> Self {
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter);
        let specific_angular_momentum = specific_angular_momentum(position, velocity);
        Hyperbola { standard_gravitational_parameter, semi_major_axis, eccentricity, argument_of_periapsis, direction, specific_angular_momentum }
    }
}

impl Conic for Hyperbola {
    fn get_theta_from_time_since_periapsis(&self, time_since_periapsis: f64) -> f64 {
        let x = self.standard_gravitational_parameter.powi(2) / self.specific_angular_momentum.powi(3);
        let mean_anomaly = x * time_since_periapsis * (self.eccentricity.powi(2) - 1.0).powf(3.0 / 2.0);
        let eccentric_anomaly = solve_kepler_equation(self.eccentricity, mean_anomaly);
        let mut true_anomaly = 2.0 * f64::atan(f64::sqrt((self.eccentricity + 1.0) / (self.eccentricity - 1.0)) * f64::tanh(eccentric_anomaly / 2.0));
        if let OrbitDirection::Clockwise = self.direction {
            true_anomaly = -true_anomaly;
        }
        if let OrbitDirection::Clockwise = self.direction {
            true_anomaly = -true_anomaly;
        }
        let mut theta = true_anomaly + self.argument_of_periapsis;
        while theta > PI {
            theta -= 2.0 * PI;
        }
        while theta < -PI {
            theta += 2.0 * PI;
        }
        theta
    }

    fn get_time_since_periapsis(&self, theta: f64) -> f64 {
        let true_anomaly = theta - self.argument_of_periapsis;
        let eccentric_anomaly = 2.0 * f64::atanh(f64::sqrt((self.eccentricity - 1.0) / (self.eccentricity + 1.0)) * f64::tan(true_anomaly / 2.0));
        let mut mean_anomaly = self.eccentricity * f64::sinh(eccentric_anomaly) - eccentric_anomaly;
        if let OrbitDirection::Clockwise = self.direction {
            mean_anomaly = -mean_anomaly;
        }
        let x = self.specific_angular_momentum.powi(3) / self.standard_gravitational_parameter.powi(2);
        mean_anomaly * x / (self.eccentricity.powi(2) - 1.0).powf(3.0 / 2.0)
    }

    fn get_position(&self, theta: f64) -> DVec2 {
        let true_anomaly = theta - self.argument_of_periapsis;
        let radius = (self.semi_major_axis * (1.0 - self.eccentricity.powi(2))) / (1.0 + self.eccentricity * true_anomaly.cos());
        vec2(radius * theta.cos(), radius * theta.sin())
    }
    
    fn get_velocity(&self, position: DVec2, theta: f64) -> DVec2 {
        let true_anomaly = theta - self.argument_of_periapsis;
        let radius = position.magnitude();
        let radius_derivative_with_respect_to_true_anomaly = self.semi_major_axis * self.eccentricity * (1.0 - self.eccentricity.powi(2)) * true_anomaly.sin()
            / (self.eccentricity * true_anomaly.cos() + 1.0).powi(2);
        let position_derivative_with_respect_to_true_anomaly = vec2(
            radius_derivative_with_respect_to_true_anomaly * theta.cos() - radius * theta.sin(), 
            radius_derivative_with_respect_to_true_anomaly * theta.sin() + radius * theta.cos());
        let angular_speed = self.specific_angular_momentum / radius.powi(2);
        position_derivative_with_respect_to_true_anomaly * angular_speed
    }

    fn get_direction(&self) -> OrbitDirection {
        self.direction
    }

    fn get_sphere_of_influence(&self, mass: f64, parent_mass: f64) -> f64 {
        self.semi_major_axis * (mass / parent_mass).powf(2.0 / 5.0)
    }

    fn get_remaining_orbits(&self, _: f64) -> i32 {
        0 // Hyperbola can never complete an entire orbit
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::object::{orbit_direction::GRAVITATIONAL_CONSTANT, conic::{semi_major_axis, eccentricity}};

    use super::*;

    #[test]
    fn test_time_from_true_anomaly() {
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::to_radians(100.0);
        let time = hyperbola.get_time_since_periapsis(theta);
        let expected_time = 1.15 * 60.0 * 60.0;
        assert!((time - expected_time).abs() < 3.0)
    }

    #[test]
    fn test_theta_from_time_since_periapsis_1() {
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let expected_theta = 0.0;
        let theta = hyperbola.get_theta_from_time_since_periapsis(0.0);
        assert!((theta - expected_theta).abs() < 0.01)
    }

    #[test]
    fn test_theta_from_time_since_periapsis_2() {
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let time = 4.15 * 60.0 * 60.0;
        let theta = hyperbola.get_theta_from_time_since_periapsis(time);
        let expected_theta = f64::to_radians(107.78);
        assert!((theta - expected_theta).abs() < 0.01);
    }

    #[test]
    fn test_theta_from_time_since_periapsis_3() {
        let position = vec2(-33839778.563934326, -31862122.134700775);
        let velocity = vec2(1187.3296202582328, 268.8766709200928);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 7.346e22;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let expected_theta = f64::atan2(position.y, position.x);
        let time_since_periapsis = hyperbola.get_time_since_periapsis(expected_theta);
        let theta = hyperbola.get_theta_from_time_since_periapsis(time_since_periapsis);
        assert!((theta - expected_theta).abs() < 0.01);
    }

    #[test]
    fn test_radius_from_true_anomaly() {
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::to_radians(107.78);
        let radius = hyperbola.get_position(theta).magnitude();
        let expected_radius = 1.63291969e08;
        assert!((radius - expected_radius).abs() < 1.0);
    }

    #[test]
    fn test_position_from_true_anomaly_1() {
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let new_position = hyperbola.get_position(0.0);
        let position_difference = new_position - position;
        assert!(position_difference.x.abs() < 0.1);
        assert!(position_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_position_from_true_anomaly_2() {
        let position = vec2(6678100.0 * f64::cos(PI / 4.0), 6678100.0 * f64::sin(PI / 4.0));
        let velocity = vec2(-15000.0 * f64::cos(PI / 4.0), 15000.0 * f64::sin(PI / 4.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let new_position = hyperbola.get_position(PI / 4.0);
        let position_difference = new_position - position;
        assert!(position_difference.x.abs() < 0.1);
        assert!(position_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_position_from_true_anomaly_3() {
        let position = vec2(-22992216.820260637, -41211039.67710246);
        let velocity = vec2(281.5681303192537, -961.5890730599444);
        let standard_gravitational_parameter = 4902720400000.0;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::atan2(position.y, position.x);
        let new_position = hyperbola.get_position(theta);
        let position_difference = new_position - position;
        assert!(position_difference.x.abs() < 0.1);
        assert!(position_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_velocity_from_true_anomaly_1() {
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = 0.0;
        let new_velocity = hyperbola.get_velocity(hyperbola.get_position(theta), theta);
        let velocity_difference = new_velocity - velocity;
        assert!(velocity_difference.x.abs() < 0.1);
        assert!(velocity_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_velocity_from_true_anomaly_2() {
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::to_radians(107.78);
        let expected_speed = 1.05126e4;
        let speed = hyperbola.get_velocity(hyperbola.get_position(theta), theta).magnitude();
        assert!((speed - expected_speed).abs() < 0.1);
    }

    #[test]
    fn test_velocity_from_true_anomaly_3() {
        let position = vec2(0.0, 6678100.0);
        let velocity = vec2(-15000.0, 0.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::to_radians(-107.78 + 90.0);
        let expected_speed = 1.05126e4;
        let speed = hyperbola.get_velocity(hyperbola.get_position(theta), theta).magnitude();
        assert!((speed - expected_speed).abs() < 0.5);
    }

    #[test]
    fn test_velocity_from_true_anomaly_4() {
        let position = vec2(6678100.0 * f64::cos(PI / 4.0), 6678100.0 * f64::sin(PI / 4.0));
        let velocity = vec2(-20000.0 * f64::cos(PI / 4.0), 20000.0 * f64::sin(PI / 4.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = PI / 4.0;
        let new_position = hyperbola.get_position(theta);
        let new_velocity = hyperbola.get_velocity(new_position, theta);
        let velocity_difference = new_velocity - velocity;
        assert!(velocity_difference.x.abs() < 0.1);
        assert!(velocity_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_velocity_from_true_anomaly_5() {
        let position = vec2(-22992216.820260637, -4211039.67710246);
        let velocity = vec2(1201.8989386523506, 73.28331093245788);
        let standard_gravitational_parameter = 4902720400000.0;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::atan2(position.y, position.x);
        let new_position = hyperbola.get_position(theta);
        let new_velocity = hyperbola.get_velocity(new_position, theta);
        let velocity_difference = new_velocity - velocity;
        assert!(velocity_difference.x.abs() < 0.1);
        assert!(velocity_difference.y.abs() < 0.1);
    }
}