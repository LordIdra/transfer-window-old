use eframe::epaint::Rgba;
use nalgebra_glm::DVec2;

use crate::{state::State, storage::entity_allocator::Entity};

pub fn add_triangle(vertices: &mut Vec<f32>, v1: DVec2, v2: DVec2, v3: DVec2, color: Rgba) {
    let v1 = dvec2_to_f32_tuple(v1);
    let v2 = dvec2_to_f32_tuple(v2);
    let v3 = dvec2_to_f32_tuple(v3);
    vertices.append(&mut vec![v1.0.0, v1.0.1, v1.1.0, v1.1.1, color.r(), color.g(), color.b(), color.a()]);
    vertices.append(&mut vec![v2.0.0, v2.0.1, v2.1.0, v2.1.1, color.r(), color.g(), color.b(), color.a()]);
    vertices.append(&mut vec![v3.0.0, v3.0.1, v3.1.0, v3.1.1, color.r(), color.g(), color.b(), color.a()]);
}

fn dvec2_to_f32_tuple(vec: DVec2) -> ((f32, f32), (f32, f32)) {
    (f64_to_f32_pair(vec.x), f64_to_f32_pair(vec.y))
}

pub fn f64_to_f32_pair(v: f64) -> (f32, f32) {
    let upper = v as f32;
    let lower = (v - upper as f64) as f32;
    (upper, lower)
}

pub fn get_root_entities(state: &State) -> Vec<Entity> {
    let mut entities = vec![];
    for entity in state.components.entity_allocator.get_entities() {
        if state.components.celestial_body_components.get(&entity).is_some() && state.components.parent_components.get(&entity).is_some() {
            entities.push(entity);
        }
    }
    entities
}