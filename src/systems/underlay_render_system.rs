use std::sync::Arc;

use eframe::{egui::{Context, Ui, CentralPanel}, epaint::PaintCallback, egui_glow::CallbackFn};

use crate::state::State;

use self::{render_segment::get_all_segment_vertices, render_object::get_all_object_vertices, render_icons::get_all_icon_vertices};

mod render_icons;
mod render_object;
mod render_segment;

const ICON_NAMES: [&str; 6] = ["star", "planet", "moon", "spacecraft", "burn", "burn-adjustment-arrow"];

fn add_painter_callback(state: &mut State, context: &Context, ui: &Ui) {
    let screen_rect = context.screen_rect();
    let orbit_renderer = state.orbit_renderer.clone();
    let object_renderer = state.object_renderer.clone();
    let icon_renderers = state.texture_renderers.clone();
    let camera = state.camera.clone();
    let callback = Arc::new(CallbackFn::new(move |_info, _painter| {
        orbit_renderer.lock().unwrap().render(screen_rect, camera.clone());
        object_renderer.lock().unwrap().render(screen_rect, camera.clone());
        for texture_renderer in icon_renderers.lock().unwrap().values() {
            texture_renderer.render(screen_rect, camera.clone());
        }
    }));
    ui.painter().add(PaintCallback { rect: screen_rect, callback });
}

pub fn underlay_render_system(state: &mut State, context: &Context) {
    CentralPanel::default().show(context, |ui| {
        let object_vertices = get_all_object_vertices(state);
        let orbit_vertices = get_all_segment_vertices(state);
        state.object_renderer.lock().unwrap().set_vertices(object_vertices);
        state.orbit_renderer.lock().unwrap().set_vertices(orbit_vertices);
        let texture_renderers_arc = state.texture_renderers.clone();
        let mut texture_renderers = texture_renderers_arc.lock().unwrap(); // Fuck this language, seriously...
        for name in ICON_NAMES {
            let texture_renderer = texture_renderers.get_mut(name).expect("Icon texture does not exist");
            let vertices = get_all_icon_vertices(state, texture_renderer.get_name());
            texture_renderer.set_vertices(vertices);
        }
        add_painter_callback(state, context, ui);
    });
}