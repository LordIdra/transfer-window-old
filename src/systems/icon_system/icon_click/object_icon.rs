use eframe::egui::{PointerButton, InputState};

use crate::{state::State, storage::entity_allocator::Entity, components::icon_component::IconState};

pub fn select_object_icon(state: &mut State, input: &InputState, selected_icon: Entity) {
    let new_selected_object = state.components.parent_components.get(&selected_icon).unwrap().get_parent();
    // If we're changing the selected object, recenter the camera to focus on that object
    if input.pointer.button_double_clicked(PointerButton::Primary) && new_selected_object != state.selected_object {
        state.selected_object = new_selected_object;
        state.camera.lock().unwrap().recenter();
    }
}

pub fn update_object_icon(state: &mut State, mouse_over: &Option<Entity>, entity: &Entity) {
    let icon_component = state.components.icon_components.get_mut(entity).unwrap();

    // If object is being hovered
    if let Some(mouse_over) = mouse_over {
        if *entity == *mouse_over {
            icon_component.set_state(IconState::Hovered);
            return;
        }
    }

    // If object is actively selected
    let parent = state.components.parent_components.get(entity).unwrap().get_parent();
    if parent == state.selected_object {
        icon_component.set_state(IconState::Selected);
        return;
    }

    // If object is not being hovered and is not actively selected
    icon_component.set_state(IconState::None)
}