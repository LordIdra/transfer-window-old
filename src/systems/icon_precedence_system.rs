use crate::{state::State, storage::entity_allocator::Entity};

fn icons_overlapping(state: &mut State, entity: Entity, other_entity: Entity) -> bool {
    let closest_allowed_distance = state.camera.lock().unwrap().get_max_distance_to_select() * 2.0;
    let icon_position = state.components.position_components.get(&entity).unwrap().get_absolute_position();
    let other_entity_position = state.components.position_components.get(&other_entity).unwrap().get_absolute_position();
    let distance = (icon_position - other_entity_position).magnitude();
    distance < closest_allowed_distance
}

fn do_icon_precedence(state: &mut State, entity: Entity, other_entity: Entity) {
    let parent = state.components.parent_components.get(&entity).unwrap().get_parent();
    let mass = state.components.mass_components.get(&parent).unwrap().get_mass();
    let other_parent = state.components.parent_components.get(&other_entity).unwrap().get_parent();
    let other_mass = state.components.mass_components.get(&other_parent).unwrap().get_mass();

    if parent == state.selected_object {
        state.components.icon_components.get_mut(&other_entity).unwrap().set_visible(false);
        return;
    }

    if parent == other_parent {
        let icon_type = state.components.icon_components.get(&entity).unwrap().get_type();
        let other_icon_type = state.components.icon_components.get(&other_entity).unwrap().get_type();
        if icon_type.takes_precedence_over(other_icon_type) {
            state.components.icon_components.get_mut(&other_entity).unwrap().set_visible(false);
        } else {
            state.components.icon_components.get_mut(&entity).unwrap().set_visible(false);
        }
        return;
    }

    if other_parent == state.selected_object {
        state.components.icon_components.get_mut(&entity).unwrap().set_visible(false);
        return;
    }

    if mass < other_mass {
        state.components.icon_components.get_mut(&entity).unwrap().set_visible(false);
    } else {
        state.components.icon_components.get_mut(&other_entity).unwrap().set_visible(false);
    }
}

/// Icon precedence is hard :(
/// What we're going to do is go through each entity and (if they have an icon) set all of their icons to visible
/// Then we'll iterate again, and if the entity has an icon, we're going to compare it against all the other
/// entities that have icons
/// If there are any overlaps, we hide the icon belonging to the entity with lower mass
/// Not the most efficient algorithm by a long shot, but the most simple one I could come up with
pub fn icon_precedence_system(state: &mut State) {
    for entity in state.components.entity_allocator.get_entities() {
        if let Some(icon_component) = state.components.icon_components.get_mut(&entity) {
            icon_component.set_visible(true);
        }
    }
    for entity in state.components.entity_allocator.get_entities() {
        if state.components.icon_components.get(&entity).is_some() {
            for other_entity in state.components.entity_allocator.get_entities() {
                if state.components.icon_components.get(&other_entity).is_some() {
                    if !icons_overlapping(state, entity, other_entity) || entity == other_entity {
                        continue;
                    }
                    do_icon_precedence(state, entity, other_entity);
                }
            }
        }
    }

}