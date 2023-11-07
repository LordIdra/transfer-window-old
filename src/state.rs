use std::{sync::{Arc, Mutex}, time::Instant};

use eframe::{egui::Context, epaint::Rgba, Frame, CreationContext};
use nalgebra_glm::vec2;

use crate::{renderer::Renderer, camera::Camera, storage::{entity_allocator::Entity, entity_builder::{add_root_object, add_child_object}}, systems::{trajectory_prediction_system::trajectory_prediction_system, camera_update_system::camera_update_system, time_step_update_system::time_step_update_system, object_selection_system::object_selection_system, trajectory_update_system::trajectory_update_system, underlay_render_system::underlay_render_system}, components::Components};

pub struct State {
    pub components: Components,
    pub time_step_level: i32,
    pub time: f64,
    pub delta_time: f64,
    pub last_frame: Instant,
    pub selected: Entity,
    pub camera: Arc<Mutex<Camera>>,
    pub orbit_renderer: Arc<Mutex<Renderer>>,
    pub object_renderer: Arc<Mutex<Renderer>>,
}

impl State {
    pub fn new(creation_context: &CreationContext) -> Self {
        egui_extras::install_image_loaders(&creation_context.egui_ctx);
        let mut components = Components::new();
        let sun = Self::init_root_object(&mut components);
        let mut state = Self {
            components,
            time_step_level: 1,
            time: 0.0,
            delta_time: 0.0,
            selected: sun,
            camera: Arc::new(Mutex::new(Camera::new())),
            orbit_renderer: Arc::new(Mutex::new(Renderer::new(creation_context.gl.as_ref().unwrap().clone()))),
            object_renderer: Arc::new(Mutex::new(Renderer::new(creation_context.gl.as_ref().unwrap().clone()))),
            last_frame: Instant::now(),
        };
        state.init_objects(sun);
        trajectory_prediction_system(&mut state, 0.0);
        state
    }

    fn init_root_object(components: &mut Components) -> Entity {
        add_root_object(components, "sun".to_string(), vec2(0.0, 0.0), vec2(0.0, 0.0), 1.9885e30, 6.957e8, Rgba::from_rgba_unmultiplied(1.0, 1.0, 0.3, 1.0))
    }

    fn init_objects(&mut self, sun: Entity) {
        let earth = add_child_object(&mut self.components, 0.0, "earth".to_string(), sun, vec2(1.521e11, 0.0), vec2(0.0, -2.929e4), 5.9722e24, 6.378e6, Rgba::from_rgba_unmultiplied(0.1, 0.4, 1.0, 1.0));
        add_child_object(&mut self.components, 0.0, "moon".to_string(), earth, vec2(0.4055e9, 0.0), vec2(0.0, -0.970e3), 7.346e22, 1.738e6, Rgba::from_rgba_unmultiplied(0.3, 0.3, 0.3, 1.0));
        add_child_object(&mut self.components, 0.0, "spacecraft".to_string(), earth, vec2(0.0, 8.0e6), vec2(0.9891e4, 0.0), 1.0e3, 1.0e5, Rgba::from_rgba_unmultiplied(0.9, 0.3, 0.3, 1.0));
    }

    pub fn get_time_step(&self) -> f64 {
        5.0_f64.powi(self.time_step_level)
    }
}

impl eframe::App for State {
    fn update(&mut self, context: &Context, _frame: &mut Frame) {
        // context.style_mut(|style| {
        //     let rounding = 20.0;
        //     let rounding_struct = Rounding { nw: rounding, ne: rounding, sw: rounding, se: rounding };

        //     style.visuals.window_fill = Color32::TRANSPARENT;
        //     style.visuals.window_shadow = Shadow::NONE;
        //     style.visuals.window_stroke = Stroke::NONE;
        //     style.visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;
        //     style.visuals.widgets.hovered.weak_bg_fill = Color32::TRANSPARENT;
        //     style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, Color32::from_white_alpha(150));
        //     style.visuals.widgets.hovered.rounding = rounding_struct;
        //     style.visuals.widgets.active.bg_fill = Color32::from_white_alpha(100);
        //     style.visuals.widgets.active.rounding = rounding_struct;
        // });

        // Window::new("")
        //     .title_bar(false)
        //     .resizable(false)
        //     .anchor(Align2::LEFT_TOP, epaint::vec2(100.0, 200.0))
        //     .show(context, |ui| {
        //         let image = Image::new(include_image!("../resources/icons/earth-custom.png"))
        //             .bg_fill(Color32::TRANSPARENT)
        //             .fit_to_exact_size(epaint::vec2(20.0, 20.0));
        //         let image_button = ImageButton::new(image);
        //         ui.add(image_button);
        // });

        time_step_update_system(self, context);
        object_selection_system(self, context);
        trajectory_update_system(self);
        camera_update_system(self, context);
        underlay_render_system(self, context);
        context.request_repaint(); // Update as soon as possible, otherwise it'll only update when some input changes
    }
}