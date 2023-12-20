use std::{rc::Weak, cell::RefCell};

use eframe::egui::{PointerButton, Context, InputState};
use nalgebra_glm::DVec2;

use crate::{state::State, storage::entity_allocator::Entity, components::{icon_component::{IconState, IconType}, trajectory_component::segment::burn::Burn}};

fn get_closest_icon_to_point(state: &State, position: DVec2) -> (Option<Entity>, f64) {
    let mut closest_distance_squared = f64::MAX;
    let mut closest_object = None;
    for child in state.components.entity_allocator.get_entities() {
        if let Some(icon_component) = state.components.icon_components.get(&child) {
            if !icon_component.is_visible() {
                continue;
            }
            let child_position = state.components.position_components.get(&child).unwrap().get_absolute_position();
            let distance_squared = (child_position - position).magnitude_squared();
            if closest_distance_squared > distance_squared {
                closest_distance_squared = distance_squared;
                closest_object = Some(child)
            }
        }
    }
    (closest_object, closest_distance_squared)
}

fn find_closest_icon_to_mouse(state: &State, position: DVec2, max_distance_to_select_squared: f64) -> Option<Entity> {
    let (closest_entity, closest_distance_squared) = get_closest_icon_to_point(state, position);
    if let Some(closest_entity) = closest_entity {
        if closest_distance_squared < max_distance_to_select_squared {
            return Some(closest_entity);
        }
    }
    None
}

fn update_object_icon(state: &mut State, mouse_over: &Option<Entity>, entity: &Entity) {
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

fn update_burn_icon(state: &mut State, mouse_over: &Option<Entity>, burn: Weak<RefCell<Burn>>, entity: &Entity) {

    // If burn is being hovered
    if let Some(mouse_over) = mouse_over {
        if *entity == *mouse_over {
            state.components.icon_components.get_mut(entity).unwrap().set_state(IconState::Hovered);
            return;
        }
    }

    // If burn is actively selected
    if let Some(selected_burn_icon) = state.selected_burn_icon.clone() {
        let selected_burn = state.components.icon_components.get(&selected_burn_icon).unwrap().get_type().as_burn_icon();
        if burn.as_ptr() == selected_burn.as_ptr() {
            state.components.icon_components.get_mut(entity).unwrap().set_state(IconState::Selected);
            return;
        }
    }

    // If burn is not being hovered and is not actively selected
    state.components.icon_components.get_mut(entity).unwrap().set_state(IconState::None)
}

fn update_icons(state: &mut State, mouse_over: &Option<Entity>) {
    for entity in &state.components.entity_allocator.get_entities() {
        if let Some(icon_component) = state.components.icon_components.get(entity) {
            match icon_component.get_type() {
                IconType::ObjectIcon => update_object_icon(state, mouse_over, entity),
                IconType::BurnIcon(burn) => update_burn_icon(state, mouse_over, burn, entity),
            }
        }
    }
}

fn select_object(state: &mut State, input: &InputState, selected_icon: Entity) {
    let new_selected_object = state.components.parent_components.get(&selected_icon).unwrap().get_parent();
    // If we're changing the selected object, recenter the camera to focus on that object
    if input.pointer.button_double_clicked(PointerButton::Primary) && new_selected_object != state.selected_object {
        state.selected_object = new_selected_object;
        state.camera.lock().unwrap().recenter();
    }
}

fn select_burn(state: &mut State, input: &InputState, new_selected_burn_icon: Entity) {
    if input.pointer.button_double_clicked(PointerButton::Primary) {
        state.selected_burn_icon = Some(new_selected_burn_icon);
    }
}

pub fn icon_click_system(state: &mut State, context: &Context) {
    if state.mouse_over_any_element {
        return;
    }

    let screen_rect = context.screen_rect();
    context.input(|input| {
        let Some(screen_position) = input.pointer.latest_pos() else {
            return;
        };
        let world_position = state.camera.lock().unwrap().window_space_to_world_space(screen_position, screen_rect);
        // This is necessary because we're about to compute distances in world spaces, and the maximum distance in world space to select depends on zoom
        let max_distance_to_select = state.camera.lock().unwrap().get_max_distance_to_select();
        let mouse_over = find_closest_icon_to_mouse(state, world_position, max_distance_to_select.powi(2));

        update_icons(state, &mouse_over);

        if let Some(selected_icon) = mouse_over {
            match state.components.icon_components.get(&selected_icon).unwrap().get_type() {
                IconType::ObjectIcon => select_object(state, input, selected_icon),
                IconType::BurnIcon(_) => select_burn(state, input, selected_icon),
            }
        };        
    });
}