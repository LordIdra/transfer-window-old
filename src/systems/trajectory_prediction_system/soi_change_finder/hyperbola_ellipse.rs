use std::{rc::Rc, cell::RefCell};

use crate::{components::trajectory_component::segment::orbit::Orbit, state::State, systems::trajectory_prediction_system::{newton_raphson::{newton_raphson_to_find_stationary_point, newton_raphson}, bisection::bisection}};

use super::util::{calculate_soi, get_min_max_estimate, exact_time_already_found, find_opposite_solution};

fn get_starting_thetas(signed_distance_function_theta: impl Fn(f64) -> f64, refined_max_estimate_x: f64, min_x: f64, max_x: f64) -> Vec<f64> {
    if refined_max_estimate_x.is_sign_negative() {
        // If the signed distance function is negative, we take the maximum as the starting point for our encounter calculation
        vec![refined_max_estimate_x]
        
    } else {
        // Otherwise, the signed distance function is positive, meaning there are two intersections with the other conic
        // We'll use these two intersections as our starting points
        // We know that one roots lies between min_x and max, and the other between max and max_x
        // We can use this to our advantage by employing a bisection algorithm
        vec![
            bisection(&signed_distance_function_theta, min_x, refined_max_estimate_x),
            bisection(&signed_distance_function_theta, refined_max_estimate_x, max_x)]
    }
}


fn get_starting_times(starting_thetas: Vec<f64>, orbit: &Rc<RefCell<Orbit>>) -> Vec<f64> {
    let mut starting_times = vec![];
    for theta in starting_thetas {
        let time = orbit.borrow().get_time_since_periapsis(theta) + orbit.borrow().get_periapsis_time();
        starting_times.push(time);
    }
    starting_times
}

pub fn get_entity_entrance_times(state: &mut State, orbit: &Rc<RefCell<Orbit>>, other_orbit: &Rc<RefCell<Orbit>>, other_mass: f64, start_time: f64, end_time: f64) -> Vec<f64> {
    let soi = calculate_soi(state, other_mass, other_orbit);
    let (min_theta, max_theta) = orbit.borrow().get_min_max_theta();

    // Construct a signed distance function between the two entities (ie, when the tntity orbit goes beyond the other orbit, function is negative)
    // This function should only ever have one maximum :)
    let signed_distance_function_theta = move |theta: f64| -> f64 {
        other_orbit.borrow().get_position_from_theta(theta).magnitude() - orbit.borrow().get_position_from_theta(theta).magnitude()
    };

    // Solutions will be to either side of the maximum so let's find the maximum
    let (_, max_estimate_x) = get_min_max_estimate(state, min_theta, max_theta, signed_distance_function_theta);
    let refined_max_estimate_x = newton_raphson_to_find_stationary_point(&signed_distance_function_theta, max_estimate_x).expect("Failed to converge");
    let refined_max_estimate_y = signed_distance_function_theta(refined_max_estimate_x);

    // If the negative of the signed distance function is larger than the SOI, there will never be an encounter
    if -refined_max_estimate_y > soi {
        return vec![];
    }
    
    let starting_thetas = get_starting_thetas(signed_distance_function_theta, refined_max_estimate_x, min_theta, max_theta);
    let starting_times = get_starting_times(starting_thetas, orbit);

    // Construct function for distance to the SOI circle at a given time
    let distance_function_time = move |time: f64| -> f64 {
        let theta_1 = orbit.borrow().get_theta_from_time(time);
        let theta_2 = other_orbit.borrow().get_theta_from_time(time);
        (orbit.borrow().get_position_from_theta(theta_1) - other_orbit.borrow().get_position_from_theta(theta_2)).magnitude() - soi
    };

    // Debug
    for i in 0..1000 {
        let x = (i as f64 / 1000.0) * end_time;
        state.debug_conic_distances_from_time.push([x, distance_function_time(x)]);
    }

    // Minimise the time-distance function to find a set of possible intersections
    let mut exact_times: Vec<f64> = vec![];
    for starting_time in starting_times {
        if let Some(exact_time) = newton_raphson(&distance_function_time, starting_time) {
            if exact_time_already_found(&exact_times, exact_time) {
                continue;
            }
            let other_exact_time = find_opposite_solution(distance_function_time, exact_time);
            if other_orbit.borrow().is_time_within_orbit(exact_time) {
                exact_times.push(exact_time);
            }
            if other_orbit.borrow().is_time_within_orbit(other_exact_time) {
                exact_times.push(other_exact_time);
            }
        }
    }

    exact_times
}