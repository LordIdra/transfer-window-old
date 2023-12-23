use eframe::egui::Context;

use crate::state::State;

use self::{burn_icon_cleanup::burn_icon_cleanup, icon_position::icon_position, icon_precedence::icon_precedence, icon_click::icon_click};

mod burn_icon_cleanup;
mod icon_click;
mod icon_position;
mod icon_precedence;

pub fn icon_system(state: &mut State, context: &Context) {
    burn_icon_cleanup(state);
    icon_position(state);
    icon_precedence(state);
    icon_click(state, context);
}