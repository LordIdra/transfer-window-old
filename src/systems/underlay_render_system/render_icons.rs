use eframe::epaint::Rgba;

use crate::{components::icon_component::{IconState, IconComponent, IconType}, camera::SCALE_FACTOR, state::State, util::add_textured_square};


fn get_icon_vertices(icon_component: &IconComponent, zoom: f64, vertices: &mut Vec<f32>) {
    let color = match icon_component.get_state() {
        IconState::None => Rgba::from_rgba_premultiplied(1.0, 1.0, 1.0, 0.5),
        IconState::Hovered => Rgba::from_rgba_premultiplied(1.0, 1.0, 1.0, 1.0),
        IconState::Selected => Rgba::from_rgba_premultiplied(1.0, 1.0, 1.0, 1.0),
    };
    let absolute_scaled_position = icon_component.get_position() * SCALE_FACTOR;
    let radius = icon_component.get_icon_size(zoom);
    add_textured_square(vertices, absolute_scaled_position, radius, color);
}

fn render_object_icon(icon_component: &IconComponent, icon_name: &String, zoom: f64, vertices: &mut Vec<f32>) {
    if *icon_component.get_icon_name() == *icon_name && icon_component.is_visible() {
        get_icon_vertices(icon_component, zoom, vertices);
    }
}

pub fn get_all_icon_vertices(state: &mut State, icon_name: String) -> Vec<f32> {
    let zoom = state.camera.lock().unwrap().get_zoom() * SCALE_FACTOR;
    let mut vertices = vec![];
    for entity in state.components.entity_allocator.get_entities() {
        if let Some(icon_component) = state.components.icon_components.get(&entity) {
            match icon_component.get_icon_type() {
                IconType::ObjectIcon => render_object_icon(icon_component, &icon_name, zoom, &mut vertices),
                IconType::BurnIcon => todo!(),
            }
        }
    }
    vertices
}