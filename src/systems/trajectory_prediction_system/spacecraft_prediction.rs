use std::{cell::RefCell, rc::Rc};

use crate::{state::State, systems::{util::{update_position_and_velocity, get_segment_at_time, is_spacecraft_with_trajectory, sync_celestial_bodies_to_time, sync_entity_to_time, update_parent}, trajectory_prediction_system::soi_change_finder::SoiChangeType, debug_system::debug_utils::format_time}, storage::entity_allocator::Entity, components::trajectory_component::segment::{Segment, orbit::Orbit}};

use super::{util::{SoiFunction, update_parent_for_prediction}, soi_change_finder::find_next_soi_change};

const SIMILARITY_THRESHOLD: f64 = 1.0e-4;
const SIMULATION_TIME_STEP: f64 = 40.0;
const MAX_CONICS: usize = 3;

// fn get_absolute_position_at_time(state: &State, entity: Entity, time: f64) -> DVec2 {
//     match state.components.trajectory_components.get(&entity) {
//         Some(_) => {
//             let segment = get_segment_at_time(state, &entity, time);
//             let orbit = segment.as_orbit().clone();
//             let theta = orbit.borrow().get_theta_from_time(time);
//             let position = orbit.borrow().get_position_from_theta(theta);
//             let parent = orbit.borrow().get_parent();
//             position + get_absolute_position_at_time(state, parent, time)
//         }
//         None => DVec2::new(0.0, 0.0),
//     }
// }

// fn get_absolute_velocity_at_time(state: &State, entity: Entity, time: f64) -> DVec2 {
//     match state.components.trajectory_components.get(&entity) {
//         Some(_) => {
//             let segment = get_segment_at_time(state, &entity, time);
//             let orbit = segment.as_orbit().clone();
//             let theta = orbit.borrow().get_theta_from_time(time);
//             let velocity = orbit.borrow().get_velocity_from_theta(theta);
//             let parent = orbit.borrow().get_parent();
//             velocity + get_absolute_velocity_at_time(state, parent, time)
//         }
//         None => DVec2::new(0.0, 0.0),
//     }
// }

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

fn update_for_prediction(state: &mut State, entity: Entity, time: f64) {
    let trajectory_component = state.components.trajectory_components.get_mut(&entity).unwrap();
    trajectory_component.predict(SIMULATION_TIME_STEP);
    match trajectory_component.get_final_segment() {
        Segment::Burn(_) => {
            panic!("Attempt to update burn for prediction")
        }
        Segment::Orbit(orbit) => {
            let new_parent = orbit.borrow().get_parent();
            update_parent(state, entity, &new_parent);
            let new_position = orbit.borrow().get_end_point().get_position();
            let new_velocity = orbit.borrow().get_end_point().get_velocity();
            update_position_and_velocity(state, &entity, new_position, new_velocity);
        }
    }
    update_parent_for_prediction(state, make_soi_function(time + SIMULATION_TIME_STEP), entity, time + SIMULATION_TIME_STEP);
}

// pub fn predict_spacecraft(state: &mut State, entity: Entity, start_time: f64) {
//     state.components.trajectory_components.get_mut(&entity).unwrap().remove_segments_after(start_time);
//     let mut conics = 0;
//     while conics < MAX_CONICS {
//         let Some((new_parent, soi_change_time)) = find_next_soi_change(state, entity) else {
//             break;
//         };

//         state.components.trajectory_components.get(&entity).unwrap().get_final_segment().as_orbit().borrow_mut().end_at(soi_change_time);

//         let absolute_entity_position = get_absolute_position_at_time(state, entity, soi_change_time);
//         let absolute_entity_velocity = get_absolute_velocity_at_time(state, entity, soi_change_time);
//         let absolute_new_parent_position = get_absolute_position_at_time(state, new_parent, soi_change_time);
//         let absolute_new_parent_velocity = get_absolute_velocity_at_time(state, new_parent, soi_change_time);
//         let new_relative_position = absolute_entity_position - absolute_new_parent_position;
//         let new_relative_velocity = absolute_entity_velocity - absolute_new_parent_velocity;

//         let orbit = Orbit::new(&state.components, new_parent, new_relative_position, new_relative_velocity, soi_change_time);
//         state.components.trajectory_components.get_mut(&entity).unwrap().add_segment(Segment::Orbit(Rc::new(RefCell::new(orbit))));
//         conics += 1;
//     }
// }

// fn find_minimums(approximation: Polynomial, function: impl Fn(f64) -> f64) -> Vec<f64> {
//     let mut approximation_prime = approximation.clone();
//     approximation_prime.differentiate();
//     let mut approximation_prime_prime = approximation_prime.clone();
//     approximation_prime_prime.differentiate();
//     let raw_roots = approximation_prime.clone().solve();
//     let mut root_and_distance_pairs: Vec<(f64, f64)> = raw_roots
//         .iter()
//         .map(|root| (*root, function(*root)))
//         .collect();
//     println!("{:?}", root_and_distance_pairs);
//     root_and_distance_pairs.sort_by(|pair_a, pair_b| {
//         if pair_a.1 > pair_b.1 {
//             Ordering::Greater
//         } else {
//             Ordering::Less
//         }});
//     println!("Distance pairs {:?}", root_and_distance_pairs);
//     let sorted_roots: Vec<f64> = root_and_distance_pairs
//         .iter()
//         .map(|pair| pair.0)
//         .collect();
//     println!("Sorted roots {:?}", sorted_roots);
//     let mut lowest_sorted_roots = vec![];
//     if let Some(x) = sorted_roots.get(0) { lowest_sorted_roots.push(x) }
//     if let Some(x) = sorted_roots.get(1) { lowest_sorted_roots.push(x) }
//     println!("Lowest roots {:?}", lowest_sorted_roots);
//     let mut closest_points = vec![];
    
//     for root in sorted_roots { //vec![-0.44, -0.54] {
//         if let Some(solution) = newton_raphson_to_find_minimum(&function, root) {
//             closest_points.push(solution);
//         }
//     }

//     if let Some(first) = closest_points.first() {
//         if let Some(second) = closest_points.get(1) {
//             if (first - second).abs() < SIMILARITY_THRESHOLD {
//                 closest_points.pop();
//             }
//         }
//     }

//     closest_points
// }

pub fn predict_spacecraft(state: &mut State, entity: Entity, start_time: f64) {
    state.components.trajectory_components.get_mut(&entity).unwrap().remove_segments_after(start_time);
    let mut time = start_time;
    let end_time = 10000000.0;
    loop {
        let Some(soi_change) = find_next_soi_change(state, entity, time, end_time) else {
            break;
        };
        time = soi_change.get_time();
        if time > end_time {
            break;
        }
        let trajectory_component = state.components.trajectory_components.get(&entity).unwrap();
        trajectory_component.get_final_segment().as_orbit().borrow_mut().end_at(time);
        let new_parent = soi_change.get_other_entity();

        match soi_change {
            SoiChangeType::Entrance(_) => {
                let parent_position = get_segment_at_time(state, &new_parent, time).get_position_at_time(time);
                let position = trajectory_component.get_final_segment().get_end_position() - parent_position;
                let parent_velocity = get_segment_at_time(state, &new_parent, time).get_velocity_at_time(time);
                let velocity = trajectory_component.get_final_segment().get_end_velocity() - parent_velocity;
                println!("soi entrance to {} {:?} {:?} {}", state.components.name_components.get(&new_parent).unwrap().get_name(), position, velocity, format_time(time));
                let orbit = Orbit::new(&state.components, new_parent, position, velocity, time);
                state.components.trajectory_components.get_mut(&entity).unwrap().add_segment(Segment::Orbit(Rc::new(RefCell::new(orbit))));
            }
            SoiChangeType::Exit(_) => {
                let parent = trajectory_component.get_final_segment().get_parent();
                let parent_position = get_segment_at_time(state, &parent, time).get_position_at_time(time);
                let position = trajectory_component.get_final_segment().get_end_position() + parent_position;
                let parent_velocity = get_segment_at_time(state, &parent, time).get_velocity_at_time(time);
                let velocity = trajectory_component.get_final_segment().get_end_velocity() + parent_velocity;
                println!("soi exit to {} {:?} {:?} {}", state.components.name_components.get(&new_parent).unwrap().get_name(), position, velocity, format_time(time));
                let orbit = Orbit::new(&state.components, new_parent, position, velocity, time);
                state.components.trajectory_components.get_mut(&entity).unwrap().add_segment(Segment::Orbit(Rc::new(RefCell::new(orbit))));
            }
        }
    }
    state.components.trajectory_components.get(&entity).unwrap().get_final_segment().as_orbit().borrow_mut().end_at(end_time);

    // state.components.trajectory_components.get(&entity).unwrap().get_final_segment().as_orbit().borrow_mut().end_at(soi_change_time);

    // let absolute_entity_position = get_absolute_position_at_time(state, entity, soi_change_time);
    // let absolute_entity_velocity = get_absolute_velocity_at_time(state, entity, soi_change_time);
    // let absolute_new_parent_position = get_absolute_position_at_time(state, new_parent, soi_change_time);
    // let absolute_new_parent_velocity = get_absolute_velocity_at_time(state, new_parent, soi_change_time);
    // let new_relative_position = absolute_entity_position - absolute_new_parent_position;
    // let new_relative_velocity = absolute_entity_velocity - absolute_new_parent_velocity;

    // let orbit = Orbit::new(&state.components, new_parent, new_relative_position, new_relative_velocity, soi_change_time);
    // state.components.trajectory_components.get_mut(&entity).unwrap().add_segment(Segment::Orbit(Rc::new(RefCell::new(orbit))));
}

// pub fn predict_spacecraft(state: &mut State, entity: Entity, start_time: f64) {
//     state.components.trajectory_components.get_mut(&entity).unwrap().remove_segments_after(start_time);
//     let mut time = start_time;
//     loop {
//         let segment = get_segment_at_time(state, &entity, time);
//         let position = segment.get_position_at_time(time);
//         // Check all lower SOI
//         let parent = segment.get_parent();
//         let parallel_entities = get_parallel_entities(state, parent, time);
//         // Check all higher SOI
//         // If any change, make change to new parent + position/velocity
//         time += SIMULATION_TIME_STEP;
//     }
// }

pub fn predict_all_spacecraft(state: &mut State, end_time: f64) {
    let mut time = state.time;
    let time_steps = ((end_time - time) / SIMULATION_TIME_STEP) as usize;

    // let spacecraft = get_entity_by_name(state, "spacecraft".to_string());
    // let moon = get_entity_by_name(state, "moon".to_string());

    //println!("{},", find_next_soi_enter(state, spacecraft, moon, time));

    for entity in state.components.entity_allocator.get_entities() {
        if is_spacecraft_with_trajectory(state, entity) {
            state.components.trajectory_components.get_mut(&entity).unwrap().remove_segments_after(time)
        }
    }

    for _ in 0..time_steps {
        sync_celestial_bodies_to_time(state, time + SIMULATION_TIME_STEP);
        for entity in state.components.entity_allocator.get_entities() {
            if is_spacecraft_with_trajectory(state, entity) {
                update_for_prediction(state, entity, time);
            }
        }
        time += SIMULATION_TIME_STEP;
    }

    for entity in state.components.entity_allocator.get_entities() {
        if state.components.trajectory_components.get(&entity).is_some() {
            sync_entity_to_time(state, entity, state.time);
        }
    }
}
