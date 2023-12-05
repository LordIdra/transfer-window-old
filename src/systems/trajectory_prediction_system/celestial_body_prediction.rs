use crate::{components::trajectory_component::segment::Segment, state::State, storage::entity_allocator::Entity, systems::util::{update_position_and_velocity, sync_to_trajectory}};

use super::util::{is_celestial_body, update_parent_for_prediction};

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

fn update_for_prediction(state: &mut State, entity: &Entity, time: f64) {
    let trajectory_component = state.components.trajectory_components.get_mut(entity).unwrap();
    trajectory_component.predict(SIMULATION_TIME_STEP);
    match trajectory_component.get_final_segment() {
        Segment::Burn(_) => {
            // We should never encounter this situation
            panic!("Attempt to update a burn segment for celestial body prediction")
        }
        Segment::Orbit(orbit) => {
            let new_position = orbit.borrow().get_end_position();
            let new_velocity = orbit.borrow().get_end_velocity();
            update_position_and_velocity(state, entity, new_position, new_velocity);
        }
    }
    update_parent_for_prediction(state, Box::new(get_sphere_of_influence), entity, time + SIMULATION_TIME_STEP);
}

pub fn predict_celestial_objects(state: &mut State, end_time: f64) {
    let mut time = state.time;
    let time_steps = ((end_time - time) / SIMULATION_TIME_STEP) as usize;
    for _ in 0..time_steps {
        for entity in state.components.entity_allocator.get_entities() {
            if is_celestial_body(state, entity) {
                update_for_prediction(state, &entity, time);
            }
        }
        time += SIMULATION_TIME_STEP;
    }

    // Reset the position, velocity, and parent of all entities, since they are changed during prediction
    for entity in state.components.entity_allocator.get_entities() {
        if is_celestial_body(state, entity) {
            sync_to_trajectory(state, &entity);
        }
    }
}
