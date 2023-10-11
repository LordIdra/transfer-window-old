use std::{rc::Rc, cell::RefCell, f32::consts::PI};

use eframe::epaint::Rgba;
use nalgebra_glm::{Vec3, Vec2};

use super::{Object, SCALE_FACTOR, visual_orbit_point::VisualOrbitPoint, orbit_point::OrbitPoint, orbit_direction::OrbitDirection, orbit_description::OrbitDescription};

const ORBIT_POINTS: usize = 256;
const ORBIT_PATH_MAX_ALPHA: f32 = 1.0;
const ORBIT_PATH_RADIUS_DELTA: f32 = 0.6;

pub struct Conic {
    parent: Rc<RefCell<Object>>,
    color: Vec3,
    orbit_description: OrbitDescription,
    start_orbit_point: OrbitPoint,
    end_orbit_point: Option<OrbitPoint>, // if none, the simulation has not yet found an end point for this conic section
    current_orbit_point: OrbitPoint,
}

impl Conic {
    pub fn new(parent: Rc<RefCell<Object>>, color: Vec3, position: Vec2, velocity: Vec2) -> Self {
        let orbit_description = OrbitDescription::new(parent.borrow().mass, position, velocity);
        let start_orbit_point = OrbitPoint::new(&orbit_description, position);
        let end_orbit_point = None;
        let current_orbit_point = start_orbit_point.clone();
        Self { parent, color, orbit_description, start_orbit_point, end_orbit_point, current_orbit_point }
    }

    fn max_color(&self) -> Rgba {
        // Scales the color up until one of the components is 1
        let max_component = f32::max(self.color.x, f32::max(self.color.y, self.color.z));
        Rgba::from_rgb(self.color.x / max_component, self.color.y / max_component, self.color.z / max_component)
    }

    // If no end point is present, just returns 2pi
    fn get_remaining_angle(&self) -> f32 {
        if let Some(end_orbit_point) = &self.end_orbit_point {
            end_orbit_point.get_angle_since_periapsis() - self.start_orbit_point.get_angle_since_periapsis()
        } else {
            2.0 * PI
        }
    }

    fn get_visual_orbit_points(&self) -> Vec<VisualOrbitPoint> {
        let absolute_parent_position = self.get_absolute_parent_position() * SCALE_FACTOR;
        let mut visual_orbit_points = vec![];
        let angle_to_rotate_through = f32::min(2.0 * PI, self.get_remaining_angle());
        for i in 0..ORBIT_POINTS {
            let mut angle = self.current_orbit_point.get_angle_since_periapsis();
            if let OrbitDirection::Clockwise = self.orbit_description.get_direction() {
                angle += (i as f32 / ORBIT_POINTS as f32) * angle_to_rotate_through
            } else {
                angle -= (i as f32 / ORBIT_POINTS as f32) * angle_to_rotate_through
            }

            let relative_point_position = self.get_scaled_position(angle);
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

    fn complete_orbit_vertices_from_points(&self, points: Vec<VisualOrbitPoint>, zoom: f32) -> Vec<f32> {
        // Visualises an entire ellipse without any gaps
        let mut previous_point = points.last().unwrap();
        let mut vertices = vec![];
        for new_point in &points {
            // Loop to create glow effect
            for i in 0..10 {
                self.add_orbit_line(&mut vertices, previous_point, new_point, zoom, i);
            }
            previous_point = new_point;
        }
        vertices
    }

    fn incomplete_orbit_vertices_from_points(&self, points: Vec<VisualOrbitPoint>, zoom: f32) -> Vec<f32> {
        // Visualises an ellipse between two angles
        let mut previous_point = None;
        let mut vertices = vec![];
        for new_point in &points {
            // Loop to create glow effect
            if let Some(previous_point) = previous_point {
                for i in 0..10 {
                    self.add_orbit_line(&mut vertices, previous_point, new_point, zoom, i);
                }
            }
            previous_point = Some(new_point);
        }
        vertices
    }

    pub fn get_scaled_position(&self, angle_since_periapsis: f32) -> Vec2 {
        self.orbit_description.get_position(angle_since_periapsis) * SCALE_FACTOR
    }

    pub fn get_absolute_parent_position(&self) -> Vec2 {
        self.parent.borrow().get_absolute_position()
    }

    pub fn get_sphere_of_influence(&self, mass: f32) -> f32 {
        self.orbit_description.get_sphere_of_influence(mass, self.parent.borrow().mass)
    }

    pub fn get_orbit_vertices(&self, zoom: f32) -> Vec<f32> {
        let points = self.get_visual_orbit_points();
        if self.get_remaining_angle() >= 2.0 * PI {
            self.complete_orbit_vertices_from_points(points, zoom)
        } else {
            self.incomplete_orbit_vertices_from_points(points, zoom)
        }
    }

    pub fn get_unscaled_position(&self) -> Vec2 {
        self.current_orbit_point.get_unscaled_position()
    }

    pub fn get_velocity(&self) -> Vec2 {
        self.current_orbit_point.get_velocity()
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_orbit_point = self.current_orbit_point.next(&self.orbit_description, delta_time);
    }

    pub fn reset(&mut self) {
        self.current_orbit_point = self.start_orbit_point.clone();
    }
}