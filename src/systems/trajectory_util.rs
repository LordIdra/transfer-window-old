use nalgebra_glm::DVec2;

use crate::{state::State, storage::entity_allocator::Entity};

pub fn update_parent(state: &mut State, entity: &Entity, new_parent: &Entity) {
    let parent_component = state.components.parent_components.get_mut(entity).unwrap();
    let old_parent = parent_component.get_parent();
    if *new_parent != old_parent {
        state.components.celestial_body_components.get_mut(&old_parent).unwrap().remove_child(entity);
        state.components.celestial_body_components.get_mut(new_parent).unwrap().add_child(*entity);
        parent_component.set_parent(*new_parent);
    }
}

pub fn update_position_and_velocity(state: &mut State, entity: &Entity, new_relative_position: DVec2, new_relative_velocity: DVec2) {
    let parent = state.components.parent_components.get(entity).unwrap().get_parent();
    let parent_absolute_position = state.components.position_components.get(&parent).unwrap().get_absolute_position();
    let parent_absolute_velocity = state.components.velocity_components.get(&parent).unwrap().get_absolute_velocity();
    state.components.position_components.get_mut(entity).unwrap().set_absolute_position(parent_absolute_position + new_relative_position);
    state.components.velocity_components.get_mut(entity).unwrap().set_absolute_velocity(parent_absolute_velocity + new_relative_velocity);
}

pub fn sync_to_trajectory(state: &mut State, entity: &Entity) {
    let trajectory_component = state.components.trajectory_components.get(entity).unwrap();
    let current_orbit = trajectory_component.get_current_orbit();
    let new_position = current_orbit.get_current_unscaled_position();
    let new_velocity = current_orbit.get_current_velocity();
    let new_parent = current_orbit.get_parent();
    update_parent(state, entity, &new_parent);
    update_position_and_velocity(state, entity, new_position, new_velocity);
}

