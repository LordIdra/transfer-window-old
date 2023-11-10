use std::sync::{Arc, Mutex};

use eframe::epaint::Rect;
use glow::Context;

use crate::camera::Camera;

use super::{shader_program::ShaderProgram, vertex_array_object::{VertexArrayObject, VertexAttribute}, texture::Texture};



pub struct IconRenderer {
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
}

impl IconRenderer {
    pub fn new(gl: Arc<Context>) -> Self {
        let program = ShaderProgram::new(gl.clone(), include_str!("../../resources/shaders/icon.vert"), include_str!("../../resources/shaders/icon.frag"));
        let vertex_array_object = VertexArrayObject::new(gl.clone(), vec![
            VertexAttribute { index: 0, count: 2 }, // x
            VertexAttribute { index: 1, count: 2 }, // y
            VertexAttribute { index: 2, count: 4 }, // rgba
            VertexAttribute { index: 3, count: 2 }, // texture coordinates
        ]);
        Self { program, vertex_array_object }
    }

    pub fn set_vertices(&mut self, vertices: Vec<f32>) {
        self.vertex_array_object.data(vertices);
    }

    pub fn render(&self, screen_size: Rect, camera: Arc<Mutex<Camera>>, texture: Texture) {
        texture.bind();
        self.program.use_program();
        self.program.uniform_mat3("zoom_matrix", camera.lock().unwrap().get_zoom_matrix(screen_size).as_slice());
        let translation_matrices = camera.lock().unwrap().get_translation_matrices();
        self.program.uniform_mat3("translation_matrix_upper", translation_matrices.0.as_slice());
        self.program.uniform_mat3("translation_matrix_lower", translation_matrices.1.as_slice());
        self.vertex_array_object.draw();
    }
}