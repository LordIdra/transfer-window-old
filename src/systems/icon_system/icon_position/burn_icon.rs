use std::{rc::Weak, cell::RefCell};

use crate::{state::State, storage::entity_allocator::Entity, components::trajectory_component::segment::burn::Burn};

pub fn update_burn_icon_position(state: &mut State, entity: Entity, burn: Weak<RefCell<Burn>>) {
    let parent = state.components.parent_components.get(&entity).unwrap().get_parent();
    let parent_position = state.components.position_components.get(&parent).unwrap().get_absolute_position();
    let relative_position = burn.upgrade().unwrap().borrow().get_start_point().get_position();
    state.components.position_components.get_mut(&entity).unwrap().set_absolute_position(parent_position + relative_position);
}