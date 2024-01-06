use eframe::egui::Context;
use nalgebra_glm::DVec2;

use crate::{state::State, storage::entity_allocator::Entity, components::icon_component::IconType};

use self::{burn_icon::{update_burn_icon, select_burn_icon}, object_icon::{update_object_icon, select_object_icon}, burn_arrow_icon::{select_burn_arrow_icon, update_burn_arrow_icon}};

mod burn_arrow_icon;
mod burn_icon;
mod object_icon;

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

fn update_icons(state: &mut State, mouse_over: &Option<Entity>) {
    for entity in &state.components.entity_allocator.get_entities() {
        if let Some(icon_component) = state.components.icon_components.get(entity) {
            match icon_component.get_type() {
                IconType::ObjectIcon => update_object_icon(state, mouse_over, entity),
                IconType::BurnIcon(burn) => update_burn_icon(state, mouse_over, burn, entity),
                IconType::BurnArrowIcon(_, _) => update_burn_arrow_icon(state, mouse_over, entity),
            }
        }
    }
}

pub fn icon_click(state: &mut State, context: &Context) {
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
            state.mouse_over_any_icon = true;
            match state.components.icon_components.get(&selected_icon).unwrap().get_type() {
                IconType::ObjectIcon => select_object_icon(state, input, selected_icon),
                IconType::BurnIcon(_) => select_burn_icon(state, input, selected_icon),
                IconType::BurnArrowIcon(burn, _type) => select_burn_arrow_icon(state, input, burn, _type),
            }
        };        
    });
}