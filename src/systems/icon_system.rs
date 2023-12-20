use eframe::egui::Context;

use crate::state::State;

use self::{burn_icon_cleanup::burn_icon_cleanup_system, icon_position_update::icon_position_update_system, icon_precedence::icon_precedence_system, icon_click::icon_click_system};

mod burn_icon_cleanup;
mod icon_click;
mod icon_position_update;
mod icon_precedence;

pub fn icon_system(state: &mut State, context: &Context) {
    burn_icon_cleanup_system(state);
    icon_position_update_system(state);
    icon_precedence_system(state);
    icon_click_system(state, context);
}