use std::f64::consts::PI;

use eframe::epaint::Rgba;
use nalgebra_glm::{Vec3, DVec2};

use crate::{object::conic::{Conic, new_conic}, util::add_triangle, state::ObjectId, storage::Storage};

use super::{SCALE_FACTOR, visual_orbit_point::VisualOrbitPoint, orbit_point::OrbitPoint, orbit_direction::OrbitDirection};

const ORBIT_POINTS: usize = 256;
const ORBIT_PATH_MAX_ALPHA: f32 = 1.0;
const ORBIT_PATH_RADIUS_DELTA: f64 = 0.6;

pub struct Orbit {
    parent: ObjectId,
    color: Vec3,
    conic: Box<dyn Conic>,
    start_orbit_point: OrbitPoint,
    end_orbit_point: OrbitPoint,
    current_orbit_point: OrbitPoint,
}

impl Orbit {
    pub fn new(storage: &Storage, parent: ObjectId, color: Vec3, position: DVec2, velocity: DVec2, time: f64) -> Self {
        let conic = new_conic(storage.get(&parent).mass, position, velocity);
        let start_orbit_point = OrbitPoint::new(&*conic, position, time);
        let end_orbit_point = start_orbit_point.clone();
        let current_orbit_point = start_orbit_point.clone();
        Self { parent, color, conic, start_orbit_point, end_orbit_point, current_orbit_point }
    }

    fn max_color(&self) -> Rgba {
        // Scales the color up until one of the components is 1
        let max_component = f32::max(self.color.x, f32::max(self.color.y, self.color.z));
        Rgba::from_rgb(self.color.x / max_component, self.color.y / max_component, self.color.z / max_component)
    }

    fn get_remaining_angle(&self) -> f64 {
        // If we have any full orbits remaining, only return up to 2pi
        if self.get_remaining_orbits() > 0 {
            return 2.0 * PI;
        }

        let mut remaining_angle = self.end_orbit_point.get_true_anomaly() - self.current_orbit_point.get_true_anomaly();
        if let OrbitDirection::Clockwise = self.conic.get_direction() {
            if remaining_angle > 0.0 {
                remaining_angle -= 2.0 * PI
            }
            remaining_angle
        } else {
            if remaining_angle < 0.0 {
                remaining_angle += 2.0 * PI
            }
            remaining_angle
        }
    }

    fn get_remaining_orbits(&self) -> i32 {
        self.conic.get_remaining_orbits(self.end_orbit_point.get_time() - self.start_orbit_point.get_time())
    }

    fn get_visual_orbit_points(&self, storage: &Storage) -> Vec<VisualOrbitPoint> {
        let absolute_parent_position = self.get_absolute_parent_position(storage) * SCALE_FACTOR;
        let mut visual_orbit_points = vec![];
        let angle_to_rotate_through = self.get_remaining_angle();
        for i in 0..=ORBIT_POINTS {
            let angle = self.current_orbit_point.get_true_anomaly() + (i as f64 / ORBIT_POINTS as f64) * angle_to_rotate_through;
            let relative_point_position = self.get_scaled_position(angle);
            visual_orbit_points.push(VisualOrbitPoint::new(absolute_parent_position + relative_point_position, relative_point_position.normalize()));
        }
        visual_orbit_points
    }

    fn add_orbit_line(&self, vertices: &mut Vec<f32>, previous_point: &VisualOrbitPoint, new_point: &VisualOrbitPoint, zoom: f64, i: i32) {
        let radius = ORBIT_PATH_RADIUS_DELTA + (i as f64 * ORBIT_PATH_RADIUS_DELTA); // Start off with non-zero radius
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

        add_triangle(vertices, v1, v2, v3, rgba);
        add_triangle(vertices, v2, v3, v4, rgba);
    }

    pub fn get_parent(&self) -> ObjectId {
        self.parent.clone()
    }

    pub fn get_scaled_position(&self, mean_anomaly: f64) -> DVec2 {
        self.conic.get_position(mean_anomaly) * SCALE_FACTOR
    }

    pub fn get_absolute_parent_position(&self, storage: &Storage) -> DVec2 {
        storage.get(&self.parent).get_absolute_position(storage)
    }

    pub fn get_sphere_of_influence(&self, storage: &Storage, mass: f64) -> f64 {
        self.conic.get_sphere_of_influence(mass, storage.get(&self.parent).mass)
    }

    pub fn get_orbit_vertices(&self, storage: &Storage, zoom: f64) -> Vec<f32> {
        let points = self.get_visual_orbit_points(storage);
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

    pub fn get_current_unscaled_position(&self) -> DVec2 {
        self.current_orbit_point.get_unscaled_position()
    }

    pub fn get_end_unscaled_position(&self) -> DVec2 {
        self.end_orbit_point.get_unscaled_position()
    }

    pub fn get_current_velocity(&self) -> DVec2 {
        self.current_orbit_point.get_velocity()
    }
    
    pub fn get_end_velocity(&self) -> DVec2 {
        self.end_orbit_point.get_velocity()
    }

    pub fn update(&mut self, delta_time: f64) {
        self.current_orbit_point = self.current_orbit_point.next(&*self.conic, delta_time);
    }

    pub fn update_for_prediction(&mut self, delta_time: f64) {
        self.end_orbit_point = self.end_orbit_point.next(&*self.conic, delta_time);
    }

    pub fn is_finished(&self) -> bool {
        self.current_orbit_point.is_after(&self.end_orbit_point)
    }
}