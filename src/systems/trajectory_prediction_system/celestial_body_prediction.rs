use eframe::epaint::ahash::{HashSet, HashSetExt};

use crate::{components::trajectory_component::segment::Segment, state::State, storage::entity_allocator::Entity, systems::util::{update_position_and_velocity, sync_to_trajectory, is_celestial_body_with_trajectory, sync_celestial_bodies_to_time}, util::get_root_entities};

use super::util::update_parent_for_prediction;

const SIMULATION_TIME_STEP: f64 = 40.0;

fn get_sphere_of_influence(state: &State, entity: &Entity) -> Option<f64> {
    let trajectory = state.components.trajectory_components.get(entity)?;
    let final_segment = trajectory.get_final_segment();
    let final_orbit = final_segment.as_orbit();
    let final_parent = final_orbit.borrow().get_parent();
    let semi_major_axis = final_orbit.borrow().get_semi_major_axis();
    let mass = state.components.mass_components.get(entity)?.get_mass();
    let parent_mass = state.components.mass_components.get(&final_parent)?.get_mass();
    Some(semi_major_axis * (mass / parent_mass).powf(2.0 / 5.0))
}

fn update_children(state: &mut State, entity: Entity, time: f64, updated_entities: &mut HashSet<Entity>) {
    if let Some(celestial_body_component) = state.components.celestial_body_components.get(&entity) {
        for child in celestial_body_component.get_children().clone() {
            // We need to keep track of which children have been updated... but why? Isn't this just recursively iterating all children?
            // Well, yes, but the children may actually change while this is in progress
            // So there is a feasible scenario where an object could move down into the tree and thus be iterated over twice
            if !is_celestial_body_with_trajectory(state, child) || updated_entities.contains(&child) {
                continue;
            }
            updated_entities.insert(child);
            update_for_prediction(state, child, time);
            update_children(state, child, time, updated_entities);
        }
    }
}

fn update_for_prediction(state: &mut State, entity: Entity, time: f64) {
    let trajectory_component = state.components.trajectory_components.get_mut(&entity).unwrap();
    trajectory_component.predict(SIMULATION_TIME_STEP);
    match trajectory_component.get_final_segment() {
        Segment::Burn(_) => {
            // We should never encounter this situation
            panic!("Attempt to update a burn segment for celestial body prediction")
        }
        Segment::Orbit(orbit) => {
            let new_position = orbit.borrow().get_end_position();
            let new_velocity = orbit.borrow().get_end_velocity();
            update_position_and_velocity(state, &entity, new_position, new_velocity);
        }
    }
    update_parent_for_prediction(state, Box::new(get_sphere_of_influence), entity, time, None);
}

pub fn predict_celestial_bodies(state: &mut State, end_time: f64) {
    let mut time = state.time;
    let time_steps = ((end_time - time) / SIMULATION_TIME_STEP) as usize;
    for _ in 0..time_steps {
        // Recursively predict all bodies, making sure a parent is updated before its children
        // If this is not done, strange non-deterministic behaviour seems to arise since
        // the objects are stored and therefore iterated in a random order
        for entity in get_root_entities(&state) {
            update_children(state, entity, time, &mut HashSet::new());
        }
        time += SIMULATION_TIME_STEP;
    }

    // Reset the position, velocity, and parent of all entities, since they are changed during prediction
    sync_celestial_bodies_to_time(state, state.time);
}
