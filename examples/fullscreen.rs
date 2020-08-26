use glfw::{Action, Key};
use vertex_engine::core::init;
use vertex_engine::windowing::{WinMode, WindowProperties};

fn main() {
    let mut ctx =
        init(WindowProperties::new(500, 500, "123", WinMode::Fullscreen))
            .unwrap();

    while !ctx.window_close_requested() {
        ctx.swap_buffers();
        ctx.poll_events();
        ctx.handle_events(|window, (_, event)| match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        })
    }
}
