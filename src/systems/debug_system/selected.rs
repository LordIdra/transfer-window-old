use std::collections::VecDeque;

use eframe::egui::Ui;
use nalgebra_glm::DVec2;

use crate::{state::State, components::trajectory_component::segment::{Segment, orbit::Orbit, burn::Burn}, systems::util::{format_time, get_segment_at_time}, storage::entity_allocator::Entity};

fn get_absolute_parent_position(state: &State, entity: Entity, time: f64) -> DVec2 {
    match state.components.parent_components.get(&entity) {
        Some(parent_component) => {
            let position = get_segment_at_time(state, &entity, time).get_position_at_time(time);
            position + get_absolute_parent_position(state, parent_component.get_parent(), time)
        }
        None => state.components.position_components.get(&entity).unwrap().get_absolute_position()
    }
}

fn get_absolute_parent_velocity(state: &State, entity: Entity, time: f64) -> DVec2 {
    match state.components.parent_components.get(&entity) {
        Some(parent_component) => {
            let velocity = get_segment_at_time(state, &entity, time).get_velocity_at_time(time);
            velocity + get_absolute_parent_velocity(state, parent_component.get_parent(), time)
        }
        None => state.components.velocity_components.get(&entity).unwrap().get_absolute_velocity()
    }
}

fn draw_absolute_point(state: &State, ui: &mut Ui, entity: Entity, time: f64, position: DVec2, velocity: DVec2) {
    let absolute_position = position + get_absolute_parent_position(state, entity, time);
    let absolute_velocity = velocity + get_absolute_parent_velocity(state, entity, time);
    ui.label(format!("Time: {}", format_time(time)));
    ui.collapsing("Absolute", |ui| {
        ui.label(format!("Position: [{:.5e} {:.5e}]", absolute_position.x, absolute_position.y));
        ui.label(format!("Velocity: [{:.5e} {:.5e}]", absolute_velocity.x, absolute_velocity.y));
    });
    ui.collapsing("Relative", |ui| {
        ui.label(format!("Position: [{:.5e} {:.5e}]", position.x, position.y));
        ui.label(format!("Velocity: [{:.5e} {:.5e}]", velocity.x, velocity.y));
    });
}

fn draw_burn(state: &mut State, ui: &mut Ui, burn: &Burn) {
    let parent_name = state.components.name_components.get(&burn.get_parent()).unwrap().get_name();
    ui.label(format!("Parent: {}", parent_name));
    ui.label(format!("Duration: {}", format_time(burn.get_duration())));
    ui.label(format!("Delta-V: {:.1}", burn.get_total_dv()));
    ui.collapsing("Start", |ui| draw_absolute_point(state, ui, burn.get_parent(), burn.get_start_time(), burn.get_start_position(), burn.get_start_velocity()));
    ui.collapsing("Current", |ui| draw_absolute_point(state, ui, burn.get_parent(), burn.get_current_time(), burn.get_current_position(), burn.get_current_velocity()));
    ui.collapsing("End", |ui| draw_absolute_point(state, ui, burn.get_parent(), burn.get_end_time(), burn.get_end_position(), burn.get_end_velocity()));
}

fn draw_orbit(state: &mut State, ui: &mut Ui, orbit: &Orbit) {
    let parent_name = state.components.name_components.get(&orbit.get_parent()).unwrap().get_name();
    ui.label(format!("Parent: {}", parent_name));
    ui.label(format!("Duration: {}", format_time(orbit.get_end_time() - orbit.get_start_time())));
    ui.label(format!("Remaining orbits: {}", orbit.get_remaining_orbits()));
    ui.label(format!("Direction: {:?}", orbit.get_direction()));
    match orbit.get_period() {
        Some(period) => {
            ui.label(format!("Type: ellipse"));
            ui.label(format!("Period: {}", format_time(period)));
        }
        None => {
            ui.label(format!("Type: hyperbola"));
        }
    }
    ui.label(format!("Semi-major axis: {:.5e}", orbit.get_semi_major_axis()));
    ui.label(format!("Semi-minor axis: {:.5e}", orbit.get_semi_minor_axis()));
    ui.label(format!("Semi-minor axis: {:.5}", orbit.get_eccentricity()));
    ui.label(format!("Argument of periapsis: {:.5e}", orbit.get_arugment_of_periapsis()));
    ui.collapsing("Start", |ui| draw_absolute_point(state, ui, orbit.get_parent(), orbit.get_start_time(), orbit.get_start_position(), orbit.get_start_velocity()));
    ui.collapsing("Current", |ui| draw_absolute_point(state, ui, orbit.get_parent(), orbit.get_current_time(), orbit.get_current_position(), orbit.get_current_velocity()));
    ui.collapsing("End", |ui| draw_absolute_point(state, ui, orbit.get_parent(), orbit.get_end_time(), orbit.get_end_position(), orbit.get_end_velocity()));
}

fn draw_trajectory(state: &mut State, ui: &mut Ui, segments: VecDeque<Segment>) {
    let segment_count = segments.len();
    for (i, segment) in segments.iter().enumerate() {
        match segment {
            Segment::Burn(burn) => {
                ui.collapsing(format!("({}) Burn", segment_count - i), |ui| draw_burn(state, ui, &*burn.borrow()));
            },
            Segment::Orbit(orbit) => {
                ui.collapsing(format!("({}) Orbit", segment_count - i), |ui| draw_orbit(state, ui, &*orbit.borrow()));
            },
        }
    }
}

pub fn selected(state: &mut State, ui: &mut Ui) {
    let entity = state.selected_entity.clone();
    let absolute_position = state.components.position_components.get(&entity).unwrap().get_absolute_position();
    let absolute_velocity = state.components.velocity_components.get(&entity).unwrap().get_absolute_velocity();
    let relative_position = state.components.trajectory_components.get(&entity).unwrap().get_current_segment().get_current_position();
    let relative_velocity = state.components.trajectory_components.get(&entity).unwrap().get_current_segment().get_current_velocity();
    ui.collapsing("Absolute", |ui| {
        ui.label(format!("Position: [{:.5e} {:.5e}]", absolute_position.x, absolute_position.y));
        ui.label(format!("Velocity: [{:.5e} {:.5e}]", absolute_velocity.x, absolute_velocity.y));
    });
    ui.collapsing("Relative", |ui| {
        ui.label(format!("Position: [{:.5e} {:.5e}]", relative_position.x, relative_position.y));
        ui.label(format!("Velocity: [{:.5e} {:.5e}]", relative_velocity.x, relative_velocity.y));
    });
    if let Some(parent_component) = state.components.parent_components.get(&entity) {
        let parent_name = state.components.name_components.get(&parent_component.get_parent()).unwrap().get_name();
        ui.label(format!("Parent: {}", parent_name));
    }
    if let Some(trajectory_component) = state.components.trajectory_components.get(&entity) {
        let segments = trajectory_component.get_segments().clone();
        ui.collapsing("Trajectory", |ui| draw_trajectory(state, ui, segments));
    }
}