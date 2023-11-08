use crate::{state::State, storage::entity_allocator::Entity, util::get_root_entities};

use super::trajectory_util::sync_to_trajectory;

fn update_trajectory(state: &mut State, entity: &Entity) {
    if let Some(trajectory_component) = state.components.trajectory_components.get_mut(entity) {
        trajectory_component.update(state.delta_time);
        sync_to_trajectory(state, entity);
    }
    if let Some(celestial_body_component) = state.components.celestial_body_components.get(entity) {
        for child in celestial_body_component.get_children().clone() {
            update_trajectory(state, &child);
        }
    }
}

pub fn trajectory_update_system(state: &mut State) {
    for entity in get_root_entities(state) {
        update_trajectory(state, &entity);
        if let Some(name_component) = state.components.name_components.get(&entity) {
            if name_component.get_name() == "spacecraft" {
                let parent = state.components.parent_components.get(&entity).unwrap().get_parent();
                let absolute_parent_position = state.components.position_components.get(&parent).unwrap().get_absolute_position();
                println!("start {}", absolute_parent_position + state.components.trajectory_components.get(&entity).unwrap().get_current_orbit().get_start_unscaled_position());
                println!("end {}", absolute_parent_position + state.components.trajectory_components.get(&entity).unwrap().get_current_orbit().get_end_unscaled_position());
            }
        }
    }
}