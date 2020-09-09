use crate::errors::InitError;
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
    pub fn new(properties: WindowProperties) -> Result<Self, InitError> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ClientApi(
            glfw::ClientApiHint::NoApi,
        ));

        // Create a windowed mode window and its OpenGL context
        let result = match properties.win_mode {
            WinMode::Windowed => glfw.create_window(
                properties.width,
                properties.height,
                &properties.title[..],
                glfw::WindowMode::Windowed,
            ),
            WinMode::Fullscreen => {
                glfw.with_primary_monitor(|param_glfw, monitor| {
                    param_glfw.create_window(
                        properties.width,
                        properties.height,
                        &properties.title[..],
                        glfw::WindowMode::FullScreen(monitor.unwrap()),
                    )
                })
            }
        };
        let (mut window, events) = match result {
            Some(v) => v,
            None => return Err(InitError::WindowCreationFailed),
        };

        window.set_all_polling(true);

        Ok(Self {
            glfw,
            events,
            internal_window: window,
        })
    }

    // Events and callbacks

    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }
}

/// The window mode to use for creating the window
pub enum WinMode {
    /// As a "windowed" window
    Windowed,
    /// As a fullscreen window
    Fullscreen,
}
