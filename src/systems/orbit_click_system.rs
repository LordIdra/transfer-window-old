use eframe::{egui::Context, epaint::{Pos2, Rect}};
use nalgebra_glm::{vec2, DVec2};

use crate::{state::State, components::trajectory_component::orbit::Orbit};

/// https://blog.chatfield.io/simple-method-for-distance-to-ellipse/
fn test_orbit_clicked(state: &State, orbit: &Orbit, position: DVec2, max_distance_to_select: f64) {
    let parent = orbit.get_parent();
    let absolute_parent_position = state.components.position_components.get(&parent).unwrap().get_absolute_position();
    let relative_nominal_position = orbit.get_position_from_mean_anomaly(orbit.get_arugment_of_periapsis());
    let nominal_position_to_center_vector = -orbit.get_semi_major_axis() * vec2(f64::cos(orbit.get_arugment_of_periapsis()), f64::sin(orbit.get_arugment_of_periapsis()));
    let center = absolute_parent_position + relative_nominal_position + nominal_position_to_center_vector;
    let displacement = center - position;
    println!("{}", displacement);
    // Starting point for our iterative algorithm
    // let initial_point = orbit.get_start_position();
    // loop {
    //     let r = (position - initial_point).magnitude();
    // }
}

/// Attempts to find the closest point on any orbit that is within a given distance of the mouse position
fn get_closest_entity_orbit(state: &mut State, screen_size: Rect, position: Pos2) {
    let position = state.camera.lock().unwrap().window_space_to_world_space(position, screen_size);
    let max_distance_to_select = state.camera.lock().unwrap().get_max_distance_to_select();
    for entity in state.components.entity_allocator.get_entities() {
        if let Some(trajectory_component) = state.components.trajectory_components.get(&entity) {
            for orbit in trajectory_component.get_orbits().clone() {
                test_orbit_clicked(state, orbit, position, max_distance_to_select);
            }
        }
    }
}

pub fn orbit_click_system(state: &mut State, context: &Context) {
    let screen_size = context.screen_rect();
    context.input(|input| {
        if let Some(position) = input.pointer.latest_pos() {
            get_closest_entity_orbit(state, screen_size, position);
        }

        if input.pointer.primary_clicked() {
            
        }
    });
}