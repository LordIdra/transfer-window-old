use eframe::egui::Context;
use nalgebra_glm::DVec2;

use crate::state::State;

pub fn camera_update_system(state: &mut State, context: &Context) {
    let selected_position;
    context.input(|input| {
        selected_position = state.selected.map(|selected| state.position_components.get(selected).unwrap().absolute_position);

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
    });
}