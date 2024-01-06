use std::{rc::Rc, cell::RefCell};

use crate::{state::State, components::trajectory_component::segment::orbit::Orbit, systems::trajectory_prediction_system::{bisection::bisection, newton_raphson::newton_raphson}};

pub fn calculate_soi(state: &State, mass: f64, orbit: &Rc<RefCell<Orbit>>) -> f64 {
    let parent_mass = state.components.mass_components.get(&orbit.borrow().get_parent()).unwrap().get_mass();
    orbit.borrow().get_semi_major_axis() * (mass / parent_mass).powf(2.0 / 5.0)
}

/// Find values at 100 equally spaced points and select the lowest as a best-guess for the minimum
pub fn get_min_max_estimate(state: &mut State, start_theta: f64, end_theta: f64, signed_distance_function_theta: impl Fn(f64) -> f64) -> (f64, f64) {
    let mut min_estimate_x = 0.0;
    let mut min_estimate_y = f64::MAX;
    let mut max_estimate_x = 0.0;
    let mut max_estimate_y = -f64::MAX;
    for i in 0..=100 {
        let x = start_theta + (i as f64 / 100.0) * (end_theta - start_theta);
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

pub fn exact_time_already_found(exact_times: &Vec<f64>, exact_time: f64) -> bool {
    for other_exact_time in exact_times {
        if (other_exact_time - exact_time).abs() < 1.0 {
            return true;
        }
    }
    false
}

// We know the function is roughly symmetric
// So add a starting value to x, evaluate the function, and then keep doing that until the new value is greater than the last one
// Oh, or we could subtract instead of add - we know which we to go based on derivative at existing solution
pub fn find_opposite_solution(distance_function_time: impl Fn(f64) -> f64, existing_solution: f64) -> f64 {
    let derivative = |time: f64| (distance_function_time(time + 0.001) - distance_function_time(time)) / 0.001;
    let mut previous_previous_x = existing_solution;
    let mut previous_x = existing_solution;
    let mut previous_y = distance_function_time(existing_solution);
    let mut increment = if derivative(existing_solution).is_sign_positive() { -16.0 } else { 16.0 };
    let interval_containing_min;
    loop {
        let new_x = previous_x + increment; // todo direction
        let new_y = distance_function_time(new_x);
        if new_y > previous_y {
            interval_containing_min = (previous_previous_x, new_x);
            break;
        }
        previous_previous_x = previous_x;
        previous_x = new_x;
        previous_y = new_y;
        increment *= 2.0;
    };
    let interval_start = f64::min(interval_containing_min.0, interval_containing_min.1);
    let interval_end = f64::max(interval_containing_min.0, interval_containing_min.1);
    let min = bisection(&derivative, interval_start, interval_end);
    // println!("min {}", min);
    let other_solution_start_time = (2.0 * min) - existing_solution;
    // println!("other start time {}", other_solution_start_time);
    newton_raphson(&distance_function_time, other_solution_start_time).expect("Failed to converge to opposite solution")
}