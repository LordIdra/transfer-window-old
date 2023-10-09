use std::{f32::consts::PI, cell::RefCell, rc::Rc};

use eframe::epaint::{Rgba, Vec2, vec2};

use self::path::Path;

mod conic;
mod orbit_point;
mod path;

const SCALE_FACTOR: f32 = 1.0 / 100000.0;

pub struct Object {
    path: Path,
    position: Vec2,
    velocity: Vec2,
    mass: f32,
    radius: f32,
    color: Rgba,
    sphere_of_influence: f32,
}

impl Object {
    pub fn new(parent: Option<Rc<RefCell<Object>>>, position: Vec2, velocity: Vec2, mass: f32, radius: f32, color: Rgba) -> Rc<RefCell<Self>> {
        let path = Path::new(parent, position, velocity);
        let sphere_of_influence = Self::calculate_sphere_of_influence(&path, mass);
        Rc::new(RefCell::new(Self { path, position, velocity, mass, radius, color, sphere_of_influence }))
    }

    fn add_triangle(vertices: &mut Vec<f32>, v1: Vec2, v2: Vec2, v3: Vec2, color: Rgba) {
        vertices.append(&mut vec![v1.x, v1.y, color.r(), color.g(), color.b(), color.a()]);
        vertices.append(&mut vec![v2.x, v2.y, color.r(), color.g(), color.b(), color.a()]);
        vertices.append(&mut vec![v3.x, v3.y, color.r(), color.g(), color.b(), color.a()]);
    }

    fn calculate_sphere_of_influence(path: &Path, mass: f32) -> f32 {
        // https://en.wikipedia.org/wiki/Sphere_of_influence_(astrodynamics)
        if let Some(conic) = path.get_current_conic() {
            conic.get_sphere_of_influence(mass)
        } else {
            0.0
        }
    }

    pub fn get_absolute_position(&self) -> Vec2 {
        self.path.get_current_absolute_parent_position() + self.position
    }

    pub fn get_object_vertices(&self) -> Vec<f32> {
        let scaled_radius = self.radius * SCALE_FACTOR;
        let absolute_scaled_position = (self.path.get_current_absolute_parent_position() + self.position) * SCALE_FACTOR;
        let mut vertices = vec![];
        let sides = 100; // TODO make this depend on something else
        let mut previous_location = absolute_scaled_position + vec2(scaled_radius, 0.0);
        for i in 1..=sides { // 1..=sides to make sure we fill in the gap between the last location and first location, wrapping back round
            let angle = (i as f32 / sides as f32) * 2.0 * PI; // both i and sides must be cast to f32 to prevent integer division problems
            let new_location = absolute_scaled_position + vec2(scaled_radius * f32::cos(angle), scaled_radius * f32::sin(angle));
            Self::add_triangle(&mut vertices, absolute_scaled_position, previous_location, new_location, self.color);
            previous_location = new_location;
        }
        vertices
    }

    pub fn get_orbit_vertices(&self, zoom: f32) -> Vec<f32> {
        self.path.get_orbit_vertices(zoom)
    }
}
