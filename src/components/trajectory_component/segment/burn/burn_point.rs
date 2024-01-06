use nalgebra_glm::DVec2;

use crate::{storage::entity_allocator::Entity, state::State, components::trajectory_component::segment::orbit::orbit_direction::GRAVITATIONAL_CONSTANT};

#[derive(Clone)]
pub struct BurnPoint {
    parent_mass: f64,
    time: f64,
    position: DVec2,
    velocity: DVec2,
}

impl BurnPoint {
    pub fn new(state: &State, entity: Entity, parent: Entity, time: f64) -> Self {
        let segment = state.components.trajectory_components.get(&entity).unwrap().get_final_segment();
        let previous_orbit = segment.as_orbit();
        let previous_orbit_ref = previous_orbit.borrow();
        let previous_orbit_end_point = previous_orbit_ref.get_end_point();
        let position = previous_orbit_end_point.get_position();
        let velocity = previous_orbit_end_point.get_velocity();
        let parent_mass = state.components.mass_components.get(&parent).unwrap().get_mass();
        Self { parent_mass, time, position, velocity }
    }

    pub fn next(&self, delta_time: f64) -> Self {
        let distance = self.position.magnitude();
        let acceleration = -self.position.normalize() * (GRAVITATIONAL_CONSTANT * self.parent_mass) / distance.powi(2);
        let parent_mass = self.parent_mass;
        let time = self.time + delta_time;
        let velocity = self.velocity + acceleration * delta_time;
        let position = self.position + self.velocity * delta_time;
        BurnPoint { parent_mass, time, position, velocity }
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }

    pub fn get_position(&self) -> DVec2 {
        self.position
    }

    pub fn get_velocity(&self) -> DVec2 {
        self.velocity
    }
}