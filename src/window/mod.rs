use crate::events::Event;

use std::sync::mpsc::Receiver;

/// The properties to open the window with.
pub struct WindowProperties {
    width: u32,
    height: u32,

    title: String,
    win_mode: WinMode,
}

impl WindowProperties {
    /// Create a WindowProperties object, given the fields
    pub fn new(
        width: u32,
        height: u32,
        title: &str,
        win_mode: WinMode,
    ) -> Self {
        Self {
            width,
            height,
            title: String::from(title),
            win_mode,
        }
    }
}

pub struct Window {
    glfw: glfw::Glfw,
    events: Receiver<(f64, glfw::WindowEvent)>,
    internal_window: glfw::Window,
}

impl Window {
    pub fn new(properties: WindowProperties) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ClientApi(
            glfw::ClientApiHint::NoApi,
        ));

        let (mut window, events) = glfw
            .create_window(
                300,
                300,
                "Hello this is window",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        window.set_all_polling(true);

        Self {
            glfw,
            events,
            internal_window: window,
        }
    }

    // Events and callbacks

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            // println!("{:?}", event);
            match event {
                glfw::WindowEvent::Close => {
                    //
                }
                _ => {}
            }
        }
    }
}

/// The window mode to use for creating the window
pub enum WinMode {
    /// As a "windowed" window
    Windowed,
    /// As a fullscreen window
    Fullscreen,
}
