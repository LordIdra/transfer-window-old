use std::sync::{Arc, Mutex};

use eframe::epaint::Rect;
use glow::Context;

use crate::camera::Camera;

use self::{shader_program::ShaderProgram, vertex_array_object::{VertexArrayObject, VertexAttribute}};

mod shader_program;
mod vertex_array_object;

pub struct Renderer {
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
}

impl Renderer {
    pub fn new(gl: Arc<Context>) -> Self {
        let program = ShaderProgram::new(gl.clone(), include_str!("../resources/shaders/geometry.vert"), include_str!("../resources/shaders/geometry.frag"));
        let vertex_array_object = VertexArrayObject::new(gl.clone(), vec![
            VertexAttribute { index: 0, count: 2 },
            VertexAttribute { index: 1, count: 2 },
            VertexAttribute { index: 2, count: 4 },
        ]);
        
        Self { program, vertex_array_object }
    }

    pub fn set_vertices(&mut self, vertices: Vec<f32>) {
        self.vertex_array_object.data(vertices);
    }

    pub fn render(&self, screen_size: Rect, camera: Arc<Mutex<Camera>>) {
        self.program.use_program();
        self.program.uniform_mat3("zoom_matrix", camera.lock().unwrap().get_zoom_matrix(screen_size).as_slice());
        let translation_matrices = camera.lock().unwrap().get_translation_matrices();
        self.program.uniform_mat3("translation_matrix_upper", translation_matrices.0.as_slice());
        self.program.uniform_mat3("translation_matrix_lower", translation_matrices.1.as_slice());
        self.vertex_array_object.draw();
    }
}