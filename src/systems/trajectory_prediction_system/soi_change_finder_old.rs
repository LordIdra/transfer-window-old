use crate::{storage::entity_allocator::Entity, state::State, systems::{util::get_segment_at_time, debug_system::debug_utils::format_time}};

const MAX_DELTA: f64 = 0.05;
const ADDITIONAL_TIME: f64 = 30.0;
const MAX_ITERATIONS: usize = 2000;
const LEARNING_RATE: f64 = 0.05;
const TIME_DELTA: f64 = 0.01;
const MAX_TIME: f64 = 30.0 * 24.0 * 60.0 * 60.0;

fn solve_black_box(function: impl Fn(f64) -> f64, start_time: f64) -> Option<f64> {
    let mut iterations = 0;
    let mut time = start_time + ADDITIONAL_TIME;
    let mut going_downhill = false;
    let initial_distance_t = function(time);
    loop {
        let distance_t_minus_h = function(time - TIME_DELTA);
        let distance_t = function(time);
        let distance_t_plus_h = function(time + TIME_DELTA);
        let distance_t_prime = (distance_t_plus_h - distance_t_minus_h) / (2.0 * TIME_DELTA);
        let mut delta = LEARNING_RATE * -distance_t / (distance_t_prime + 0.01);

        // println!("delta {}", delta);

        if delta > 0.0 {
            // println!("Going downhill");
            going_downhill = true;
        }
        // if !going_downhill {
            delta = delta.abs();
        // }
        
        time += delta;
        iterations += 1;

        // println!("{} {}", format_time(time), delta);

        if iterations > MAX_ITERATIONS {
            return None;
        }
        if delta.abs() < MAX_DELTA {
            break;
        }

    }

    Some(time)
}

fn calculate_soi(state: &State, entity: Entity, time: f64) -> f64 {
    let segment = get_segment_at_time(state, &entity, time);
    let orbit = segment.as_orbit();
    let mass = state.components.mass_components.get(&entity).unwrap().get_mass();
    let parent_mass = state.components.mass_components.get(&orbit.borrow().get_parent()).unwrap().get_mass();
    let soi = orbit.borrow().get_semi_major_axis() * (mass / parent_mass).powf(2.0 / 5.0);
    soi
}

fn find_next_soi_enter_time(state: &State, entity: Entity, child: Entity, start_time: f64) -> Option<f64> {
    let child_segment = get_segment_at_time(state, &child, start_time);
    let child_orbit = child_segment.as_orbit();
    let entity_segment = state.components.trajectory_components.get(&entity).unwrap().get_final_segment();
    let entity_orbit =  entity_segment.as_orbit();
    let soi = calculate_soi(state, child, start_time);

    // let scaling_function = |_: f64| {
    //     1.0
    // };

    let function = move |time: f64| -> f64 {
        let child_theta = child_orbit.borrow().get_theta_from_time(time);
        let child_position = child_orbit.borrow().get_position_from_theta(child_theta);
        let entity_theta = entity_orbit.borrow().get_theta_from_time(time);
        let entity_position = entity_orbit.borrow().get_position_from_theta(entity_theta);
        (child_position - entity_position).magnitude_squared() - soi.powi(2)
    };

    // let derivative = |time: f64| -> f64 {
    //     (function(time + TIME_DELTA) - function(time - TIME_DELTA)) / (2.0 * TIME_DELTA)
    // };

    // solve_black_box(function, start_time)d
    // epsilon?????????? imaginary???????? 
    //let x = find_roots_with_newton_polishing(&scaling_function, &function, &derivative, start_time, MAX_TIME, 4, 0.001, 10, 1000.0, 0.0, 1.0, f64::MAX);
    // if let Ok(mut roots) = x {
    //     println!("{:?}", roots);
    //     roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    //     roots.first().cloned()
    // } else {
    //     println!("{}", x.unwrap_err());
    //     None
    // }

    solve_black_box(function, start_time)
}

fn find_next_soi_exit_time(state: &State, entity: Entity, parent: Entity, start_time: f64) -> Option<f64> {
    let segment = state.components.trajectory_components.get(&entity).unwrap().get_final_segment();
    let orbit =  segment.as_orbit();
    let soi = calculate_soi(state, parent, start_time);

    // let scaling_function = |_: f64| {
    //     1.0
    // };

    let function = move |time: f64| {
        let theta = orbit.borrow().get_theta_from_time(time);
        let position = orbit.borrow().get_position_from_theta(theta);
        position.magnitude_squared() - soi.powi(2)
    };

    // let derivative = |time: f64| -> f64 {
    //     (function(time + TIME_DELTA) - function(time - TIME_DELTA)) / (2.0 * TIME_DELTA)
    // };

    // let x = find_roots_with_newton_polishing(&scaling_function, &function, &derivative, start_time, MAX_TIME, 4, 1.0, 10, 10.0, 0.01, 1000.0, f64::MAX);
    // if let Ok(mut roots) = x {
    //     println!("{:?}", roots);
    //     roots.sort_by(|a, b| a.partial_cmp(b).unwrap());
    //     roots.first().cloned()
    // } else {
    //     println!("{}", x.unwrap_err());
    //     None
    // }

    solve_black_box(function, start_time)
}

fn find_parent(state: &State, entity: Entity, time: f64) -> Entity {
    get_segment_at_time(state, &entity, time).get_parent()
}

fn find_children(state: &State, entity: Entity, time: f64) -> Vec<Entity> {
    let mut children = vec![];
    for other_entity in state.components.entity_allocator.get_entities() {
        if state.components.trajectory_components.get(&other_entity).is_some() && state.components.celestial_body_components.get(&other_entity).is_some() {
            let other_entity_parent = get_segment_at_time(state, &other_entity, time).get_parent();
            if other_entity_parent == entity {
                children.push(other_entity);
            }
        }
    }
    children
}

pub fn find_next_soi_change(state: &State, entity: Entity, ) -> Option<(Entity, f64)> {
    let mut soi_changes = vec![];

    let segment = state.components.trajectory_components.get(&entity).unwrap().get_final_segment();
    let orbit = segment.as_orbit();
    let time = orbit.borrow().get_end_point().get_time();
    let parent = orbit.borrow().get_parent();
    println!("Finding changes starting at {} from parent {}", format_time(time), state.components.name_components.get(&parent).unwrap().get_name());
    if let Some(soi_change_time) = find_next_soi_exit_time(state, entity, parent, time) {
        let new_parent = find_parent(state, parent, time);
        println!("Found exit change at {} to {}", format_time(soi_change_time), state.components.name_components.get(&new_parent).unwrap().get_name());
        soi_changes.push((new_parent, soi_change_time));
    }

    let children = find_children(state, parent, time);
    for child in children {
        if child != entity {
            if let Some(soi_change_time) = find_next_soi_enter_time(state, entity, child, time) {
                println!("Found entrance change at {} to {}", format_time(soi_change_time), state.components.name_components.get(&child).unwrap().get_name());
                soi_changes.push((child, soi_change_time));
            }
        }
    }

    soi_changes.sort_by(|a, b| a.1.total_cmp(&b.1));
    soi_changes.first().cloned()
}