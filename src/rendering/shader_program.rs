use std::sync::Arc;

use glow::{Context, Program, HasContext, VERTEX_SHADER, FRAGMENT_SHADER, NativeUniformLocation};

struct Shader {
    gl: Arc<Context>,
    shader: glow::Shader,
}

impl Shader {
    fn new(gl: Arc<Context>, shader_source: String, shader_type: u32) -> Self {
        unsafe {
            let shader = gl.create_shader(shader_type).expect("Failed to create shader");
            gl.shader_source(shader, &shader_source);
            gl.compile_shader(shader);
            assert!(gl.get_shader_compile_status(shader), "Failed to compile shader:\n{}", gl.get_shader_info_log(shader));
            Shader { gl, shader }
        }
    }

    fn attach(&self, program: Program) {
        unsafe {
            self.gl.attach_shader(program, self.shader);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { 
            self.gl.delete_shader(self.shader) 
        };
    }
}

pub struct ShaderProgram {
    gl: Arc<Context>, 
    program: Program,
}

impl ShaderProgram {
    pub fn new(gl: Arc<Context>, vertex_shader_source: &str, fragment_shader_source: &str) -> Self {
        let vertex_shader = Shader::new(gl.clone(), vertex_shader_source.to_string(), VERTEX_SHADER);
        let fragment_shader = Shader::new(gl.clone(), fragment_shader_source.to_string(), FRAGMENT_SHADER);
        let program = unsafe { gl.create_program().expect("Failed to create shader program") };
        vertex_shader.attach(program);
        fragment_shader.attach(program);

        unsafe {
            gl.link_program(program);
            assert!(gl.get_program_link_status(program), "{}", gl.get_program_info_log(program));
        }

        ShaderProgram { gl, program }
    }

    pub fn use_program(&self) {
        unsafe { self.gl.use_program(Some(self.program)) };
    }

    fn get_location(&self, name: &str) -> NativeUniformLocation {
        unsafe { self.gl.get_uniform_location(self.program, name).unwrap_or_else(|| panic!("Failed to find uniform location '{}'", name)) }
    }

    pub fn uniform_mat3(&self, name: &str, v: &[f32]) {
        self.use_program();
        unsafe { self.gl.uniform_matrix_3_f32_slice(Some(&Self::get_location(self, name)), false, v); }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { 
            self.gl.delete_program(self.program) 
        };
    }
}