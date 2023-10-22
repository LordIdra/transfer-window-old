use std::{sync::{Arc, Mutex}, time::Instant};

use eframe::{egui::{Context, CentralPanel, Slider, Ui}, epaint::{Rgba, PaintCallback}, Frame, CreationContext, egui_glow::CallbackFn};
use nalgebra_glm::vec2;

use crate::{object::Object, renderer::Renderer, camera::Camera, storage::Storage};

const TIME_STEP: f64 = 86400.0 * 365.0 / 250.0;

pub type ObjectId = String;

pub struct App {
    name: String,
    age: i32,
    time: f64,
    camera: Arc<Mutex<Camera>>,
    orbit_renderer: Arc<Mutex<Renderer>>,
    object_renderer: Arc<Mutex<Renderer>>,
    object_storage: Storage,
    last_frame: Instant,
}

impl App {
    pub fn new(creation_context: &CreationContext) -> Self {
        let mut camera = Camera::new();
        let mut object_storage = Storage::new();
        let sun = Object::new(&mut object_storage, "sun".to_string(), None, vec2(0.0, 0.0), vec2(0.0, 0.0), 1.9885e30, 6.957e8, Rgba::from_rgba_unmultiplied(1.0, 1.0, 0.3, 1.0), 0.0);
        let earth = Object::new(&mut object_storage, "earth".to_string(), Some(sun), vec2(1.521e11, 0.0), vec2(0.0, -2.929e4), 5.9722e24, 6.378e6, Rgba::from_rgba_unmultiplied(0.1, 0.4, 1.0, 1.0), 0.0);
        Object::new(&mut object_storage, "moon".to_string(), Some(earth.clone()), vec2(0.4055e9, 0.0), vec2(0.0, -0.970e3), 7.346e22, 1.738e6, Rgba::from_rgba_unmultiplied(0.3, 0.3, 0.3, 1.0), 0.0);
        let spacecraft1 = Object::new(&mut object_storage, "spacecraft1".to_string(), Some(earth.clone()), vec2(0.0, 8.0e6), vec2(0.979e4, 0.0), 1.0e3, 1.0e5, Rgba::from_rgba_unmultiplied(0.9, 0.3, 0.3, 1.0), 0.0);

        camera.follow(spacecraft1);
        object_storage.do_full_prediction(0.0);

        let camera = Arc::new(Mutex::new(camera));
        
        Self {
            name: "oh no".to_string(), 
            age: 0,
            time: 0.0,
            camera: camera.clone(),
            orbit_renderer: Arc::new(Mutex::new(Renderer::new(creation_context.gl.as_ref().unwrap().clone(), camera.clone()))),
            object_renderer: Arc::new(Mutex::new(Renderer::new(creation_context.gl.as_ref().unwrap().clone(), camera.clone()))),
            object_storage,
            last_frame: Instant::now(),
        }
    }

    fn render_underlay(&self, context: &Context, ui: &Ui) {
        let object_vertices = self.object_storage.get_object_vertices();
        self.object_renderer.lock().unwrap().set_vertices(object_vertices);
        let orbit_vertices = self.object_storage.get_orbit_vertices(self.camera.lock().unwrap().get_zoom());
        self.orbit_renderer.lock().unwrap().set_vertices(orbit_vertices);

        let rect = context.screen_rect();
        let orbit_renderer = self.orbit_renderer.clone();
        let object_renderer = self.object_renderer.clone();
        let callback = Arc::new(CallbackFn::new(move |_info, _painter| {
            orbit_renderer.lock().unwrap().render(rect);
            object_renderer.lock().unwrap().render(rect);
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
            self.orbit_renderer.lock().unwrap().update(context, &self.object_storage);
            self.object_renderer.lock().unwrap().update(context, &self.object_storage);
            self.render_underlay(context, ui);
            self.render_ui(ui);
            let delta_time = (Instant::now() - self.last_frame).as_secs_f64() * TIME_STEP;
            self.time += delta_time;
            self.object_storage.update(delta_time);
            self.last_frame = Instant::now();
        });            
    }
}