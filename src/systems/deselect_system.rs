use eframe::egui::{Context, PointerButton};

use crate::state::State;

pub fn deselect_system(state: &mut State, context: &Context) {
    if state.mouse_over_any_element {
        return;
    }
    context.input(|input| {
        if input.pointer.button_clicked(PointerButton::Primary) {
            state.selected_burn_icon = None;
            state.orbit_click_point = None;
        }
    })
}