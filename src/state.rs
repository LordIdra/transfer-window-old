use std::{sync::{Arc, Mutex}, time::Instant};

use eframe::{egui::{Context, CentralPanel, Ui, Key, InputState, PointerButton, include_image, Image, Window, ImageButton}, epaint::{Rgba, PaintCallback, Rect, self, Color32, Rounding, Shadow, Stroke}, Frame, CreationContext, egui_glow::CallbackFn, emath::Align2};
use nalgebra_glm::vec2;

use crate::{object::Object, renderer::Renderer, camera::Camera, id_storage::IdStorage, components::{celestial_body_component::CelestialBodyComponent, mass_component::MassComponent, parent_component::ParentComponent, position_component::PositionComponent, trajectory_component::TrajectoryComponent, velocity_component::VelocityComponent}};

pub type Entity = usize;

const MIN_TIME_STEP_LEVELS: i32 = 1;
const MAX_TIME_STEP_LEVELS: i32 = 8;

pub struct State {
    pub time_step_level: i32,
    pub time: f64,
    pub last_frame: Instant,
    pub selected: Entity,
    pub camera: Arc<Mutex<Camera>>,
    pub orbit_renderer: Arc<Mutex<Renderer>>,
    pub object_renderer: Arc<Mutex<Renderer>>,
    pub celestial_body_components: IdStorage<CelestialBodyComponent>,
    pub mass_components: IdStorage<MassComponent>,
    pub parent_components: IdStorage<ParentComponent>,
    pub position_components: IdStorage<PositionComponent>,
    pub trajectory_components: IdStorage<TrajectoryComponent>,
    pub velocity_component: IdStorage<VelocityComponent>,
}

impl State {
    pub fn new(creation_context: &CreationContext) -> Self {
        egui_extras::install_image_loaders(&creation_context.egui_ctx);

        let mut storage = Storage::new();
        let sun = Object::new(&mut storage, "sun".to_string(), None, vec2(0.0, 0.0), vec2(0.0, 0.0), 1.9885e30, 6.957e8, Rgba::from_rgba_unmultiplied(1.0, 1.0, 0.3, 1.0), 0.0);
        let earth = Object::new(&mut storage, "earth".to_string(), Some(sun.clone()), vec2(1.521e11, 0.0), vec2(0.0, -2.929e4), 5.9722e24, 6.378e6, Rgba::from_rgba_unmultiplied(0.1, 0.4, 1.0, 1.0), 0.0);
        Object::new(&mut storage, "moon".to_string(), Some(earth.clone()), vec2(0.4055e9, 0.0), vec2(0.0, -0.970e3), 7.346e22, 1.738e6, Rgba::from_rgba_unmultiplied(0.3, 0.3, 0.3, 1.0), 0.0);
        let spacecraft = Object::new(&mut storage, "spacecraft".to_string(), Some(earth.clone()), vec2(0.0, 8.0e6), vec2(0.989e4, 0.0), 1.0e3, 1.0e5, Rgba::from_rgba_unmultiplied(0.9, 0.3, 0.3, 1.0), 0.0);
        storage.set_root(sun);
        
        let camera = Camera::new();
        storage.do_full_prediction(0.0);

        let camera = Arc::new(Mutex::new(camera));
        
        Self {
            time_step_level: 1,
            time: 0.0,
            selected: spacecraft,
            camera: camera.clone(),
            orbit_renderer: Arc::new(Mutex::new(Renderer::new(creation_context.gl.as_ref().unwrap().clone()))),
            object_renderer: Arc::new(Mutex::new(Renderer::new(creation_context.gl.as_ref().unwrap().clone()))),
            storage,
            last_frame: Instant::now(),
        }
    }

    fn get_time_step(&self) -> f64 {
        5.0_f64.powi(self.time_step_level)
    }

    fn update_time_step_level(&mut self, input: &InputState) {
        if input.key_pressed(Key::ArrowLeft) && self.time_step_level > MIN_TIME_STEP_LEVELS {
            self.time_step_level -= 1;
        }
        if input.key_pressed(Key::ArrowRight) && self.time_step_level < MAX_TIME_STEP_LEVELS {
            self.time_step_level += 1;
        }
    }

    fn update_selected_object(&mut self, input: &InputState, screen_size: Rect) {
        if input.pointer.button_double_clicked(PointerButton::Primary) {
            if let Some(screen_position) = input.pointer.latest_pos() {
                let world_position = self.camera.lock().unwrap().window_space_to_world_space(screen_position, screen_size);
                let max_distance_to_select = self.camera.lock().unwrap().get_max_distance_to_select();
                if let Some(selected_object) = self.storage.get_selected_object(world_position, max_distance_to_select) {
                    if self.selected != selected_object {
                        self.selected = selected_object.clone();
                        self.camera.lock().unwrap().update_selected(Some(selected_object.clone()));
                    }
                }
            }
        }
    }

    fn update_camera_translation(&self, input: &InputState) {
        if input.key_pressed(Key::R) {
            self.camera.lock().unwrap().recenter();
        }
    }

    fn render_underlay(&self, context: &Context, ui: &Ui) {
        let object_vertices = self.storage.get_object_vertices();
        self.object_renderer.lock().unwrap().set_vertices(object_vertices);
        let orbit_vertices = self.storage.get_orbit_vertices(self.camera.lock().unwrap().get_zoom());
        self.orbit_renderer.lock().unwrap().set_vertices(orbit_vertices);
        let rect = context.screen_rect();
        let orbit_renderer = self.orbit_renderer.clone();
        let object_renderer = self.object_renderer.clone();
        let camera = self.camera.clone();
        let callback = Arc::new(CallbackFn::new(move |_info, _painter| {
            orbit_renderer.lock().unwrap().render(rect, camera.clone());
            object_renderer.lock().unwrap().render(rect, camera.clone());
        }));

        ui.painter().add(PaintCallback { rect, callback });
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
                let image = Image::new(include_image!("../resources/icons/earth-custom.png"))
                    .bg_fill(Color32::TRANSPARENT)
                    .fit_to_exact_size(epaint::vec2(20.0, 20.0));
                let image_button = ImageButton::new(image);
                ui.add(image_button);
        });

        let screen_size = context.screen_rect();
        context.input(|input| self.update_selected_object(input, screen_size));
        context.input(|input| self.update_time_step_level(input));
        context.input(|input| self.update_camera_translation(input));
        self.camera.lock().unwrap().update(&self.storage, context);

        CentralPanel::default().show(context, |ui| {
            self.render_underlay(context, ui);
        });

        let delta_time = (Instant::now() - self.last_frame).as_secs_f64() * self.get_time_step();
        self.time += delta_time;
        self.storage.update(delta_time);
        self.last_frame = Instant::now();
            
        context.request_repaint(); // Update as soon as possible, otherwise it'll only update when some input changes
    }
}