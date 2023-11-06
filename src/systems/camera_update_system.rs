use std::sync::MutexGuard;

use eframe::{egui::{Context, Key}, epaint::{Vec2, Pos2, Rect}};
use nalgebra_glm::DVec2;

use crate::{state::State, camera::Camera};

const ZOOM_SENSITIVITY: f64 = 0.003;

fn update_translation(camera: &mut MutexGuard<'_, Camera>, mouse_delta: Option<Vec2>, key_r_pressed: bool) {
    if let Some(mouse_delta) = mouse_delta  {
        camera.translate(DVec2::new(-mouse_delta.x as f64, mouse_delta.y as f64));
    }
    if key_r_pressed {
        camera.recenter();
    }
}

fn update_zoom(camera: &mut MutexGuard<'_, Camera>, mouse_delta: Option<Vec2>, latest_mouse_position: Option<Pos2>, scroll_delta: Vec2, screen_size: Rect) {
    if let Some(latest_mouse_position) = latest_mouse_position {
        let screen_size = DVec2::new(screen_size.width() as f64, screen_size.height() as f64);
        let new_zoom = camera.get_zoom() * (1.0 + ZOOM_SENSITIVITY * scroll_delta.y as f64);
        let delta_zoom = (camera.get_zoom() - new_zoom) / new_zoom;
        let mouse_position = DVec2::new(
            -(latest_mouse_position.x as f64 - (screen_size.x / 2.0)), 
              latest_mouse_position.y as f64 - (screen_size.y / 2.0));
        camera.translate(mouse_position * delta_zoom);
        camera.set_zoom(new_zoom);
    }
}

pub fn camera_update_system(state: &mut State, context: &Context) {
    let mut latest_mouse_position;
    let mut mouse_delta = None;
    let mut scroll_delta;
    let mut key_r_pressed;
    
    context.input(|input| {
        if input.pointer.secondary_down() {
            mouse_delta = Some(input.pointer.delta());
        }
        latest_mouse_position = input.pointer.latest_pos();
        scroll_delta = input.scroll_delta;
        key_r_pressed = input.key_pressed(Key::R);
    });

    let mut camera = state.camera.lock().unwrap();
    update_translation(&mut camera, mouse_delta, key_r_pressed);
    update_zoom(&mut camera, mouse_delta, latest_mouse_position, scroll_delta, context.screen_rect());
}