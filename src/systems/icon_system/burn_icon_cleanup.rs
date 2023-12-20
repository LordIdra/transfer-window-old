use std::{cell::RefCell, rc::Weak};

use crate::{state::State, storage::entity_allocator::Entity, components::{icon_component::IconType, trajectory_component::segment::burn::Burn}};

pub fn cleanup_burn_icon(state: &mut State, burn: Weak<RefCell<Burn>>, icon: Entity) {
    let Some(burn) = burn.upgrade() else {
        state.components.deallocate(icon);
        return;
    };

    if burn.borrow().get_start_point().get_time() < state.time {
        state.components.deallocate(icon);
        state.selected_burn_icon = None;
    }
}

pub fn burn_icon_cleanup_system(state: &mut State) {
    for icon in state.components.entity_allocator.get_entities() {
        if let Some(icon_component) = state.components.icon_components.get(&icon) {
            match icon_component.get_type() {
                IconType::ObjectIcon => (),
                IconType::BurnIcon(burn) => cleanup_burn_icon(state, burn, icon),
            }
        }
    }
}