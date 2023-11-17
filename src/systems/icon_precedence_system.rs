use crate::{state::State, storage::entity_allocator::Entity, util::get_root_entities};

use super::util::get_all_entity_children;

fn entities_overlap(state: &mut State, entity: &Entity, other_entity: &Entity) -> bool {
    let closest_allowed_distance = state.camera.lock().unwrap().get_max_distance_to_select() * 2.0;
    let entity_position = state.components.position_components.get(entity).unwrap().get_absolute_position();
    let entity_mass = state.components.mass_components.get(entity).unwrap().get_mass();
    let other_entity_position = state.components.position_components.get(other_entity).unwrap().get_absolute_position();
    let other_entity_mass = state.components.mass_components.get(other_entity).unwrap().get_mass();
    let distance = (entity_position - other_entity_position).magnitude();
    distance < closest_allowed_distance && entity_mass < other_entity_mass
}

fn is_icon_overlapping(state: &mut State, entity: &Entity, entities_at_layer: &Vec<Entity>) -> bool {
    // Check proximity to parent icon
    if let Some(parent_component) = state.components.parent_components.get(entity) {
        if entities_overlap(state, entity, &parent_component.get_parent()) {
            return true;
        }
    }

    // Now check all children
    for other_entity in entities_at_layer {
        if *entity == *other_entity {
            continue;
        }
        if entities_overlap(state, entity, other_entity) {
            return true;
        }
    }
    false
}

fn do_icon_precedence_for_layer(state: &mut State, entities_at_layer: &Vec<Entity>) {
    for entity in entities_at_layer {
        if let Some(parent_component) = state.components.parent_components.get(entity) {
            let parent_visible = state.components.icon_components.get(&parent_component.get_parent()).unwrap().is_visible();
            if !parent_visible {
                state.components.icon_components.get_mut(entity).unwrap().set_visible(false);
                continue;
            }
            let is_visible = !is_icon_overlapping(state, entity, entities_at_layer);
            state.components.icon_components.get_mut(entity).unwrap().set_visible(is_visible);
        }
    }
}


/// Hides icons that overlap other icons using two rules
/// 1) Icons that are higher in the hierarchy take precedence (eg, earth takes precedence over moon)
/// 2) If there are multiple icons at the same layer in the hierarchy, the one with the greatest mass takes precedence
/// To do this, we traverse each layer and compute whether the children should be shown or not
/// In each layer, if the parent is hidden, all the children are hidden
/// Otherwise, we check the distance of the entity to all other children of its parent to determine whether it should be hidden
pub fn icon_precedence_system(state: &mut State) {
    let mut entities = get_root_entities(state);
    loop {
        if entities.is_empty() {
            // We've reached the final layer, since it contains no more entities, so no selected entity has been found
            return;
        }
        do_icon_precedence_for_layer(state, &entities);
        entities = get_all_entity_children(state, &entities);
    }
}