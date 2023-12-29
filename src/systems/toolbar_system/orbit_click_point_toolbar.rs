use std::{cell::RefCell, rc::Rc};

use eframe::{egui::{Context, Window, Image, ImageButton, Ui, Layout, Label, Style}, emath::{Align2, Align}, epaint::{self, Color32}};

use crate::{state::{State, Selected}, components::trajectory_component::segment::{Segment, burn::Burn, orbit::Orbit}, systems::{util::get_segment_at_time, warp_update_system::WarpDescription, trajectory_prediction_system::spacecraft_prediction::predict_spacecraft, orbit_point_selection_system::OrbitClickPoint, debug_system::debug_utils::format_time}, storage::entity_builder::build_burn_icon};

use super::apply_toolbar_style;

fn warp_to_point(state: &mut State, orbit_click_point: OrbitClickPoint) {
    state.current_warp = Some(WarpDescription::new(state.time, orbit_click_point.get_time()));
}

fn create_burn(state: &mut State, orbit_click_point: OrbitClickPoint) {
    let time = orbit_click_point.get_time();
    let entity = orbit_click_point.get_entity();
    let segment_containing_burn = get_segment_at_time(state, &entity, time);
    let orbit_containing_burn = segment_containing_burn.as_orbit();
    let parent = orbit_containing_burn.borrow().get_parent();
    
    state.components.trajectory_components.get_mut(&entity).unwrap().remove_segments_after(time);
    let velocity_direction = state.components.trajectory_components.get_mut(&entity).unwrap().get_final_segment().as_orbit().borrow().get_end_point().get_velocity().normalize();
    let burn = Burn::new(&state, entity, parent, velocity_direction, time);
    let orbit_start_time = burn.get_end_point().get_time();
    let orbit = Orbit::new(&state.components, parent, burn.get_end_point().get_position(), burn.get_end_point().get_velocity(), orbit_start_time);
    let burn = Rc::new(RefCell::new(burn));

    state.components.trajectory_components.get_mut(&entity).unwrap().add_segment(Segment::Burn(burn.clone()));
    state.components.trajectory_components.get_mut(&entity).unwrap().add_segment(Segment::Orbit(Rc::new(RefCell::new(orbit))));

    state.selected = Selected::BurnIcon(build_burn_icon(&mut state.components, burn, parent));
    predict_spacecraft(state, entity, orbit_start_time)
}

fn draw(state: &mut State, ui: &mut Ui, orbit_click_point: OrbitClickPoint) {
    ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
        let warp_image = Image::new(state.resources.get_texture_image("warp-here"))
            .bg_fill(Color32::TRANSPARENT)
            .fit_to_exact_size(epaint::vec2(15.0, 15.0));
        let warp_button = ImageButton::new(warp_image);
        if ui.add(warp_button).clicked() {
            warp_to_point(state, orbit_click_point.clone());
        }

        let entity = orbit_click_point.get_entity();
        if state.components.celestial_body_components.get(&entity).is_none() {
            let burn_image = Image::new(state.resources.get_texture_image("burn"))
                .bg_fill(Color32::TRANSPARENT)
                .fit_to_exact_size(epaint::vec2(15.0, 15.0));
            let burn_button = ImageButton::new(burn_image);
            if ui.add(burn_button).clicked() {
                create_burn(state, orbit_click_point.clone());
            }
        }
    });

    let remaining_time = orbit_click_point.get_time() - state.time;
    ui.add(Label::new("T-".to_string() + format_time(remaining_time).as_str()));

    state.register_ui(ui);
}

pub fn orbit_click_point_toolbar(state: &mut State, context: &Context) {
    let Selected::OrbitClickPoint(orbit_click_point) = state.selected.clone() else {
        return;
    };

    apply_toolbar_style(context);

    let window = Window::new("Click Point Toolbar")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, epaint::vec2(0.0, 0.0));
    window.show(context, |ui| draw(state, ui, orbit_click_point));

    context.set_style(Style::default());
}