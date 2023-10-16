use std::{sync::{Arc, Mutex}, time::Instant};

use eframe::{egui::{Context, CentralPanel, Slider, Ui}, epaint::{Rgba, PaintCallback}, Frame, CreationContext, egui_glow::CallbackFn};
use nalgebra_glm::vec2;

use crate::{object::{Object, trajectory_integrator::do_full_trajectory_integration}, renderer::Renderer, camera::Camera};

const TIME_STEP: f64 = 86400.0 * 365.0 / 1000.0;

pub struct App {
    name: String,
    age: i32,
    time: f64,
    camera: Arc<Mutex<Camera>>,
    orbit_renderer: Arc<Mutex<Renderer>>,
    object_renderer: Arc<Mutex<Renderer>>,
    objects: Vec<Arc<Object>>,
    last_frame: Instant,
}

impl App {
    pub fn new(creation_context: &CreationContext) -> Self {
        let camera = Arc::new(Mutex::new(Camera::new()));
        let mut app = Self {
            name: "oh no".to_string(), 
            age: 0,
            time: 0.0,
            camera: camera.clone(),
            orbit_renderer:  Arc::new(Mutex::new(Renderer::new(creation_context.gl.as_ref().unwrap().clone(), camera.clone()))),
            object_renderer: Arc::new(Mutex::new(Renderer::new(creation_context.gl.as_ref().unwrap().clone(), camera.clone()))),
            objects: vec![],
            last_frame: Instant::now(),
        };
        app.init_objects();
        app
    }

    fn init_objects(&mut self) {
        let sun = Object::new("sun".to_string(), None, vec2(0.0, 0.0), vec2(0.0, 0.0), 1.9885e30, 6.957e8, Rgba::from_rgba_unmultiplied(1.0, 1.0, 0.3, 1.0));
        //let earth = Object::new("earth".to_string(), Some(sun.clone()), vec2(1.521e11, 0.0), vec2(0.0, 500.0), 5.9722e24, 6.378e6, Rgba::from_rgba_unmultiplied(0.1, 0.4, 1.0, 1.0));
        let earth = Object::new("earth".to_string(), Some(sun.clone()), vec2(1.521e11, 0.0), vec2(0.0, -2000.0), 5.9722e24, 6.378e6, Rgba::from_rgba_unmultiplied(0.1, 0.4, 1.0, 1.0));
        //let planet = Object::new("planet".to_string(), Some(sun.clone()), vec2(-1.521e11, 0.0), vec2(0.0, 2.909e4), 5.9722e25, 6.378e7, Rgba::from_rgba_unmultiplied(0.1, 0.4, 1.0, 1.0));
        let moon = Object::new("moon".to_string(), Some(earth.clone()), vec2(3.633e8, 0.0), vec2(0.0, -1.082e3), 7.346e22, 1.738e6, Rgba::from_rgba_unmultiplied(0.3, 0.3, 0.3, 1.0));
        let spacecraft1 = Object::new("spacecraft1".to_string(), Some(earth.clone()), vec2(0.0, 8.0e6), vec2(-0.989e4, 0.0), 1.0e3, 1.0e5, Rgba::from_rgba_unmultiplied(0.9, 0.3, 0.3, 1.0));
        //let spacecraft2 = Object::new("spacecraft2".to_string(), Some(moon.clone()), vec2(-49286929.74569702, -45453097.38960475), vec2(828.8125, 131.115), 1.0e3, 1.0e5, Rgba::from_rgba_unmultiplied(0.3, 0.3, 1.0, 1.0));
        self.camera.lock().unwrap().follow(spacecraft1.clone());
        self.objects.push(sun);
        self.objects.push(earth);
        //self.objects.push(planet);
        self.objects.push(moon);
        self.objects.push(spacecraft1);
        //self.objects.push(spacecraft2);
        do_full_trajectory_integration(&self.objects);

        // spacecraft -185.8, 2.37
        // moon -1016 -52

        // spacecraft -185.4 0.005
        // moon -1014 -131
    }

    fn render_underlay(&self, context: &Context, ui: &Ui) {
        let mut object_vertices = vec![];
        for object in &self.objects {
            object_vertices.extend(object.get_object_vertices());
        }
        self.object_renderer.lock().unwrap().set_vertices(object_vertices);

        let mut orbit_vertices = vec![];
        for object in &self.objects {
            orbit_vertices.extend(object.get_orbit_vertices(self.camera.lock().unwrap().get_zoom()));
        }
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
            self.orbit_renderer.lock().unwrap().update(context);
            self.object_renderer.lock().unwrap().update(context);
            self.render_underlay(context, ui);
            self.render_ui(ui);
            let delta_time = (Instant::now() - self.last_frame).as_secs_f64() * TIME_STEP;
            self.time += delta_time;
            self.objects.iter().for_each(|object| object.update(delta_time));
            self.last_frame = Instant::now();
        });            
    }
}