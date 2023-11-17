use std::f64::consts::PI;

use nalgebra_glm::{DVec2, vec2};

use crate::{storage::entity_allocator::Entity, components::Components};

use super::{conic::{Conic, new_conic}, orbit_point::OrbitPoint, orbit_direction::OrbitDirection};

fn normalize_angle(mut theta: f64) -> f64 {
    while theta < 0.0      { theta += 2.0 * PI }
    while theta > 2.0 * PI { theta -= 2.0 * PI }
    theta
}

pub struct Orbit {
    parent: Entity,
    conic: Box<dyn Conic>,
    start_orbit_point: OrbitPoint,
    end_orbit_point: OrbitPoint,
    current_orbit_point: OrbitPoint,
}

impl Orbit {
    pub fn new(components: &Components, parent: Entity, position: DVec2, velocity: DVec2, time: f64) -> Self {
        let parent_mass = components.mass_components.get(&parent).unwrap().get_mass();
        let conic = new_conic(parent_mass, position, velocity);
        let start_orbit_point = OrbitPoint::new(&*conic, position, time);
        let end_orbit_point = start_orbit_point.clone();
        let current_orbit_point = start_orbit_point.clone();
        Self { parent, conic, start_orbit_point, end_orbit_point, current_orbit_point }
    }

    pub fn get_remaining_angle(&self) -> f64 {
        // If we have any full orbits remaining, only return up to 2pi
        if self.get_remaining_orbits() > 0 {
            return 2.0 * PI;
        }

        let mut remaining_angle = self.end_orbit_point.get_theta() - self.current_orbit_point.get_theta();
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

    pub fn get_semi_major_axis(&self) -> f64 {
        self.conic.get_semi_major_axis()
    }

    pub fn get_semi_minor_axis(&self) -> f64 {
        self.conic.get_semi_minor_axis()
    }

    pub fn get_arugment_of_periapsis(&self) -> f64 {
        self.conic.get_argument_of_periapsis()
    }

    pub fn get_eccentricity(&self) -> f64 {
        self.conic.get_eccentricity()
    }

    pub fn is_finished(&self) -> bool {
        self.current_orbit_point.is_after(&self.end_orbit_point)
    }

    pub fn get_position_from_theta(&self, theta: f64) -> DVec2 {
        self.conic.get_position(theta)
    }

    pub fn get_time_since_periapsis(&self, theta: f64) -> f64 {
        self.conic.get_time_since_periapsis(theta)
    }

    pub fn is_time_within_orbit(&self, time_since_periapsis: f64) -> bool {
        self.conic.is_time_between_points(&self.current_orbit_point, &self.end_orbit_point, time_since_periapsis)
    }

    pub fn get_start_position(&self) -> DVec2 {
        self.start_orbit_point.get_unscaled_position()
    }

    pub fn get_current_position(&self) -> DVec2 {
        self.current_orbit_point.get_unscaled_position()
    }

    pub fn get_end_position(&self) -> DVec2 {
        self.end_orbit_point.get_unscaled_position()
    }

    pub fn get_current_velocity(&self) -> DVec2 {
        self.current_orbit_point.get_velocity()
    }
    
    pub fn get_end_velocity(&self) -> DVec2 {
        self.end_orbit_point.get_velocity()
    }

    pub fn get_current_true_anomaly(&self) -> f64 {
        self.current_orbit_point.get_theta()
    }

    pub fn solve_for_closest_point(&self, p: DVec2) -> DVec2 {
        self.conic.solve_for_closest_point(p)
    }

    pub fn predict(&mut self, delta_time: f64) {
        self.end_orbit_point = self.end_orbit_point.next(&*self.conic, delta_time);
    }

    pub fn update(&mut self, delta_time: f64) {
        self.current_orbit_point = self.current_orbit_point.next(&*self.conic, delta_time);
    }
}