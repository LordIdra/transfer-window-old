use std::f64::consts::PI;

use nalgebra_glm::{vec2, DVec2};
use rand::Rng;

use crate::components::trajectory_component::segment::orbit::{orbit_direction::OrbitDirection, orbit_point::OrbitPoint};

use super::{argument_of_periapsis, Conic, specific_angular_momentum, copysign};

fn solve_kepler_equation(eccentricity: f64, mean_anomaly: f64, start_offset: f64) -> f64 {
    let max_delta_squared = (1.0e-5_f64).powi(2);
    let max_attempts = 500;
    let mut eccentric_anomaly = mean_anomaly + start_offset;
    let mut attempts = 0;
    for _ in 0..1000 {
        let delta = -(eccentricity * f64::sinh(eccentric_anomaly) - eccentric_anomaly - mean_anomaly) / (eccentricity * f64::cosh(eccentric_anomaly) - 1.0);
        if delta.powi(2) < max_delta_squared {
            break;
        }
        if attempts > max_attempts {
            // Try with different start value
            let mut rng = rand::thread_rng();
            let start_offset = (rng.gen::<f64>() - 0.5) * 5.0;
            return solve_kepler_equation(eccentricity, mean_anomaly, start_offset)
        }
        eccentric_anomaly += delta;
        attempts += 1;
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
        let eccentric_anomaly = solve_kepler_equation(self.eccentricity, mean_anomaly, 0.0);
        let true_anomaly = 2.0 * f64::atan(f64::sqrt((self.eccentricity + 1.0) / (self.eccentricity - 1.0)) * f64::tanh(eccentric_anomaly / 2.0));
        let theta = true_anomaly + self.argument_of_periapsis;
        let theta = theta % (2.0 * PI);
        if theta < 0.0 {
            theta + 2.0 * PI
        } else {
            theta
        }
    }

    fn get_time_since_periapsis(&self, theta: f64) -> f64 {
        let true_anomaly = theta - self.argument_of_periapsis;
        let eccentric_anomaly = 2.0 * f64::atanh(f64::sqrt((self.eccentricity - 1.0) / (self.eccentricity + 1.0)) * f64::tan(true_anomaly / 2.0));
        let mean_anomaly = self.eccentricity * f64::sinh(eccentric_anomaly) - eccentric_anomaly;
        let x = self.specific_angular_momentum.powi(3) / self.standard_gravitational_parameter.powi(2);
        mean_anomaly * x / (self.eccentricity.powi(2) - 1.0).powf(3.0 / 2.0)
    }

    fn get_time_since_last_periapsis(&self, orbit_point: &OrbitPoint) -> f64 {
        orbit_point.get_time_since_periapsis()
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

    fn get_semi_major_axis(&self) -> f64 {
        self.semi_major_axis
    }

    fn get_semi_minor_axis(&self) -> f64 {
        self.semi_major_axis * f64::sqrt(self.eccentricity.powi(2) - 1.0)
    }

    fn get_argument_of_periapsis(&self) -> f64 {
        self.argument_of_periapsis
    }

    fn get_eccentricity(&self) -> f64 {
        self.eccentricity
    }

    fn get_remaining_orbits(&self, _: f64) -> i32 {
        0 // Hyperbola can never complete an entire orbit
    }

    /// This solver actually kinda... doesn't work...
    /// But as we get closer to the line the solution gets more accurate
    /// So this actually works fine for our purposes despite being broken (lol)
    fn solve_for_closest_point(&self, p: DVec2) -> DVec2 {
        let px = f64::abs(p[0]);
        let py = f64::abs(p[1]);

        let a = self.semi_major_axis;
        let b = self.get_semi_minor_axis();
    
        let mut t = -0.05;
    
        for _ in 0..80 {
            let x = -a * f64::cosh(t);
            let y = -b * f64::sinh(t);
    
            let ex =  (a*a + b*b) * f64::cosh(t).powi(3) / a;
            let ey = -(b*b + a*a) * f64::sinh(t).powi(3) / b;
    
            let rx = x - ex;
            let ry = y - ey;
    
            let qx = px - ex;
            let qy = py - ey;
    
            let r = vec2(ry, rx).magnitude();
            let q = vec2(qy, qx).magnitude();
    
            let delta_c = r * f64::asinh((rx*qy - ry*qx)/(r*q));
            let delta_t = delta_c / f64::sqrt(a.powi(2) * f64::sinh(t).powi(2) + b.powi(2) * f64::cosh(t).powi(2));
    
            t += delta_t;
        }
    
        vec2(copysign(a * f64::cosh(t), p[0]), copysign(b * f64::sinh(t), p[1]))
    }

    fn is_time_between_points(&self, start: &OrbitPoint, end: &OrbitPoint, time: f64) -> bool {
        time > start.get_time() && time < end.get_time()
    }

    fn get_period(&self) -> Option<f64> {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::components::trajectory_component::segment::orbit::{orbit_direction::GRAVITATIONAL_CONSTANT, conic::{semi_major_axis, eccentricity}};

    use super::*;

    #[test]
    fn test_time_from_true_anomaly_1() {
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
    fn test_time_from_true_anomaly_2() {
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::to_radians(-100.0);
        let time = hyperbola.get_time_since_periapsis(theta);
        let expected_time = -1.15 * 60.0 * 60.0;
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
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let time = -4.15 * 60.0 * 60.0;
        let theta = hyperbola.get_theta_from_time_since_periapsis(time);
        let expected_theta = f64::to_radians(-107.78);
        assert!((theta - expected_theta).abs() < 0.01);
    }

    #[test]
    fn test_theta_from_time_since_periapsis_4() {
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