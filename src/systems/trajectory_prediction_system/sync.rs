use crate::{state::State, storage::entity_allocator::Entity, systems::util::{get_segment_at_time, sync_to_segment}};

use super::util::is_spacecraft;

fn sync_entity_to_time(state: &mut State, entity: &Entity, time: f64) {
    let mut segment = get_segment_at_time(state, entity, time);
    let delta_time = time - segment.get_start_time();
    segment.reset();
    segment.update(delta_time);
    sync_to_segment(state, segment, entity)
}

pub fn sync_celestial_bodies_to_time(state: &mut State, time: f64) {
    for entity in state.components.entity_allocator.get_entities() {
        if is_spacecraft(state, entity) {
            sync_entity_to_time(state, &entity, time);
        }
    }
}
