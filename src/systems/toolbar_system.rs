use eframe::{egui::Context, epaint::{Rounding, Color32, Shadow, Stroke}};

use crate::state::State;

use self::{orbit_click_point_toolbar::orbit_click_point_toolbar, burn_toolbar::burn_toolbar};

pub mod burn_toolbar;
pub mod orbit_click_point_toolbar;

fn apply_toolbar_style(context: &Context) {
    context.style_mut(|style| {
        let rounding = 10.0;
        let rounding_struct = Rounding { nw: rounding, ne: rounding, sw: rounding, se: rounding };

        style.visuals.window_fill = Color32::TRANSPARENT;
        style.visuals.window_shadow = Shadow::NONE;
        style.visuals.window_stroke = Stroke::NONE;
        style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        style.visuals.widgets.hovered.weak_bg_fill = Color32::TRANSPARENT;
        style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_white_alpha(150));
        style.visuals.widgets.hovered.rounding = rounding_struct;
        style.visuals.widgets.active.bg_fill = Color32::from_white_alpha(100);
        style.visuals.widgets.active.rounding = rounding_struct;
    });
}

pub fn toolbar_system(state: &mut State, context: &Context) {
    orbit_click_point_toolbar(state, context);
    burn_toolbar(state, context);
}