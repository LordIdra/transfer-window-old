use std::{cell::RefCell, rc::Rc};

use crate::{state::State, storage::entity_allocator::Entity, components::{icon_component::IconType, trajectory_component::segment::burn::Burn}};

pub fn update_object_icon_position(state: &mut State, entity: Entity) {
    let parent = state.components.parent_components.get(&entity).unwrap().get_parent();
    let new_position = state.components.position_components.get(&parent).unwrap().get_absolute_position();
    state.components.position_components.get_mut(&entity).unwrap().set_absolute_position(new_position);
}

pub fn update_burn_icon_position(state: &mut State, entity: Entity, burn: Rc<RefCell<Burn>>) {
    let parent = state.components.parent_components.get(&entity).unwrap().get_parent();
    let parent_position = state.components.position_components.get(&parent).unwrap().get_absolute_position();
    let relative_position = burn.borrow().get_start_position();
    state.components.position_components.get_mut(&entity).unwrap().set_absolute_position(parent_position + relative_position);
}

pub fn icon_position_update_system(state: &mut State) {
    for entity in state.components.entity_allocator.get_entities() {
        if state.components.icon_components.get(&entity).is_some() {
            match state.components.icon_components.get(&entity).unwrap().get_type() {
                IconType::ObjectIcon => update_object_icon_position(state, entity),
                IconType::BurnIcon(burn) => update_burn_icon_position(state, entity, burn),
            }
        }
    }
}