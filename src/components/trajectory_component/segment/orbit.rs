use std::f64::consts::PI;

use nalgebra_glm::DVec2;

use crate::{storage::entity_allocator::Entity, components::Components, systems::debug_system::debug_utils::format_time};

use self::{conic::{Conic, new_conic}, orbit_point::OrbitPoint, orbit_direction::OrbitDirection};

mod conic;
pub mod orbit_direction;
pub mod orbit_point;

pub struct Orbit {
    parent: Entity,
    conic: Box<dyn Conic>,
    start_point: OrbitPoint,
    end_point: OrbitPoint,
    current_point: OrbitPoint,
}

impl Orbit {
    pub fn new(components: &Components, parent: Entity, position: DVec2, velocity: DVec2, time: f64) -> Self {
        let parent_mass = components.mass_components.get(&parent).unwrap().get_mass();
        let conic = new_conic(parent_mass, position, velocity);
        let start_point = OrbitPoint::new(&*conic, position, time);
        let end_point = start_point.clone();
        let current_point = start_point.clone();
        Self { parent, conic, start_point, end_point, current_point }
    }

    pub fn get_start_point(&self) -> &OrbitPoint {
        &self.start_point
    }

    pub fn get_current_point(&self) -> &OrbitPoint {
        &self.current_point
    }

    pub fn get_end_point(&self) -> &OrbitPoint {
        &self.end_point
    }

    pub fn get_remaining_angle(&self) -> f64 {
        // If we have any full orbits remaining, only return up to 2pi
        if self.get_remaining_orbits() > 0 {
            return 2.0 * PI;
        }

        let mut adjusted_end_theta = self.end_point.get_theta() % (2.0 * PI);
        if adjusted_end_theta < 0.0 {
            adjusted_end_theta += 2.0 * PI;
        }
        let mut adjusted_current_theta = self.current_point.get_theta() % (2.0 * PI);
        if adjusted_current_theta < 0.0 {
            adjusted_current_theta += 2.0 * PI;
        }

        let mut remaining_angle = adjusted_end_theta - adjusted_current_theta;
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

    pub fn get_remaining_orbits(&self) -> i32 {
        self.conic.get_remaining_orbits(self.end_point.get_time() - self.current_point.get_time())
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

    pub fn get_eccentricity(&self) -> f64 {
        self.conic.get_eccentricity()
    }

    pub fn get_arugment_of_periapsis(&self) -> f64 {
        self.conic.get_argument_of_periapsis()
    }

    pub fn is_finished(&self) -> bool {
        self.current_point.is_after(&self.end_point)
    }

    pub fn get_position_from_theta(&self, theta: f64) -> DVec2 {
        self.conic.get_position(theta)
    }

    pub fn get_velocity_from_theta(&self, theta: f64) -> DVec2 {
        self.conic.get_velocity(self.get_position_from_theta(theta), theta)
    }

    pub fn get_time_since_periapsis(&self, theta: f64) -> f64 {
        self.conic.get_time_since_periapsis(theta)
    }

    pub fn get_overshot_time(&self, time: f64) -> f64 {
        time - self.end_point.get_time()
    }

    pub fn get_period(&self) -> Option<f64> {
        self.conic.get_period()
    }

    pub fn get_min_max_theta(&self) -> (f64, f64) {
        self.conic.get_min_max_theta()
    }

    pub fn get_periapsis_time(&self) -> f64 {
        let time_since_last_periapsis = self.conic.get_time_since_last_periapsis(&self.start_point);
        self.start_point.get_time() - time_since_last_periapsis
    }

    pub fn get_position_from_time_since_periapsis(&self, time_since_periapsis: f64) -> DVec2 {
        let theta = self.conic.get_theta_from_time_since_periapsis(time_since_periapsis);
        self.conic.get_position(theta)
    }

    pub fn is_time_within_orbit(&self, time: f64) -> bool {
        self.conic.is_time_between_points(&self.current_point, &self.end_point, time)
    }

    pub fn get_direction(&self) -> OrbitDirection {
        self.conic.get_direction()
    }

    pub fn get_theta_from_time(&self, time: f64) -> f64 {
        let time_since_periapsis = time - self.get_periapsis_time();
        self.conic.get_theta_from_time_since_periapsis(time_since_periapsis)
    }

    pub fn solve_for_closest_point(&self, p: DVec2) -> DVec2 {
        self.conic.solve_for_closest_point(p)
    }

    pub fn end_at(&mut self, time: f64) {
        let theta = self.get_theta_from_time(time);
        let position = self.conic.get_position(theta);
        self.end_point = OrbitPoint::new(&*self.conic, position, time);
    }

    pub fn reset(&mut self) {
        self.current_point = self.start_point.clone();
    }

    pub fn trim_to_end_at(&mut self, time: f64) {
        let position = self.conic.get_position(self.get_theta_from_time(time));
        self.end_point = OrbitPoint::new(&*self.conic, position, time);
    }

    pub fn predict(&mut self, delta_time: f64) {
        self.end_point = self.end_point.next(&*self.conic, delta_time);
    }

    pub fn update(&mut self, delta_time: f64) {
        self.current_point = self.current_point.next(&*self.conic, delta_time);
    }
}