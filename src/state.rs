use std::{sync::{Arc, Mutex}, time::Instant, collections::HashMap, f64::consts::PI};

use eframe::{egui::{Context, Ui}, epaint::Rgba, Frame, CreationContext};
use nalgebra_glm::vec2;

use crate::{camera::Camera, storage::{entity_allocator::Entity, entity_builder::{add_root_object, add_child_celestial_object, add_child_object}}, systems::{trajectory_prediction_system::trajectory_prediction_system, camera_update_system::camera_update_system, time_step_update_system::{time_step_update_system, TimeStepDescription}, object_selection_system::object_selection_system, trajectory_update_system::trajectory_update_system, underlay_render_system::underlay_render_system, icon_precedence_system::icon_precedence_system, orbit_point_selection_system::{orbit_click_system, OrbitClickPoint}, orbit_point_toolbar_system::orbit_point_toolbar_system, mouse_over_any_element_system::was_mouse_over_any_element_last_frame_system, warp_update_system::{warp_update_system, WarpDescription}, delta_time_update_system::delta_time_update_system}, components::Components, resources::Resources, rendering::{geometry_renderer::GeometryRenderer, texture_renderer::TextureRenderer}};

pub struct State {
    pub resources: Resources,
    pub components: Components,
    pub mouse_over_any_element_cache: bool,
    pub mouse_over_any_element: bool,
    pub time_step_description: TimeStepDescription,
    pub time: f64,
    pub delta_time: f64,
    pub last_frame: Instant,
    pub selected_entity: Entity,
    pub orbit_click_point: Option<OrbitClickPoint>,
    pub current_warp: Option<WarpDescription>,
    pub camera: Arc<Mutex<Camera>>,
    pub orbit_renderer: Arc<Mutex<GeometryRenderer>>,
    pub object_renderer: Arc<Mutex<GeometryRenderer>>,
    pub texture_renderers: Arc<Mutex<HashMap<String, TextureRenderer>>>,
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
        let icon_renderers = Self::init_texture_renderers(&gl, &mut resources);
        let mut state = Self {
            resources,
            components,
            mouse_over_any_element_cache: false,
            mouse_over_any_element: false,
            time_step_description: TimeStepDescription::Level(1),
            time: 0.0,
            delta_time: 0.0,
            last_frame: Instant::now(),
            selected_entity: sun,
            orbit_click_point: None,
            current_warp: None,
            camera: Arc::new(Mutex::new(Camera::new())),
            orbit_renderer,
            object_renderer,
            texture_renderers: icon_renderers,
        };
        state.init_objects(sun);
        trajectory_prediction_system(&mut state);
        state
    }

    fn init_root_object(components: &mut Components) -> Entity {
        add_root_object(components, "star".to_string(), "sun".to_string(), vec2(0.0, 0.0), vec2(0.0, 0.0), 1.9885e30, 6.957e8, Rgba::from_rgba_unmultiplied(1.0, 1.0, 0.3, 1.0))
    }

    fn init_texture_renderers(gl: &Arc<glow::Context>, resources: &mut Resources) -> Arc<Mutex<HashMap<String, TextureRenderer>>> {
        let mut texture_renderers = HashMap::new();
        for texture_name in resources.get_texture_names().clone() {
            texture_renderers.insert(texture_name.to_string(), TextureRenderer::new(gl.clone(), resources.get_gl_texture(gl.clone(), texture_name.as_str()).clone(), texture_name));
        }
        Arc::new(Mutex::new(texture_renderers))
    }

    fn init_objects(&mut self, sun: Entity) {
        let earth = add_child_celestial_object(&mut self.components, 0.0, "planet".to_string(), "earth".to_string(), sun, vec2(1.521e11, 0.0), vec2(0.0, -2.729e4), 5.9722e24, 6.378e6, Rgba::from_rgba_unmultiplied(0.1, 0.4, 1.0, 1.0));
        add_child_celestial_object(&mut self.components, 0.0, "moon".to_string(), "moon".to_string(), earth, 
            vec2(0.4055e9 * f64::cos(2.0), 0.4055e9 * f64::sin(2.0)), vec2(0.970e3 * f64::cos(2.0 + PI / 2.0), 0.970e3 * f64::sin(2.0 + PI / 2.0)), 
            //vec2(0.4055e9 * f64::cos(30.0), 0.4055e9 * f64::sin(30.0)), vec2(-1.303e3 * f64::cos(30.0 + PI / 2.0), -1.303e3 * f64::sin(30.0 + PI / 2.0)), 
            7.346e22, 1.738e6, Rgba::from_rgba_unmultiplied(0.3, 0.3, 0.3, 1.0));
        add_child_object(&mut self.components, 0.0, "spacecraft".to_string(), "spacecraft".to_string(), earth, vec2(0.0, 8.0e6), vec2(-0.987e4, 0.0), 1.0e3);
    }

    pub fn get_time_step(&self) -> f64 {
        match self.time_step_description {
            TimeStepDescription::Level(level) => 5.0_f64.powi(level-1),
            TimeStepDescription::Raw(raw) => raw,
        }
    }

    pub fn register_ui(&mut self, ui: &Ui) {
        if ui.ui_contains_pointer() {
            self.mouse_over_any_element_cache = true;
        }
    }
}

impl eframe::App for State {
    fn update(&mut self, context: &Context, _frame: &mut Frame) {
        delta_time_update_system(self);
        warp_update_system(self);
        time_step_update_system(self, context);
        icon_precedence_system(self);
        object_selection_system(self, context);
        trajectory_update_system(self);
        camera_update_system(self, context);
        orbit_click_system(self, context);
        orbit_point_toolbar_system(self, context);
        underlay_render_system(self, context);
        was_mouse_over_any_element_last_frame_system(self);
        context.request_repaint(); // Update as soon as possible, otherwise it'll only update when some input changes
    }
}