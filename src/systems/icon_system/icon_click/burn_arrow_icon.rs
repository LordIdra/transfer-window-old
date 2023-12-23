use std::{rc::Weak, cell::RefCell};

use eframe::egui::{InputState, PointerButton};

use crate::{state::State, storage::entity_allocator::Entity, components::{icon_component::{IconState, BurnArrowIconType}, trajectory_component::segment::burn::Burn}};

pub fn select_burn_arrow_icon(state: &mut State, input: &InputState, icon: Entity, burn: Weak<RefCell<Burn>>, _type: BurnArrowIconType) {
    if !input.pointer.button_down(PointerButton::Primary) {
        return;
    }

}

pub fn update_burn_arrow_icon(state: &mut State, mouse_over: &Option<Entity>, entity: &Entity) {
    // If burn is being hovered
    if let Some(mouse_over) = mouse_over {
        if *entity == *mouse_over {
            state.components.icon_components.get_mut(entity).unwrap().set_state(IconState::Hovered);
            return;
        }
    }

    // If burn is not being hovered and is not actively selected
    state.components.icon_components.get_mut(entity).unwrap().set_state(IconState::None)
}