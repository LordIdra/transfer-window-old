use crate::{state::State, systems::util::{sync_to_trajectory, update_position_and_velocity, get_segment_at_time}, storage::entity_allocator::Entity, components::trajectory_component::segment::Segment};

use super::{util::{is_spacecraft, update_parent_for_prediction, SoiFunction}, sync::{sync_celestial_bodies_to_time, sync_entity_to_time}};

const SIMULATION_TIME_STEP: f64 = 40.0;

fn make_soi_function(time: f64) -> SoiFunction {
    Box::new(move |state: &State, entity: &Entity| {
        let segment = get_segment_at_time(state, entity, time);
        let orbit = segment.as_orbit();
        let parent = orbit.borrow().get_parent();
        let semi_major_axis = orbit.borrow().get_semi_major_axis();
        let mass = state.components.mass_components.get(entity)?.get_mass();
        let parent_mass = state.components.mass_components.get(&parent)?.get_mass();
        println!("sma {}", semi_major_axis);
        Some(semi_major_axis * (mass / parent_mass).powf(2.0 / 5.0))
    })
}

fn update_for_prediction(state: &mut State, entity: &Entity, time: f64) {
    if let Some(trajectory_component) = state.components.trajectory_components.get_mut(entity) {
        trajectory_component.predict(SIMULATION_TIME_STEP);
        match trajectory_component.get_final_segment() {
            Segment::Burn(_) => {
                todo!()
            }
            Segment::Orbit(orbit) => {
                let new_position = orbit.borrow().get_end_position();
                let new_velocity = orbit.borrow().get_end_velocity();
                update_position_and_velocity(state, entity, new_position, new_velocity);
            }
        }
        update_parent_for_prediction(state, make_soi_function(time), entity, time + SIMULATION_TIME_STEP);
    }
}

pub fn predict_spacecraft_objects(state: &mut State, end_time: f64) {
    let mut time = state.time;
    let time_steps = ((end_time - time) / SIMULATION_TIME_STEP) as usize;

    for entity in state.components.entity_allocator.get_entities() {
        if is_spacecraft(state, entity) {
            state.components.trajectory_components.get_mut(&entity).unwrap().remove_segments_after(time)
        }
    }

    for _ in 0..time_steps {
        sync_celestial_bodies_to_time(state, time);
        for entity in state.components.entity_allocator.get_entities() {
            if is_spacecraft(state, entity) {
                update_for_prediction(state, &entity, time);
            }
        }
        time += SIMULATION_TIME_STEP;
    }

    // Reset the position, velocity, and parent of all entities, since they are changed during prediction
    for entity in state.components.entity_allocator.get_entities() {
        if is_spacecraft(state, entity) {
            sync_entity_to_time(state, &entity, state.time);
            sync_to_trajectory(state, &entity);
        }
    }
}
