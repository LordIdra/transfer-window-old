use std::{cell::RefCell, rc::Rc};

use crate::{components::trajectory_component::segment::orbit::Orbit, storage::entity_allocator::Entity, state::State};

const MAX_DELTA: f64 = 0.0005;
const MAX_ITERATIONS: usize = 2000;
const LEARNING_RATE: f64 = 0.05;
const TIME_DELTA: f64 = 0.01;

fn distance_squared(orbit_1: &Rc<RefCell<Orbit>>, orbit_2: &Rc<RefCell<Orbit>>, time: f64) -> f64 {
    let theta_1 = orbit_1.borrow().get_theta_from_time(time);
    let theta_2 = orbit_2.borrow().get_theta_from_time(time);
    let position_1 = orbit_1.borrow().get_position_from_theta(theta_1);
    let position_2 = orbit_2.borrow().get_position_from_theta(theta_2);
    (position_1 - position_2).magnitude_squared()
}

fn distance_to_soi_edge(orbit_1: &Rc<RefCell<Orbit>>, orbit_2: &Rc<RefCell<Orbit>>, soi: f64, time: f64) -> f64 {
    distance_squared(orbit_1, orbit_2, time) - soi.powi(2)
}

pub fn get_next_intersection(state: &State, entity_1: Entity, entity_2: Entity, start_time: f64) -> f64 {
    let mass_1 = state.components.mass_components.get(&entity_1).unwrap().get_mass();
    let mass_2 = state.components.mass_components.get(&entity_2).unwrap().get_mass();
    let (soi_entity, non_soi_entity, soi_mass) = if mass_1 > mass_2 { 
        (entity_1, entity_2, mass_1)
    } else { 
        (entity_2, entity_1, mass_2)
    };

    let soi_segment = state.components.trajectory_components.get(&soi_entity).unwrap().get_final_segment();
    let soi_orbit =  soi_segment.as_orbit();
    let non_soi_segment = state.components.trajectory_components.get(&non_soi_entity).unwrap().get_final_segment();
    let non_soi_orbit =  non_soi_segment.as_orbit();
    let soi_parent_mass = state.components.mass_components.get(&non_soi_orbit.borrow().get_parent()).unwrap().get_mass();

    let soi = soi_orbit.borrow().get_semi_major_axis() * (soi_mass / soi_parent_mass).powf(2.0 / 5.0);
    let mut iterations = 0;
    let mut delta = f64::MAX;
    let mut time = start_time;

    while delta > MAX_DELTA {
        let distance_t_minus_h = distance_to_soi_edge(soi_orbit, non_soi_orbit, soi, time - TIME_DELTA);
        let distance_t = distance_to_soi_edge(soi_orbit, non_soi_orbit, soi, time);
        let distance_t_plus_h = distance_to_soi_edge(soi_orbit, non_soi_orbit, soi, time + TIME_DELTA);
        let distance_t_prime = (distance_t_plus_h - distance_t_minus_h) / (2.0 * TIME_DELTA);
        delta = f64::abs(LEARNING_RATE * -distance_t / distance_t_prime);
        time += delta;
        iterations += 1;
        if iterations > MAX_ITERATIONS {
            panic!("Exceeded max iterations solving for next SOI intersection");
        }
    }
    
    time
}