use nalgebra_glm::DVec2;

use crate::{components::{trajectory_component::TrajectoryComponent, position_component::PositionComponent, velocity_component::VelocityComponent, parent_component::ParentComponent}, state::State, storage::entity_allocator::Entity};

fn update_position_and_velocity(state: &mut State, entity: Entity, new_relative_position: Option<DVec2>, new_relative_velocity: Option<DVec2>) {
    let parent = state.components.parent_components.get(&entity).unwrap().get_parent();
    if let Some(new_relative_position) = new_relative_position {
        let parent_absolute_position = state.components.position_components.get(&parent).unwrap().get_absolute_position();
        state.components.position_components.get(&entity).unwrap().set_absolute_position(parent_absolute_position + new_relative_position);
    }
    if let Some(velocity) = new_velocity {
        self.velocity = velocity;
    }
}

fn sync_to_trajectory(trajectory_component: &TrajectoryComponent, position_component: &PositionComponent, velocity_component: &VelocityComponent) {
    let new_position = trajectory_component.get_current_unscaled_position();
    let new_velocity = trajectory_component.get_current_velocity();
    update_position_and_velocity(new_position, new_velocity);
    self.parent = self.trajectory.borrow().get_current_parent();
}

