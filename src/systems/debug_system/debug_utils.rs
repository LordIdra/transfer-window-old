use crate::{storage::entity_allocator::Entity, state::State};

pub fn get_entity_by_name(state: &mut State, name: String) -> Entity {
    for entity in state.components.entity_allocator.get_entities() {
        if let Some(name_component) = state.components.name_components.get(&entity) {
            if name_component.get_name() == name {
                return entity;
            };
        }
    }
    panic!();
}