use crate::state::State;

pub fn was_mouse_over_any_element_last_frame_system(state: &mut State) {
    state.mouse_over_any_element = state.mouse_over_any_element_cache;
    state.mouse_over_any_element_cache = false;
}