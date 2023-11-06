use std::sync::Arc;

use eframe::{egui::{Context, Ui, CentralPanel}, epaint::PaintCallback, egui_glow::CallbackFn};

use crate::state::State;

use self::{render_orbit::get_all_orbit_vertices, render_object::get_all_object_vertices};

mod render_object;
mod render_orbit;

fn add_painter_callback(state: &mut State, context: &Context, ui: &Ui) {
    let screen_rect = context.screen_rect();
    let orbit_renderer = state.orbit_renderer.clone();
    let object_renderer = state.object_renderer.clone();
    let camera = state.camera.clone();
    let callback = Arc::new(CallbackFn::new(move |_info, _painter| {
        orbit_renderer.lock().unwrap().render(screen_rect, camera.clone());
        object_renderer.lock().unwrap().render(screen_rect, camera.clone());
    }));
    ui.painter().add(PaintCallback { rect: screen_rect, callback });
}

pub fn underlay_render_system(state: &mut State, context: &Context) {
    CentralPanel::default().show(context, |ui| {
        state.object_renderer.lock().unwrap().set_vertices(get_all_object_vertices(state));
        state.orbit_renderer.lock().unwrap().set_vertices(get_all_orbit_vertices(state));
        add_painter_callback(state, context, ui);
    });
}