use eframe::egui::Ui;
use egui_plot::{Plot, Line, PlotPoints};

use crate::state::State;

pub fn trajectory_prediction(state: &mut State, ui: &mut Ui) {
    ui.collapsing("Conic approximation from theta", |ui| {
        Plot::new("my_plot").view_aspect(2.0)
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::new(state.debug_conic_distances_from_theta.clone())));
        });
    });

    ui.collapsing("Conic approximation from time", |ui| {
        Plot::new("my_plot").view_aspect(2.0)
            .show(ui, |plot_ui| {
                plot_ui.line(Line::new(PlotPoints::new(state.debug_conic_distances_from_time.clone())));
        });
    });
}