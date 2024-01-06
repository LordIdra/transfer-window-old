use std::{cell::RefCell, rc::Rc, cmp::Ordering};

use crate::{state::State, storage::entity_allocator::Entity, systems::{util::get_segment_at_time, debug_system::debug_utils::format_time}, components::trajectory_component::segment::{orbit::Orbit, Segment}};

use self::util::calculate_soi;

use super::newton_raphson::newton_raphson;

mod ellipse_ellipse;
mod hyperbola_ellipse;
mod util;

#[derive(Clone)]
pub enum SoiChangeType {
    Entrance(SoiChange),
    Exit(SoiChange),
}

impl SoiChangeType {
    pub fn partial_cmp(&self, other: &Self) -> Ordering {
        let soi_1 = match self {
            SoiChangeType::Entrance(entrance) => entrance,
            SoiChangeType::Exit(exit) => exit,
        };
        let soi_2 = match other {
            SoiChangeType::Entrance(entrance) => entrance,
            SoiChangeType::Exit(exit) => exit,
        };
        soi_2.partial_cmp(soi_1)
    }

    pub fn get_time(&self) -> f64 {
        match self {
            SoiChangeType::Entrance(entrance) => entrance.get_time(),
            SoiChangeType::Exit(exit) => exit.get_time(),
        }
    }

    pub fn get_other_entity(&self) -> Entity {
        match self {
            SoiChangeType::Entrance(entrance) => entrance.get_other_entity(),
            SoiChangeType::Exit(exit) => exit.get_other_entity(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SoiChange {
    other_entity: Entity,
    time: f64,
}

impl SoiChange {
    pub fn get_other_entity(&self) -> Entity {
        self.other_entity
    }

    pub fn get_time(&self) -> f64 {
        self.time
    }
}

impl SoiChange {
    pub fn new(other_entity: Entity, time: f64) -> Self {
        Self { other_entity, time }
    }

    pub fn partial_cmp(&self, other: &Self) -> Ordering {
        self.time.partial_cmp(&other.time).unwrap()
    }
}

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

fn get_entrance_changes(state: &mut State, entity: Entity, orbit: &Rc<RefCell<Orbit>>, start_time: f64, end_time: f64) -> Vec<SoiChangeType> {
    let mut entrance_changes = vec![];
    for other_entity in get_parallel_entities(state, orbit.borrow().get_parent(), start_time) {
        if other_entity == entity {
            continue;
        }
        if let Some(other_trajectory_component) = state.components.trajectory_components.get(&other_entity) {
            for other_segment in other_trajectory_component.get_segments().clone() {
                if let Segment::Orbit(other_orbit) = other_segment {
                    if   other_orbit.borrow().get_parent() != orbit.borrow().get_parent()
                      || other_orbit.borrow().get_end_point().get_time() < start_time
                      || other_orbit.borrow().get_start_point().get_time() > end_time {
                        continue;
                    }
                    let other_mass = state.components.mass_components.get(&other_entity).unwrap().get_mass();
                    let entity_entrance_times = ellipse_ellipse::get_entity_entrance_times(state, orbit, &other_orbit, other_mass, start_time, end_time);
                    for entrance_time in entity_entrance_times {
                        entrance_changes.push(SoiChangeType::Entrance(SoiChange::new(other_entity, entrance_time)));
                    }
                }
            }
        }
    }
    entrance_changes
}

fn get_exit_change(state: &State, orbit: &Rc<RefCell<Orbit>>, parent: Entity, start_time: f64, end_time: f64) -> Option<SoiChangeType> {
    let parent_mass = state.components.mass_components.get(&parent).unwrap().get_mass();
    let parent_segment = get_segment_at_time(state, &parent, start_time);
    let parent_orbit = parent_segment.as_orbit();
    let soi = calculate_soi(state, parent_mass, parent_orbit);
    let distance_function_time = move |time: f64| -> f64 {
        let theta = orbit.borrow().get_theta_from_time(time);
        orbit.borrow().get_position_from_theta(theta).magnitude() - soi
    };
    if let Some(first_solution) = newton_raphson(&distance_function_time, start_time) {
        let periapsis_time = orbit.borrow().get_periapsis_time();
        let time_to_periapsis = (periapsis_time - first_solution).abs();
        let positive_solution = newton_raphson(&distance_function_time, periapsis_time + time_to_periapsis).expect("Failed to converge to other solution");
        if positive_solution > start_time && positive_solution < end_time {
            let new_parent = parent_orbit.borrow().get_parent();
            Some(SoiChangeType::Exit(SoiChange::new(new_parent, positive_solution)))
        } else { 
            None
        }
    } else {
        None
    }
}

pub fn find_next_soi_change(state: &mut State, entity: Entity, start_time: f64, end_time: f64) -> Option<SoiChangeType> {
    let segment = state.components.trajectory_components.get(&entity).unwrap().get_final_segment();
    let orbit = segment.as_orbit();
    let parent = orbit.borrow().get_parent();
    let mut soi_changes = get_entrance_changes(state, entity, orbit, start_time, end_time);
    if let Some(change) = get_exit_change(state, orbit, parent, start_time, end_time) {
        soi_changes.push(change);
    }
    soi_changes.sort_by(|a, b| a.partial_cmp(b));
    soi_changes.reverse();
    // make sure we're not using the same SOI change that just happened (if one happened)
    if let Some(soi_change) = soi_changes.first() {
        if (soi_change.get_time() - start_time).abs() < 100.0 {
            soi_changes.pop();
        }
    }
    
    println!("[ possible changes ]");
    for change in &soi_changes {
        println!("{} {} {}", state.components.name_components.get(&change.get_other_entity()).unwrap().get_name(), 
            change.get_time(), format_time(change.get_time()));
    }
    println!();

    soi_changes.first().cloned()
}
