use eframe::{epaint::Rect, egui::Context};
use nalgebra_glm::{DVec2, translate2d, DMat3, scale2d, Mat3};

const ZOOM_SENSITIVITY: f64 = 0.003;

pub struct Camera {
    world_translation: DVec2,
    zoom: f64,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            world_translation: DVec2::new(0.0, 0.0),
            zoom: 0.0002, 
        }
    }

    fn translate(&mut self, amount: DVec2) {
        self.world_translation += amount / self.zoom;
    }

    pub fn update(&mut self, context: &Context) {
        context.input(|input| {
            if input.pointer.secondary_down() {
                self.translate(0.5 * DVec2::new(-input.pointer.delta().x as f64, input.pointer.delta().y as f64));
            }

            if let Some(latest_mouse_position) = input.pointer.latest_pos() {
                let screen_size = DVec2::new(context.screen_rect().width() as f64, context.screen_rect().height() as f64);
                let new_zoom = self.zoom * (1.0 + ZOOM_SENSITIVITY * input.scroll_delta.y as f64);
                let delta_zoom = (self.zoom - new_zoom) / new_zoom;
                let mouse_position = DVec2::new(
                    -(latest_mouse_position.x as f64 - (screen_size.x / 2.0)), 
                      latest_mouse_position.y as f64 - (screen_size.y / 2.0));
                self.translate(mouse_position * delta_zoom);
                self.zoom = new_zoom;
            }
        });
    }

    pub fn get_matrix(&self, screen_size: Rect) -> Mat3 {
        let mut matrix = DMat3::identity();
        matrix = scale2d(&matrix, &DVec2::new(2.0 / screen_size.width() as f64, 2.0 / screen_size.height() as f64));
        matrix = scale2d(&matrix, &DVec2::new(self.zoom, self.zoom));
        matrix = translate2d(&matrix, &DVec2::new(-self.world_translation.x,-self.world_translation.y));
        Mat3::new(
            matrix.m11 as f32, matrix.m12 as f32, matrix.m13 as f32,
            matrix.m21 as f32, matrix.m22 as f32, matrix.m23 as f32,
            matrix.m31 as f32, matrix.m32 as f32, matrix.m33 as f32,
        )
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom as f32
    }
}