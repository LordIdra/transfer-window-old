use std::{f32::consts::PI, cell::RefCell, rc::Rc};

use eframe::epaint::{Rgba, Vec2, vec2};

use conic::Conic;

use self::{orbit_point::OrbitPoint, path::Path};

mod conic;
mod orbit_point;
mod path;

const SCALE_FACTOR: f32 = 1.0 / 100000.0;
const ORBIT_POINTS: usize = 256;
const ORBIT_PATH_MAX_ALPHA: f32 = 1.0;
const ORBIT_PATH_RADIUS_DELTA: f32 = 0.6;

pub struct Object {
    path: Path,
    position: Vec2,
    velocity: Vec2,
    mass: f32,
    radius: f32,
    color: Rgba,
    
}

impl Object {
    pub fn new(parent: Option<Rc<RefCell<Object>>>, position: Vec2, velocity: Vec2, mass: f32, radius: f32, color: Rgba) -> Rc<RefCell<Self>> {
        let path = Path::new(parent, position, velocity);
        Rc::new(RefCell::new(Self { path, position, velocity, mass, radius, color }))
    }

    fn add_triangle(vertices: &mut Vec<f32>, v1: Vec2, v2: Vec2, v3: Vec2, color: Rgba) {
        vertices.append(&mut vec![v1.x, v1.y, color.r(), color.g(), color.b(), color.a()]);
        vertices.append(&mut vec![v2.x, v2.y, color.r(), color.g(), color.b(), color.a()]);
        vertices.append(&mut vec![v3.x, v3.y, color.r(), color.g(), color.b(), color.a()]);
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

    fn max_color(&self) -> Rgba {
        // Scales the color up until one of the components is 1
        let max_component = f32::max(self.color.r(), f32::max(self.color.g(), self.color.b()));
        Rgba::from_rgb(self.color.r() / max_component, self.color.g() / max_component, self.color.b() / max_component)
    }

    fn get_orbit_points(&self, orbit_description: &Conic) -> Vec<OrbitPoint> {
        let absolute_parent_position = self.get_absolute_parent_position() * SCALE_FACTOR;
        let mut orbit_points = vec![];
        for i in 0..ORBIT_POINTS {
            let relative_point_position = orbit_description.get_scaled_displacement((i as f32 / ORBIT_POINTS as f32) * 2.0 * PI);
            orbit_points.push(OrbitPoint::new( absolute_parent_position + relative_point_position, relative_point_position.normalized()));
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

        Self::add_triangle(vertices, v1, v2, v3, rgba);
        Self::add_triangle(vertices, v2, v3, v4, rgba);
    }

    pub fn get_orbit_vertices(&self, zoom: f32) -> Vec<f32> {
        let Some(orbit_description) = &self.orbit_description else {
            return vec![];
        };

        let mut vertices = vec![];
        let points = self.get_orbit_points(orbit_description);
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
}
