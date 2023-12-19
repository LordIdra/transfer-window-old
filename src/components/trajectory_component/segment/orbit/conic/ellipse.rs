use std::f64::consts::PI;

use nalgebra_glm::{vec2, DVec2};
use rand::Rng;

use crate::components::trajectory_component::segment::orbit::{orbit_direction::OrbitDirection, orbit_point::OrbitPoint};

use super::{argument_of_periapsis, Conic, specific_angular_momentum, copysign};

fn period(standard_gravitational_parameter: f64, semi_major_axis: f64) -> f64 {
    2.0 * PI * f64::sqrt(semi_major_axis.powi(3) / standard_gravitational_parameter)
}
fn solve_kepler_equation(eccentricity: f64, mean_anomaly: f64, start_offset: f64) -> f64 {
    let max_delta_squared = (1.0e-7_f64).powi(2);
    let max_attempts = 500;
    // Choosing an initial seed: https://www.aanda.org/articles/aa/full_html/2022/02/aa41423-21/aa41423-21.html#S5
    // Yes, they're actually serious about that 0.999999 thing (lmao)
    let mut eccentric_anomaly = mean_anomaly + start_offset
        + (0.999999 * 4.0 * eccentricity * mean_anomaly * (PI - mean_anomaly))
        / (8.0 * eccentricity * mean_anomaly + 4.0 * eccentricity * (eccentricity - PI) + PI.powi(2));
    let mut attempts = 0;
    loop {
        let delta = -(eccentric_anomaly - eccentricity * f64::sin(eccentric_anomaly) - mean_anomaly) / (1.0 - eccentricity * f64::cos(eccentric_anomaly));
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
pub struct Ellipse {
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
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter);
        let specific_angular_momentum = specific_angular_momentum(position, velocity);
        Ellipse { semi_major_axis, eccentricity, period, argument_of_periapsis, direction, specific_angular_momentum }
    }
}

impl Conic for Ellipse {
    fn get_theta_from_time_since_periapsis(&self, time_since_periapsis: f64) -> f64 {
        let mean_anomaly = (2.0 * PI * time_since_periapsis) / self.period;
        let eccentric_anomaly = solve_kepler_equation(self.eccentricity, mean_anomaly, 0.0);
        let mut true_anomaly = 2.0 * f64::atan(f64::sqrt((1.0 + self.eccentricity) / (1.0 - self.eccentricity)) * f64::tan(eccentric_anomaly / 2.0));
        // The sign of atan flips halfway through the orbit
        // So we need to add 2pi halfway through the orbit to keep things consistent
        if let OrbitDirection::Clockwise = self.direction {
            true_anomaly = -true_anomaly;
        }
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
        let eccentric_anomaly = 2.0 * f64::atan(f64::sqrt((1.0 - self.eccentricity) / (1.0 + self.eccentricity)) * f64::tan(true_anomaly / 2.0));
        let mut mean_anomaly = eccentric_anomaly - self.eccentricity * f64::sin(eccentric_anomaly);
        if let OrbitDirection::Clockwise = self.direction {
            mean_anomaly = -mean_anomaly;
        }
        mean_anomaly * self.period / (2.0 * PI)
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
        // Alternative method which seemed to have some slight errors, I never quite figured it out
        /*
            let mean_anomaly = true_anomaly - self.argument_of_periapsis;
            let radial_speed = (self.standard_gravitational_parameter / self.specific_angular_momentum) * self.eccentricity * mean_anomaly.sin();
            let normal_speed = self.specific_angular_momentum / position.magnitude();
            let radial_direction = position.normalize();
            let mut normal_direction = vec2(-radial_direction.y, radial_direction.x);
            if let OrbitDirection::Clockwise = self.direction {
                normal_direction = -normal_direction;
            }
            (radial_speed * radial_direction) + (normal_speed * normal_direction)
        */

        let true_anomaly = theta - self.argument_of_periapsis;
        let radius = position.magnitude();
        let radius_derivative_with_respect_to_theta = self.semi_major_axis * self.eccentricity * (1.0 - self.eccentricity.powi(2)) * true_anomaly.sin()
            / (self.eccentricity * true_anomaly.cos() + 1.0).powi(2);
        let position_derivative_with_respect_to_theta = vec2(
            radius_derivative_with_respect_to_theta * theta.cos() - radius * theta.sin(), 
            radius_derivative_with_respect_to_theta * theta.sin() + radius * theta.cos());
        let angular_speed = self.specific_angular_momentum / radius.powi(2);
        position_derivative_with_respect_to_theta * angular_speed
    }

    fn get_direction(&self) -> OrbitDirection {
        self.direction
    }

    fn get_semi_major_axis(&self) -> f64 {
        self.semi_major_axis
    }

    fn get_semi_minor_axis(&self) -> f64 {
        self.semi_major_axis * f64::sqrt(1.0 - self.eccentricity.powi(2))
    }

    fn get_argument_of_periapsis(&self) -> f64 {
        self.argument_of_periapsis
    }

    fn get_eccentricity(&self) -> f64 {
        self.eccentricity
    }

    fn get_remaining_orbits(&self, remaining_time: f64) -> i32 {
        (remaining_time / self.period) as i32
    }

    /// https://stackoverflow.com/a/46007540
    fn solve_for_closest_point(&self, p: DVec2) -> DVec2 {
        let px = f64::abs(p[0]);
        let py = f64::abs(p[1]);

        let a = self.semi_major_axis;
        let b = self.get_semi_minor_axis();

        let mut t = PI / 4.0;

        for _ in 0..80 {
            let x = a * f64::cos(t);
            let y = b * f64::sin(t);

            let ex = (a*a - b*b) * f64::cos(t).powi(3) / a;
            let ey = (b*b - a*a) * f64::sin(t).powi(3) / b;

            let rx = x - ex;
            let ry = y - ey;

            let qx = px - ex;
            let qy = py - ey;

            let r = vec2(ry, rx).magnitude();
            let q = vec2(qy, qx).magnitude();

            let delta_c = r * f64::asin((rx*qy - ry*qx) / (r*q));
            let delta_t = delta_c / f64::sqrt(a.powi(2) * f64::sin(t).powi(2) + b.powi(2) * f64::cos(t).powi(2));

            t += delta_t;
            t = f64::min(PI / 2.0, f64::max(0.0, t))
        }

        vec2(copysign(a * f64::cos(t), p[0]), copysign(b * f64::sin(t), p[1]))
    }

    fn is_time_between_points(&self, start: &OrbitPoint, end: &OrbitPoint, time: f64) -> bool {
        time > start.get_time() && time < end.get_time()
    }

    fn get_period(&self) -> Option<f64> {
        Some(self.period)
    }
}

#[cfg(test)]
mod tests {

    use crate::components::trajectory_component::segment::orbit::{orbit_direction::GRAVITATIONAL_CONSTANT, conic::{semi_major_axis, eccentricity}};

    use super::*;

    #[test]
    fn test_period_1() {
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/earthfact.html
        let position = vec2(1.52100e11, 0.0);
        let velocity = vec2(0.0, 2.929e4);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.9895e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let period = period(standard_gravitational_parameter, semi_major_axis) / (60.0 * 60.0 * 24.0);
        let expected_period = 364.9;
        assert!((period - expected_period).abs() < 0.1);
    }

    #[test]
    fn test_period_2() {
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/mercuryfact.html
        let position = vec2(6.9818e10, 0.0);
        let velocity = vec2(0.0, 3.886e4);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.9895e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let period = period(standard_gravitational_parameter, semi_major_axis) / (60.0 * 60.0 * 24.0);
        let expected_period = 87.969;
        assert!((period - expected_period).abs() < 0.1);
    }

    #[test]
    fn test_period_3() {
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/mercuryfact.html
        let position = vec2(6.9818e10 * f64::cos(-PI / 6.0), 6.9818e10 * f64::sin(-PI / 6.0));
        let velocity = vec2(3.886e4 * f64::cos(-PI / 6.0 + PI / 2.0), 3.886e4 * f64::sin(-PI / 6.0 + PI / 2.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.9895e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let period = period(standard_gravitational_parameter, semi_major_axis) / (60.0 * 60.0 * 24.0);
        let expected_period = 87.969;
        assert!((period - expected_period).abs() < 0.1);
    }
    
    #[test]
    fn test_time_since_periapsis_from_theta() {
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = 1.53000e7;
        let eccentricity = 0.3725;
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::to_radians(120.0);
        let time = ellipse.get_time_since_periapsis(theta);
        let expected_time = 1.13 * 60.0 * 60.0;
        assert!((time - expected_time).abs() < 30.0);
    }

    #[test]
    fn test_theta_from_time_since_periapsis_1() {
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = 1.53000e7;
        let eccentricity = 0.3725;
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let time = 3.0 * 3600.0;
        let theta = ellipse.get_theta_from_time_since_periapsis(time);
        let expected_theta = f64::to_radians(193.16 - 360.0);
        assert!((theta - expected_theta).abs() < 0.01);
    }

    #[test]
    fn test_theta_from_time_since_periapsis_2() {
        let position = vec2(-83760632.16012573, -305649596.3836937);
        let velocity = vec2(-929.2507297680404, 1168.0344669650149);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let expected_theta = f64::atan2(position.y, position.x);
        let time = ellipse.get_time_since_periapsis(expected_theta);
        let theta = ellipse.get_theta_from_time_since_periapsis(time);
        assert!((theta - expected_theta).abs() < 0.01);
    }

    #[test]
    fn test_position_from_true_anomaly_1() {
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
        assert!(position_difference.x.abs() < 5000.0);
        assert!(position_difference.y.abs() < 0.1);
    }

    #[test]
    fn test_position_from_true_anomaly_2() {
        let position = vec2(321699434.0757532, 238177462.81333557);
        let velocity = vec2(-448.8853759438255, 386.13875843572083);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = 0.6373110791759163;
        let new_position = ellipse.get_position(theta);
        let position_difference = new_position - position;
        assert!(position_difference.x.abs() < 0.01);
        assert!(position_difference.y.abs() < 0.01);
    }

    #[test]
    fn test_velocity_from_true_anomaly_1() {
        let position = vec2(1.52100e11,  0.0);
        let velocity = vec2(0.0, 2.929e4);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.988500e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = PI;
        let new_position = ellipse.get_position(theta);
        let new_velocity = ellipse.get_velocity(new_position, theta);
        let expected_velocity = vec2(0.0, -3.029e4);
        let velocity_difference = new_velocity - expected_velocity;
        assert!(velocity_difference.x.abs() < 0.01);
        assert!(velocity_difference.y.abs() < 10.0);
    }

    #[test]
    fn test_velocity_from_true_anomaly_2() {
        let position = vec2(234851481.38196197, 174455271.78610012);
        let velocity = vec2(-250.6798696407834, 817.5591126812552);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let ellipse = Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction);
        let theta = f64::atan2(position.y, position.x);
        let new_position = ellipse.get_position(theta);
        let new_velocity = ellipse.get_velocity(new_position, theta);
        let velocity_difference = new_velocity - velocity;
        assert!(velocity_difference.x.abs() < 0.01);
        assert!(velocity_difference.y.abs() < 0.01);
    }
}