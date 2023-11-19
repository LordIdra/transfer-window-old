use std::{rc::Rc, cell::RefCell};

use eframe::{egui::{Context, InputState}, epaint::{Pos2, Rect, Rgba}};
use nalgebra_glm::{vec2, DVec2};

use crate::{state::State, components::trajectory_component::orbit::Orbit, util::add_textured_square, camera::SCALE_FACTOR, storage::entity_allocator::Entity};

const SELECTION_CIRCLE_SIZE: f64 = 5.0;

struct ClickPointTempInfo {
    absolute_position: DVec2,
    distance: f64,
}

pub struct ClickPoint {
    entity: Entity,
    orbit: Rc<RefCell<Orbit>>,
    time_since_periapsis: f64,
}

impl ClickPoint {
    pub fn get_position(state: &State, click_point: &ClickPoint) -> DVec2 {
        click_point.orbit.borrow().get_position_from_time_since_periapsis(click_point.time_since_periapsis)
    }
}

fn test_orbit_clicked(state: &State, orbit: &Rc<RefCell<Orbit>>, click_position: DVec2, max_distance_to_select: f64) -> Option<ClickPoint> {
    let parent = orbit.borrow().get_parent();
    let absolute_parent_position = state.components.position_components.get(&parent).unwrap().get_absolute_position();
    let relative_nominal_position = orbit.borrow().get_position_from_theta(orbit.borrow().get_arugment_of_periapsis());
    let nominal_position_to_center_vector = -orbit.borrow().get_semi_major_axis() * vec2(f64::cos(orbit.borrow().get_arugment_of_periapsis()), f64::sin(orbit.borrow().get_arugment_of_periapsis()));
    let center = absolute_parent_position + relative_nominal_position + nominal_position_to_center_vector;
    let click_position_relative_to_center = click_position - center;
    let closest_point_relative_to_center = orbit.borrow().solve_for_closest_point(click_position_relative_to_center);
    let closest_point_relative_to_parent = (center + closest_point_relative_to_center) - absolute_parent_position;
    let theta_relative_to_parent = f64::atan2(closest_point_relative_to_parent.y, closest_point_relative_to_parent.x);
    let time_since_periapsis = orbit.borrow().get_time_since_periapsis(theta_relative_to_parent);
    let closest_point_relative_to_parent = orbit.borrow().get_position_from_theta(theta_relative_to_parent);
    let closest_point = absolute_parent_position + closest_point_relative_to_parent;
    let distance = (click_position - closest_point).magnitude();
    let entity = orbit.borrow().
    if orbit.borrow().is_time_within_orbit(time_since_periapsis) && distance < max_distance_to_select {
       Some(ClickPoint {  })
    } else {
       None
    }
}

fn get_closest_click_point(click_points: Vec<ClickPointTempInfo>) -> Option<ClickPointTempInfo> {
    let mut lowest_distance = f64::MAX;
    let mut lowest_distance_point = None;
    for click_point in click_points {
        if click_point.distance < lowest_distance {
            lowest_distance = click_point.distance;
            lowest_distance_point = Some(click_point);
        }
    }
    lowest_distance_point
}

fn click_point_overlaps_any_icon(state: &State, click_point: &ClickPointTempInfo) -> bool {
    for entity in state.components.entity_allocator.get_entities() {
        if state.components.icon_components.get(&entity).is_some() {
            let position_component = state.components.position_components.get(&entity).unwrap();
            let distance = (position_component.get_absolute_position() - click_point.absolute_position).magnitude();
            let max_distance = state.camera.lock().unwrap().get_max_distance_to_select();
            if distance < max_distance {
                return true;
            }
        }
    }
    false
}

fn get_click_point(state: &mut State, screen_size: Rect, position: Pos2) -> Option<ClickPoint> {
    let position = state.camera.lock().unwrap().window_space_to_world_space(position, screen_size);
    let max_distance_to_select = state.camera.lock().unwrap().get_max_distance_to_select();
    let mut click_points = vec![];
    for entity in state.components.entity_allocator.get_entities() {
        if let Some(trajectory_component) = state.components.trajectory_components.get(&entity) {
            for orbit in trajectory_component.get_orbits() {
                let click_point = test_orbit_clicked(state, orbit, position, max_distance_to_select);
                if let Some(click_point) = click_point {
                    click_points.push(ClickPoint { entity, orbit,  });
                }
            }
        }
    }
    let click_point = get_closest_click_point(click_points);
    if let Some(click_point) = click_point {
        if !click_point_overlaps_any_icon(state, &click_point) {
            return Some(click_point);
        }
    }
    None
}

fn render_click_point(state: &State, click_point: &ClickPointTempInfo, opacity: f32) {
    let radius = SELECTION_CIRCLE_SIZE / state.camera.lock().unwrap().get_zoom();
    let color = Rgba::from_rgba_unmultiplied(1.0, 1.0, 1.0, opacity);
    let mut vertices = vec![];
    add_textured_square(&mut vertices, click_point.absolute_position * SCALE_FACTOR, radius, color);
    state.texture_renderers.lock().unwrap().get_mut("circle").unwrap().set_vertices(vertices);
}

fn on_new_click_point_exists(state: &mut State, input: &InputState, click_point: &ClickPointTempInfo) {
    if input.pointer.primary_clicked() {
        state.orbit_click_point = Some(click_point.clone());
    } else {
        if state.orbit_click_point.is_none() {
            render_click_point(state, &click_point, 0.6);
        }
    }
}

fn on_new_click_point_no_exists(state: &mut State, input: &InputState) {
    if input.pointer.primary_clicked() {
        state.orbit_click_point = None;
    }
}

fn update_click_point_position(state: &mut State) {
    if let Some(click_point) = state.orbit_click_point {
        
    }
}

pub fn orbit_click_system(state: &mut State, context: &Context) {
    let screen_size = context.screen_rect();
    state.texture_renderers.lock().unwrap().get_mut("circle").unwrap().set_vertices(vec![]);
    context.input(|input| {
        if let Some(position) = input.pointer.latest_pos() {
            if let Some(click_point) = get_click_point(state, screen_size, position) {
                on_new_click_point_exists(state, input, &click_point);
            } else {
                on_new_click_point_no_exists(state, input);
            }
        }

        if let Some(click_point) = &state.orbit_click_point {
            render_click_point(state, click_point, 1.0);
        }
    });
}