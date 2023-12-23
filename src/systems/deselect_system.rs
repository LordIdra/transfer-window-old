use eframe::egui::{Context, PointerButton};

use crate::state::{State, Selected};

use super::util::delete_burn_arrow_icons;

pub fn deselect_system(state: &mut State, context: &Context) {
    context.input(|input| {
        if input.pointer.button_clicked(PointerButton::Primary) {
            if !state.mouse_over_any_element && !state.mouse_over_any_icon {
                if let Selected::BurnIcon(selected) = state.selected {
                    delete_burn_arrow_icons(state, selected);
                }
                state.selected = Selected::None;
            }
        }
    })
}