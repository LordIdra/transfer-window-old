use eframe::epaint::Rgba;
use nalgebra_glm::vec2;

use crate::{components::{icon_component::{IconComponent, IconState}, position_component::PositionComponent}, camera::SCALE_FACTOR, state::State, util::add_textured_triangle};


fn get_entity_icon_vertices(position_component: &PositionComponent, icon_component: &IconComponent, zoom: f64, vertices: &mut Vec<f32>) {
    let absolute_scaled_position = position_component.get_absolute_position() * SCALE_FACTOR;
    let radius = icon_component.get_icon_size(zoom);
    let v1 = vec2(absolute_scaled_position.x - radius, absolute_scaled_position.y - radius);
    let v2 = vec2(absolute_scaled_position.x - radius, absolute_scaled_position.y + radius);
    let v3 = vec2(absolute_scaled_position.x + radius, absolute_scaled_position.y - radius);
    let v4 = vec2(absolute_scaled_position.x + radius, absolute_scaled_position.y + radius);
    let color = match icon_component.get_state() {
        IconState::None => Rgba::from_rgba_premultiplied(1.0, 1.0, 1.0, 0.5),
        IconState::Hovered => Rgba::from_rgba_premultiplied(1.0, 1.0, 1.0, 1.0),
        IconState::Selected => Rgba::from_rgba_premultiplied(1.0, 1.0, 1.0, 1.0),
    };
    add_textured_triangle(vertices, v1, v2, v3, color, vec2(0.0, 1.0), vec2(0.0, 0.0), vec2(1.0, 1.0));
    add_textured_triangle(vertices, v4, v2, v3, color, vec2(1.0, 0.0), vec2(0.0, 0.0), vec2(1.0, 1.0));
}

pub fn get_all_icon_vertices(state: &mut State, name: String) -> Vec<f32> {
    let zoom = state.camera.lock().unwrap().get_zoom() * SCALE_FACTOR;
    let mut vertices = vec![];
    for entity in state.components.entity_allocator.get_entities() {
        let Some(position_component) = state.components.position_components.get(&entity) else {
            continue;
        };
        let Some(icon_component) = state.components.icon_components.get(&entity) else {
            continue;
        };
        if *icon_component.get_icon_name() != name || !icon_component.is_visible() {
            continue;
        }
        get_entity_icon_vertices(position_component, icon_component, zoom, &mut vertices);
    }
    vertices
}