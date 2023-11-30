use std::cell::Ref;

use eframe::{epaint::Rgba, egui::Context};
use nalgebra_glm::{DVec2, vec2};

use crate::{state::State, util::add_triangle, camera::SCALE_FACTOR, components::trajectory_component::segment::{orbit::Orbit, Segment}};

const ORBIT_PATH_MAX_ALPHA: f32 = 1.0;
const ORBIT_PATH_RADIUS_DELTA: f64 = 0.6;
const TESSELLATION_THRESHOLD: f64 = 5000.0;

struct ScreenLines {
    rightx: f64,
    leftx: f64,
    topy: f64,
    bottomy: f64,
}

impl ScreenLines {
    pub fn relative_to_center(&self, state: &State, orbit: &Ref<Orbit>) -> ScreenLines {
        let parent = orbit.get_parent();
        let absolute_parent_position = state.components.position_components.get(&parent).unwrap().get_absolute_position();
        let argument_of_periapsis = orbit.get_arugment_of_periapsis();
        let relative_nominal_position = orbit.get_position_from_theta(argument_of_periapsis);
        let nominal_position_to_center_vector = -orbit.get_semi_major_axis() * vec2(f64::cos(argument_of_periapsis), f64::sin(argument_of_periapsis));
        let center = absolute_parent_position + relative_nominal_position + nominal_position_to_center_vector;
        ScreenLines { 
            rightx: self.rightx - center.x,
            leftx: self.leftx - center.x,
            topy: self.topy - center.y,
            bottomy: self.bottomy - center.y,
        }
    }
}

enum ScreenIntersection {
    RenderNothing, // don't render
    RenderEverything,
    RenderSegment(DVec2, DVec2),
}

#[derive(Clone)]
struct VisualOrbitPoint {
    theta: f64,
    absolute_position: DVec2,
    displacement_direction: DVec2,
}

fn color(orbit_index: usize) -> Rgba {
    let colors = [
        Rgba::from_rgb(1.0, 0.0, 0.0),
        Rgba::from_rgb(0.0, 1.0, 0.0),
        Rgba::from_rgb(0.0, 0.0, 1.0),

        Rgba::from_rgb(1.0, 1.0, 0.0),
        Rgba::from_rgb(1.0, 0.0, 1.0),
        Rgba::from_rgb(0.0, 1.0, 1.0)];
    colors[orbit_index % colors.len()]
}

fn add_orbit_line(vertices: &mut Vec<f32>, previous_point: &VisualOrbitPoint, new_point: &VisualOrbitPoint, zoom: f64, i: i32, orbit_index: usize) {
    let radius = ORBIT_PATH_RADIUS_DELTA + (i as f64 * ORBIT_PATH_RADIUS_DELTA); // Start off with non-zero radius
    let mut alpha = ORBIT_PATH_MAX_ALPHA;
    if i != 0 {
        // Scale the alpha non-linearly so we have lots of values close to zero
        alpha /= 7.0 * i as f32;
    }

    let rgb = color(orbit_index);
    let rgba = Rgba::from_rgba_unmultiplied(rgb.r(), rgb.g(), rgb.b(), alpha);

    let v1 = previous_point.absolute_position + (previous_point.displacement_direction * radius / zoom);
    let v2 = previous_point.absolute_position - (previous_point.displacement_direction * radius / zoom);
    let v3 = new_point.absolute_position + (new_point.displacement_direction * radius / zoom);
    let v4 = new_point.absolute_position - (new_point.displacement_direction * radius / zoom);

    add_triangle(vertices, v1, v2, v3, rgba);
    add_triangle(vertices, v2, v3, v4, rgba);
}

fn create_visual_orbit_point(orbit: &Orbit, absolute_parent_position: DVec2, theta: f64) -> VisualOrbitPoint {
    let relative_point_position = orbit.get_position_from_theta(theta) * SCALE_FACTOR;
    let absolute_position = absolute_parent_position + relative_point_position;
    let displacement_direction = relative_point_position.normalize();
    VisualOrbitPoint { absolute_position, displacement_direction, theta }
}

// https://www.omnicalculator.com/math/3-sides-triangle-area#calculating-the-area-of-a-triangle-with-3-sides-herons-formula
fn triangle_area(p1: DVec2, p2: DVec2, p3: DVec2) -> f64 {
    let a = (p1 - p2).magnitude();
    let b = (p2 - p3).magnitude();
    let c = (p3 - p1).magnitude();
    0.25 * f64::sqrt((a + b + c) * (-a + b + c) * (a - b + c) * (a + b - c))
}

fn should_tessellate(triangle_area: f64, distance_to_camera: f64, camera_zoom: f64) -> bool {
    //println!("{} {} {}", triangle_area, distance_to_camera.powi(2), TESSELLATION_THRESHOLD / camera_zoom);
    triangle_area / distance_to_camera.powf(0.3) > TESSELLATION_THRESHOLD / camera_zoom
}

fn tessellate(orbit: &Orbit, absolute_parent_position: DVec2, a: &VisualOrbitPoint, b: &VisualOrbitPoint) -> VisualOrbitPoint {
    let theta = (a.theta + b.theta) / 2.0;
    create_visual_orbit_point(orbit, absolute_parent_position, theta)
}

/// https://forum.kerbalspaceprogram.com/topic/201736-developer-insights-9-%E2%80%93-orbit-tessellation/
/// Uses triangle heuristic - the further from the camera the middle point, the larger the triangle can be without tessallating
fn do_orbit_tessellation(mut visual_orbit_points: Vec<VisualOrbitPoint>, orbit: &Orbit, absolute_parent_position: DVec2,  camera_position: DVec2, camera_zoom: f64) -> Vec<VisualOrbitPoint> {
    let mut i = 2;
    while i < visual_orbit_points.len() {
        let point_1 = visual_orbit_points[i-2].clone();
        let point_2 = visual_orbit_points[i-1].clone();
        let point_3 = visual_orbit_points[i].clone();
        let triangle_area = triangle_area(point_1.absolute_position, point_2.absolute_position, point_3.absolute_position);
        let distance_to_camera = (camera_position - point_2.absolute_position).magnitude();
        if should_tessellate(triangle_area, distance_to_camera, camera_zoom) {
        //if true {
            visual_orbit_points.insert(i-1, tessellate(orbit, absolute_parent_position, &point_1, &point_2));
            visual_orbit_points.insert(i+1, tessellate(orbit, absolute_parent_position, &point_2, &point_3));
        } else {
            i += 1;
        }
    }
    //println!("{}", i);
    visual_orbit_points
    // if was_anything_tessellated {
    //     do_orbit_tessellation(new_visual_orbit_points, orbit, absolute_parent_position, camera_position, camera_zoom)
    // } else {
    //     new_visual_orbit_points
    // }
}

fn get_visual_orbit_points(state: &State, orbit: &Orbit) -> Vec<VisualOrbitPoint> {
    let parent = orbit.get_parent();
    let absolute_parent_position = state.components.position_components.get(&parent).unwrap().get_absolute_position() * SCALE_FACTOR;
    let mut visual_orbit_points = vec![];
    let angle_to_rotate_through = orbit.get_remaining_angle();
    visual_orbit_points.push(create_visual_orbit_point(orbit, absolute_parent_position, orbit.get_current_true_anomaly()));
    visual_orbit_points.push(create_visual_orbit_point(orbit, absolute_parent_position, orbit.get_current_true_anomaly() + 0.25 * angle_to_rotate_through));
    visual_orbit_points.push(create_visual_orbit_point(orbit, absolute_parent_position, orbit.get_current_true_anomaly() + 0.5 * angle_to_rotate_through));
    visual_orbit_points.push(create_visual_orbit_point(orbit, absolute_parent_position, orbit.get_current_true_anomaly() + 0.75 * angle_to_rotate_through));
    visual_orbit_points.push(create_visual_orbit_point(orbit, absolute_parent_position, orbit.get_current_true_anomaly() + 1.0 * angle_to_rotate_through));
    let camera_position = state.camera.lock().unwrap().get_translation();
    let camera_zoom = state.camera.lock().unwrap().get_zoom();
    do_orbit_tessellation(visual_orbit_points, orbit, absolute_parent_position, camera_position, camera_zoom)
}

fn get_entity_orbit_vertices(state: &State, orbit: &Orbit, orbit_index: usize) -> Vec<f32> {
    let zoom = state.camera.lock().unwrap().get_zoom();
    let points = get_visual_orbit_points(state, orbit);
    let mut previous_point = None;
    let mut vertices = vec![];
    for new_point in &points {
        // Loop to create glow effect
        if let Some(previous_point) = previous_point {
            for i in 0..10 {
                add_orbit_line(&mut vertices, previous_point, new_point, zoom, i, orbit_index);
            }
        }
        previous_point = Some(new_point);
    }
    vertices
}

fn get_entity_segment_vertices(state: &State, segment: &Segment, orbit_index: usize) -> Vec<f32> {
    match segment {
        Segment::Burn(_) => todo!(),
        Segment::Orbit(orbit) => get_entity_orbit_vertices(state, &*orbit.borrow(), orbit_index),
    }
}

// fn get_screen_lines(state: &State, context: &Context) -> ScreenLines {
//     let camera_position = state.camera.lock().unwrap().get_translation() / SCALE_FACTOR;
//     let camera_zoom = state.camera.lock().unwrap().get_zoom();
//     let width = context.screen_rect().width() as f64 / camera_zoom / SCALE_FACTOR;
//     let height = context.screen_rect().height() as f64 / camera_zoom / SCALE_FACTOR;
//     ScreenLines { 
//         rightx: camera_position.x + width / 2.0, 
//         leftx: camera_position.x - width / 2.0, 
//         topy: camera_position.y + height / 2.0, 
//         bottomy: camera_position.y - height / 2.0 
//     }
// }

// fn get_screen_intersection(state: &State, screen_lines: &ScreenLines, orbit: Ref<Orbit>) -> ScreenIntersection {
//     let screen_lines = screen_lines.relative_to_center(state, &orbit);
//     println!("{} {} {} {}", screen_lines.leftx, screen_lines.rightx, screen_lines.bottomy, screen_lines.topy); // 156248226 -152101793 058
//     let a = orbit.get_semi_major_axis();
//     let b = orbit.get_semi_minor_axis();
//     // TODO make hyperbola version (kill me)
//     let righty = b * f64::sqrt(1.0 - screen_lines.rightx.powi(2) / a.powi(2));
//     let lefty = b * f64::sqrt(1.0 - screen_lines.leftx.powi(2) / a.powi(2));
//     let topx = a * f64::sqrt(1.0 - screen_lines.topy.powi(2) / b.powi(2));
//     let bottomx = a * f64::sqrt(1.0 - screen_lines.topy.powi(2) / b.powi(2));
//     let possible_solutions = vec![
//         vec2(topx, screen_lines.topy), vec2(-topx, screen_lines.topy),
//         vec2(bottomx, screen_lines.bottomy), vec2(-bottomx, screen_lines.bottomy),
//         vec2(screen_lines.rightx, righty), vec2(screen_lines.rightx, -righty),
//         vec2(screen_lines.leftx, lefty), vec2(screen_lines.leftx, -lefty)];
//     let mut solutions: Vec<DVec2> = vec![];
//     for solution in possible_solutions {
//         println!("s {} {}", solution.x, solution.y);
//         if solution.x >= screen_lines.leftx && solution.x <= screen_lines.rightx && solution.y >= screen_lines.bottomy && solution.y <= screen_lines.topy {
//             solutions.push(vec2(solution.x, solution.y));
//         }
//     }
//     match solutions.len() {
//         0 => {
//             if screen_lines.rightx > 0.0 && screen_lines.leftx < 0.0 && screen_lines.topy > 0.0 && screen_lines.bottomy < 0.0 {
//                 println!("everything (none)");
//                 ScreenIntersection::RenderEverything
//             } else {
//                 println!("nothing");
//                 ScreenIntersection::RenderNothing
//             }
//         },
//         2 => {
//             println!("segment");
//             ScreenIntersection::RenderSegment(solutions[0], solutions[1])
//         }
//         _ => {
//             println!("everything (many)");
//             ScreenIntersection::RenderEverything
//         }
//     }
// }

pub fn get_all_orbit_vertices(state: &mut State, context: &Context) -> Vec<f32> {
    let mut vertices = vec![];
    for entity in state.components.entity_allocator.get_entities() {
        if let Some(trajectory_component) = state.components.trajectory_components.get(&entity) {
            for (orbit_index, segment) in trajectory_component.get_segments().iter().enumerate() {
                vertices.append(&mut get_entity_segment_vertices(state, segment, orbit_index));
            }
        }
    }
    vertices
}