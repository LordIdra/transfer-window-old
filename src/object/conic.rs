use std::{rc::Rc, cell::RefCell, f32::consts::PI};

use eframe::epaint::Rgba;
use nalgebra_glm::{Vec3, Vec2, vec2, vec3};

use super::{Object, SCALE_FACTOR, orbit_point::OrbitPoint};

const GRAVITATIONAL_CONSTANT: f32 = 6.674e-11;
const ORBIT_POINTS: usize = 256;
const ORBIT_PATH_MAX_ALPHA: f32 = 1.0;
const ORBIT_PATH_RADIUS_DELTA: f32 = 0.6;

pub struct Conic {
    parent: Rc<RefCell<Object>>,
    color: Vec3,
    semi_major_axis: f32,
    semi_minor_axis: f32,
    eccentricity: f32,
    argument_of_periapsis: f32,
    areal_velocity: f32,
    current_angle: f32,
}

impl Conic {
    pub fn new(parent: Rc<RefCell<Object>>, color: Vec3, position: Vec2, velocity: Vec2) -> Self {
        // https://phys.libretexts.org/Bookshelves/Astronomy__Cosmology/Celestial_Mechanics_(Tatum)/09%3A_The_Two_Body_Problem_in_Two_Dimensions/9.08%3A_Orbital_Elements_and_Velocity_Vector#mjx-eqn-9.5.31
        // Lots of the formulae come from here ^
        let reduced_mass = GRAVITATIONAL_CONSTANT * parent.borrow().mass;
        let semi_major_axis = Self::semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = Self::eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let semi_minor_axis = semi_major_axis * (1.0 - eccentricity.powi(2)).sqrt();
        let argument_of_periapsis = Self::argument_of_periapsis(position, velocity, reduced_mass, eccentricity);
        let areal_velocity = 0.5 * vec3(position.x, position.y, 0.0).cross(&vec3(velocity.x, velocity.y, 0.0)).magnitude();
        let current_angle = position.angle(&vec2(1.0, 0.0));
        Self { parent, color, semi_major_axis, semi_minor_axis, eccentricity, argument_of_periapsis, areal_velocity, current_angle }
    }
    
    fn semi_major_axis(displacement: Vec2, velocity: Vec2, reduced_mass: f32) -> f32 {
        ((2.0 / displacement.magnitude()) - (velocity.magnitude().powi(2) / reduced_mass)).powi(-1)
    }

    fn transverse_velocity(position: Vec2, velocity: Vec2) -> f32 {
        // Component of velocity perpendicular to the displacement
        let perpendicular_to_displacement = vec2(position.y, position.x).normalize();
        let cos = perpendicular_to_displacement.dot(&velocity) / (perpendicular_to_displacement.magnitude() * velocity.magnitude()).abs();
        velocity.magnitude() * cos
    }

    fn eccentricity(position: Vec2, velocity: Vec2, reduced_mass: f32, semi_major_axis: f32) -> f32 {
        let transverse_velocity = Self::transverse_velocity(position, velocity);
        (1.0 - ((position.magnitude_squared() * transverse_velocity.powi(2)) / (reduced_mass * semi_major_axis))).sqrt()
    }

    fn argument_of_periapsis(position: Vec2, velocity: Vec2, reduced_mass: f32, eccentricity: f32) -> f32 {
        let transverse_velocity = Self::transverse_velocity(position, velocity);
        position.angle(&vec2(1.0, 0.0)) - ((((position.magnitude() * transverse_velocity.powi(2)) / (reduced_mass)) - 1.0) / eccentricity).acos()
    }

    fn get_displacement(&self, angle: f32) -> Vec2 {
        let radius = (self.semi_major_axis * (1.0 - self.eccentricity.powi(2))) / (1.0 + self.eccentricity * (angle - self.argument_of_periapsis).cos());
        vec2(radius * angle.cos(), radius * angle.sin())
    }

    fn max_color(&self) -> Rgba {
        // Scales the color up until one of the components is 1
        let max_component = f32::max(self.color.x, f32::max(self.color.y, self.color.z));
        Rgba::from_rgb(self.color.x / max_component, self.color.y / max_component, self.color.z / max_component)
    }

    fn get_orbit_points(&self) -> Vec<OrbitPoint> {
        let absolute_parent_position = self.get_absolute_parent_position() * SCALE_FACTOR;
        let mut orbit_points = vec![];
        for i in 0..ORBIT_POINTS {
            let relative_point_position = self.get_scaled_displacement((i as f32 / ORBIT_POINTS as f32) * 2.0 * PI);
            orbit_points.push(OrbitPoint::new(absolute_parent_position + relative_point_position, relative_point_position.normalize()));
        }
        orbit_points
    }

    fn add_orbit_line(&self, vertices: &mut Vec<f32>, previous_point: &OrbitPoint, new_point: &OrbitPoint, zoom: f32, i: i32) {
        let radius = ORBIT_PATH_RADIUS_DELTA + (i as f32 * ORBIT_PATH_RADIUS_DELTA); // Start off with non-zero radius
        let mut alpha = ORBIT_PATH_MAX_ALPHA;
        if i != 0 {
            // Scale the alpha non-linearly so we have lots of values close to zero
            alpha /= 7.0 * i as f32;
        }

        let rgb = self.max_color();
        let rgba = Rgba::from_rgba_unmultiplied(rgb.r(), rgb.g(), rgb.b(), alpha);

        let v1 = previous_point.absolute_position + (previous_point.displacement_direction * radius / zoom);
        let v2 = previous_point.absolute_position - (previous_point.displacement_direction * radius / zoom);
        let v3 = new_point.absolute_position + (new_point.displacement_direction * radius / zoom);
        let v4 = new_point.absolute_position - (new_point.displacement_direction * radius / zoom);

        Object::add_triangle(vertices, v1, v2, v3, rgba);
        Object::add_triangle(vertices, v2, v3, v4, rgba);
    }

    pub fn get_scaled_displacement(&self, angle: f32) -> Vec2 {
        self.get_displacement(angle - self.argument_of_periapsis) * SCALE_FACTOR
    }

    pub fn get_absolute_parent_position(&self) -> Vec2 {
        self.parent.borrow().get_absolute_position()
    }

    pub fn get_sphere_of_influence(&self, mass: f32) -> f32 {
        self.semi_major_axis * (mass / self.parent.borrow().mass).powf(2.0 / 5.0)
    }

    pub fn get_orbit_vertices(&self, zoom: f32) -> Vec<f32> {
        let mut vertices = vec![];
        let points = self.get_orbit_points();
        let mut previous_point = points.last().unwrap();
        for new_point in &points {
            // Loop to create glow effect
            for i in 0..10 {
                self.add_orbit_line(&mut vertices, previous_point, new_point, zoom, i);
            }
            previous_point = new_point;
        }
        
        vertices
    }

    fn angle_from_swept_area(&self, area: f32) -> f32 {
        // Assumes we start at current_angle
        f32::atan(
            ((self.semi_minor_axis / self.semi_major_axis) * f32::tan((2.0 * area) / (self.semi_major_axis * self.semi_minor_axis)))
            + f32::atan((self.semi_major_axis / self.semi_minor_axis) * f32::tan(self.current_angle))
        )
    }

    pub fn delta_time_to_angle(&self, time: f32) -> f32 {
        let delta_area = time * self.areal_velocity;
        self.angle_from_swept_area(delta_area)
    }

    pub fn get_current_position(&self) -> Vec2 {
        self.get_displacement(self.current_angle)
    }

    pub fn update(&mut self, delta_simulated_time: f32) {
        self.current_angle = self.delta_time_to_angle(delta_simulated_time);
    }
}