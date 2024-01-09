use crate::state::State;

use super::time_step_update_system::TimeStepDescription;

pub struct WarpDescription {
    pub start_time: f64,
    pub end_time: f64,
}

impl WarpDescription {
    fn max_warp_speed(&self) -> f64 {
        1.0 * (self.end_time - self.start_time)
    }

    fn fraction_completed(&self, time: f64) -> f64 {
        let current_duration = time - self.start_time;
        let total_duration = self.end_time - self.start_time;
        current_duration / total_duration
    }
    
    pub fn calculate_warp_speed(&self, time: f64) -> f64 {
        if self.fraction_completed(time) < 0.95 {
            self.max_warp_speed()
        } else {
            let fraction_of_last_fraction_completed = (self.fraction_completed(time) - 0.95) * 20.0;
            let multiplier = (fraction_of_last_fraction_completed - 1.0).powi(2) + 0.06;
            multiplier * self.max_warp_speed()
        }
    }
}

fn check_warp_finished(state: &mut State) {
    // Weird double if needed because of borrow checker
    let warp_finished = if let Some(current_warp) = &state.current_warp {
        state.time >= current_warp.end_time
    } else {
        return;
    };
    if warp_finished {
        state.current_warp = None;
        state.time_step_description = TimeStepDescription::Level(1);
    }
}

fn update_warp(state: &mut State) {
    if let Some(current_warp) = &state.current_warp {
        let mut warp_speed = current_warp.calculate_warp_speed(state.time);
        let final_time = state.time + warp_speed * state.delta_time;
        if final_time > current_warp.end_time {
            // Oh no, we're about to overshoot
            // Calculate required warp speed to perfectly land at target point
            // Add small amount so next frame actually counts this as 'finished'
            warp_speed = (current_warp.end_time - state.time) / state.delta_time + 0.01;
        }
        state.time_step_description = TimeStepDescription::Raw(warp_speed);
    }
}

pub fn warp_update_system(state: &mut State) {
    check_warp_finished(state);
    update_warp(state);
}