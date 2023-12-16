use std::{rc::Rc, cell::RefCell};

use nalgebra_glm::DVec2;

use crate::{state::State, storage::entity_allocator::Entity, components::trajectory_component::segment::{Segment, orbit::{Orbit, orbit_direction::GRAVITATIONAL_CONSTANT}}, systems::util::update_parent};

pub type SoiFunction = Box<dyn Fn(&State, &Entity) -> Option<f64>>;

fn position_relative_to_parent(state: &State, entity: &Entity, parent: &Entity) -> DVec2 {
    state.components.position_components.get(entity).unwrap().get_absolute_position()- state.components.position_components.get(parent).unwrap().get_absolute_position()
}

fn velocity_relative_to_parent(state: &State, entity: &Entity, parent: &Entity) -> DVec2 {
    state.components.velocity_components.get(entity).unwrap().get_absolute_velocity() - state.components.velocity_components.get(parent).unwrap().get_absolute_velocity()
}

pub fn change_parent(state: &mut State, entity: &Entity, new_parent: Entity, time: f64) {
    let new_position = position_relative_to_parent(state, entity, &new_parent);
    let new_velocity = velocity_relative_to_parent(state, entity, &new_parent);
    let new_orbit = Orbit::new(&state.components, new_parent, new_position, new_velocity, time);
    state.components.trajectory_components.get_mut(entity).unwrap().add_segment(Segment::Orbit(Rc::new(RefCell::new(new_orbit))));
}

pub fn entity_causing_highest_acceleration(state: &State, entity: &Entity, entities: Vec<Entity>) -> Option<Entity> {
    let highest_acceleration = 0.0;
    let mut object_causing_highest_acceleration = None;
    for other_entity in &entities {
        let position = state.components.position_components.get(entity).unwrap().get_absolute_position();
        let other_position = state.components.position_components.get(other_entity).unwrap().get_absolute_position();
        let other_mass = state.components.mass_components.get(other_entity).unwrap().get_mass();
        let acceleration = other_mass * GRAVITATIONAL_CONSTANT / (position - other_position).magnitude_squared();
        if acceleration > highest_acceleration {
            object_causing_highest_acceleration = Some(*other_entity);
        }
    }
    object_causing_highest_acceleration
}

/// Looks to ascend HIGHER into the entity tree to compute a parent
/// For example, this could be a spacecraft leaving the Earth's SOI to enter the Sun's SOI
/// In this case, there's only one possible new parent (ie, the current parent's parent), making this fairly simple
fn compute_new_parent_upper(state: &State, soi_function: &SoiFunction, entity: &Entity, parent: &Entity) -> Option<Entity> {
    // Check if we've left the SOI of our parent
    let parent_sphere_of_influence_squared = soi_function(state, parent)?;
    if position_relative_to_parent(state, entity, parent).magnitude() < parent_sphere_of_influence_squared {
        return None;
    }
    state.components.parent_components.get(parent).map(|parent_parent| parent_parent.get_parent())
}

/// Looks to descend LOWER into the entity tree to compute a parent
/// For example, this could be a spacecraft leaving the Earth's SOI to enter the Moon's SOI
/// This means there could be multiple entities to check
/// Also, it's possible we'll be in the SOI of several entities
/// In this case, we calculate which entity is causing the highest acceleration and choose that one
fn compute_new_parent_lower(state: &State, soi_function: &SoiFunction, entity: &Entity, parent: &Entity) -> Option<Entity> {
    // Check if we've entered the SOI of any objects with the same parent
    let mut potential_children = vec![];
    for child in state.components.celestial_body_components.get(parent).unwrap().get_children() {
        if *child == *entity {
            continue;
        }
        if let Some(parent_sphere_of_influence) = soi_function(state, child) {
            let position = state.components.position_components.get(entity).unwrap().get_absolute_position();
            let other_position = state.components.position_components.get(child).unwrap().get_absolute_position();
            if (position - other_position).magnitude() < parent_sphere_of_influence {
                potential_children.push(*child);
            }
        }
    }
    entity_causing_highest_acceleration(state, entity, potential_children)
}

pub fn update_parent_for_prediction(state: &mut State, soi_function: SoiFunction, entity: &Entity, time: f64) {
    let parent_component = state.components.parent_components.get(entity).unwrap();
    let parent = parent_component.get_parent();
    if let Some(new_parent) = compute_new_parent_lower(state, &soi_function, entity, &parent) {
        change_parent(state, entity, new_parent, time);
        update_parent(state, entity, &new_parent);
    } else if let Some(new_parent) = compute_new_parent_upper(state, &soi_function, entity, &parent) {
        change_parent(state, entity, new_parent, time);
        update_parent(state, entity, &new_parent);
    }
}
