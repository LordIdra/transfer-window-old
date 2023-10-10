use std::{rc::Rc, cell::RefCell, f32::consts::PI};

use eframe::epaint::Rgba;
use nalgebra_glm::{Vec3, Vec2};

use super::{Object, SCALE_FACTOR, visual_orbit_point::VisualOrbitPoint, orbit_point::OrbitPoint, scary_maths::{self, OrbitDirection}};

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
    period: f32,
    argument_of_periapsis: f32,
    start_angle: f32,
    end_angle: f32,
    direction: OrbitDirection,
    current_orbit_point: OrbitPoint,
}

impl Conic {
    pub fn new(parent: Rc<RefCell<Object>>, color: Vec3, position: Vec2, velocity: Vec2) -> Self {
        // https://phys.libretexts.org/Bookshelves/Astronomy__Cosmology/Celestial_Mechanics_(Tatum)/09%3A_The_Two_Body_Problem_in_Two_Dimensions/9.08%3A_Orbital_Elements_and_Velocity_Vector#mjx-eqn-9.5.31
        // Lots of the formulae come from here ^
        let reduced_mass = GRAVITATIONAL_CONSTANT * parent.borrow().mass;
        let semi_major_axis = scary_maths::semi_major_axis(position, velocity, reduced_mass);
        let eccentricity = scary_maths::eccentricity(position, velocity, reduced_mass, semi_major_axis);
        let semi_minor_axis = scary_maths::semi_minor_axis(semi_major_axis, eccentricity);
        let argument_of_periapsis = scary_maths::argument_of_periapsis(position, velocity, reduced_mass, eccentricity);
        let period = scary_maths::period(reduced_mass, semi_major_axis);
        let direction = OrbitDirection::from_position_and_velocity(position, velocity);
        let current_orbit_point = OrbitPoint::new(semi_major_axis, eccentricity, period, argument_of_periapsis, direction, position);
        let start_angle = current_orbit_point.get_angle_since_periapsis();
        let end_angle = f32::MAX;
        Self { parent, color, semi_major_axis, semi_minor_axis, eccentricity, period, argument_of_periapsis, start_angle, end_angle, direction, current_orbit_point }
    }

    fn max_color(&self) -> Rgba {
        // Scales the color up until one of the components is 1
        let max_component = f32::max(self.color.x, f32::max(self.color.y, self.color.z));
        Rgba::from_rgb(self.color.x / max_component, self.color.y / max_component, self.color.z / max_component)
    }

    fn get_visual_orbit_points(&self) -> Vec<VisualOrbitPoint> {
        let absolute_parent_position = self.get_absolute_parent_position() * SCALE_FACTOR;
        let mut visual_orbit_points = vec![];
        for i in 0..ORBIT_POINTS {
            let relative_point_position = self.get_scaled_position((i as f32 / ORBIT_POINTS as f32) * 2.0 * PI);
            visual_orbit_points.push(VisualOrbitPoint::new(absolute_parent_position + relative_point_position, relative_point_position.normalize()));
        }
        visual_orbit_points
    }

    fn add_orbit_line(&self, vertices: &mut Vec<f32>, previous_point: &VisualOrbitPoint, new_point: &VisualOrbitPoint, zoom: f32, i: i32) {
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

    pub fn get_scaled_position(&self, angle_since_periapsis: f32) -> Vec2 {
        scary_maths::position(self.argument_of_periapsis, self.semi_major_axis, self.eccentricity, angle_since_periapsis) * SCALE_FACTOR
    }

    pub fn get_absolute_parent_position(&self) -> Vec2 {
        self.parent.borrow().get_absolute_position()
    }

    pub fn get_sphere_of_influence(&self, mass: f32) -> f32 {
        self.semi_major_axis * (mass / self.parent.borrow().mass).powf(2.0 / 5.0)
    }

    pub fn get_orbit_vertices(&self, zoom: f32) -> Vec<f32> {
        let mut vertices = vec![];
        let points = self.get_visual_orbit_points();
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

    pub fn get_unscaled_position(&self) -> Vec2 {
        self.current_orbit_point.get_unscaled_position()
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_orbit_point = self.current_orbit_point.next(self.semi_major_axis, self.eccentricity, self.argument_of_periapsis, self.period, self.direction, delta_time);
    }
}