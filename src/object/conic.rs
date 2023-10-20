use std::{f64::consts::PI, fmt::Debug};

use nalgebra_glm::{vec2, DVec2};

use self::{ellipse::Ellipse, hyperbola::Hyperbola};

use super::orbit_direction::{OrbitDirection, GRAVITATIONAL_CONSTANT};

mod ellipse;
mod hyperbola;

// https://phys.libretexts.org/Bookshelves/Astronomy__Cosmology/Celestial_Mechanics_(Tatum)/09%3A_The_Two_Body_Problem_in_Two_Dimensions/9.08%3A_Orbital_Elements_and_Velocity_Vector#mjx-eqn-9.5.31
// https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/elliptical-orbits.html

pub fn transverse_velocity(position: DVec2, velocity: DVec2) -> f64 {  // TODO I'm very suspicious about this. Is it correct?
    // Component of velocity perpendicular to the displacement
    let angle = -f64::atan2(position.y, position.x);
    let normalized_velocity = vec2(velocity.x * angle.cos() - velocity.y * angle.sin(), velocity.y * angle.cos() + velocity.x * angle.sin());
    normalized_velocity.y
    
    //let perpendicular_to_displacement = vec2(position.y, position.x).normalize();
    //let cos = perpendicular_to_displacement.dot(&velocity) / (perpendicular_to_displacement.magnitude() * velocity.magnitude());
    //velocity.magnitude() * cos
}


fn semi_major_axis(position: DVec2, velocity: DVec2, reduced_mass: f64) -> f64 {
    ((2.0 / position.magnitude()) - (velocity.magnitude().powi(2) / reduced_mass)).powi(-1)
}

fn eccentricity(position: DVec2, velocity: DVec2, reduced_mass: f64, semi_major_axis: f64) -> f64 {
    (1.0 - ((position.magnitude_squared() * transverse_velocity(position, velocity).powi(2)) / (reduced_mass * semi_major_axis))).sqrt()
}

fn period(reduced_mass: f64, semi_major_axis: f64) -> f64 {
    2.0 * PI * f64::sqrt(semi_major_axis.powi(3) / reduced_mass)
}

fn argument_of_periapsis(position: DVec2, velocity: DVec2, reduced_mass: f64, eccentricity: f64) -> f64 {
    let mut x = ((position.magnitude() * transverse_velocity(position, velocity).powi(2) / reduced_mass) - 1.0) / eccentricity;
    // Make sure x is between -1 and 1; sometimes it will go slightly out of bounds due to floating point errors
    x = f64::min(x, 1.0);
    x = f64::max(x, -1.0);
    f64::atan2(position.y, position.x) + x.acos()
}

fn solve_kepler_equation_for_ellipse(eccentricity: f64, mean_anomaly: f64) -> f64 {
    let mut eccentric_anomaly = mean_anomaly;
    for _ in 0..1000 {
        eccentric_anomaly = mean_anomaly + eccentricity * f64::sin(eccentric_anomaly);
    }
    eccentric_anomaly
}

fn solve_kepler_equation_for_hyperbola(eccentricity: f64, mean_anomaly: f64) -> f64 {
    let mut eccentric_anomaly = mean_anomaly;
    for _ in 0..1000 {
        eccentric_anomaly = eccentric_anomaly - (eccentricity * f64::sinh(eccentric_anomaly) - eccentric_anomaly - mean_anomaly) / (eccentricity * f64::cosh(eccentric_anomaly) - 1.0);
    }
    eccentric_anomaly
}

fn specific_angular_momentum(position: DVec2, velocity: DVec2) -> f64 {
    position.magnitude() * velocity.magnitude()
}

fn eccentric_anomaly(eccentricity: f64, true_anomaly: f64) -> f64 {
    2.0 * f64::atan(f64::sqrt((eccentricity + 1.0) / (eccentricity - 1.0)) * f64::tan(true_anomaly / 2.0))
}

pub fn new_conic(parent_mass: f64, position: DVec2, velocity: DVec2) -> Box<dyn Conic> {
    let reduced_mass = GRAVITATIONAL_CONSTANT * parent_mass;
    let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
    let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
    let direction = OrbitDirection::from_position_and_velocity(position, velocity);
    println!("{}", semi_major_axis);
    if eccentricity <= 1.0 {
        Box::new(Ellipse::new(position, velocity, reduced_mass, semi_major_axis, eccentricity, direction))
    } else {
        Box::new(Hyperbola::new(position, velocity, reduced_mass, semi_major_axis, eccentricity, direction))
    }
}

// Describes all the static parmeters of an elliptic orbit, but says nothing about the current state of the object in the orbit
pub trait Conic: Debug + Send {
    fn get_true_anomaly_from_position(&self, position: DVec2) -> f64;
    fn get_true_anomaly_from_time_since_periapsis(&self, time: f64) -> f64;
    fn get_time_since_periapsis(&self, true_anomaly: f64) -> f64;
    fn get_position(&self, true_anomaly: f64) -> DVec2;
    fn get_velocity(&self, position: DVec2, true_anomaly: f64) -> DVec2;
    fn get_sphere_of_influence(&self, mass: f64, parent_mass: f64) -> f64;
    fn get_direction(&self) -> OrbitDirection;
    fn debug(&self);
}

#[cfg(test)]
mod tests {
    use super::*;

    use nalgebra_glm::vec2;

    use crate::object::orbit_direction::GRAVITATIONAL_CONSTANT;

    #[test]
    fn test_semi_major_axis() {
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/mercuryfact.html
        let position = vec2(6.9818e10 * f64::cos(PI / 6.0),  6.9818e10 * f64::sin(PI / 6.0));
        let velocity = vec2(3.886e4 * f64::cos(PI / 6.0 + PI / 2.0), 3.886e4 * f64::sin(PI / 6.0 + PI / 2.0));
        let reduced_mass = GRAVITATIONAL_CONSTANT * 1.989e30;
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        // actual SMA is slightly different due to N-body perturbations and the like
        assert!((semi_major_axis - 5.790375e10).abs() < 10000.0); 
    }

    #[test]
    fn test_eccentricity_elliptical() {
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/mercuryfact.html
        let position = vec2(6.9818e10 * f64::cos(-PI / 6.0), 6.9818e10 * f64::sin(-PI / 6.0),);
        let velocity = vec2(3.886e4 * f64::cos(-PI / 6.0 + PI / 2.0), 3.886e4 * f64::sin(-PI / 6.0 + PI / 2.0));
        let reduced_mass = GRAVITATIONAL_CONSTANT * 1.989e30;
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
        assert!((eccentricity - 0.2056).abs() < 0.001);
    }

    #[test]
    fn test_eccentricity_hyperbolic() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
        let position = vec2(6678100.0 * f64::cos(-PI / 6.0), 6678100.0 * f64::sin(-PI / 6.0));
        let velocity = vec2(15000.0 * f64::cos(-PI / 6.0 + PI / 2.0), 15000.0 * f64::sin(-PI / 6.0 + PI / 2.0));
        let reduced_mass = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = eccentricity(position, velocity, reduced_mass, semi_major_axis);
        assert!((eccentricity - 2.7696).abs() < 0.001);
    }

    #[test]
    fn test_direction() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
        let position = vec2(6678100.0,  0.0);
        let velocity = vec2(0.0, 15000.0);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        assert_eq!(direction, OrbitDirection::AntiClockwise);
    }
}