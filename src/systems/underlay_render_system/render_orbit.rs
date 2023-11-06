use eframe::epaint::Rgba;
use nalgebra_glm::DVec2;

use crate::{state::State, util::add_triangle, camera::SCALE_FACTOR, components::{position_component::PositionComponent, trajectory_component::orbit::Orbit, celestial_body_component::CelestialBodyComponent}};

const ORBIT_POINTS: usize = 256;
const ORBIT_PATH_MAX_ALPHA: f32 = 1.0;
const ORBIT_PATH_RADIUS_DELTA: f64 = 0.6;

struct VisualOrbitPoint {
    absolute_position: DVec2,
    displacement_direction: DVec2,
}

fn max_color(celestial_body_component: &CelestialBodyComponent) -> Rgba {
    // Scales the color up until one of the components is 1
    let color = celestial_body_component.get_color();
    let max_component = f32::max(color.r(), f32::max(color.g(), color.b()));
    Rgba::from_rgb(color.r() / max_component, color.g() / max_component, color.b() / max_component)
}

fn add_orbit_line(celestial_body_component: &CelestialBodyComponent, vertices: &mut Vec<f32>, previous_point: &VisualOrbitPoint, new_point: &VisualOrbitPoint, zoom: f64, i: i32) {
    let radius = ORBIT_PATH_RADIUS_DELTA + (i as f64 * ORBIT_PATH_RADIUS_DELTA); // Start off with non-zero radius
    let mut alpha = ORBIT_PATH_MAX_ALPHA;
    if i != 0 {
        // Scale the alpha non-linearly so we have lots of values close to zero
        alpha /= 7.0 * i as f32;
    }

    let rgb = max_color(celestial_body_component);
    let rgba = Rgba::from_rgba_unmultiplied(rgb.r(), rgb.g(), rgb.b(), alpha);

    let v1 = previous_point.absolute_position + (previous_point.displacement_direction * radius / zoom);
    let v2 = previous_point.absolute_position - (previous_point.displacement_direction * radius / zoom);
    let v3 = new_point.absolute_position + (new_point.displacement_direction * radius / zoom);
    let v4 = new_point.absolute_position - (new_point.displacement_direction * radius / zoom);

    add_triangle(vertices, v1, v2, v3, rgba);
    add_triangle(vertices, v2, v3, v4, rgba);
}

fn get_visual_orbit_points(position_component: &PositionComponent, orbit: &Orbit) -> Vec<VisualOrbitPoint> {
    let absolute_parent_position = position_component.get_absolute_position() * SCALE_FACTOR;
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

fn get_entity_orbit_vertices(state: &mut State, position_component: &PositionComponent, celestial_body_component: &CelestialBodyComponent, orbit: &Orbit) -> Vec<f32> {
    let zoom = state.camera.lock().unwrap().get_zoom();
    let points = get_visual_orbit_points(position_component, orbit);
    let mut previous_point = None;
    let mut vertices = vec![];
    for new_point in &points {
        // Loop to create glow effect
        if let Some(previous_point) = previous_point {
            for i in 0..10 {
                add_orbit_line(celestial_body_component, &mut vertices, previous_point, new_point, zoom, i);
            }
        }
        previous_point = Some(new_point);
    }
    vertices
}

pub fn get_all_orbit_vertices(state: &mut State) -> Vec<f32> {
    let mut vertices = vec![];
    for entity in state.components.entity_allocator.get_entities() {
        let Some(trajectory_component) = state.components.trajectory_components.get(entity) else {
            continue;
        };
        let Some(position_component) = state.components.position_components.get(entity) else {
            continue;
        };
        let Some(celestial_body_component) = state.components.celestial_body_components.get(entity) else {
            continue;
        };
        for orbit in &trajectory_component.get_orbits() {
            vertices.append(&mut get_entity_orbit_vertices(state, position_component, celestial_body_component, orbit));
        }
    }
    vertices
}