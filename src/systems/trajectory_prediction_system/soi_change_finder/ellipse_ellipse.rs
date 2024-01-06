use std::{rc::Rc, cell::RefCell, f64::consts::PI};

use crate::{components::trajectory_component::segment::orbit::Orbit, state::State, systems::{trajectory_prediction_system::{newton_raphson::{newton_raphson_to_find_stationary_point, newton_raphson}, bisection::bisection}, debug_system::debug_utils::format_time}};

use super::util::{calculate_soi, get_min_max_estimate, exact_time_already_found, find_opposite_solution};

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
        let refined_max_estimate_x = newton_raphson_to_find_stationary_point(&signed_distance_function_theta, max_estimate_x).expect("Failed to converge");
        let point_1 = f64::min(refined_min_estimate_x, refined_max_estimate_x);
        let point_2 = f64::max(refined_min_estimate_x, refined_max_estimate_x);
        let point_3 = point_1 + 2.0 * PI;
        vec![
            bisection(&signed_distance_function_theta, point_1, point_2),
            bisection(&signed_distance_function_theta, point_2, point_3)]
    }
}

fn get_starting_times(starting_thetas: Vec<f64>, orbit: &Rc<RefCell<Orbit>>) -> Vec<f64> {
    let mut starting_times = vec![];
    for theta in starting_thetas {
        let mut time = orbit.borrow().get_time_since_periapsis(theta) + orbit.borrow().get_periapsis_time();

        // Make sure we always start behind the orbit point to catch any edge-cases where it rapidly enters another SOI after starting the orbit
        // Might be unecessary so something to try removing for optimisation
        let period = orbit.borrow().get_period().unwrap();
        if time > orbit.borrow().get_start_point().get_time() {
            time -= period;
        }
        starting_times.push(time);
    }
    starting_times
}

pub fn get_entity_entrance_times(state: &mut State, orbit: &Rc<RefCell<Orbit>>, other_orbit: &Rc<RefCell<Orbit>>, other_mass: f64, start_time: f64, end_time: f64) -> Vec<f64> {
    let soi = calculate_soi(state, other_mass, other_orbit);

    // Construct a signed distance function between the two entities (ie, when the tntity orbit goes beyond the other orbit, function is negative)
    // This function should only ever have one minimum and one maximum :)
    let signed_distance_function_theta = move |theta: f64| -> f64 {
        other_orbit.borrow().get_position_from_theta(theta).magnitude() - orbit.borrow().get_position_from_theta(theta).magnitude()
    };

    let (min_estimate_x, max_estimate_x) = get_min_max_estimate(state, -2.0*PI, 2.0*PI, signed_distance_function_theta);

    // It's possible for the solver to fail if the orbits are both perfectly circular. But this is VERY unlikely
    let refined_min_estimate_x = newton_raphson_to_find_stationary_point(&signed_distance_function_theta, min_estimate_x).expect("Failed to converge");
    let refined_min_estimate_y = signed_distance_function_theta(refined_min_estimate_x);

    // If the signed distance function is larger than the SOI, there will never be an encounter
    if refined_min_estimate_y > soi {
        return vec![];
    }
    
    let starting_thetas = get_starting_thetas(signed_distance_function_theta, max_estimate_x, refined_min_estimate_x, refined_min_estimate_y);
    let starting_times = get_starting_times(starting_thetas, orbit);

    // Construct function for distance to the SOI circle at a given time
    let distance_function_time = move |time: f64| -> f64 {
        let theta_1 = orbit.borrow().get_theta_from_time(time);
        let theta_2 = other_orbit.borrow().get_theta_from_time(time);
        //println!("{} {}", theta_1, theta_2);
        (orbit.borrow().get_position_from_theta(theta_1) - other_orbit.borrow().get_position_from_theta(theta_2)).magnitude() - soi
    };

    println!("tsp {} {}", orbit.borrow().get_arugment_of_periapsis(), orbit.borrow().get_periapsis_time());
    let theta_1 = orbit.borrow().get_theta_from_time(0.0);
    let theta_2 = other_orbit.borrow().get_theta_from_time(0.0); 
    println!("df-test {} {} {}", theta_1, theta_2, distance_function_time(1316436.3849267222)); 
    //correct    65727348.960877724 
    //INCORRECT -9779668.680723324 
    //4.557718764079313  4.452966458969298
    //4.5562815520575395 4.452966458969298
    //4.574182198879981 4.452966458969298
    // tsp -43.6207779729109
    // tsp -321.7762599235648
    // tsp -15.666570367640816
    // incorrect 0.49056177493290964

    // Debug
    for i in 0..1000 {
        let x = (i as f64 / 1000.0) * end_time;
        state.debug_conic_distances_from_time.push([x, distance_function_time(x)]);
    }

    // Minimise the time-distance function to find a set of possible intersections
    let mut exact_times = vec![];
    for starting_time in starting_times {
        let mut time = starting_time;
        while time < end_time {
            if let Some(exact_time) = newton_raphson(&distance_function_time, time) {
                // found exact 1376799.4139631523 15d22h26m39s starting at 1732147.1991374264 20d1h9m7s
                // found exact 8745932.145570587 101d5h25m32s starting at 8790107.489875177 101d17h41m47s
                // found exact 1458726.2471029025 16d21h12m6s starting at 288210.1676602635 3d8h3m30s
                // found exact 1376799.4139631528 15d22h26m39s starting at 1296490.2091942278 15d0h8m10s
                // found exact 6336238.85407491 73d8h3m59s starting at 6337890.416864051 73d8h31m30s
                println!("found exact {} {} starting at {} {}", exact_time, format_time(exact_time), time, format_time(time));
                if exact_time_already_found(&exact_times, exact_time) {
                    time += orbit.borrow().get_period().unwrap();
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
            time += orbit.borrow().get_period().unwrap();
        }
    }

    exact_times
}