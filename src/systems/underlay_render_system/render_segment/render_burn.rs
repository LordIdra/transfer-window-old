use nalgebra_glm::DVec2;

use crate::{components::trajectory_component::segment::burn::Burn, state::State, camera::SCALE_FACTOR, storage::entity_allocator::Entity};

use super::{util::{add_orbit_line, get_entity_color}, visual_segment_point::VisualBurnPoint};

const BURN_PATH_MAX_ALPHA: f32 = 1.0;
const POINTS_PER_SECOND: f64 = 0.5;

fn create_visual_orbit_point(burn: &Burn, absolute_parent_position: DVec2, time: f64) -> VisualBurnPoint {
    let relative_point_position = burn.get_point_at_time(time).get_position() * SCALE_FACTOR;
    let absolute_position = absolute_parent_position + relative_point_position;
    let displacement_direction = relative_point_position.normalize();
    VisualBurnPoint { absolute_position, displacement_direction }
}

fn get_visual_burn_points(state: &State, burn: &Burn) -> Vec<VisualBurnPoint> {
    let parent = burn.get_parent();
    let absolute_parent_position = state.components.position_components.get(&parent).unwrap().get_absolute_position() * SCALE_FACTOR;
    let mut visual_points = vec![];
    let start_time = f64::max(burn.get_start_time(), state.time);
    let points = (burn.get_end_time() - start_time * POINTS_PER_SECOND) as i32 + 1;
    for i in 0..points {
        let time = start_time + (i as f64 / points as f64) * burn.get_duration();
        visual_points.push(create_visual_orbit_point(burn, absolute_parent_position, time));
    }
    visual_points
}

pub fn get_entity_burn_vertices(state: &State, entity: &Entity, burn: &Burn) -> Vec<f32> {
    let zoom = state.camera.lock().unwrap().get_zoom();
    let points = get_visual_burn_points(state, burn);
    let mut previous_point = None;
    let mut vertices = vec![];
    let color = get_entity_color(state, entity);
    for new_point in &points {
        // Loop to create glow effect
        if let Some(previous_point) = previous_point {
            for i in 0..10 {
                add_orbit_line(&mut vertices, previous_point, new_point, BURN_PATH_MAX_ALPHA, zoom, i, color);
            }
        }
        previous_point = Some(new_point);
    }
    vertices
}