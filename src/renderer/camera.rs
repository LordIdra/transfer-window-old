use eframe::{epaint::{Vec2, vec2, Rect}, egui::Context};
use nalgebra_glm::{scale, Mat4, vec3, translate};

pub struct Camera {
    world_translation: Vec2,
    zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            world_translation: vec2(0.0, 0.0),
            zoom: 1.0, 
        }
    }

    pub fn update(&mut self, context: &Context) {
        context.input(|input| {
            if input.pointer.secondary_down() {
                println!("{:?}", self.world_translation);
                self.world_translation += vec2(-input.pointer.delta().x, input.pointer.delta().y);
            }
        });
    }

    fn get_screen_translation(&self, screen_size: Rect) -> Vec2 {
        vec2(self.world_translation.x / screen_size.width(), self.world_translation.y / screen_size.height())
    }

    pub fn get_matrix(&self, screen_size: Rect) -> Mat4 {
        let matrix = Mat4::identity();
        let matrix = translate(&matrix, &vec3(
            -self.get_screen_translation(screen_size).x,
            -self.get_screen_translation(screen_size).y,
            0.0));
        scale(&matrix, &vec3(self.zoom * 2.0 / screen_size.width(), self.zoom * 2.0 / screen_size.height(), 1.0))
    }
}