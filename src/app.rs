use std::{sync::{Arc, Mutex}, time::Instant};

use eframe::{egui::{Context, CentralPanel, Slider, Ui, Key, InputState, PointerButton}, epaint::{Rgba, PaintCallback, Rect}, Frame, CreationContext, egui_glow::CallbackFn};
use nalgebra_glm::vec2;

use crate::{object::Object, renderer::Renderer, camera::Camera, storage::Storage};

pub type ObjectId = String;

const MIN_TIME_STEP_LEVELS: i32 = 1;
const MAX_TIME_STEP_LEVELS: i32 = 8;

pub struct App {
    name: String,
    age: i32,
    time_step_level: i32,
    time: f64,
    selected_object: ObjectId,
    camera: Arc<Mutex<Camera>>,
    orbit_renderer: Arc<Mutex<Renderer>>,
    object_renderer: Arc<Mutex<Renderer>>,
    storage: Storage,
    last_frame: Instant,
}

impl App {
    pub fn new(creation_context: &CreationContext) -> Self {
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
            name: "oh no".to_string(), 
            age: 0,
            time_step_level: 1,
            time: 0.0,
            selected_object: spacecraft,
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
                    if self.selected_object != selected_object {
                        self.selected_object = selected_object.clone();
                        self.camera.lock().unwrap().update_selected(Some(selected_object.clone()));
                    }
                }
            }
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

    fn render_ui(&mut self, ui: &mut Ui) {
        ui.heading("My egui Application");
        ui.horizontal(|ui| {
            let name_label = ui.label("Your name: ");
            ui.text_edit_singleline(&mut self.name)
                .labelled_by(name_label.id);
        });

        ui.add(Slider::new(&mut self.age, 0..=120).text("age"));
        if ui.button("Click each year").clicked() {
            self.age += 1;
        }

        ui.label(format!("Hello '{}'", self.name));
    }
}

impl eframe::App for App {
    fn update(&mut self, context: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(context, |ui| {
            self.camera.lock().unwrap().update(&self.storage, context);
            self.render_underlay(context, ui);
            self.render_ui(ui);
            let delta_time = (Instant::now() - self.last_frame).as_secs_f64() * self.get_time_step();
            self.time += delta_time;
            self.storage.update(delta_time);
            self.last_frame = Instant::now();

            let screen_size = context.screen_rect();
            
            context.input(|input| self.update_time_step_level(input));
            context.input(move |input| self.update_selected_object(input, screen_size));

            context.request_repaint(); // Update as soon as possible, otherwise it'll only update when some input changes
        });
    }
}