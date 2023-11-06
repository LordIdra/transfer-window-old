use std::time::Instant;

use eframe::egui::{Key, Context};

use crate::state::State;

const MIN_TIME_STEP_LEVELS: i32 = 1;
const MAX_TIME_STEP_LEVELS: i32 = 8;

fn update_time_step_level(state: &mut State, context: &Context) {
    context.input(|input| {
        if input.key_pressed(Key::ArrowLeft) && state.time_step_level > MIN_TIME_STEP_LEVELS {
            state.time_step_level -= 1;
        }
        if input.key_pressed(Key::ArrowRight) && state.time_step_level < MAX_TIME_STEP_LEVELS {
            state.time_step_level += 1;
        }
    });
}

fn update_delta_time(state: &mut State, context: &Context) {
    let delta_time = (Instant::now() - state.last_frame).as_secs_f64() * state.get_time_step();
    state.time += delta_time;
    state.last_frame = Instant::now();
}

pub fn time_step_update_system(state: &mut State, context: &Context) {
    update_time_step_level(state, context);
    update_delta_time(state, context);
}