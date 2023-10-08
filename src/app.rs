use std::{sync::{Arc, Mutex}, cell::RefCell, rc::Rc};

use eframe::{egui::{Context, CentralPanel, Slider, Ui}, epaint::{vec2, Rgba, PaintCallback}, Frame, CreationContext, egui_glow::CallbackFn};

use crate::{object::Object, renderer::Renderer, camera::Camera};

pub struct App {
    name: String,
    age: i32,
    camera: Arc<Mutex<Camera>>,
    orbit_renderer: Arc<Mutex<Renderer>>,
    object_renderer: Arc<Mutex<Renderer>>,
    objects: Vec<Rc<RefCell<Object>>>,
}

impl App {
    pub fn new(creation_context: &CreationContext) -> Self {
        let camera = Arc::new(Mutex::new(Camera::new()));
        let sun = Object::new(None, vec2(0.0, 0.0), vec2(0.0, 0.0), 1.9885e30, 6.957e8, Rgba::from_rgba_unmultiplied(1.0, 1.0, 0.3, 1.0));
        let earth = Object::new(Some(sun.clone()), vec2(1.521e11, 0.0), vec2(0.0, 2.929e4), 5.9722e24, 6.378e6, Rgba::from_rgba_unmultiplied(0.1, 0.4, 1.0, 1.0));
        let moon = Object::new(Some(earth.clone()), vec2(3.633e8, 0.0), vec2(0.0, -1.082e3), 7.346e22, 1.738e6, Rgba::from_rgba_unmultiplied(0.3, 0.3, 0.3, 1.0));
        Self {
            name: "oh no".to_string(), 
            age: 0,
            camera: camera.clone(),
            orbit_renderer:  Arc::new(Mutex::new(Renderer::new(creation_context.gl.as_ref().unwrap().clone(), camera.clone()))),
            object_renderer: Arc::new(Mutex::new(Renderer::new(creation_context.gl.as_ref().unwrap().clone(), camera.clone()))),
            objects: vec![sun, earth, moon],
        }
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
        });            
    }
}