use eframe::{epaint::Rect, egui::Context};
use nalgebra_glm::{scale, Mat4, vec3, translate, Vec2, vec2};

const ZOOM_SENSITIVITY: f32 = 0.003;

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

    fn translate(&mut self, amount: Vec2) {
        self.world_translation += amount / self.zoom;
        println!("{:?} {:?}", self.world_translation, amount / self.zoom)
    }

    pub fn update(&mut self, context: &Context) {
        context.input(|input| {
            if input.pointer.secondary_down() {
                self.translate(0.5 * vec2(-input.pointer.delta().x, input.pointer.delta().y));
            }

            if let Some(latest_mouse_position) = input.pointer.latest_pos() {
                let screen_size = vec2(context.screen_rect().width(), context.screen_rect().height());
                let new_zoom = self.zoom * (1.0 + ZOOM_SENSITIVITY * input.scroll_delta.y);
                let delta_zoom = (self.zoom - new_zoom) / new_zoom;
                let mouse_position = vec2(
                    -(latest_mouse_position.x - (screen_size.x / 2.0)), 
                      latest_mouse_position.y - (screen_size.y / 2.0));
                self.translate(mouse_position * delta_zoom);
                self.zoom = new_zoom;
            }
        });
    }

    pub fn get_matrix(&self, screen_size: Rect) -> Mat4 {
        let matrix = Mat4::identity();
        let matrix = scale(&matrix, &vec3(2.0 / screen_size.width(), 2.0 / screen_size.height(), 1.0));
        let matrix = scale(&matrix, &vec3(self.zoom, self.zoom, 1.0));
        translate(&matrix, &vec3(
            -self.world_translation.x,
            -self.world_translation.y,
            0.0))
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }
}