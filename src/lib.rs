pub mod errors;
pub mod events;
pub mod window;

pub trait VertexEngineApplication {
    fn on_update(&mut self);

    fn get_window(&mut self) -> &mut window::Window;

    fn is_running(&self) -> bool;

    fn on_event(&mut self, event: events::Event);
}

pub fn run_application(application: &mut impl VertexEngineApplication) {
    while application.is_running() {
        application.get_window().poll_events();
        application.get_window().handle_events(application.on_event);
        application.on_update();
    }
}
