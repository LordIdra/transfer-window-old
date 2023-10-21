use nalgebra_glm::{vec2, DVec2};

use crate::object::orbit_direction::OrbitDirection;

use super::{argument_of_periapsis, Conic, specific_angular_momentum, solve_kepler_equation_for_hyperbola};

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
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter, eccentricity, direction);
        let specific_angular_momentum = specific_angular_momentum(position, velocity);
        Hyperbola { standard_gravitational_parameter, semi_major_axis, eccentricity, argument_of_periapsis, direction, specific_angular_momentum }
    }
}

impl Conic for Hyperbola {
    fn get_true_anomaly_from_position(&self, position: DVec2) -> f64 {
        let mut true_anomaly = f64::atan2(position.y, position.x) - self.argument_of_periapsis;
        if let OrbitDirection::Clockwise = self.direction {
            true_anomaly = -true_anomaly
        }
        true_anomaly
    }

    fn get_true_anomaly_from_time_since_periapsis(&self, time: f64) -> f64 {
        // Can't take the easy route as we do with ellipses, since we don't have a period
        // So we can use a trick with angular momentum instead
        // Note we have to do some operations with f64s because the numbers just get too big for f64s...
        let x = self.standard_gravitational_parameter.powi(2) / self.specific_angular_momentum.powi(3);
        let mean_anomaly = x * time * (self.eccentricity.powi(2) - 1.0).powf(3.0 / 2.0);
        let eccentric_anomaly = solve_kepler_equation_for_hyperbola(self.eccentricity, mean_anomaly);
        let mut true_anomaly = 2.0 * f64::atan(f64::sqrt((self.eccentricity - 1.0) / (self.eccentricity + 1.0)) * f64::tanh(eccentric_anomaly / 2.0));
        if let OrbitDirection::Clockwise = self.direction {
            true_anomaly = -true_anomaly;
        }
        true_anomaly
    }

    fn get_time_since_periapsis(&self, true_anomaly: f64) -> f64 {
        let eccentric_anomaly = 2.0 * f64::atanh(f64::sqrt((self.eccentricity - 1.0) / (self.eccentricity + 1.0)) * f64::tan(true_anomaly / 2.0));
        let mut mean_anomaly = self.eccentricity * f64::sinh(eccentric_anomaly) - eccentric_anomaly;
        if let OrbitDirection::Clockwise = self.direction {
            mean_anomaly = -mean_anomaly;
        }
        // Can't take the easy route as we do with ellipses, since we don't have a period
        // So we can use a trick with angular momentum instead.
        let x = self.specific_angular_momentum.powi(3) / self.standard_gravitational_parameter.powi(2);
        mean_anomaly * x / (self.eccentricity.powi(2) - 1.0).powf(3.0 / 2.0)
    }

    fn get_position(&self, true_anomaly: f64) -> DVec2 {
        let mean_anomaly = true_anomaly - self.argument_of_periapsis;
        let radius = (self.semi_major_axis * (1.0 - self.eccentricity.powi(2))) / (1.0 + self.eccentricity * mean_anomaly.cos());
        vec2(radius * true_anomaly.cos(), radius * true_anomaly.sin())
    }
    
    fn get_velocity(&self, position: DVec2, true_anomaly: f64) -> DVec2 {
        let mean_anomaly = true_anomaly - self.argument_of_periapsis;
        let radius = position.magnitude();
        let radius_derivative_with_respect_to_true_anomaly = self.semi_major_axis * self.eccentricity * (1.0 - self.eccentricity.powi(2)) * mean_anomaly.sin()
            / (self.eccentricity * mean_anomaly.cos() + 1.0).powi(2);
        let position_derivative_with_respect_to_true_anomaly = vec2(
            radius_derivative_with_respect_to_true_anomaly * true_anomaly.cos() - radius * true_anomaly.sin(), 
            radius_derivative_with_respect_to_true_anomaly * true_anomaly.sin() + radius * true_anomaly.cos());
        //let speed = f64::sqrt(self.standard_gravitational_parameter * ((2.0 / position.magnitude()) - (1.0 / self.semi_major_axis)));
        //let angular_speed = speed / radius;
        let angular_speed = self.specific_angular_momentum / radius.powi(2);
        position_derivative_with_respect_to_true_anomaly * angular_speed
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
        println!("aop {}", self.argument_of_periapsis);
        println!("sam {}", self.specific_angular_momentum);
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::object::{orbit_direction::GRAVITATIONAL_CONSTANT, conic::{semi_major_axis, eccentricity}};

    use super::*;

    #[test]
    fn test_time_from_true_anomaly() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let true_anomaly = f64::to_radians(100.0);
        let time = hyperbola.get_time_since_periapsis(true_anomaly);
        let expected_time = 1.15 * 60.0 * 60.0;
        assert!((time - expected_time).abs() < 3.0)
    }

    #[test]
    fn test_true_anomaly_from_time() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let time = 4.15 * 60.0 * 60.0;
        let true_anomaly = hyperbola.get_true_anomaly_from_time_since_periapsis(time);
        let expected_true_anomaly = f64::to_radians(107.78);
        assert!((true_anomaly - expected_true_anomaly).abs() < 3.0);
    }

    #[test]
    fn test_radius_from_true_anomaly() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let true_anomaly = f64::to_radians(107.78);
        let radius = hyperbola.get_position(true_anomaly).magnitude();
        let expected_radius = 1.63291969e08; // site has a slightly different number but I think this stems from rounding eccentricity etc
        assert!((radius - expected_radius).abs() < 1.0);
    }

    #[test]
    fn test_position_from_true_anomaly_1() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
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
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
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
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
        let position = vec2(-22992216.820260637, -41211039.67710246);
        let velocity = vec2(281.5681303192537, -961.5890730599444);
        let standard_gravitational_parameter = 4902720400000.0; // moon reduced mass
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let true_anomaly = f64::atan2(position.y, position.x);
        let new_position = hyperbola.get_position(true_anomaly);
        let position_difference = new_position - position;
        assert!(position_difference.x.abs() < 0.1);
        assert!(position_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_velocity_from_true_anomaly_1() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let true_anomaly = 0.0; // TODO the position isn't correct!!!!!!! true anomaly should be pi / 2!!!!!!!!!!!!!!!1
        let new_velocity = hyperbola.get_velocity(hyperbola.get_position(true_anomaly), true_anomaly);
        let velocity_difference = new_velocity - velocity;
        assert!(velocity_difference.x.abs() < 0.1);
        assert!(velocity_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_velocity_from_true_anomaly_2() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let true_anomaly = f64::to_radians(107.78);
        let expected_speed = 1.05126e4;
        let speed = hyperbola.get_velocity(hyperbola.get_position(true_anomaly), true_anomaly).magnitude();
        assert!((speed - expected_speed).abs() < 0.1);
    }

    #[test]
    fn test_velocity_from_true_anomaly_3() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
        let position = vec2(0.0, 6678100.0);
        let velocity = vec2(-15000.0, 0.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let true_anomaly = f64::to_radians(-107.78 + 90.0);
        let expected_speed = 1.05126e4;
        let speed = hyperbola.get_velocity(hyperbola.get_position(true_anomaly), true_anomaly).magnitude();
        assert!((speed - expected_speed).abs() < 0.5);
    }

    #[test]
    fn test_velocity_from_true_anomaly_4() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
        let position = vec2(6678100.0 * f64::cos(PI / 4.0), 6678100.0 * f64::sin(PI / 4.0));
        let velocity = vec2(-20000.0 * f64::cos(PI / 4.0), 20000.0 * f64::sin(PI / 4.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let true_anomaly = PI / 4.0;
        let new_position = hyperbola.get_position(true_anomaly);
        let new_velocity = hyperbola.get_velocity(new_position, true_anomaly);
        let velocity_difference = new_velocity - velocity;
        assert!(velocity_difference.x.abs() < 0.1);
        assert!(velocity_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_velocity_from_true_anomaly_5() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
        let position = vec2(-22992216.820260637, -4211039.67710246);
        let velocity = vec2(1201.8989386523506, 73.28331093245788);
        let standard_gravitational_parameter = 4902720400000.0; // moon reduced mass
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let true_anomaly = f64::atan2(position.y, position.x);
        let new_position = hyperbola.get_position(true_anomaly);
        let new_velocity = hyperbola.get_velocity(new_position, true_anomaly);
        let velocity_difference = new_velocity - velocity;
        assert!(velocity_difference.x.abs() < 0.1);
        assert!(velocity_difference.y.abs() < 0.1);
    }
}