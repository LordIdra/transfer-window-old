use crate::state::State;

use super::util::sync_all_entities;

pub fn trajectory_update_system(state: &mut State) {
    let time_step = state.get_time_step();
    for entity in &state.get_entities_sorted_by_mass() {
        if let Some(trajectory_component) = state.components.trajectory_components.get_mut(entity) {
            trajectory_component.update(state.time, state.delta_time * time_step);
        }
    }
    sync_all_entities(state)
}