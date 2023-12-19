use eframe::{egui::{Context, Key}, epaint::{Vec2, Pos2, Rect}};
use nalgebra_glm::DVec2;

use crate::state::State;

const ZOOM_SENSITIVITY: f64 = 0.003;

/// Camera needs to know the translation of the selected entity
/// ...but it needs to be accessed in a callback (ie multithreaded context) (where state can't be accessed)
/// Solution: Store the position of the selected entity in the camera and update each frame
fn update_selected_translation(state: &mut State) {
    let selected_absolute_position = state.components.position_components.get(&state.selected_object).unwrap().get_absolute_position();
    state.camera.lock().unwrap().set_selected_translation(selected_absolute_position);
}

fn update_translation(state: &mut State, mouse_delta: Option<Vec2>, key_r_pressed: bool) {
    let mut camera = state.camera.lock().unwrap();
    if let Some(mouse_delta) = mouse_delta  {
        camera.translate(DVec2::new(-mouse_delta.x as f64, mouse_delta.y as f64));
    }
    if key_r_pressed {
        camera.recenter();
    }
}

fn update_zoom(state: &mut State, latest_mouse_position: Option<Pos2>, scroll_delta: Vec2, screen_size: Rect) {
    if let Some(latest_mouse_position) = latest_mouse_position {
        let mut camera = state.camera.lock().unwrap();
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
    context.input(|input| {
        let mouse_delta = if input.pointer.secondary_down() {
            Some(input.pointer.delta())
        } else {
            None
        };

        let latest_mouse_position = input.pointer.latest_pos();
        let scroll_delta = input.scroll_delta;
        let key_r_pressed = input.key_pressed(Key::R);

        update_selected_translation(state);
        update_translation(state, mouse_delta, key_r_pressed);
        update_zoom(state, latest_mouse_position, scroll_delta, context.screen_rect());
    });
}