use crate::{components::icon_component::IconState, camera::SCALE_FACTOR, state::State, util::add_textured_square, storage::entity_allocator::Entity};


fn get_icon_vertices(state: &mut State, entity: Entity, zoom: f64, vertices: &mut Vec<f32>) {
    let icon_component = state.components.icon_components.get(&entity).unwrap();
    let color = match icon_component.get_state() {
        IconState::None => icon_component.get_color() * 0.75,
        IconState::Hovered => icon_component.get_color(),
        IconState::Selected => icon_component.get_color(),
    };
    let absolute_scaled_position = state.components.position_components.get(&entity).unwrap().get_absolute_position() * SCALE_FACTOR;
    let radius = icon_component.get_size(zoom);
    add_textured_square(vertices, absolute_scaled_position, radius, color);
}

fn render_icon(state: &mut State, entity: Entity, icon_name: &String, zoom: f64, vertices: &mut Vec<f32>) {
    let icon_component = state.components.icon_components.get(&entity).unwrap();
    if *icon_component.get_name() == *icon_name && icon_component.is_visible() {
        get_icon_vertices(state, entity, zoom, vertices);
    }
}

pub fn get_all_icon_vertices(state: &mut State, icon_name: String) -> Vec<f32> {
    let zoom = state.camera.lock().unwrap().get_zoom() * SCALE_FACTOR;
    let mut vertices = vec![];
    for entity in state.components.entity_allocator.get_entities() {
        if state.components.icon_components.get(&entity).is_some() {
            render_icon(state, entity, &icon_name, zoom, &mut vertices)
        }
    }
    vertices
}