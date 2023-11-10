use state::State;
use eframe::{CreationContext, NativeOptions, Renderer, run_native};

mod camera;
mod components;
mod storage;
mod state;
mod rendering;
mod resources;
mod systems;
mod util;

fn create_app(creation_context: &CreationContext<'_>) -> Box<dyn eframe::App> {
    Box::new(State::new(creation_context))
}

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions {
        renderer: Renderer::Glow,
        multisampling: 4,
        ..Default::default()
    };
    
    run_native("Transfer Window", options, Box::new(create_app))
}
