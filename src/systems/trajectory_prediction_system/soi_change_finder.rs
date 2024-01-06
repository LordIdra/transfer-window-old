use std::{f64::consts::PI, cell::RefCell, rc::Rc, collections::HashMap};

use crate::{state::State, storage::entity_allocator::Entity, systems::{util::get_segment_at_time, debug_system::debug_utils::format_time}, components::trajectory_component::segment::orbit::Orbit};

use super::{newton_raphson::{newton_raphson_to_find_minimum, newton_raphson}, bisection::bisection_to_find_minimum};

fn get_parallel_entities(state: &State, parent: Entity, time: f64) -> Vec<Entity> {
    let mut parallel_entities = vec![];
    for other_entity in state.components.entity_allocator.get_entities() {
        if state.components.trajectory_components.get(&other_entity).is_some() {
            let other_segment = get_segment_at_time(state, &other_entity, time);
            let other_parent = other_segment.get_parent();
            if other_parent == parent {
                parallel_entities.push(other_entity);
            }
        }
    }
    parallel_entities
}

fn calculate_soi(state: &State, other_entity: Entity, other_orbit: &Rc<RefCell<Orbit>>) -> f64 {
    let mass = state.components.mass_components.get(&other_entity).unwrap().get_mass();
    let parent_mass = state.components.mass_components.get(&other_orbit.borrow().get_parent()).unwrap().get_mass();
    other_orbit.borrow().get_semi_major_axis() * (mass / parent_mass).powf(2.0 / 5.0)
}

/// Find values at 100 equally spaced points and select the lowest as a best-guess for the minimum
fn get_min_max_estimate(state: &mut State, signed_distance_function_theta: impl Fn(f64) -> f64) -> (f64, f64) {
    let mut min_estimate_x = 0.0;
    let mut min_estimate_y = f64::MAX;
    let mut max_estimate_x = 0.0;
    let mut max_estimate_y = f64::MAX;
    for i in 0..100 {
        let x = i as f64 * 2.0 * PI / 100.0;
        let y = signed_distance_function_theta(x);
        state.debug_conic_distances_from_theta.push([x, y]);
        if y < min_estimate_y {
            min_estimate_x = x;
            min_estimate_y = y;
        } 
        if y > max_estimate_y {
            max_estimate_x = x;
            max_estimate_y = y;
        }
    }
    (min_estimate_x, max_estimate_x)
}

fn get_starting_thetas(signed_distance_function_theta: impl Fn(f64) -> f64, max_estimate_x: f64, refined_min_estimate_x: f64, refined_min_estimate_y: f64) -> Vec<f64> {
    if refined_min_estimate_y.is_sign_positive() {
        // If the signed distance function is positive, we take the minimum as the starting point for our encounter calculation
        vec![refined_min_estimate_x]
        
    } else {
        // Otherwise, the signed distance function is negative, meaning there are two intersections with the other conic
        // We'll use these two intersections as our starting points
        // We know that one roots lies between min and max, and the other between max and min
        // We can use this to our advantage by employing a bisection algorithm
        // Interval 1 = point_low <-> point_high
        // Interval 2 = point_high <-> point_low+2pi
        let refined_max_estimate_x = newton_raphson_to_find_minimum(&signed_distance_function_theta, max_estimate_x).expect("Failed to converge");
        let point_1 = f64::min(refined_min_estimate_x, refined_max_estimate_x);
        let point_2 = f64::max(refined_min_estimate_x, refined_max_estimate_x);
        let point_3 = point_1 + 2.0 * PI;
        vec![
            bisection_to_find_minimum(&signed_distance_function_theta, point_1, point_2),
            bisection_to_find_minimum(&signed_distance_function_theta, point_2, point_3)]
    }
}

fn get_starting_times(starting_thetas: Vec<f64>, orbit: &Rc<RefCell<Orbit>>) -> Vec<f64> {
    let mut starting_times = vec![];
    for theta in starting_thetas {
        let mut time = orbit.borrow().get_time_since_periapsis(theta) + orbit.borrow().get_periapsis_time();
        if let Some(period) = orbit.borrow().get_period() {
            if time < orbit.borrow().get_start_point().get_time() {
                time += period;
            }
        }
        starting_times.push(time);
    }
    starting_times
}

fn get_entity_entrance_time_estimates(state: &mut State, orbit: &Rc<RefCell<Orbit>>, entity: Entity, other_entity: Entity, time: f64) -> Vec<f64> {
    let other_segment = get_segment_at_time(state, &other_entity, time);
    let other_orbit = other_segment.as_orbit();
    let soi = calculate_soi(state, other_entity, other_orbit);

    // Construct a signed distance function between the two entities (ie, when the tntity orbit goes beyond the other orbit, function is negative)
    // This function should only ever have one minimum and one maximum :)
    let signed_distance_function_theta = move |theta: f64| -> f64 {
        other_orbit.borrow().get_position_from_theta(theta).magnitude() - orbit.borrow().get_position_from_theta(theta).magnitude()
    };

    let (min_estimate_x, max_estimate_x) = get_min_max_estimate(state, signed_distance_function_theta);

    // It's possible for the solver to fail if the orbits are both perfectly circular. But this is VERY unlikely
    let refined_min_estimate_x = newton_raphson_to_find_minimum(&signed_distance_function_theta, min_estimate_x).expect("Failed to converge");
    let refined_min_estimate_y = signed_distance_function_theta(refined_min_estimate_x);

    // If the signed distance function is larger than the SOI, there will never be an encounter
    if refined_min_estimate_y > soi {
        return vec![];
    }
    
    let starting_thetas = get_starting_thetas(signed_distance_function_theta, max_estimate_x, refined_min_estimate_x, refined_min_estimate_y);
    let starting_times = get_starting_times(starting_thetas, orbit);

    // starting_times

    // Construct function for distance to the SOI circle at a given time
    let distance_function_time = move |time: f64| -> f64 {
        let theta_1 = orbit.borrow().get_theta_from_time(time);
        let theta_2 = other_orbit.borrow().get_theta_from_time(time);
        (orbit.borrow().get_position_from_theta(theta_1) - other_orbit.borrow().get_position_from_theta(theta_2)).magnitude() - soi
    };

    for i in 0..100 {
        let x = (i as f64 / 100.0) * 10000000.0;
        state.debug_conic_distances_from_time.push([x, distance_function_time(x)]);
    }

    // Minimise the time-distance function to find a set of possible intersections
    let mut solutions = vec![];
    for time in starting_times {
        if let Some(solution) = newton_raphson(&distance_function_time, time) {
            solutions.push(solution);
        }
    }

    solutions
}

fn get_entrance_time_estimates(state: &mut State, entity: Entity, orbit: &Rc<RefCell<Orbit>>, time: f64) -> HashMap<Entity, Vec<f64>> {
    let mut entrance_time_estimates = HashMap::new();
    for other_entity in get_parallel_entities(state, orbit.borrow().get_parent(), time) {
        if other_entity == entity {
            continue;
        }
        let entity_entrance_time_estimates = get_entity_entrance_time_estimates(state, orbit, entity, other_entity, time);
        if !entity_entrance_time_estimates.is_empty() {
            entrance_time_estimates.insert(other_entity, entity_entrance_time_estimates);
        }
    }
    entrance_time_estimates
}

// fn get_exit_time(state: &State, entity: Entity, time: f64) -> Option<f64> {

// }

pub fn find_next_soi_entrance(state: &mut State, entity: Entity, time: f64) {
    // TODO these algorithms won't work for hyperbolas as they don't go to 2pi, also wouldn't be evenly spaced :(
    let segment = state.components.trajectory_components.get(&entity).unwrap().get_final_segment();
    let orbit = segment.as_orbit();
    let entrance_time_estimates = get_entrance_time_estimates(state, entity, orbit, time);
    println!("{:?}", entrance_time_estimates);
    // let exit_time = get_exit_time(state, entity, time);
}