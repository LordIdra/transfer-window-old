use std::sync::{Arc, Mutex};

use eframe::{epaint::Rect, egui::{self}};
use glow::Context;

use crate::camera::Camera;

use self::{shader_program::ShaderProgram, vertex_array_object::{VertexArrayObject, VertexAttribute}};

mod shader_program;
mod vertex_array_object;

pub struct Renderer {
    program: ShaderProgram,
    vertex_array_object: VertexArrayObject,
    camera: Arc<Mutex<Camera>>,
}

impl Renderer {
    pub fn new(gl: Arc<Context>, camera: Arc<Mutex<Camera>>) -> Self {
        let program = ShaderProgram::new(gl.clone(), include_str!("../resources/shaders/geometry.vert"), include_str!("../resources/shaders/geometry.frag"));
        let vertex_array_object = VertexArrayObject::new(gl.clone(), vec![
            VertexAttribute { index: 0, count: 2 },
            VertexAttribute { index: 1, count: 4 },
        ]);
        
        Self { program, vertex_array_object, camera }
    }

    pub fn set_vertices(&mut self, vertices: Vec<f32>) {
        self.vertex_array_object.data(vertices);
    }

    pub fn update(&mut self, context: &egui::Context) {
        self.camera.lock().unwrap().update(context);
    }

    pub fn render(&self, screen_size: Rect) {
        self.program.use_program();
        self.program.uniform_mat3("matrix", self.camera.lock().unwrap().get_matrix(screen_size).as_slice());
        self.vertex_array_object.draw();
    }
}