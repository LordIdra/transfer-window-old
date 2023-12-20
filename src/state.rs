use std::{sync::{Arc, Mutex}, time::Instant, collections::HashMap, f64::consts::PI, cmp::Ordering};

use eframe::{egui::{Context, Ui}, epaint::Rgba, Frame, CreationContext};
use nalgebra_glm::vec2;

use crate::{camera::Camera, storage::{entity_allocator::Entity, entity_builder::{add_root_object, add_child_celestial_object, add_child_object}}, systems::{camera_update_system::camera_update_system, time_step_update_system::{time_step_update_system, TimeStepDescription}, trajectory_update_system::trajectory_update_system, underlay_render_system::underlay_render_system, orbit_point_selection_system::{orbit_click_system, OrbitClickPoint}, mouse_over_any_element_system::was_mouse_over_any_element_last_frame_system, warp_update_system::{warp_update_system, WarpDescription}, delta_time_update_system::delta_time_update_system, trajectory_prediction_system::{celestial_body_prediction::predict_celestial_bodies, spacecraft_prediction::predict_all_spacecraft}, debug_system::debug_system, deselect_system::deselect_system, icon_system::icon_system, toolbar_system::toolbar_system}, components::Components, resources::Resources, rendering::{geometry_renderer::GeometryRenderer, texture_renderer::TextureRenderer}};

pub struct State {
    pub resources: Resources,
    pub components: Components,
    pub mouse_over_any_element_cache: bool,
    pub mouse_over_any_element: bool,
    pub time_step_description: TimeStepDescription,
    pub paused: bool,
    pub debug_mode: bool,
    pub time: f64,
    pub delta_time: f64,
    pub last_frame: Instant,
    pub selected_object: Entity,
    pub selected_burn_icon: Option<Entity>,
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
            paused: false,
            debug_mode: false,
            time: 0.0,
            delta_time: 0.0,
            last_frame: Instant::now(),
            selected_object: sun,
            selected_burn_icon: None,
            orbit_click_point: None,
            current_warp: None,
            camera: Arc::new(Mutex::new(Camera::new())),
            orbit_renderer,
            object_renderer,
            texture_renderers: icon_renderers,
        };
        state.init_objects(sun);
        predict_celestial_bodies(&mut state, 10000000.0);
        predict_all_spacecraft(&mut state, 10000000.0);
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
        let spacecraft = add_child_object(&mut self.components, 0.0, "spacecraft".to_string(), "spacecraft".to_string(), earth, vec2(0.0, 8.0e6), vec2(-1.387e4, 0.0), 1.0e3);
        self.selected_object = spacecraft;
    }

    pub fn get_time_step(&self) -> f64 {
        if self.paused {
            return 0.0;
        }
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

    /// Sorted from highest to lowest mass
    /// Only contains entities with mass
    pub fn get_entities_sorted_by_mass(&self) -> Vec<Entity> {
        let mut entities: Vec<Entity> = self.components.entity_allocator
            .get_entities()
            .into_iter()
            .filter(|entity| self.components.mass_components.get(entity).is_some())
            .collect();
        entities.sort_by(|a, b| {
            let mass_a = self.components.mass_components.get(a).unwrap().get_mass();
            let mass_b = self.components.mass_components.get(b).unwrap().get_mass();
            if mass_a > mass_b {
                Ordering::Less
            } else if mass_a < mass_b {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });
        entities
    }
}

impl eframe::App for State {
    fn update(&mut self, context: &Context, _frame: &mut Frame) {
        delta_time_update_system(self);
        warp_update_system(self);
        time_step_update_system(self, context);
        trajectory_update_system(self);
        deselect_system(self, context);
        icon_system(self, context);
        camera_update_system(self, context);
        orbit_click_system(self, context);
        debug_system(self, context);
        toolbar_system(self, context);
        underlay_render_system(self, context);
        was_mouse_over_any_element_last_frame_system(self);
        context.request_repaint(); // Update as soon as possible, otherwise it'll only update when some input changes
    }
}