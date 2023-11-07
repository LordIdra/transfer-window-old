use eframe::egui::{PointerButton, Context};
use nalgebra_glm::DVec2;

use crate::{state::State, storage::entity_allocator::Entity, util::get_root_entities};

fn get_all_objects_at_layer(state: &State, layer: i32, entities: &Vec<Entity>) -> Vec<Entity> {
    if layer == 0 {
        return entities.clone();
    }

    let mut new_entities = vec![];
    for entity in entities {
        new_entities.extend(state.components.celestial_body_components.get(entity).unwrap().get_children());
    }
    new_entities
}

fn breadth_first_radius_search(state: &State, position: DVec2, max_distance_to_select_squared: f64) -> Option<Entity> {
    let mut layer = 0;
    let mut objects_at_layer = get_root_entities(state);
    loop {
        let mut closest_distance_squared = f64::MAX;
        let mut closest_object = None;
        objects_at_layer = get_all_objects_at_layer(state, layer, &objects_at_layer);
        if objects_at_layer.is_empty() {
            break;
        }
        for child in &objects_at_layer {
            let child_position = state.components.position_components.get(child).unwrap().get_absolute_position();
            let distance_squared = (child_position - position).magnitude_squared();
            if closest_distance_squared > distance_squared {
                closest_distance_squared = distance_squared;
                closest_object = Some(*child)
            }
        }
        if closest_object.is_some() && closest_distance_squared < max_distance_to_select_squared {
            return closest_object;
        }
        layer += 1;
    }
    None
}

pub fn get_selected_object(state: &State, world_position: DVec2, max_distance_to_select: f64) -> Option<Entity> {
    breadth_first_radius_search(state, world_position, max_distance_to_select.powi(2))
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
        let max_distance_to_select = state.camera.lock().unwrap().get_max_distance_to_select();
        let Some(selected) = get_selected_object(state, world_position, max_distance_to_select) else {
            return;
        };
        if selected != state.selected {
            state.selected = selected;
            state.camera.lock().unwrap().recenter();
        }
    });
}