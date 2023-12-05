use eframe::epaint::Rgba;


use crate::{util::add_triangle, state::State, storage::entity_allocator::Entity};

use super::visual_segment_point::VisualSegmentPoint;

const SEGMENT_RADIUS_DELTA: f64 = 0.6;

pub fn add_orbit_line(vertices: &mut Vec<f32>, previous_point: &dyn VisualSegmentPoint, new_point: &dyn VisualSegmentPoint, max_alpha: f32, zoom: f64, i: i32, color: Rgba) {
    let radius = SEGMENT_RADIUS_DELTA + (i as f64 * SEGMENT_RADIUS_DELTA); // Start off with non-zero radius
    let mut alpha = max_alpha;
    if i != 0 {
        // Scale the alpha non-linearly so we have lots of values close to zero
        alpha /= 7.0 * i as f32;
    }

    let rgba = Rgba::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);

    let v1 = previous_point.get_absolute_position() + (previous_point.get_displacement_direction() * radius / zoom);
    let v2 = previous_point.get_absolute_position() - (previous_point.get_displacement_direction() * radius / zoom);
    let v3 = new_point.get_absolute_position() + (new_point.get_displacement_direction() * radius / zoom);
    let v4 = new_point.get_absolute_position() - (new_point.get_displacement_direction() * radius / zoom);

    add_triangle(vertices, v1, v2, v3, rgba);
    add_triangle(vertices, v2, v3, v4, rgba);
}

pub fn get_entity_color(state: &State, entity: &Entity) -> Rgba {
    if let Some(celestial_body_component) = state.components.celestial_body_components.get(entity) {
        celestial_body_component.get_color()
    } else {
        Rgba::from_rgba_unmultiplied(1.0, 1.0, 1.0, 1.0)
    }
}