use nalgebra_glm::DVec2;

use crate::{state::State, storage::entity_allocator::Entity, components::trajectory_component::segment::Segment};

/// So... why is this an entire function? Surely we can just find the parent component and use that to set the new parent?
/// Well, the problem with that is that the old parent will still have the entity in its children
/// So we actually need to do 3 things
/// 1) update the parent component of the specified entity
/// 2) remove the specified entity from the children (celestial body) component of its old parent
/// 3) add the specified entity to the children (celestial body) component of its new parent
pub fn update_parent(state: &mut State, entity: Entity, new_parent: &Entity) {
    let old_parent = state.components.parent_components.get(&entity).unwrap().get_parent();
    if *new_parent != old_parent {
        state.components.celestial_body_components.get_mut(&old_parent).unwrap().remove_child(entity);
        state.components.celestial_body_components.get_mut(new_parent).unwrap().add_child(entity);
        state.components.parent_components.get_mut(&entity).unwrap().set_parent(*new_parent);
    }
}

/// Takes in relative positions/velocities, turns them into absolute positions/velocities, and updates the entity's position/velocity components accordingly
pub fn update_position_and_velocity(state: &mut State, entity: &Entity, new_relative_position: DVec2, new_relative_velocity: DVec2) {
    let parent = state.components.parent_components.get(entity).unwrap().get_parent();
    let parent_absolute_position = state.components.position_components.get(&parent).unwrap().get_absolute_position();
    let parent_absolute_velocity = state.components.velocity_components.get(&parent).unwrap().get_absolute_velocity();
    state.components.position_components.get_mut(entity).unwrap().set_absolute_position(parent_absolute_position + new_relative_position);
    state.components.velocity_components.get_mut(entity).unwrap().set_absolute_velocity(parent_absolute_velocity + new_relative_velocity);
}

/// Sync the position, velocity, and parent of the entity to the position, velocity, and parent of the current orbit
pub fn sync_to_segment(state: &mut State, segment: Segment, entity: Entity) {
    let new_position = segment.get_current_position();
    let new_velocity = segment.get_current_velocity();
    let new_parent = segment.get_parent();
    update_parent(state, entity, &new_parent);
    update_position_and_velocity(state, &entity, new_position, new_velocity);
}

pub fn sync_to_trajectory(state: &mut State, entity: Entity) {
    let segment = state.components.trajectory_components.get(&entity).unwrap().get_current_segment();
    sync_to_segment(state, segment, entity);
}


pub fn get_segment_at_time(state: &State, entity: &Entity, time: f64) -> Segment {
    for segment in state.components.trajectory_components.get(entity).unwrap().get_segments() {
        let start_time = segment.get_start_time();
        let end_time = segment.get_end_time();
        if time >= start_time && time <= end_time {
            return segment.clone();
        }
    }
    panic!("Failed to move to time; no trajectory segment contains the requested time")
}


pub fn get_all_entity_children(state: &State, entities: &Vec<Entity>) -> Vec<Entity> {
    let mut new_entities = vec![];
    for entity in entities {
        if let Some(celestial_body_component) = state.components.celestial_body_components.get(entity) {
            new_entities.extend(celestial_body_component.get_children());
        }
    }
    new_entities
}

pub fn format_time(time: f64) -> String {
    let years_quotient = f64::floor(time / (360.0 * 24.0 * 60.0 * 60.0));
    let years_remainder = time % (360.0 * 24.0 * 60.0 * 60.0);
    let days_quotient = f64::floor(years_remainder / (24.0 * 60.0 * 60.0));
    let days_remainder = years_remainder % (24.0 * 60.0 * 60.0);
    let hours_quotient = f64::floor(days_remainder / (60.0 * 60.0));
    let hours_remainder = days_remainder % (60.0 * 60.0);
    let minutes_quotient = f64::floor(hours_remainder / 60.0);
    let seconds = f64::round(hours_remainder % 60.0);
    if years_quotient != 0.0 {
        "".to_string()
            + years_quotient.to_string().as_str() + "y"
            + days_quotient.to_string().as_str() + "d"
            + hours_quotient.to_string().as_str() + "h"
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else if days_quotient != 0.0 {
        "".to_string()
            + days_quotient.to_string().as_str() + "d"
            + hours_quotient.to_string().as_str() + "h"
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else if hours_quotient != 0.0 {
        "".to_string()
            + hours_quotient.to_string().as_str() + "h"
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else if minutes_quotient != 0.0 {
        "".to_string()
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else {
        "".to_string()
            + seconds.to_string().as_str() + "s"
    }
}

pub fn is_celestial_body_with_trajectory(state: &State, entity: Entity) -> bool {
    state.components.trajectory_components.get(&entity).is_some() && state.components.celestial_body_components.get(&entity).is_some()
}

pub fn is_spacecraft_with_trajectory(state: &State, entity: Entity) -> bool {
    state.components.trajectory_components.get(&entity).is_some() && state.components.celestial_body_components.get(&entity).is_none()
}

pub fn sync_entity_to_time(state: &mut State, entity: Entity, time: f64) {
    let mut segment = get_segment_at_time(state, &entity, time);
    let delta_time = time - segment.get_start_time();
    segment.reset();
    segment.update(delta_time);
    sync_to_segment(state, segment, entity)
}

/// We could in theory just do this recursively, but there's an edge case that pops up with
/// this approach. Here's the problem: Imagine we have the following system:
/// Sun
/// |- Earth
///    |- Spacecraft
///    |- Moon
/// Now, imagine we update the absolute position of the spacecraft, then the moon
/// No problem right?
/// Now imagine we do the same, except the spacecraft changes SOI so it's now in the moon's SOI
/// Okay, well now we've updated the spacecraft's parent followed by its position and velocity...
/// But the position/velocity of its new parent has NOT been updated yet
/// So we're actually slightly behind
/// This is solved by simply updating elements from the highest to lowest mass

pub fn sync_all_entities(state: &mut State) {
    for entity in state.get_entities_sorted_by_mass() {
        if state.components.trajectory_components.get_mut(&entity).is_some() {
            sync_to_trajectory(state, entity);
        }
    }
}

pub fn sync_celestial_bodies_to_time(state: &mut State, time: f64) {
    for entity in state.get_entities_sorted_by_mass() {
        if is_celestial_body_with_trajectory(state, entity) {
            sync_entity_to_time(state, entity, time);
        }
    }
}