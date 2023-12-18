use crate::state::State;

pub fn icon_position_update_system(state: &mut State) {
    for entity in state.components.entity_allocator.get_entities() {
        if let Some(icon_component) = state.components.icon_components.get_mut(&entity) {
            if let Some(position_component) = state.components.position_components.get(&entity) {
                icon_component.set_position(position_component.get_absolute_position());
            }
        }
    }
}