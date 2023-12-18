use eframe::egui::{PointerButton, Context};
use nalgebra_glm::DVec2;

use crate::{state::State, storage::entity_allocator::Entity, util::get_root_entities, components::icon_component::IconState};

use super::util::get_all_entity_children;

fn get_closest_entity_to_point(state: &State, position: DVec2, entities: &Vec<Entity>) -> (Option<Entity>, f64) {
    let mut closest_distance_squared = f64::MAX;
    let mut closest_object = None;
    for child in entities {
        let icon_visible = state.components.icon_components.get(&child).unwrap().is_visible();
        if !icon_visible {
            continue;
        }
        let child_position = state.components.position_components.get(child).unwrap().get_absolute_position();
        let distance_squared = (child_position - position).magnitude_squared();
        if closest_distance_squared > distance_squared {
            closest_distance_squared = distance_squared;
            closest_object = Some(*child)
        }
    }
    (closest_object, closest_distance_squared)
}

/// Search all entities to find if any are close enough to the clicked position to be selected
/// This is done in a breadth-first way - ie, we first check all root objects, then all their children, etc
/// This is so we don't end up, for example, selecting the moon when we double click what looks like the Earth at a distance
fn breadth_first_radius_search(state: &State, position: DVec2, max_distance_to_select_squared: f64) -> Option<Entity> {
    let mut entities = get_root_entities(state);
    loop {
        if entities.is_empty() {
            // We've reached the final layer, since it contains no more entities, so no selected entity has been found
            return None;
        }

        let (closest_entity, closest_distance_squared) = get_closest_entity_to_point(state, position, &entities);
        if let Some(closest_entity) = closest_entity {
            if closest_distance_squared < max_distance_to_select_squared {
                return Some(closest_entity);
            }
        }

        entities = get_all_entity_children(state, &entities);
    }
}

fn update_icons(state: &mut State, selected: &Option<Entity>) {
    for entity in &state.components.entity_allocator.get_entities() {
        if let Some(icon_component) = state.components.icon_components.get_mut(entity) {
            // If entity is being hovered
            if let Some(selected) = selected {
                if *entity == *selected {
                    icon_component.set_state(IconState::Hovered);
                    continue;
                }
            }

            // If entity is actively selected
            if *entity == state.selected_entity {
                icon_component.set_state(IconState::Selected);
                continue;
            }

            // If entity is not being hovered and is not actively selected
            icon_component.set_state(IconState::None)
        }
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
        let selected = breadth_first_radius_search(state, world_position, max_distance_to_select.powi(2));

        update_icons(state, &selected);

        if let Some(selected) = selected {
            // If we're changing the selected object, recenter the camera to focus on that object
            if input.pointer.button_double_clicked(PointerButton::Primary) && selected != state.selected_entity {
                state.selected_entity = selected;
                state.camera.lock().unwrap().recenter();
            }
        };        
    });
}