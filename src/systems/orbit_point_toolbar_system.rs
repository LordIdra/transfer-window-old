use std::{cell::RefCell, rc::Rc};

use eframe::{egui::{Context, Window, Image, ImageButton, Ui, Layout, Label}, emath::{Align2, Align}, epaint::{self, Color32, Rounding, Shadow, Stroke}};

use crate::{state::State, components::trajectory_component::segment::{Segment, burn::Burn, orbit::Orbit}, systems::util::get_segment_at_time, storage::entity_builder::build_burn_icon};

use super::{warp_update_system::WarpDescription, util::format_time, trajectory_prediction_system::spacecraft_prediction::predict_spacecraft};

fn warp_to_point(state: &mut State) {
    let click_point = state.orbit_click_point.as_ref().unwrap();
    state.current_warp = Some(WarpDescription { start_time: state.time, end_time: click_point.get_time() });
}

fn create_burn(state: &mut State) {
    let time = state.orbit_click_point.as_ref().unwrap().get_time();
    let entity = state.orbit_click_point.as_ref().unwrap().get_entity();
    let segment_containing_burn = get_segment_at_time(state, &entity, time);
    let orbit_containing_burn = segment_containing_burn.as_orbit();
    let parent = orbit_containing_burn.borrow().get_parent();
    let velocity_direction = orbit_containing_burn.borrow().get_end_point().get_velocity().normalize();

    state.components.trajectory_components.get_mut(&entity).unwrap().remove_segments_after(time);
    let burn = Burn::new(&state, entity, parent, velocity_direction, time);
    let orbit_start_time = burn.get_end_point().get_time();
    let orbit = Orbit::new(&state.components, parent, burn.get_end_point().get_position(), burn.get_end_point().get_velocity(), orbit_start_time);
    let burn = Rc::new(RefCell::new(burn));

    state.components.trajectory_components.get_mut(&entity).unwrap().add_segment(Segment::Burn(burn.clone()));
    state.components.trajectory_components.get_mut(&entity).unwrap().add_segment(Segment::Orbit(Rc::new(RefCell::new(orbit))));

    build_burn_icon(&mut state.components, burn, parent);
    predict_spacecraft(state, entity, orbit_start_time, 10000000.0)
}



fn draw(state: &mut State, ui: &mut Ui) {
    ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
        let warp_image = Image::new(state.resources.get_texture_image("warp-here"))
            .bg_fill(Color32::TRANSPARENT)
            .fit_to_exact_size(epaint::vec2(15.0, 15.0));
        let warp_button = ImageButton::new(warp_image);
        if ui.add(warp_button).clicked() {
            warp_to_point(state);
        }

        let entity = state.orbit_click_point.as_ref().unwrap().get_entity();
        if state.components.celestial_body_components.get(&entity).is_none() {
            let burn_image = Image::new(state.resources.get_texture_image("burn"))
                .bg_fill(Color32::TRANSPARENT)
                .fit_to_exact_size(epaint::vec2(15.0, 15.0));
            let burn_button = ImageButton::new(burn_image);
            if ui.add(burn_button).clicked() {
                create_burn(state);
            }
        }
    });

    let remaining_time = state.orbit_click_point.as_ref().unwrap().get_time() - state.time;
    ui.add(Label::new("T-".to_string() + format_time(remaining_time).as_str()));

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
    window.show(context, |ui| draw(state, ui));
}