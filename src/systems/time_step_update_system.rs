use eframe::egui::{Key, Context};

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

pub fn time_step_update_system(state: &mut State, context: &Context) {
    update_time_step_level(state, context);
    update_time_step(state);
}