use std::{sync::{Arc, Mutex}, time::Instant, collections::HashMap};

use eframe::{egui::{Context, Window, ImageButton, Image}, epaint::{Rgba, Rounding, Color32, Shadow, Stroke, self}, Frame, CreationContext, emath::Align2};
use nalgebra_glm::vec2;

use crate::{camera::Camera, storage::{entity_allocator::Entity, entity_builder::{add_root_object, add_child_object}}, systems::{trajectory_prediction_system::trajectory_prediction_system, camera_update_system::camera_update_system, time_step_update_system::time_step_update_system, object_selection_system::object_selection_system, trajectory_update_system::trajectory_update_system, underlay_render_system::underlay_render_system, icon_precedence_system::icon_precedence_system, orbit_click_system::orbit_click_system}, components::Components, resources::Resources, rendering::{geometry_renderer::GeometryRenderer, texture_renderer::TextureRenderer}};

const ICON_NAMES: [&str; 4] = ["star", "planet", "moon", "spacecraft"];

pub struct State {
    pub resources: Resources,
    pub components: Components,
    pub time_step_level: i32,
    pub time: f64,
    pub delta_time: f64,
    pub last_frame: Instant,
    pub selected: Entity,
    pub camera: Arc<Mutex<Camera>>,
    pub orbit_renderer: Arc<Mutex<GeometryRenderer>>,
    pub object_renderer: Arc<Mutex<GeometryRenderer>>,
    pub icon_renderers: Arc<Mutex<HashMap<String, TextureRenderer>>>,
}

impl State {
    pub fn new(creation_context: &CreationContext) -> Self {
        egui_extras::install_image_loaders(&creation_context.egui_ctx);
        let mut resources = Resources::new();
        let mut components = Components::new();
        let sun = Self::init_root_object(&mut components);
        let gl = creation_context.gl.as_ref().unwrap().clone();
        let orbit_renderer = Arc::new(Mutex::new(GeometryRenderer::new(gl.clone())));
        let object_renderer = Arc::new(Mutex::new(GeometryRenderer::new(gl.clone())));
        let icon_renderers = Self::init_icon_renderers(&gl, &mut resources);
        let mut state = Self {
            resources,
            components,
            time_step_level: 1,
            time: 0.0,
            delta_time: 0.0,
            last_frame: Instant::now(),
            selected: sun,
            camera: Arc::new(Mutex::new(Camera::new())),
            orbit_renderer,
            object_renderer,
            icon_renderers,
        };
        state.init_objects(sun);
        trajectory_prediction_system(&mut state, 0.0);
        state
    }

    fn init_root_object(components: &mut Components) -> Entity {
        add_root_object(components, "star".to_string(), "sun".to_string(), vec2(0.0, 0.0), vec2(0.0, 0.0), 1.9885e30, 6.957e8, Rgba::from_rgba_unmultiplied(1.0, 1.0, 0.3, 1.0))
    }

    fn init_icon_renderers(gl: &Arc<glow::Context>, resources: &mut Resources) -> Arc<Mutex<HashMap<String, TextureRenderer>>> {
        let mut icon_renderers = HashMap::new();
        for icon_name in ICON_NAMES {
            icon_renderers.insert(icon_name.to_string(), TextureRenderer::new(gl.clone(), resources.get_gl_texture(gl.clone(), icon_name).clone(), icon_name.to_string()));
        }
        Arc::new(Mutex::new(icon_renderers))
    }

    fn init_objects(&mut self, sun: Entity) {
        let earth = add_child_object(&mut self.components, 0.0, "planet".to_string(), "earth".to_string(), sun, vec2(1.521e11, 0.0), vec2(0.0, -2.929e4), 5.9722e24, 6.378e6, Rgba::from_rgba_unmultiplied(0.1, 0.4, 1.0, 1.0));
        add_child_object(&mut self.components, 0.0, "moon".to_string(), "moon".to_string(), earth, vec2(0.4055e9, 0.0), vec2(0.0, -0.970e3), 7.346e22, 1.738e6, Rgba::from_rgba_unmultiplied(0.3, 0.3, 0.3, 1.0));
        add_child_object(&mut self.components, 0.0, "spacecraft".to_string(), "spacecraft".to_string(), earth, vec2(0.0, 8.0e6), vec2(0.989e4, 0.0), 1.0e3, 1.0e5, Rgba::from_rgba_unmultiplied(0.9, 0.3, 0.3, 1.0));
    }

    pub fn get_time_step(&self) -> f64 {
        5.0_f64.powi(self.time_step_level)
    }
}

impl eframe::App for State {
    fn update(&mut self, context: &Context, _frame: &mut Frame) {
        context.style_mut(|style| {
            let rounding = 20.0;
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

        Window::new("")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_TOP, epaint::vec2(100.0, 200.0))
            .show(context, |ui| {
                let image = Image::new(self.resources.get_texture_image("planet"))
                    .bg_fill(Color32::TRANSPARENT)
                    .fit_to_exact_size(epaint::vec2(20.0, 20.0));
                let image_button = ImageButton::new(image);
                ui.add(image_button);
        });

        time_step_update_system(self, context);
        object_selection_system(self, context);
        icon_precedence_system(self);
        orbit_click_system(self, context);
        trajectory_update_system(self);
        camera_update_system(self, context);
        underlay_render_system(self, context);
        context.request_repaint(); // Update as soon as possible, otherwise it'll only update when some input changes
    }
}