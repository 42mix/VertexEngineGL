use vertex_engine::events::event_types::Event;
use vertex_engine::window::{WinMode, Window, WindowProperties};

struct Application {
    window: vertex_engine::window::Window,
    running: bool,
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
            running: true,
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
        self.running
    }

    fn on_event(&mut self, event: Event) {
        println!("Event: {:?}", event);

        match event {
            Event::WindowCloseEvent => self.running = false,
            _ => {}
        }
    }
}

#[test]
fn main_test() {
    vertex_engine::run_application(&mut Application::new());
}
