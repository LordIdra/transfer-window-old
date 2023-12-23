use std::{cell::RefCell, rc::Weak};

use crate::{state::State, storage::entity_allocator::Entity, components::{icon_component::IconType, trajectory_component::segment::burn::Burn}, systems::util::delete_burn_icon};

pub fn cleanup_burn_icon(state: &mut State, burn: Weak<RefCell<Burn>>, icon: Entity) {
    let Some(burn) = burn.upgrade() else {
        delete_burn_icon(state, icon);
        return;
    };

    if burn.borrow().get_start_point().get_time() < state.time {
        delete_burn_icon(state, icon);
    }
}

pub fn burn_icon_cleanup(state: &mut State) {
    for icon in state.components.entity_allocator.get_entities() {
        if let Some(icon_component) = state.components.icon_components.get(&icon) {
            if let IconType::BurnIcon(burn) = icon_component.get_type() {
                cleanup_burn_icon(state, burn, icon);
            }
        }
    }
}