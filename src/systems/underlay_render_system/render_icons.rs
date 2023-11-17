use eframe::epaint::Rgba;

use crate::{components::{icon_component::{IconComponent, IconState}, position_component::PositionComponent}, camera::SCALE_FACTOR, state::State, util::add_textured_square};


fn get_entity_icon_vertices(position_component: &PositionComponent, icon_component: &IconComponent, zoom: f64, vertices: &mut Vec<f32>) {
    let color = match icon_component.get_state() {
        IconState::None => Rgba::from_rgba_premultiplied(1.0, 1.0, 1.0, 0.5),
        IconState::Hovered => Rgba::from_rgba_premultiplied(1.0, 1.0, 1.0, 1.0),
        IconState::Selected => Rgba::from_rgba_premultiplied(1.0, 1.0, 1.0, 1.0),
    };
    let absolute_scaled_position = position_component.get_absolute_position() * SCALE_FACTOR;
    let radius = icon_component.get_icon_size(zoom);
    add_textured_square(vertices, absolute_scaled_position, radius, color);
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