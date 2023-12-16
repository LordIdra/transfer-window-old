use crate::{state::State, systems::util::{update_position_and_velocity, get_segment_at_time, is_spacecraft, sync_celestial_bodies_to_time, sync_entity_to_time, update_parent}, storage::entity_allocator::Entity, components::trajectory_component::segment::Segment};

use super::util::{SoiFunction, update_parent_for_prediction};

const SIMULATION_TIME_STEP: f64 = 40.0;

fn make_soi_function(time: f64) -> SoiFunction {
    Box::new(move |state: &State, entity: &Entity| {
        if state.components.trajectory_components.get(entity).is_none() {
            // Root entity, so SOI infinite
            return Some(f64::MAX);
        }
        let segment = get_segment_at_time(state, entity, time);
        let orbit = segment.as_orbit();
        let parent = orbit.borrow().get_parent();
        let semi_major_axis = orbit.borrow().get_semi_major_axis();
        let mass = state.components.mass_components.get(entity)?.get_mass();
        let parent_mass = state.components.mass_components.get(&parent)?.get_mass();
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
                // let new_parent = orbit.borrow().get_parent();
                // update_parent(state, entity, &new_parent);
                let new_position = orbit.borrow().get_end_position();
                let new_velocity = orbit.borrow().get_end_velocity();
                update_position_and_velocity(state, entity, new_position, new_velocity);
            }
        }
        update_parent_for_prediction(state, make_soi_function(time), entity, time + SIMULATION_TIME_STEP);
    }
}

pub fn predict_spacecraft(state: &mut State, entity: Entity, start_time: f64, end_time: f64) {
    let mut time = start_time;
    let time_steps = ((end_time - time) / SIMULATION_TIME_STEP) as usize;

    state.components.trajectory_components.get_mut(&entity).unwrap().remove_segments_after(time);

    for _ in 0..time_steps {
        sync_celestial_bodies_to_time(state, time);
        update_for_prediction(state, &entity, time);
        time += SIMULATION_TIME_STEP;
    }

    sync_entity_to_time(state, &entity, state.time);
    sync_celestial_bodies_to_time(state, state.time);
}

pub fn predict_all_spacecraft(state: &mut State, end_time: f64) {
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

    for entity in state.components.entity_allocator.get_entities() {
        if state.components.trajectory_components.get(&entity).is_some() {
            sync_entity_to_time(state, &entity, state.time);
        }
    }
}
