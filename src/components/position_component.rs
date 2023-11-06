use std::sync::{Mutex, Arc};

use nalgebra_glm::{DVec2, vec2};

use crate::camera::Camera;

pub struct PositionComponent {
    absolute_position: DVec2,
    camera_relative_position: DVec2,
}

impl PositionComponent {
    pub fn new(absolute_position: DVec2) -> Self {
        let camera_relative_position = vec2(0.0, 0.0); // Will be updated by a system before any rendering occurs
        Self { absolute_position, camera_relative_position }
    }

    pub fn get_absolute_position(&self) -> DVec2 {
        self.absolute_position
    }

    pub fn get_camera_relative_position(&self) -> DVec2 {
        self.camera_relative_position
    }

    pub fn set_absolute_position(&mut self, new_absolute_position: DVec2, camera: Arc<Mutex<Camera>>) {
        self.absolute_position = new_absolute_position;
        self.camera_relative_position = camera.lock().unwrap().get_absolute_position() // TODO START HERE TOMORROW
    }
}