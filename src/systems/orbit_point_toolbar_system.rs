use eframe::{egui::{Context, Window, Image, ImageButton, Ui, Layout}, emath::{Align2, Align}, epaint::{self, Color32}};

use crate::state::State;

use super::warp_update_system::WarpDescription;

fn warp_to_point(state: &mut State) {
    let click_point = state.orbit_click_point.as_ref().unwrap();
    state.current_warp = Some(WarpDescription { start_time: state.time, end_time: click_point.get_time() });
}

fn draw_toolbar(state: &mut State, ui: &mut Ui) {
    ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
        let burn_image = Image::new(state.resources.get_texture_image("burn"))
            .bg_fill(Color32::TRANSPARENT)
            .fit_to_exact_size(epaint::vec2(15.0, 15.0));
        let burn_button = ImageButton::new(burn_image);
        if ui.add(burn_button).clicked() {
            println!("click 1");
        }

        let warp_image = Image::new(state.resources.get_texture_image("warp-here"))
            .bg_fill(Color32::TRANSPARENT)
            .fit_to_exact_size(epaint::vec2(15.0, 15.0));
        let warp_button = ImageButton::new(warp_image);
        if ui.add(warp_button).clicked() {
            warp_to_point(state);
        }
    });

    state.register_ui(ui);

}

pub fn orbit_point_toolbar_system(state: &mut State, context: &Context) {
    if state.orbit_click_point.is_none() {
        return;
    };

    // context.style_mut(|style| {
    //     let rounding = 20.0;
    //     let rounding_struct = Rounding { nw: rounding, ne: rounding, sw: rounding, se: rounding };

    //     style.visuals.window_fill = Color32::TRANSPARENT;
    //     style.visuals.window_shadow = Shadow::NONE;
    //     style.visuals.window_stroke = Stroke::NONE;
    //     style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
    //     style.visuals.widgets.hovered.weak_bg_fill = Color32::TRANSPARENT;
    //     style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_white_alpha(150));
    //     style.visuals.widgets.hovered.rounding = rounding_struct;
    //     style.visuals.widgets.active.bg_fill = Color32::from_white_alpha(100);
    //     style.visuals.widgets.active.rounding = rounding_struct;
    // });

    let window = Window::new("")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(100.0, 200.0));
    window.show(context, |ui| draw_toolbar(state, ui));
}