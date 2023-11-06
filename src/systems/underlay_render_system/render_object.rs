use std::f64::consts::PI;

use nalgebra_glm::vec2;

use crate::{state::State, components::{celestial_body_component::CelestialBodyComponent, position_component::PositionComponent}, camera::SCALE_FACTOR, util::add_triangle};


fn get_entity_object_vertices(state: &mut State, position_component: &PositionComponent, celestial_body_component: &CelestialBodyComponent) -> Vec<f32> {
    let scaled_radius = celestial_body_component.get_radius() * SCALE_FACTOR;
    let absolute_scaled_position = position_component.get_absolute_position() * SCALE_FACTOR;
    let mut vertices = vec![];
    let sides = 100; // TODO make this depend on something else ie zoom/translation
    let mut previous_location = absolute_scaled_position + vec2(scaled_radius, 0.0);
    for i in 1..=sides { // 1..=sides to make sure we fill in the gap between the last location and first location, wrapping back round
        let angle = (i as f64 / sides as f64) * 2.0 * PI; // both i and sides must be cast to prevent integer division problems
        let new_location = absolute_scaled_position + vec2(scaled_radius * f64::cos(angle), scaled_radius * f64::sin(angle));
        add_triangle(&mut vertices, absolute_scaled_position, previous_location, new_location, celestial_body_component.get_color());
        previous_location = new_location;
    }
    vertices
}

pub fn get_all_object_vertices(state: &mut State) -> Vec<f32> {
    let mut vertices = vec![];
    for entity in state.components.entity_allocator.get_entities() {
        let Some(position_component) = state.components.position_components.get(entity) else {
            continue;
        };
        let Some(celestial_body_component) = state.components.celestial_body_components.get(entity) else {
            continue;
        };
        vertices.append(&mut get_entity_object_vertices(state, position_component, celestial_body_component));
    }
    vertices
}