use crate::{state::State, components::trajectory_component::segment::Segment, storage::entity_allocator::Entity};

use self::{render_orbit::get_entity_orbit_vertices, render_burn::get_entity_burn_vertices};

mod render_burn;
mod render_orbit;
mod util;
mod visual_segment_point;

fn get_entity_segment_vertices(state: &State, entity: &Entity, segment: &Segment, orbit_index: usize) -> Vec<f32> {
    match segment {
        Segment::Burn(burn) => get_entity_burn_vertices(state, entity, &*burn.borrow()),
        Segment::Orbit(orbit) => get_entity_orbit_vertices(state, entity, &*orbit.borrow()),
    }
}

pub fn get_all_segment_vertices(state: &mut State) -> Vec<f32> {
    let mut vertices = vec![];
    for entity in state.components.entity_allocator.get_entities() {
        if let Some(trajectory_component) = state.components.trajectory_components.get(&entity) {
            for (orbit_index, segment) in trajectory_component.get_segments().iter().enumerate() {
                vertices.append(&mut get_entity_segment_vertices(state, &entity, segment, orbit_index));
            }
        }
    }
    vertices
}