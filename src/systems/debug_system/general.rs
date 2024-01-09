use eframe::egui::Ui;

use crate::{state::State, systems::util::format_time};

pub fn general(state: &mut State, ui: &mut Ui) {
    ui.label(format!("Time: {}", format_time(state.time)));
}