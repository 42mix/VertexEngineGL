use vertex_engine::graphics::events::Event;
use vertex_engine::graphics::window::{self, Window};

struct Application {
    window: Window,
    running: bool,
}

impl Application {
    fn new() -> Self {
        Application {
            window: Window::new(&window::Properties::new(
                1024,
                728,
                "SandBox",
                window::Mode::Windowed,
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
            Event::WindowClose => self.running = false,
            _ => {}
        }
    }
}

fn main() {
    vertex_engine::run_application(&mut Application::new());
}
