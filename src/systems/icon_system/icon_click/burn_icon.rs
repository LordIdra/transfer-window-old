use std::{rc::Weak, cell::RefCell};

use eframe::egui::{InputState, PointerButton};

use crate::{state::{State, Selected}, storage::{entity_allocator::Entity, entity_builder::build_burn_arrow_icon}, components::{trajectory_component::segment::burn::Burn, icon_component::{IconState, BurnArrowIconType}}};

pub fn select_burn_icon(state: &mut State, input: &InputState, new_selected_burn_icon: Entity) {
    if !input.pointer.button_clicked(PointerButton::Primary) {
        return;
    }

    if let Selected::BurnIcon(selected_icon) = state.selected {
        if new_selected_burn_icon == selected_icon {
            let burn = state.components.icon_components.get(&selected_icon).unwrap().get_type().as_burn_icon();
            let parent = state.components.parent_components.get(&selected_icon).unwrap().get_parent();
            build_burn_arrow_icon(&mut state.components, burn.clone(), BurnArrowIconType::FRONT, parent);
            build_burn_arrow_icon(&mut state.components, burn.clone(), BurnArrowIconType::RIGHT, parent);
            build_burn_arrow_icon(&mut state.components, burn.clone(), BurnArrowIconType::BACK, parent);
            build_burn_arrow_icon(&mut state.components, burn.clone(), BurnArrowIconType::LEFT, parent);
            return;
        }
    }

    state.selected = Selected::BurnIcon(new_selected_burn_icon);
}

pub fn update_burn_icon(state: &mut State, mouse_over: &Option<Entity>, burn: Weak<RefCell<Burn>>, entity: &Entity) {
    // If burn is being hovered
    if let Some(mouse_over) = mouse_over {
        if *entity == *mouse_over {
            state.components.icon_components.get_mut(entity).unwrap().set_state(IconState::Hovered);
            return;
        }
    }

    // If burn is actively selected
    if let Selected::BurnIcon(selected_burn_icon) = state.selected.clone() {
        let selected_burn = state.components.icon_components.get(&selected_burn_icon).unwrap().get_type().as_burn_icon();
        if burn.as_ptr() == selected_burn.as_ptr() { // Compare the burns pointed to by the icons to check if they're the same icon
            state.components.icon_components.get_mut(entity).unwrap().set_state(IconState::Selected);
            return;
        }
    }


    // If burn is not being hovered and is not actively selected
    state.components.icon_components.get_mut(entity).unwrap().set_state(IconState::None)
}