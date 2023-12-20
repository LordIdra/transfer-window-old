use eframe::{egui::{Key, Context, Window, Style, TextStyle}, emath::Align2, epaint::{self, Stroke, Shadow, Color32}};

use crate::state::State;

const MIN_TIME_STEP_LEVELS: i32 = 1;
const MAX_TIME_STEP_LEVELS: i32 = 9;

pub enum TimeStepDescription {
    Level(i32),
    Raw(f64),
}

fn update_time_step_level(state: &mut State, context: &Context) {
    context.input(|input| {
        if input.key_pressed(Key::F11) {
            state.paused = !state.paused;
        }

        if let TimeStepDescription::Level(level) = &mut state.time_step_description {
            if input.key_pressed(Key::ArrowLeft) && *level > MIN_TIME_STEP_LEVELS {
                *level -= 1;
            }
            if input.key_pressed(Key::ArrowRight) && *level < MAX_TIME_STEP_LEVELS {
                *level += 1;
            }
        }
    });
}

fn update_time_step(state: &mut State) {
    state.time += state.delta_time * state.get_time_step();
}

fn draw_paused_ui(state: &State, context: &Context) {
    if state.paused {
        context.style_mut(|style| {
            style.visuals.window_fill = Color32::TRANSPARENT;
            style.visuals.window_shadow = Shadow::NONE;
            style.visuals.window_stroke = Stroke::NONE;
            style.override_text_style = Some(TextStyle::Heading)
        });

        let window = Window::new("Paused")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::CENTER_BOTTOM, epaint::vec2(0.0, -30.0));
        window.show(context, |ui| {
            ui.label("SIMULATION PAUSED")
        });

        context.set_style(Style::default());
    }
}

pub fn time_step_update_system(state: &mut State, context: &Context) {
    update_time_step_level(state, context);
    update_time_step(state);
    draw_paused_ui(state, context);
}