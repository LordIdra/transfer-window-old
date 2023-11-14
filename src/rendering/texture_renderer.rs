use std::sync::{Arc, Mutex};

use eframe::epaint::Rect;
use glow::Context;

use crate::camera::Camera;

use super::{shader_program::ShaderProgram, vertex_array_object::{VertexArrayObject, VertexAttribute}, texture::Texture};



pub struct TextureRenderer {
    name: String,
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
    texture: Texture,
}

impl TextureRenderer {
    pub fn new(gl: Arc<Context>, texture: Texture, name: String) -> Self {
        let program = ShaderProgram::new(gl.clone(), include_str!("../../resources/shaders/icon.vert"), include_str!("../../resources/shaders/icon.frag"));
        let vertex_array_object = VertexArrayObject::new(gl.clone(), vec![
            VertexAttribute { index: 0, count: 2 }, // x
            VertexAttribute { index: 1, count: 2 }, // y
            VertexAttribute { index: 2, count: 4 }, // rgba
            VertexAttribute { index: 3, count: 2 }, // texture coordinates
        ]);
        Self { name, program, vertex_array_object, texture }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_vertices(&mut self, vertices: Vec<f32>) {
        self.vertex_array_object.data(vertices);
    }

    pub fn render(&self, screen_size: Rect, camera: Arc<Mutex<Camera>>) {
        self.texture.bind();
        self.program.use_program();
        self.program.uniform_mat3("zoom_matrix", camera.lock().unwrap().get_zoom_matrix(screen_size).as_slice());
        let translation_matrices = camera.lock().unwrap().get_translation_matrices();
        self.program.uniform_mat3("translation_matrix_upper", translation_matrices.0.as_slice());
        self.program.uniform_mat3("translation_matrix_lower", translation_matrices.1.as_slice());
        self.vertex_array_object.draw();
    }
}