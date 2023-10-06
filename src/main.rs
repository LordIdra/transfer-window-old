use app::App;
use eframe::{CreationContext, NativeOptions, Renderer, run_native};

mod app;
mod object;
mod renderer;

fn create_app(creation_context: &CreationContext<'_>) -> Box<dyn eframe::App> {
    Box::new(App::new(creation_context))
}

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        renderer: Renderer::Glow,
        multisampling: 4,
        ..Default::default()
    };
    
    run_native("Transfer Window", options, Box::new(create_app))
}
