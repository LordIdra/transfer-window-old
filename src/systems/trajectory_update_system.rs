use crate::{state::State, storage::entity_allocator::Entity, util::get_root_entities};

use super::util::sync_to_trajectory;

/// Recursive function that:
/// 1) Updates the position, velocity, and parent of the entity in accordance with its current trajectory
/// 2) Calls itself for every child of that entity
/// So, why do this recursively instead of just looping over ever entity?
/// Well, we're actually going to use each entity's parent to compute its absolute position
/// If we update in an arbitrary order, some of those parents will be updated and some won't
/// So we'd end up with entities that are actually 1 time step behind because their parents weren't updated in time!
/// So starting from the roots and recursing prevents this problem
/// BUT, it's still an issue if the parent is updated during the time step, because we could be entering an adjacent entity's SOI
/// This isn't really a big deal though - might fix in future?
fn update_trajectory(state: &mut State, entity: &Entity) {
    if let Some(trajectory_component) = state.components.trajectory_components.get_mut(entity) {
        trajectory_component.update(state.delta_time);
        sync_to_trajectory(state, entity);
    }
    if let Some(celestial_body_component) = state.components.celestial_body_components.get(entity) {
        for child in celestial_body_component.get_children().clone() {
            update_trajectory(state, &child);
        }
    }
}

pub fn trajectory_update_system(state: &mut State) {
    for entity in get_root_entities(state) {
        update_trajectory(state, &entity);
    }
}