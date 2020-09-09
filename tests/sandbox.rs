use vertex_engine::events::Event;
use vertex_engine::window::{WinMode, Window, WindowProperties};

struct Application {
    window: vertex_engine::window::Window,
}

impl Application {
    fn new() -> Self {
        Application {
            window: Window::new(WindowProperties::new(
                1024,
                728,
                "SandBox",
                WinMode::Windowed,
            ))
            .unwrap(),
        }
    }
}

impl vertex_engine::VertexEngineApplication for Application {
    fn on_update(&mut self) {
        assert!(true);
    }

    fn get_window(&mut self) -> &mut Window {
        &mut self.window
    }

    fn is_running(&self) -> bool {
        true
    }

    fn on_event(&mut self, event: Event) {
        println!("Event: {:?}", event);
    }
}

#[test]
fn main_test() {
    vertex_engine::run_application(&mut Application::new());
}
