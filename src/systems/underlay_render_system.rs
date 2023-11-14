use std::sync::Arc;

use eframe::{egui::{Context, Ui, CentralPanel}, epaint::PaintCallback, egui_glow::CallbackFn};

use crate::state::State;

use self::{render_orbit::get_all_orbit_vertices, render_object::get_all_object_vertices, render_icons::get_all_icon_vertices};

mod render_icons;
mod render_object;
mod render_orbit;

fn add_painter_callback(state: &mut State, context: &Context, ui: &Ui) {
    let screen_rect = context.screen_rect();
    let orbit_renderer = state.orbit_renderer.clone();
    let object_renderer = state.object_renderer.clone();
    let icon_renderers = state.icon_renderers.clone();
    let camera = state.camera.clone();
    let callback = Arc::new(CallbackFn::new(move |_info, _painter| {
        orbit_renderer.lock().unwrap().render(screen_rect, camera.clone());
        object_renderer.lock().unwrap().render(screen_rect, camera.clone());
        for icon_renderer in icon_renderers.lock().unwrap().values() {
            icon_renderer.render(screen_rect, camera.clone());
        }
    }));
    ui.painter().add(PaintCallback { rect: screen_rect, callback });
}

pub fn underlay_render_system(state: &mut State, context: &Context) {
    CentralPanel::default().show(context, |ui| {
        let object_vertices = get_all_object_vertices(state);
        let orbit_vertices = get_all_orbit_vertices(state);
        state.object_renderer.lock().unwrap().set_vertices(object_vertices);
        state.orbit_renderer.lock().unwrap().set_vertices(orbit_vertices);
        for icon_renderer in state.icon_renderers.clone().lock().unwrap().values_mut() {
            let vertices = get_all_icon_vertices(state, icon_renderer.get_name());
            icon_renderer.set_vertices(vertices);
        }
        add_painter_callback(state, context, ui);
    });
}