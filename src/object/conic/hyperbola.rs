use std::f64::consts::PI;

use nalgebra_glm::{vec2, DVec2};

use crate::object::orbit_direction::{OrbitDirection, GRAVITATIONAL_CONSTANT};

use super::{argument_of_periapsis, Conic, specific_angular_momentum, solve_kepler_equation_for_hyperbola, semi_major_axis, eccentricity};

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
        let mut angle_since_periapsis = f64::atan2(position.y, position.x) - self.argument_of_periapsis;
        if let OrbitDirection::Clockwise = self.direction {
            angle_since_periapsis = -angle_since_periapsis
        }
        angle_since_periapsis
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
        let angle = true_anomaly + self.argument_of_periapsis;
        let radius = (self.semi_major_axis * (1.0 - self.eccentricity.powi(2))) / (1.0 + self.eccentricity * true_anomaly.cos());
        vec2(radius * angle.cos(), radius * angle.sin())
    }
    
    fn get_velocity(&self, _: DVec2, true_anomaly: f64) -> DVec2 {
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
        let speed_in_direction_of_position_vector = (self.reduced_mass / self.specific_angular_momentum) * self.eccentricity * true_anomaly.sin();
        let speed_perpendicular_to_position_vector = self.specific_angular_momentum / self.get_position(true_anomaly).magnitude();
        let mut a = speed_perpendicular_to_position_vector * f64::cos(true_anomaly + PI / 2.0);
        let mut b = speed_perpendicular_to_position_vector * f64::sin(true_anomaly + PI / 2.0);
        if let OrbitDirection::Clockwise = self.direction {
            a = -a;
            b = -b;
        }
        let v = DVec2::new(a + speed_in_direction_of_position_vector * true_anomaly.cos(), b + speed_in_direction_of_position_vector * true_anomaly.sin());
        println!("spd {} {} {}", speed_in_direction_of_position_vector, speed_perpendicular_to_position_vector, f64::sqrt(speed_in_direction_of_position_vector.powi(2) + speed_perpendicular_to_position_vector.powi(2)));
        println!("vel {} {}", DVec2::new(a, b), DVec2::new(speed_in_direction_of_position_vector * true_anomaly.cos(), speed_in_direction_of_position_vector * true_anomaly.sin()));
        v
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

#[test]
fn test() {
    // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
    //pub(in super) fn new(position: DVec2, velocity: DVec2, reduced_mass: f64, semi_major_axis: f64, eccentricity: f64, direction: OrbitDirection) -> Self {
    let position = vec2(6678100.0,  0.0);
    let velocity = vec2(0.0, 15000.0);
    let reduced_mass = GRAVITATIONAL_CONSTANT * 5.972e27;
    let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
    let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
    assert!(eccentricity - 2.7696 < 0.0001);
    let direction = OrbitDirection::from_position_and_velocity(position, velocity);
    assert_eq!(direction, OrbitDirection::Clockwise);
    let hyperbola = Hyperbola::new(position, velocity, reduced_mass, semi_major_axis, eccentricity, direction);
}