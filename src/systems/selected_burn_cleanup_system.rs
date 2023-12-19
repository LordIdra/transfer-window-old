use std::{cell::RefCell, rc::Rc};

use crate::{state::State, storage::entity_allocator::Entity, components::{icon_component::IconType, trajectory_component::segment::burn::Burn}};

pub fn cleanup_burn_icon(state: &mut State, burn: Rc<RefCell<Burn>>, entity: Entity) {
    if burn.borrow().get_start_time() < state.time {
        state.components.deallocate(entity);
        state.selected_burn = None;
    }
}

pub fn selected_burn_cleanup_system(state: &mut State) {
    for entity in state.components.entity_allocator.get_entities() {
        if let Some(icon_component) = state.components.icon_components.get(&entity) {
            match icon_component.get_type() {
                IconType::ObjectIcon => (),
                IconType::BurnIcon(burn) => cleanup_burn_icon(state, burn, entity),
            }
        }
    }
}