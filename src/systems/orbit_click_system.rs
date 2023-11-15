use eframe::{egui::Context, epaint::{Pos2, Rect}};
use nalgebra_glm::{vec2, DVec2};

use crate::{state::State, components::trajectory_component::orbit::Orbit};

fn r1(semi_major_axis: f64, eccentricity: f64, argument_of_periapsis: f64, theta: f64) -> f64 {
    (semi_major_axis * (1.0 - eccentricity.powi(2))) / (1.0 + eccentricity * f64::cos(theta - argument_of_periapsis))
}

fn r1_prime(semi_major_axis: f64, eccentricity: f64, argument_of_periapsis: f64, theta: f64) -> f64 {
    (semi_major_axis * eccentricity * (1.0 - eccentricity.powi(2)) * f64::sin(theta - argument_of_periapsis)) / (eccentricity * f64::cos(theta - argument_of_periapsis) + 1.0).powi(2)
}

fn r2(semi_major_axis: f64, eccentricity: f64, argument_of_periapsis: f64, theta: f64, p2: DVec2) -> f64 {
    let r1 = r1(semi_major_axis, eccentricity, argument_of_periapsis, theta);
    f64::sqrt(r1.powi(2) + p2.x.powi(2) + p2.y.powi(2) - 2.0 * r1 * p2.x * f64::cos(theta) - 2.0 * r1 *p2.y * f64::sin(theta))
}
fn r2_prime(semi_major_axis: f64, eccentricity: f64, argument_of_periapsis: f64, theta: f64, p2: DVec2) -> f64 {
    let r1_prime = r1_prime(semi_major_axis, eccentricity, argument_of_periapsis, theta);
    let r2 = r2(semi_major_axis, eccentricity, argument_of_periapsis, theta, p2);
    r1_prime + p2.x * f64::sin(theta) - p2.y * f64::cos(theta) - (r1_prime / r2) * (p2.x * f64::cos(theta) + p2.y * f64::sin(theta))
}

fn solve_for_closest_point(semi_major_axis: f64, eccentricity: f64, argument_of_periapsis: f64, p2: DVec2) -> DVec2 {
    let mut theta = f64::atan2(p2.y, p2.x);
    for _ in 0..100 {
        let new_theta = r2(semi_major_axis, eccentricity, argument_of_periapsis, theta, p2) / r2_prime(semi_major_axis, eccentricity, argument_of_periapsis, theta, p2);
        let delta = new_theta - theta;
        theta += delta;
    }
    let final_radius = r1(semi_major_axis, eccentricity, argument_of_periapsis, theta);
    vec2(final_radius * f64::cos(theta), final_radius * f64::sin(theta))
}

/// https://blog.chatfield.io/simple-method-for-distance-to-ellipse/
fn test_orbit_clicked(state: &State, orbit: &Orbit, position: DVec2, max_distance_to_select: f64) {
    let parent = orbit.get_parent();
    let absolute_parent_position = state.components.position_components.get(&parent).unwrap().get_absolute_position();
    let relative_nominal_position = orbit.get_position_from_mean_anomaly(orbit.get_arugment_of_periapsis());
    let nominal_position_to_center_vector = -orbit.get_semi_major_axis() * vec2(f64::cos(orbit.get_arugment_of_periapsis()), f64::sin(orbit.get_arugment_of_periapsis()));
    let center = absolute_parent_position + relative_nominal_position + nominal_position_to_center_vector;
    let displacement = center - position;
    solve_for_closest_point(orbit.get_semi_major_axis(), orbit.get_eccentricity(), orbit.get_arugment_of_periapsis(), displacement);
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
            for orbit in trajectory_component.get_orbits() {
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