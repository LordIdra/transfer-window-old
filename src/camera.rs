use eframe::{epaint::{Rect, Pos2}, egui::Context};
use nalgebra_glm::{DVec2, translate2d, DMat3, scale2d, Mat3, Vec2};

use crate::{app::ObjectId, storage::Storage, util::f64_to_f32_pair, object::SCALE_FACTOR};

const ZOOM_SENSITIVITY: f64 = 0.003;
const SELECT_DISTANCE: f64 = 10000.0;

pub struct Camera {
    world_translation: DVec2,
    relative_translation: DVec2,
    zoom: f64,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            world_translation: DVec2::new(0.0, 0.0),
            relative_translation: DVec2::new(0.0, 0.0),
            zoom: 0.0002,
        }
    }

    fn translate(&mut self, amount: DVec2) {
        self.relative_translation += amount / self.zoom;
    }

    pub fn reset_relative_translation(&mut self) {
        self.relative_translation = DVec2::new(0.0, 0.0)
    }

    pub fn update(&mut self, context: &Context, storage: &Storage, selected: &ObjectId) {
        context.input(|input| {
            if input.pointer.secondary_down() {
                self.translate(DVec2::new(-input.pointer.delta().x as f64, input.pointer.delta().y as f64));
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

            self.world_translation = storage.get(selected).get_absolute_scaled_position(storage);
        });
    }

    pub fn get_zoom_matrix(&self, screen_size: Rect) -> Mat3 {
        let mut mat = DMat3::identity();
        mat = scale2d(&mat, &DVec2::new(2.0 / screen_size.width() as f64, 2.0 / screen_size.height() as f64));
        mat = scale2d(&mat, &DVec2::new(self.zoom, self.zoom));
        Mat3::new(
            mat.m11 as f32, mat.m12 as f32, mat.m13 as f32,
            mat.m21 as f32, mat.m22 as f32, mat.m23 as f32,
            mat.m31 as f32, mat.m32 as f32, mat.m33 as f32,
        )
    }

    pub fn get_translation_matrices(&self) -> (Mat3, Mat3) {
        let final_translation = self.world_translation + self.relative_translation;
        let world_translation_x_pair = f64_to_f32_pair(final_translation.x);
        let world_translation_y_pair = f64_to_f32_pair(final_translation.y);
        let mat1 = translate2d(&Mat3::identity(), &Vec2::new(-world_translation_x_pair.0, -world_translation_y_pair.0));
        let mat2 = translate2d(&Mat3::identity(), &Vec2::new(-world_translation_x_pair.1, -world_translation_y_pair.1));
        (mat1, mat2)
    }

    pub fn get_zoom(&self) -> f64 {
        self.zoom
    }

    pub fn get_max_distance_to_select(&self) -> f64 {
        SELECT_DISTANCE / self.zoom / SCALE_FACTOR
    }

    pub fn window_space_to_world_space(&self, window_coords: Pos2, screen_size: Rect) -> DVec2 {
        (self.world_translation + self.relative_translation + DVec2::new(
            (window_coords.x - (screen_size.width() / 2.0)) as f64 / self.zoom,
            ((screen_size.height() / 2.0) - window_coords.y) as f64 / self.zoom)) / SCALE_FACTOR
    }
}