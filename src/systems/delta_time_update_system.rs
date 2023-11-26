use std::time::Instant;

use crate::state::State;

pub fn delta_time_update_system(state: &mut State) {
    let delta_time = (Instant::now() - state.last_frame).as_secs_f64();
    state.delta_time = delta_time;
    state.last_frame = Instant::now();
}