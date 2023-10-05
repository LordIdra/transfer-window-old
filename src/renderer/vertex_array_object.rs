use std::sync::Arc;

use glow::{VertexArray, Buffer, Context, HasContext, ARRAY_BUFFER, FLOAT, DYNAMIC_DRAW, TRIANGLES};

pub struct VertexAttribute {
    pub index: u32,
    pub count: i32,
}

impl VertexAttribute {
    pub fn get_size(&self) -> i32 {
        self.count * std::mem::size_of::<f32>() as i32
    }
}

pub struct VertexArrayObject {
    gl: Arc<Context>,
    vertices: i32,
    vertices_per_triangle: i32,
    vertex_array: VertexArray,
    vertex_buffer: Buffer,
}

impl VertexArrayObject {
    pub fn new(gl: Arc<Context>, vertex_attributes: Vec<VertexAttribute>) -> Self {
        let vertex_array: VertexArray;
        let vertex_buffer: Buffer;
        let vertices_per_triangle = vertex_attributes.iter().map(|attribute| attribute.count).sum();
        let stride = vertex_attributes.iter().map(|attribute| attribute.get_size()).sum();

        unsafe { 
            vertex_array = gl.create_vertex_array().expect("Cannot create vertex array");
            vertex_buffer = gl.create_buffer().expect("Cannot create vertex buffer");
            gl.bind_vertex_array(Some(vertex_array));
            gl.bind_buffer(ARRAY_BUFFER, Some(vertex_buffer))
        }

        let mut offset = 0;
        for attribute in vertex_attributes {
            unsafe { 
                gl.vertex_attrib_pointer_f32(attribute.index, attribute.count, FLOAT, false, stride, offset);
                gl.enable_vertex_attrib_array(attribute.index);
            };
            offset += attribute.get_size();
        }

        VertexArrayObject { gl, vertices: 0, vertices_per_triangle, vertex_array, vertex_buffer }
    }

    fn bind(&self) {
        unsafe {
            self.gl.bind_vertex_array(Some(self.vertex_array));
            self.gl.bind_buffer(ARRAY_BUFFER, Some(self.vertex_buffer))
        }
    }

    pub fn data(&mut self, data: Vec<f32>) {
        let byte_count = data.len() * std::mem::size_of::<f32>();
        self.vertices = data.len() as i32;
        unsafe {
            let bytes = std::slice::from_raw_parts(data.as_ptr() as *const u8, byte_count);
            self.bind();
            self.gl.buffer_data_u8_slice(ARRAY_BUFFER, bytes, DYNAMIC_DRAW);
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.bind();
            self.gl.draw_arrays(TRIANGLES, 0, self.vertices / self.vertices_per_triangle);
        }
    }
}

impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_vertex_array(self.vertex_array);
            self.gl.delete_buffer(self.vertex_buffer);
        }
    }
}