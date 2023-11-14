use std::fmt::Debug;

use nalgebra_glm::{vec2, DVec2};

use self::{ellipse::Ellipse, hyperbola::Hyperbola};

use super::orbit_direction::{OrbitDirection, GRAVITATIONAL_CONSTANT};

mod ellipse;
mod hyperbola;

// https://phys.libretexts.org/Bookshelves/Astronomy__Cosmology/Celestial_Mechanics_(Tatum)/09%3A_The_Two_Body_Problem_in_Two_Dimensions/9.08%3A_Orbital_Elements_and_Velocity_Vector#mjx-eqn-9.5.31
// https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/elliptical-orbits.html

pub fn transverse_velocity(position: DVec2, velocity: DVec2) -> f64 {
    // Component of velocity perpendicular to the displacement
    let angle = -f64::atan2(position.y, position.x);
    let normalized_velocity = vec2(velocity.x * angle.cos() - velocity.y * angle.sin(), velocity.y * angle.cos() + velocity.x * angle.sin());
    normalized_velocity.y
}

fn semi_major_axis(position: DVec2, velocity: DVec2, standard_gravitational_parameter: f64) -> f64 {
    ((2.0 / position.magnitude()) - (velocity.magnitude().powi(2) / standard_gravitational_parameter)).powi(-1)
}

fn eccentricity(position: DVec2, velocity: DVec2, standard_gravitational_parameter: f64, semi_major_axis: f64) -> f64 {
    (1.0 - ((position.magnitude_squared() * transverse_velocity(position, velocity).powi(2)) / (standard_gravitational_parameter * semi_major_axis))).sqrt()
}

fn argument_of_periapsis(position: DVec2, velocity: DVec2, standard_gravitational_parameter: f64) -> f64 {
    let eccentricity_vector = ((velocity.magnitude().powi(2) - standard_gravitational_parameter / position.magnitude()) * position - (position.dot(&velocity) * velocity)) / standard_gravitational_parameter;
    f64::atan2(eccentricity_vector.y, eccentricity_vector.x)
}

fn specific_angular_momentum(position: DVec2, velocity: DVec2) -> f64 {
    position.magnitude() * transverse_velocity(position, velocity)
}

pub fn new_conic(parent_mass: f64, position: DVec2, velocity: DVec2) -> Box<dyn Conic> {
    let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * parent_mass;
    let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
    let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
    let direction = OrbitDirection::from_position_and_velocity(position, velocity);
    if eccentricity <= 1.0 {
        Box::new(Ellipse::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction))
    } else {
        Box::new(Hyperbola::new(position, velocity, standard_gravitational_parameter, semi_major_axis, eccentricity, direction))
    }
}

// Describes all the static parmeters of an elliptic orbit, but says nothing about the current state of the object in the orbit
pub trait Conic: Debug + Send {
    fn get_theta_from_time_since_periapsis(&self, time: f64) -> f64;
    fn get_time_since_periapsis(&self, theta: f64) -> f64;
    fn get_position(&self, theta: f64) -> DVec2;
    fn get_velocity(&self, position: DVec2, theta: f64) -> DVec2;
    fn get_direction(&self) -> OrbitDirection;
    fn get_semi_major_axis(&self) -> f64;
    fn get_argument_of_periapsis(&self) -> f64;
    fn get_eccentricity(&self) -> f64;
    fn get_remaining_orbits(&self, remaining_time: f64) -> i32;
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    use nalgebra_glm::vec2;

    #[test]
    fn test_semi_major_axis() {
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/mercuryfact.html
        let position = vec2(6.9818e10 * f64::cos(PI / 6.0),  6.9818e10 * f64::sin(PI / 6.0));
        let velocity = vec2(3.886e4 * f64::cos(PI / 6.0 + PI / 2.0), 3.886e4 * f64::sin(PI / 6.0 + PI / 2.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.989e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        // actual SMA is slightly different due to N-body perturbations and the like
        assert!((semi_major_axis - 5.790375e10).abs() < 10000.0); 
    }

    #[test]
    fn test_eccentricity_1() {
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/mercuryfact.html
        let position = vec2(6.9818e10 * f64::cos(-PI / 6.0), 6.9818e10 * f64::sin(-PI / 6.0),);
        let velocity = vec2(3.886e4 * f64::cos(-PI / 6.0 + PI / 2.0), 3.886e4 * f64::sin(-PI / 6.0 + PI / 2.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.989e30;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        assert!((eccentricity - 0.2056).abs() < 0.001);
    }

    #[test]
    fn test_eccentricity_2() {
        // https://orbital-mechanics.space/time-since-periapsis-and-keplers-equation/hyperbolic-trajectory-example.html
        let position = vec2(6678100.0 * f64::cos(-PI / 6.0), 6678100.0 * f64::sin(-PI / 6.0));
        let velocity = vec2(15000.0 * f64::cos(-PI / 6.0 + PI / 2.0), 15000.0 * f64::sin(-PI / 6.0 + PI / 2.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.972e24;
        let semi_major_axis = semi_major_axis(position, velocity, standard_gravitational_parameter);
        let eccentricity = eccentricity(position, velocity, standard_gravitational_parameter, semi_major_axis);
        assert!((eccentricity - 2.7696).abs() < 0.001);
    }

    #[test]
    fn test_argument_of_periapsis_1() {
        // https://nssdc.gsfc.nasa.gov/planetary/factsheet/mercuryfact.html
        let position = vec2(6.9818e10 * f64::cos(-PI / 6.0), 6.9818e10 * f64::sin(-PI / 6.0),);
        let velocity = vec2(-3.886e4 * f64::cos(-PI / 6.0 + PI / 2.0), -3.886e4 * f64::sin(-PI / 6.0 + PI / 2.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 1.9895e30;
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter);
        let expected_argument_of_periapsis = -PI / 6.0 + PI;
        assert!((argument_of_periapsis - expected_argument_of_periapsis).abs() < 0.01);
    }

    #[test]
    fn test_argument_of_periapsis_2() {
        let position = vec2(0.4055e9 * f64::cos(PI/6.0), 0.4055e9 * f64::sin(PI/6.0));
        let velocity = vec2(0.570e3 * f64::cos(PI/6.0 + PI/2.0), 0.570e3 * f64::sin(PI/6.0 + PI/2.0));
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter);
        let expected_argument_of_periapsis = PI / 6.0 - PI;
        assert!((argument_of_periapsis - expected_argument_of_periapsis).abs() < 0.01);
    }

    #[test]
    fn test_argument_of_periapsis_3() {
        let position = vec2(369236029.3588132, 143598629.71966434);
        let velocity = vec2(47.79968959560202, -607.3920534306773);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter);
        let expected_argument_of_periapsis = PI / 6.0 - PI;
        assert!((argument_of_periapsis - expected_argument_of_periapsis).abs() < 0.01);
    }

    #[test]
    fn test_argument_of_periapsis_4() {
        let position = vec2(221244867.9581085, 278127601.0974563);
        let velocity = vec2(772.33035113478, -73.80334890759599);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter);
        let expected_argument_of_periapsis = PI / 6.0 - PI;
        assert!((argument_of_periapsis - expected_argument_of_periapsis).abs() < 0.01);
    }

    #[test]
    fn test_argument_of_periapsis_5() {
        let position = vec2(321699434.0757532, 238177462.81333557);
        let velocity = vec2(-448.8853759438255, 386.13875843572083);
        let standard_gravitational_parameter = GRAVITATIONAL_CONSTANT * 5.9722e24;
        let argument_of_periapsis = argument_of_periapsis(position, velocity, standard_gravitational_parameter);
        let expected_argument_of_periapsis = -2.615930001576588;
        assert!((argument_of_periapsis - expected_argument_of_periapsis).abs() < 0.01);
    }
}