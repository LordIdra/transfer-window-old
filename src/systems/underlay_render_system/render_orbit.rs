use eframe::epaint::Rgba;
use nalgebra_glm::DVec2;

use crate::{state::State, util::add_triangle, camera::SCALE_FACTOR, components::trajectory_component::orbit::Orbit};

const ORBIT_POINTS: usize = 256;
const ORBIT_PATH_MAX_ALPHA: f32 = 1.0;
const ORBIT_PATH_RADIUS_DELTA: f64 = 0.6;

struct VisualOrbitPoint {
    absolute_position: DVec2,
    displacement_direction: DVec2,
}

fn color(orbit_index: usize) -> Rgba {
    let colors = [
        Rgba::from_rgb(1.0, 0.0, 0.0),
        Rgba::from_rgb(0.0, 1.0, 0.0),
        Rgba::from_rgb(0.0, 0.0, 1.0),

        Rgba::from_rgb(1.0, 1.0, 0.0),
        Rgba::from_rgb(1.0, 0.0, 1.0),
        Rgba::from_rgb(0.0, 1.0, 1.0)];
    colors[orbit_index % colors.len()]
}

fn add_orbit_line(vertices: &mut Vec<f32>, previous_point: &VisualOrbitPoint, new_point: &VisualOrbitPoint, zoom: f64, i: i32, orbit_index: usize) {
    let radius = ORBIT_PATH_RADIUS_DELTA + (i as f64 * ORBIT_PATH_RADIUS_DELTA); // Start off with non-zero radius
    let mut alpha = ORBIT_PATH_MAX_ALPHA;
    if i != 0 {
        // Scale the alpha non-linearly so we have lots of values close to zero
        alpha /= 7.0 * i as f32;
    }

    let rgb = color(orbit_index);
    let rgba = Rgba::from_rgba_unmultiplied(rgb.r(), rgb.g(), rgb.b(), alpha);

    let v1 = previous_point.absolute_position + (previous_point.displacement_direction * radius / zoom);
    let v2 = previous_point.absolute_position - (previous_point.displacement_direction * radius / zoom);
    let v3 = new_point.absolute_position + (new_point.displacement_direction * radius / zoom);
    let v4 = new_point.absolute_position - (new_point.displacement_direction * radius / zoom);

    add_triangle(vertices, v1, v2, v3, rgba);
    add_triangle(vertices, v2, v3, v4, rgba);
}

fn get_visual_orbit_points(state: &State, orbit: &Orbit) -> Vec<VisualOrbitPoint> {
    let parent = orbit.get_parent();
    let absolute_parent_position = state.components.position_components.get(&parent).unwrap().get_absolute_position() * SCALE_FACTOR;
    let mut visual_orbit_points = vec![];
    let angle_to_rotate_through = orbit.get_remaining_angle();
    for i in 0..=ORBIT_POINTS {
        let angle = orbit.get_current_true_anomaly() + (i as f64 / ORBIT_POINTS as f64) * angle_to_rotate_through;
        let relative_point_position = orbit.get_scaled_position(angle);
        let absolute_position = absolute_parent_position + relative_point_position;
        let displacement_direction = relative_point_position.normalize();
        visual_orbit_points.push(VisualOrbitPoint { absolute_position, displacement_direction });
    }
    visual_orbit_points
}

fn get_entity_orbit_vertices(state: &State, orbit: &Orbit, orbit_index: usize) -> Vec<f32> {
    let zoom = state.camera.lock().unwrap().get_zoom();
    let points = get_visual_orbit_points(state, orbit);
    let mut previous_point = None;
    let mut vertices = vec![];
    for new_point in &points {
        // Loop to create glow effect
        if let Some(previous_point) = previous_point {
            for i in 0..10 {
                add_orbit_line(&mut vertices, previous_point, new_point, zoom, i, orbit_index);
            }
        }
        previous_point = Some(new_point);
    }
    vertices
}

pub fn get_all_orbit_vertices(state: &mut State) -> Vec<f32> {
    let mut vertices = vec![];
    for entity in state.components.entity_allocator.get_entities() {
        if let Some(trajectory_component) = state.components.trajectory_components.get(&entity) {
            for (orbit_index, orbit) in trajectory_component.get_orbits().iter().enumerate() {
                vertices.append(&mut get_entity_orbit_vertices(state, orbit, orbit_index));
            }
        }
    }
    vertices
}