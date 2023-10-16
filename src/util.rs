use eframe::epaint::Rgba;
use nalgebra_glm::{Vec2, DVec2, vec2};

pub fn add_triangle(vertices: &mut Vec<f32>, v1: DVec2, v2: DVec2, v3: DVec2, color: Rgba) {
    let v1 = drop_precision(v1);
    let v2 = drop_precision(v2);
    let v3 = drop_precision(v3);
    vertices.append(&mut vec![v1.x, v1.y, color.r(), color.g(), color.b(), color.a()]);
    vertices.append(&mut vec![v2.x, v2.y, color.r(), color.g(), color.b(), color.a()]);
    vertices.append(&mut vec![v3.x, v3.y, color.r(), color.g(), color.b(), color.a()]);
}

pub fn drop_precision(vec: DVec2) -> Vec2 {
    vec2(vec.x as f32, vec.y as f32)
}