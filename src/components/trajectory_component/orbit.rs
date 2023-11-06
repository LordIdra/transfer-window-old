use std::f64::consts::PI;

use nalgebra_glm::{Vec3, DVec2};

use crate::{storage::{entity_allocator::Entity, components::Components}, camera::SCALE_FACTOR};

use super::{conic::{Conic, new_conic}, orbit_point::OrbitPoint, orbit_direction::OrbitDirection};



pub struct Orbit {
    parent: Entity,
    color: Vec3,
    conic: Box<dyn Conic>,
    start_orbit_point: OrbitPoint,
    end_orbit_point: OrbitPoint,
    current_orbit_point: OrbitPoint,
}

impl Orbit {
    pub fn new(components: &Components, parent: Entity, color: Vec3, position: DVec2, velocity: DVec2, time: f64) -> Self {
        let parent_mass = components.mass_components.get(&parent).unwrap().get_mass();
        let conic = new_conic(parent_mass, position, velocity);
        let start_orbit_point = OrbitPoint::new(&*conic, position, time);
        let end_orbit_point = start_orbit_point.clone();
        let current_orbit_point = start_orbit_point.clone();
        Self { parent, color, conic, start_orbit_point, end_orbit_point, current_orbit_point }
    }

    pub fn get_remaining_angle(&self) -> f64 {
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

    pub fn get_parent(&self) -> Entity {
        self.parent
    }

    pub fn get_scaled_position(&self, mean_anomaly: f64) -> DVec2 {
        self.conic.get_position(mean_anomaly) * SCALE_FACTOR
    }

    pub fn get_absolute_parent_position(&self, components: &Components) -> DVec2 {
        components.position_components.get(&self.parent).unwrap().get_absolute_position()
    }

    pub fn get_sphere_of_influence(&self, components: &Components, mass: f64) -> f64 {
        let parent_mass = components.mass_components.get(&self.parent).unwrap().get_mass();
        self.conic.get_sphere_of_influence(mass, parent_mass)
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

    pub fn get_current_true_anomaly(&self) -> f64 {
        self.current_orbit_point.get_true_anomaly()
    }
    
    pub fn get_end_velocity(&self) -> DVec2 {
        self.end_orbit_point.get_velocity()
    }

    pub fn update(&mut self, delta_time: f64) {
        self.current_orbit_point = self.current_orbit_point.next(&*self.conic, delta_time);
    }

    pub fn predict(&mut self, delta_time: f64) {
        self.end_orbit_point = self.end_orbit_point.next(&*self.conic, delta_time);
    }

    pub fn is_finished(&self) -> bool {
        self.current_orbit_point.is_after(&self.end_orbit_point)
    }
}