use eframe::egui::{Ui, Context, Window, Key};

use crate::state::State;

use self::{general::general, selected::selected, trajectory_prediction::trajectory_prediction};

pub mod debug_utils;
mod general;
mod selected;
mod trajectory_prediction;

fn draw(state: &mut State, ui: &mut Ui) {
    ui.collapsing("General", |ui| general(state, ui));
    ui.collapsing("Selected", |ui| selected(state, ui));
    ui.collapsing("Trajectory Prediction", |ui| trajectory_prediction(state, ui));
}

pub fn debug_system(state: &mut State, context: &Context) {
    context.input(|input| {
        if input.key_pressed(Key::F12) {
            state.debug_mode = !state.debug_mode;
        }
    });
    if state.debug_mode {
        Window::new("Debug").show(context, |ui| draw(state, ui));
    }
}