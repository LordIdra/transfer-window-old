use std::f64::consts::PI;

use nalgebra_glm::{vec2, DVec2};

use crate::object::orbit_direction::OrbitDirection;

use super::{period, argument_of_periapsis, Conic, solve_kepler_equation_for_ellipse, eccentric_anomaly};

#[derive(Debug)]
pub struct Ellipse {
    reduced_mass: f64,
    semi_major_axis: f64,
    eccentricity: f64,
    direction: OrbitDirection,
    period: f64,
    argument_of_periapsis: f64,
}

impl Ellipse {
    pub(in super) fn new(position: DVec2, velocity: DVec2, reduced_mass: f64, semi_major_axis: f64, eccentricity: f64, direction: OrbitDirection) -> Self {
        let period = period(reduced_mass, semi_major_axis);
        let argument_of_periapsis = argument_of_periapsis(position, velocity, reduced_mass, eccentricity);
        Ellipse { reduced_mass, semi_major_axis, eccentricity, period, argument_of_periapsis, direction }
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
        let mut time = self.period * (true_anomaly - adjusted_true_anomaly) / (2.0 * PI);
        let eccentric_anomaly = 2.0 * f64::atan(f64::sqrt((1.0 - self.eccentricity) / (1.0 + self.eccentricity)) * f64::tan(true_anomaly / 2.0));
        let mean_anomaly = eccentric_anomaly - self.eccentricity * f64::sin(eccentric_anomaly);
        time += mean_anomaly * self.period / (2.0 * PI);
        time
    }

    fn get_position(&self, true_anomaly: f64) -> DVec2 {
        let angle = true_anomaly + self.argument_of_periapsis;
        let radius = (self.semi_major_axis * (1.0 - self.eccentricity.powi(2))) / (1.0 + self.eccentricity * true_anomaly.cos());
        vec2(radius * angle.cos(), radius * angle.sin())
    }
    
    fn get_velocity(&self, position: DVec2, true_anomaly: f64) -> DVec2 {
        // // todo this is wrong
        // let speed = position.magnitude() * f64::sqrt(self.reduced_mass / (self.semi_major_axis.powi(3) * (1.0 - self.eccentricity.powi(2)).powi(3))) * (1.0 + (self.eccentricity * f64::cos(true_anomaly))).powi(2);
        // let mut velocity_unit = vec2(-position.y, position.x).normalize();
        // if let OrbitDirection::Clockwise = self.direction {
        //     velocity_unit *= -1.0;
        // }
        // speed * velocity_unit
        let speed = f64::sqrt(self.reduced_mass * ((2.0 / position.magnitude()) - (1.0 / self.semi_major_axis)));
        let eccentric_anomaly = eccentric_anomaly(self.eccentricity, true_anomaly);
        // let intermediate_value = f64::sqrt(1.0 + self.eccentricity.powi(2) + 2.0 * self.eccentricity * true_anomaly.cos());
        // let velocity_unit = vec2(-f64::sin(true_anomaly) / intermediate_value, (self.eccentricity + f64::cos(true_anomaly)) / intermediate_value);
        speed * vec2(f64::cos(eccentric_anomaly), f64::sin(eccentric_anomaly))
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
        println!("aop {}", self.period);
        println!("aop {}", self.argument_of_periapsis);
    }
}