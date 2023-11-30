use std::{cell::RefCell, rc::Rc};

use eframe::{egui::{Context, Window, Image, ImageButton, Ui, Layout, Label}, emath::{Align2, Align}, epaint::{self, Color32, Rounding, Shadow, Stroke}};

use crate::{state::State, components::trajectory_component::segment::{Segment, burn::Burn}};

use super::warp_update_system::WarpDescription;

fn warp_to_point(state: &mut State) {
    let click_point = state.orbit_click_point.as_ref().unwrap();
    state.current_warp = Some(WarpDescription { start_time: state.time, end_time: click_point.get_time() });
}

fn create_burn(state: &mut State) {
    let time = state.orbit_click_point.as_ref().unwrap().get_time();
    let entity = state.orbit_click_point.as_ref().unwrap().get_entity();
    let trajectory_component = state.components.trajectory_components.get_mut(&entity).unwrap();
    let final_orbit = trajectory_component.get_final_segment().as_orbit();
    let parent = final_orbit.borrow().get_parent();
    let velocity_direction = final_orbit.borrow().get_end_velocity().normalize();
    trajectory_component.remove_segments_after(time);
    let burn_segment = Segment::Burn(Rc::new(RefCell::new(Burn::new(&state, entity, parent, velocity_direction, time))));
    state.components.trajectory_components.get_mut(&entity).unwrap().add_segment(burn_segment);
}

fn format_time(time: f64) -> String {
    let years_quotient = f64::floor(time / (360.0 * 24.0 * 60.0 * 60.0));
    let years_remainder = time % (360.0 * 24.0 * 60.0 * 60.0);
    let days_quotient = f64::floor(years_remainder / (24.0 * 60.0 * 60.0));
    let days_remainder = years_remainder % (24.0 * 60.0 * 60.0);
    let hours_quotient = f64::floor(days_remainder / (60.0 * 60.0));
    let hours_remainder = days_remainder % (60.0 * 60.0);
    let minutes_quotient = f64::floor(hours_remainder / 60.0);
    let seconds = f64::round(hours_remainder % 60.0);
    if years_quotient != 0.0 {
        "T-".to_string()
            + years_quotient.to_string().as_str() + "y"
            + days_quotient.to_string().as_str() + "d"
            + hours_quotient.to_string().as_str() + "h"
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else if days_quotient != 0.0 {
        "T-".to_string()
            + days_quotient.to_string().as_str() + "d"
            + hours_quotient.to_string().as_str() + "h"
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else if hours_quotient != 0.0 {
        "T-".to_string()
            + hours_quotient.to_string().as_str() + "h"
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else if minutes_quotient != 0.0 {
        "T-".to_string()
            + minutes_quotient.to_string().as_str() + "m"
            + seconds.to_string().as_str() + "s"
    } else {
        "T-".to_string()
            + seconds.to_string().as_str() + "s"
    }
}

fn draw_toolbar(state: &mut State, ui: &mut Ui) {
    ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
        let burn_image = Image::new(state.resources.get_texture_image("burn"))
            .bg_fill(Color32::TRANSPARENT)
            .fit_to_exact_size(epaint::vec2(15.0, 15.0));
        let burn_button = ImageButton::new(burn_image);
        if ui.add(burn_button).clicked() {
            create_burn(state);
        }

        let warp_image = Image::new(state.resources.get_texture_image("warp-here"))
            .bg_fill(Color32::TRANSPARENT)
            .fit_to_exact_size(epaint::vec2(15.0, 15.0));
        let warp_button = ImageButton::new(warp_image);
        if ui.add(warp_button).clicked() {
            warp_to_point(state);
        }
    });

    let remaining_time = state.orbit_click_point.as_ref().unwrap().get_time() - state.time;
    ui.add(Label::new(format_time(remaining_time)));

    state.register_ui(ui);
}

pub fn orbit_point_toolbar_system(state: &mut State, context: &Context) {
    if state.orbit_click_point.is_none() {
        return;
    };

    context.style_mut(|style| {
        let rounding = 10.0;
        let rounding_struct = Rounding { nw: rounding, ne: rounding, sw: rounding, se: rounding };

        style.visuals.window_fill = Color32::TRANSPARENT;
        style.visuals.window_shadow = Shadow::NONE;
        style.visuals.window_stroke = Stroke::NONE;
        style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        style.visuals.widgets.hovered.weak_bg_fill = Color32::TRANSPARENT;
        style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_white_alpha(150));
        style.visuals.widgets.hovered.rounding = rounding_struct;
        style.visuals.widgets.active.bg_fill = Color32::from_white_alpha(100);
        style.visuals.widgets.active.rounding = rounding_struct;
    });

    let window = Window::new("")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0));
    window.show(context, |ui| draw_toolbar(state, ui));
}