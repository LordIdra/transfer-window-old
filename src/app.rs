use std::{sync::{Arc, Mutex}, cell::RefCell, rc::Rc, time::Instant};

use eframe::{egui::{Context, CentralPanel, Slider, Ui}, epaint::{Rgba, PaintCallback}, Frame, CreationContext, egui_glow::CallbackFn};
use nalgebra_glm::vec2;

use crate::{object::Object, renderer::Renderer, camera::Camera};

const TIME_STEP: f32 = 86400.0 * 365.0 / 20.0;

pub struct App {
    name: String,
    age: i32,
    time: f32,
    camera: Arc<Mutex<Camera>>,
    orbit_renderer: Arc<Mutex<Renderer>>,
    object_renderer: Arc<Mutex<Renderer>>,
    objects: Vec<Rc<RefCell<Object>>>,
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
        let sun = Object::new(None, vec2(0.0, 0.0), vec2(0.0, 0.0), 1.9885e30, 6.957e8, Rgba::from_rgba_unmultiplied(1.0, 1.0, 0.3, 1.0));
        //let earth1 = Object::new(Some(sun.clone()), vec2(1.521e11, 0.0), vec2(0.0, 1.929e4), 5.9722e24, 6.378e6, Rgba::from_rgba_unmultiplied(0.1, 0.4, 1.0, 1.0));
        //let earth2 = Object::new(Some(sun.clone()), vec2(-1.521e11, 0.0), vec2(0.0, 1.929e4), 5.9722e24, 6.378e6, Rgba::from_rgba_unmultiplied(0.1, 0.4, 1.0, 1.0));
        let planet = Object::new(None, vec2(1.0e11, 1.0e11), vec2(0.0, -1.929e4), 5.9722e24, 6.378e6, Rgba::from_rgba_unmultiplied(0.1, 0.4, 1.0, 1.0));
        //let moon1 = Object::new(Some(earth1.clone()), vec2(3.633e8, 0.0), vec2(0.0, -1.082e3), 7.346e22, 1.738e6, Rgba::from_rgba_unmultiplied(0.3, 0.3, 0.3, 1.0));
        //let moon2 = Object::new(Some(earth2.clone()), vec2(3.633e8, 0.0), vec2(0.0, -1.082e3), 7.346e22, 1.738e6, Rgba::from_rgba_unmultiplied(0.3, 0.3, 0.3, 1.0));
        let moon3 = Object::new(Some(planet.clone()), vec2(3.633e8, 0.0), vec2(0.0, -1.082e3), 7.346e22, 1.738e7, Rgba::from_rgba_unmultiplied(0.3, 0.3, 0.3, 1.0));
        self.objects.push(sun);
        //self.objects.push(earth1);
        //self.objects.push(earth2);
        self.objects.push(planet);
        //self.objects.push(moon1);
        //self.objects.push(moon2);
        self.objects.push(moon3);
    }

    fn render_underlay(&self, context: &Context, ui: &Ui) {
        let mut object_vertices = vec![];
        for object in &self.objects {
            object_vertices.extend(object.borrow().get_object_vertices());
        }
        self.object_renderer.lock().unwrap().set_vertices(object_vertices);

        let mut orbit_vertices = vec![];
        for object in &self.objects {
            orbit_vertices.extend(object.borrow().get_orbit_vertices(self.camera.lock().unwrap().get_zoom()));
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
            let delta_time = (Instant::now() - self.last_frame).as_secs_f32() * TIME_STEP;
            self.time += delta_time;
            self.objects.iter().for_each(|object| object.borrow_mut().update(delta_time));
            self.last_frame = Instant::now();
        });            
    }
}