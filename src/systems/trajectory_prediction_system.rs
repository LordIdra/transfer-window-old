use nalgebra_glm::DVec2;

use crate::{state::State, components::trajectory_component::{orbit::Orbit, orbit_direction::GRAVITATIONAL_CONSTANT}, storage::entity_allocator::Entity};

use super::util::{update_parent, update_position_and_velocity, sync_to_trajectory};

const SIMULATION_TIME_STEP: f64 = 40.0;
const SIMULATION_TIME_STEPS: i32 = 200000;

fn position_relative_to_parent(state: &State, entity: &Entity, parent: &Entity) -> DVec2 {
    state.components.position_components.get(entity).unwrap().get_absolute_position() - state.components.position_components.get(parent).unwrap().get_absolute_position()
}

fn velocity_relative_to_parent(state: &State, entity: &Entity, parent: &Entity) -> DVec2 {
    state.components.velocity_components.get(entity).unwrap().get_absolute_velocity() - state.components.velocity_components.get(parent).unwrap().get_absolute_velocity()
}

fn change_parent(state: &mut State, entity: &Entity, new_parent: Entity, time: f64) {
    let new_position = position_relative_to_parent(state, entity, &new_parent);
    let new_velocity = velocity_relative_to_parent(state, entity, &new_parent);
    let new_orbit = Orbit::new(&state.components, new_parent, new_position, new_velocity, time);
    state.components.trajectory_components.get_mut(entity).unwrap().add_orbit(new_orbit);
}

fn get_sphere_of_influence_squared(state: &State, entity: &Entity) -> Option<f64> {
    let trajectory = state.components.trajectory_components.get(entity)?;
    let final_orbit = trajectory.get_final_orbit();
    let final_parent = final_orbit.borrow().get_parent();
    let semi_major_axis = final_orbit.borrow().get_semi_major_axis();
    let mass = state.components.mass_components.get(entity)?.get_mass();
    let parent_mass = state.components.mass_components.get(&final_parent)?.get_mass();
    Some((semi_major_axis * (mass / parent_mass).powf(2.0 / 5.0)).powi(2))
}

/// Looks to ascend HIGHER into the entity tree to compute a parent
/// For example, this could be a spacecraft leaving the Earth's SOI to enter the Sun's SOI
/// In this case, there's only one possible new parent (ie, the current parent's parent), making this fairly simple
fn compute_new_parent_upper(state: &State, entity: &Entity, parent: &Entity) -> Option<Entity> {
    // Check if we've left the SOI of our parent
    let parent_sphere_of_influence_squared = get_sphere_of_influence_squared(state, parent)?;
    if position_relative_to_parent(state, entity, parent).magnitude_squared() < parent_sphere_of_influence_squared {
        return None;
    }
    state.components.parent_components.get(parent).map(|parent_parent| parent_parent.get_parent())
}

fn entity_causing_highest_acceleration(state: &State, entity: &Entity, entities: Vec<Entity>) -> Option<Entity> {
    let highest_acceleration = 0.0;
    let mut object_causing_highest_acceleration = None;
    for other_entity in &entities {
        let position = state.components.position_components.get(entity).unwrap().get_absolute_position();
        let other_position = state.components.position_components.get(other_entity).unwrap().get_absolute_position();
        let other_mass = state.components.mass_components.get(other_entity).unwrap().get_mass();
        let acceleration = other_mass * GRAVITATIONAL_CONSTANT / (position - other_position).magnitude_squared();
        if acceleration > highest_acceleration {
            object_causing_highest_acceleration = Some(*other_entity);
        }
    }
    object_causing_highest_acceleration
}

/// Looks to descend LOWER into the entity tree to compute a parent
/// For example, this could be a spacecraft leaving the Earth's SOI to enter the Moon's SOI
/// This means there could be multiple entities to check
/// Also, it's possible we'll be in the SOI of several entities
/// In this case, we calculate which entity is causing the highest acceleration and choose that one
fn compute_new_parent_lower(state: &State, entity: &Entity, parent: &Entity) -> Option<Entity> {
    // Check if we've entered the SOI of any objects with the same parent
    let mut potential_children = vec![];
    for child in state.components.celestial_body_components.get(parent).unwrap().get_children() {
        if *child == *entity {
            continue;
        }
        if let Some(parent_sphere_of_influence_squared) = get_sphere_of_influence_squared(state, child) {
            let position = state.components.position_components.get(entity).unwrap().get_absolute_position();
            let other_position = state.components.position_components.get(child).unwrap().get_absolute_position();
            if (position - other_position).magnitude_squared() < parent_sphere_of_influence_squared {
                potential_children.push(*child);
            }
        }
    }
    entity_causing_highest_acceleration(state, entity, potential_children)
}

fn update_parent_for_prediction(state: &mut State, entity: &Entity, time: f64) {
    if let Some(parent_component) = state.components.parent_components.get(entity) {
        let parent = parent_component.get_parent();
        if let Some(new_parent) = compute_new_parent_lower(state, entity, &parent) {
            change_parent(state, entity, new_parent, time);
            update_parent(state, entity, &new_parent);
        } else if let Some(new_parent) = compute_new_parent_upper(state, entity, &parent) {
            change_parent(state, entity, new_parent, time);
            update_parent(state, entity, &new_parent);
        }
    }
}

pub fn update_for_prediction(state: &mut State, entity: &Entity, time: f64) {
    if let Some(trajectory_component) = state.components.trajectory_components.get_mut(entity) {
        trajectory_component.predict(SIMULATION_TIME_STEP);
        let final_orbit = trajectory_component.get_final_orbit();
        let new_position = final_orbit.borrow().get_end_position();
        let new_velocity = final_orbit.borrow().get_end_velocity();
        update_position_and_velocity(state, entity, new_position, new_velocity);
        // We add SIMULATION_TIME_STEP here because we've just updated position and velocity, so the time step has actually already increased
        update_parent_for_prediction(state, entity, time + SIMULATION_TIME_STEP);
    }
}

pub fn trajectory_prediction_system(state: &mut State) {
    let mut time = 0.0;
    for _ in 0..SIMULATION_TIME_STEPS {
        for entity in state.components.entity_allocator.get_entities() {
            update_for_prediction(state, &entity, time);
        }
        time += SIMULATION_TIME_STEP;
    }

    // Reset the position, velocity, and parent of all entities, since they are changed during prediction
    for entity in state.components.entity_allocator.get_entities() {
        if state.components.trajectory_components.get(&entity).is_some() {
            sync_to_trajectory(state, &entity);
        }
    }
}