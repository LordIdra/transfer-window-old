use eframe::egui::{PointerButton, Context};
use nalgebra_glm::DVec2;

use crate::{state::State, storage::entity_allocator::Entity, util::get_root_entities};

/// Recursively get all entities at a specific depth in the entity tree
fn get_all_entities_at_layer(state: &State, layer: i32, entities: &Vec<Entity>) -> Vec<Entity> {
    // Base case; we're at the destination layer
    if layer == 0 {
        return entities.clone();
    }

    let mut new_entities = vec![];
    for entity in entities {
        new_entities.extend(state.components.celestial_body_components.get(entity).unwrap().get_children());
    }
    new_entities
}

fn get_closest_entity_to_point(state: &State, position: DVec2, entities: &Vec<Entity>) -> (Option<Entity>, f64) {
    let mut closest_distance_squared = f64::MAX;
    let mut closest_object = None;
    for child in entities {
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
    let mut layer = 0;
    let mut entities_at_layer = get_root_entities(state);
    loop {
        entities_at_layer = get_all_entities_at_layer(state, layer, &entities_at_layer);
        if entities_at_layer.is_empty() {
            // We've reached the final layer, since it contains no more entities, so no selected entity has been found
            return None;
        }

        let (closest_entity, closest_distance_squared) = get_closest_entity_to_point(state, position, &entities_at_layer);
        if let Some(closest_entity) = closest_entity {
            if closest_distance_squared < max_distance_to_select_squared {
                return Some(closest_entity);
            }
        }

        layer += 1;
    }
}

pub fn object_selection_system(state: &mut State, context: &Context) {
    let screen_rect = context.screen_rect();
    context.input(|input| {
        if !input.pointer.button_double_clicked(PointerButton::Primary) {
            return;
        };
        let Some(screen_position) = input.pointer.latest_pos() else {
            return;
        };
        let world_position = state.camera.lock().unwrap().window_space_to_world_space(screen_position, screen_rect);
        // This is necessary because we're about to compute distances in world spaces, and the maximum distance in world space to select depends on zoom
        let max_distance_to_select = state.camera.lock().unwrap().get_max_distance_to_select();
        let Some(selected) = breadth_first_radius_search(state, world_position, max_distance_to_select.powi(2)) else {
            return;
        };

        // If we're changing the selected object, recenter the camera to focus on that object
        if selected != state.selected {
            state.selected = selected;
            state.camera.lock().unwrap().recenter();
        }
    });
}