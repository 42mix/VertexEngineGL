pub mod events;
pub mod window;

pub trait VertexEngineApplication {
    fn on_update(&mut self);

    fn get_window(&mut self) -> &mut window::Window;

    fn is_running(&self) -> bool;
}

pub fn run_application(application: &mut impl VertexEngineApplication) {
    while application.is_running() {
        application.get_window().poll_events();
        application.on_update();
    }
}
