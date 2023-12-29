use eframe::egui::Ui;

use crate::{state::State};

use super::debug_utils::format_time;

pub fn general(state: &mut State, ui: &mut Ui) {
    ui.label(format!("Time: {}", format_time(state.time)));
}