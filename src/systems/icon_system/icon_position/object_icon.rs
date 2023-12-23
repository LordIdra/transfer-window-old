use crate::{state::State, storage::entity_allocator::Entity};

pub fn update_object_icon_position(state: &mut State, entity: Entity) {
    let parent = state.components.parent_components.get(&entity).unwrap().get_parent();
    let new_position = state.components.position_components.get(&parent).unwrap().get_absolute_position();
    state.components.position_components.get_mut(&entity).unwrap().set_absolute_position(new_position);
}