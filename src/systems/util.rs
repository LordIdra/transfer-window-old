use nalgebra_glm::DVec2;

use crate::{state::State, storage::entity_allocator::Entity};

/// So... why is this an entire function? Surely we can just find the parent component and use that to set the new parent?
/// Well, the problem with that is that the old parent will still have the entity in its children
/// So we actually need to do 3 things
/// 1) update the parent component of the specified entity
/// 2) remove the specified entity from the children (celestial body) component of its old parent
/// 3) add the specified entity to the children (celestial body) component of its new parent
pub fn update_parent(state: &mut State, entity: &Entity, new_parent: &Entity) {
    let parent_component = state.components.parent_components.get_mut(entity).unwrap();
    let old_parent = parent_component.get_parent();
    if *new_parent != old_parent {
        state.components.celestial_body_components.get_mut(&old_parent).unwrap().remove_child(entity);
        state.components.celestial_body_components.get_mut(new_parent).unwrap().add_child(*entity);
        parent_component.set_parent(*new_parent);
    }
}

/// Takes in relative positions/velocities, turns them into absolute positions/velocities, and updates the entity's position/velocity components accordingly
pub fn update_position_and_velocity(state: &mut State, entity: &Entity, new_relative_position: DVec2, new_relative_velocity: DVec2) {
    let parent = state.components.parent_components.get(entity).unwrap().get_parent();
    let parent_absolute_position = state.components.position_components.get(&parent).unwrap().get_absolute_position();
    let parent_absolute_velocity = state.components.velocity_components.get(&parent).unwrap().get_absolute_velocity();
    state.components.position_components.get_mut(entity).unwrap().set_absolute_position(parent_absolute_position + new_relative_position);
    state.components.velocity_components.get_mut(entity).unwrap().set_absolute_velocity(parent_absolute_velocity + new_relative_velocity);
}

/// Sync the position, velocity, and parent of the entity to the position, velocity, and parent of the current orbit
pub fn sync_to_trajectory(state: &mut State, entity: &Entity) {
    let trajectory_component = state.components.trajectory_components.get(entity).unwrap();
    let current_orbit = trajectory_component.get_current_orbit();
    let new_position = current_orbit.get_current_unscaled_position();
    let new_velocity = current_orbit.get_current_velocity();
    let new_parent = current_orbit.get_parent();
    update_parent(state, entity, &new_parent);
    update_position_and_velocity(state, entity, new_position, new_velocity);
}


/// Recursively get all entities at a specific depth in the entity tree
pub fn get_all_entities_at_layer(state: &State, layer: i32, entities: &Vec<Entity>) -> Vec<Entity> {
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