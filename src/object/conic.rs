use std::{rc::Rc, cell::RefCell};

use eframe::epaint::{Vec2, vec2};

use super::{Object, SCALE_FACTOR};

const GRAVITATIONAL_CONSTANT: f32 = 6.674e-11;

pub struct Conic {
    parent: Rc<RefCell<Object>>,
    semi_major_axis: f32,
    eccentricity: f32,
    argument_of_periapsis: f32,
}

impl Conic {
    fn semi_major_axis(displacement: Vec2, velocity: Vec2, reduced_mass: f32) -> f32 {
        ((2.0 / displacement.length()) - (velocity.length().powi(2) / reduced_mass)).powi(-1)
    }

    fn transverse_velocity(position: Vec2, velocity: Vec2) -> f32 {
        // Component of velocity perpendicular to the displacement
        let perpendicular_to_displacement = vec2(position.y, position.x).normalized();
        let cos = perpendicular_to_displacement.dot(velocity) / (perpendicular_to_displacement.length() * velocity.length()).abs();
        velocity.length() * cos
    }

    fn eccentricity(position: Vec2, velocity: Vec2, reduced_mass: f32, semi_major_axis: f32) -> f32 {
        let transverse_velocity = Self::transverse_velocity(position, velocity);
        (1.0 - ((position.length_sq() * transverse_velocity.powi(2)) / (reduced_mass * semi_major_axis))).sqrt()
    }

    fn argument_of_periapsis(position: Vec2, velocity: Vec2, reduced_mass: f32, eccentricity: f32) -> f32 {
        let transverse_velocity = Self::transverse_velocity(position, velocity);
        position.angle() - ((((position.length() * transverse_velocity.powi(2)) / (reduced_mass)) - 1.0) / eccentricity).acos()
    }

    fn get_displacement(&self, angle: f32) -> Vec2 {
        let radius = (self.semi_major_axis * (1.0 - self.eccentricity.powi(2))) / (1.0 + self.eccentricity * (angle - self.argument_of_periapsis).cos());
        vec2(radius * angle.cos(), radius * angle.sin())
    }

    pub fn new(parent: Rc<RefCell<Object>>, position: Vec2, velocity: Vec2) -> Self {
        // https://phys.libretexts.org/Bookshelves/Astronomy__Cosmology/Celestial_Mechanics_(Tatum)/09%3A_The_Two_Body_Problem_in_Two_Dimensions/9.08%3A_Orbital_Elements_and_Velocity_Vector#mjx-eqn-9.5.31
        // Lots of the formulae come from here ^
        let reduced_mass = GRAVITATIONAL_CONSTANT * parent.borrow().mass;
        let semi_major_axis = Self::semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = Self::eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let argument_of_periapsis = Self::argument_of_periapsis(position, velocity, reduced_mass, eccentricity);
        println!("{} {:?} {}", semi_major_axis, eccentricity, argument_of_periapsis);
        Self { parent, semi_major_axis, eccentricity, argument_of_periapsis }
    }

    pub fn get_scaled_displacement(&self, angle: f32) -> Vec2 {
        self.get_displacement(angle) * SCALE_FACTOR
    }

    pub fn get_absolute_parent_position(&self) -> Vec2 {
        self.parent.borrow().get_absolute_position()
    }
}