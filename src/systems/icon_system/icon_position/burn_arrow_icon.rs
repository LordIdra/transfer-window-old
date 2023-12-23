use std::{cell::RefCell, rc::Weak};

use crate::{state::State, storage::entity_allocator::Entity, components::{trajectory_component::segment::burn::Burn, icon_component::BurnArrowIconType}};

pub fn update_burn_arrow_icon_position(state: &mut State, entity: Entity, burn: Weak<RefCell<Burn>>, _type: BurnArrowIconType) {
    let parent = state.components.parent_components.get(&entity).unwrap().get_parent();
    let parent_position = state.components.position_components.get(&parent).unwrap().get_absolute_position();
    let center_position = burn.upgrade().unwrap().borrow().get_start_point().get_position();
    let forward_direction = burn.upgrade().unwrap().borrow().get_tangent_direction();
    let relative_position = _type.get_relative_position(forward_direction, state.camera.lock().unwrap().get_zoom());
    state.components.icon_components.get_mut(&entity).unwrap().set_facing(relative_position.normalize());
    state.components.position_components.get_mut(&entity).unwrap().set_absolute_position(parent_position + center_position + relative_position);
}