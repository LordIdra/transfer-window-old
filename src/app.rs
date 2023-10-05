use std::sync::{Arc, Mutex};

use eframe::{egui::{Context, CentralPanel, Slider, Ui}, epaint::{vec2, Rgba, PaintCallback}, Frame, CreationContext, egui_glow::CallbackFn};

use crate::{object::Object, camera::Camera, renderer::Renderer};

pub struct App {
    name: String,
    age: i32,
    camera: Arc<Camera>,
    renderer: Arc<Mutex<Renderer>>,
    objects: Vec<Object>,
}

impl App {
    pub fn new(creation_context: &CreationContext) -> Self {
        let camera = Arc::new(Camera::new());
        Self {
            name: "oh no".to_string(), 
            age: 0,
            camera: camera.clone(),
            renderer: Arc::new(Mutex::new(Renderer::new(creation_context.gl.as_ref().unwrap().clone(), camera))),
            objects: vec![
                Object::new(vec2(0.0, 0.0), 500.0, 500.0, Rgba::from_rgba_unmultiplied(0.3, 0.3, 0.3, 1.0)),
                Object::new(vec2(200.0, 200.0), 50.0, 80.0, Rgba::from_rgba_unmultiplied(0.8, 0.3, 0.3, 1.0)),
            ],
        }
    }

    fn render_underlay(&self, context: &Context, ui: &Ui) {
        let mut vertices = vec![];
        for object in &self.objects {
            vertices.extend(object.get_vertices());
        }
        self.renderer.lock().unwrap().set_vertices(vertices);

        let rect = context.screen_rect();
        let renderer = self.renderer.clone();
        let callback = Arc::new(CallbackFn::new(move |_info, _painter| {
            renderer.lock().unwrap().render(rect);
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
            self.render_underlay(context, ui);
            self.render_ui(ui);
        });            
    }
}