#![crate_name = "vertex_engine"]
#![crate_type = "lib"]
// #![deny(missing_docs)]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/VertexEngine/VertexEngine/master/assets/VertexEngine.png",
    html_logo_url = "https://raw.githubusercontent.com/VertexEngine/VertexEngine/master/assets/VertexEngine.png"
)]

//! A rust reimplementation of the [VertexEngine](https://github.com/VertexEngine/VertexEngine) game engine.

extern crate glfw;
extern crate vulkano;

pub mod graphics;

pub trait VertexEngineApplication {
    fn on_update(&mut self);

    fn get_window(&mut self) -> &mut graphics::window::Window;

    fn is_running(&self) -> bool;

    fn on_event(&mut self, event: graphics::events::Event);
}

pub fn run_application(application: &mut impl VertexEngineApplication) {
    while application.is_running() {
        for event in application.get_window().flush_events() {
            application.on_event(event);
        }
        application.on_update();
    }
}
