use nalgebra_glm::{vec2, DVec2};

use crate::object::orbit_direction::OrbitDirection;

use super::{argument_of_periapsis, Conic, specific_angular_momentum, solve_kepler_equation_for_hyperbola};

#[derive(Debug)]
pub struct Hyperbola {
    reduced_mass: f64,
    semi_major_axis: f64,
    eccentricity: f64,
    direction: OrbitDirection,
    argument_of_periapsis: f64,
    specific_angular_momentum: f64,
}

impl Hyperbola {
    pub(in super) fn new(position: DVec2, velocity: DVec2, reduced_mass: f64, semi_major_axis: f64, eccentricity: f64, direction: OrbitDirection) -> Self {
        let argument_of_periapsis = argument_of_periapsis(position, velocity, reduced_mass, eccentricity);
        let specific_angular_momentum = specific_angular_momentum(position, velocity);
        Hyperbola { reduced_mass, semi_major_axis, eccentricity, argument_of_periapsis, direction, specific_angular_momentum }
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
        let x = self.reduced_mass.powi(2) / self.specific_angular_momentum.powi(3);
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
        let x = self.specific_angular_momentum.powi(3) / self.reduced_mass.powi(2);
        mean_anomaly * x / (self.eccentricity.powi(2) - 1.0).powf(3.0 / 2.0)
    }

    fn get_position(&self, true_anomaly: f64) -> DVec2 {
        let mean_anomaly = true_anomaly - self.argument_of_periapsis;
        let radius = (self.semi_major_axis * (1.0 - self.eccentricity.powi(2))) / (1.0 + self.eccentricity * mean_anomaly.cos());
        vec2(radius * true_anomaly.cos(), radius * true_anomaly.sin())
    }
    
    fn get_velocity(&self, position: DVec2, true_anomaly: f64) -> DVec2 {
        // THIS FUNCTION IS WHERE THIS STUPID BUG STEMS FROM
        // TODO 
        // DANGER
        // VELOCITY DIRECTION IS NOT BEING COMPUTED CORRECTLY. MAGNITUDE IS CORRECT
        /*let speed = position.magnitude() * f64::sqrt(self.reduced_mass / (self.semi_major_axis.powi(3) * (1.0 - self.eccentricity.powi(2)).powi(3))) * (1.0 + (self.eccentricity * f64::cos(true_anomaly))).powi(2);
        println!("spd{}", speed);
        let mut velocity_unit = vec2(-position.y, position.x).normalize();
        if let OrbitDirection::Anticlockwise = self.direction {
            velocity_unit *= -1.0;
        }
        speed * velocity_unit*/
        /*let radius = (self.semi_major_axis * (1.0 - self.eccentricity.powi(2))) / (1.0 + self.eccentricity * true_anomaly.cos());
        let radius_derivative = self.semi_major_axis * (1.0 - self.eccentricity.powi(2)) * (self.eccentricity * true_anomaly.sin()) / (1.0 + self.eccentricity * true_anomaly.cos()).powi(2);
        let a = self.argument_of_periapsis.cos() * true_anomaly.cos() - self.argument_of_periapsis.sin() * true_anomaly.sin();
        let b = self.argument_of_periapsis.cos() * true_anomaly.sin() + self.argument_of_periapsis.sin() * true_anomaly.cos();
        DVec2::new(radius_derivative * a - radius * b, radius_derivative * b + radius * a)*/
        //f64::sqrt(self.reduced_mass * ((2.0 / position.magnitude()) - (1.0 / self.semi_major_axis)))
        let mean_anomaly = true_anomaly - self.argument_of_periapsis;
        let radial_speed = (self.reduced_mass / self.specific_angular_momentum) * self.eccentricity * mean_anomaly.sin();
        let normal_speed = self.specific_angular_momentum / position.magnitude();
        let radial_direction = position.normalize();
        let mut normal_direction = vec2(-radial_direction.y, radial_direction.x);
        if let OrbitDirection::Clockwise = self.direction {
            normal_direction = -normal_direction;
        }
        let v = (radial_speed * radial_direction) + (normal_speed * normal_direction);
        println!("scp {} {}", radial_speed, normal_speed);
        println!("spd {}", v.magnitude());
        println!("vel {}", v);
        v
        //let speed = f64::sqrt(self.reduced_mass * ((2.0 / position.magnitude()) - (1.0 / self.semi_major_axis)));
        //let eccentric_anomaly = eccentric_anomaly(self.eccentricity, true_anomaly);
        // let intermediate_value = f64::sqrt(1.0 + self.eccentricity.powi(2) + 2.0 * self.eccentricity * true_anomaly.cos());
        // let velocity_unit = vec2(-f64::sin(true_anomaly) / intermediate_value, (self.eccentricity + f64::cos(true_anomaly)) / intermediate_value);
        // println!("{}", velocity_unit);
        //speed * vec2(f64::cos(eccentric_anomaly), f64::sin(eccentric_anomaly))
    }

    fn get_sphere_of_influence(&self, mass: f64, parent_mass: f64) -> f64 {
        self.semi_major_axis * (mass / parent_mass).powf(2.0 / 5.0)
    }

    fn get_direction(&self) -> OrbitDirection {
        self.direction
    }

    fn debug(&self) {
        println!("rm {}", self.reduced_mass);
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
        let reduced_mass = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, reduced_mass, semi_major_axis, eccentricity, direction);
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
        let reduced_mass = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, reduced_mass, semi_major_axis, eccentricity, direction);
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
        let reduced_mass = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, reduced_mass, semi_major_axis, eccentricity, direction);
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
        let reduced_mass = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, reduced_mass, semi_major_axis, eccentricity, direction);
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
        let reduced_mass = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, reduced_mass, semi_major_axis, eccentricity, direction);
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
        let reduced_mass = 4902720400000.0; // moon reduced mass
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, reduced_mass, semi_major_axis, eccentricity, direction);
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
        let reduced_mass = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, reduced_mass, semi_major_axis, eccentricity, direction);
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
        let reduced_mass = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, reduced_mass, semi_major_axis, eccentricity, direction);
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
        let reduced_mass = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, reduced_mass, semi_major_axis, eccentricity, direction);
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
        let reduced_mass = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, reduced_mass, semi_major_axis, eccentricity, direction);
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
        let reduced_mass = 4902720400000.0; // moon reduced mass
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let hyperbola = Hyperbola::new(position, velocity, reduced_mass, semi_major_axis, eccentricity, direction);
        let true_anomaly = f64::atan2(position.y, position.x);
        let new_position = hyperbola.get_position(true_anomaly);
        let new_velocity = hyperbola.get_velocity(new_position, true_anomaly);
        let velocity_difference = new_velocity - velocity;
        assert!(velocity_difference.x.abs() < 0.1);
        assert!(velocity_difference.y.abs() < 0.1);
    }
}