use crate::errors::InitError;
use crate::events::{Event, EventTypes};

use std::sync::mpsc::Receiver;

use glfw::Context;

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

    /// Returns whether the user requested the window to close(for example clicking on the X button).
    /// # Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #     use vertex_engine::prelude::*;
    ///     use vertex_engine::{
    ///         windowing::{WindowProperties, WinMode},
    ///         core::init
    ///     };
    /// let mut context = init(WindowProperties::new(1024, 720, "title here", WinMode::Windowed))?;
    ///     while !context.window_close_requested() {
    ///         context.poll_events();
    ///     }
    /// #     Ok(())
    /// # }
    /// ```
    pub fn window_close_requested(&self) -> bool {
        self.internal_window.should_close()
    }

    /// Swaps the window buffers. This function should be called every frame.
    /// # Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #     use vertex_engine::prelude::*;
    ///     use vertex_engine::{
    ///         windowing::{WindowProperties, WinMode},
    ///         core::init
    ///     };
    /// #     let mut context = init(WindowProperties::new(1024, 720, "title here", WinMode::Windowed))?;
    ///     while !context.window_close_requested() {
    ///         context.poll_events();
    ///         context.swap_buffers();
    ///     }
    /// #     Ok(())
    /// # }
    /// ```
    pub fn swap_buffers(&mut self) {
        self.internal_window.swap_buffers()
    }

    /// Polls the events from glfw.
    /// # Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #     use vertex_engine::prelude::*;
    ///     use vertex_engine::{
    ///         windowing::{WindowProperties, WinMode},
    ///         core::init
    ///     };
    /// #     let mut context = init(WindowProperties::new(1024, 720, "title here", WinMode::Windowed))?;
    ///     while !context.window_close_requested() {
    ///         context.poll_events();
    ///     }
    /// #     Ok(())
    /// # }
    /// ```
    pub fn poll_events(&mut self) {
        self.glfw.poll_events();
    }

    /// Handles all events received on the window, by calling `handler` on each of them.
    /// # Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// #     use vertex_engine::prelude::*;
    ///     use vertex_engine::{
    ///         windowing::{WindowProperties, WinMode},
    ///         core::init
    ///     };
    ///     use glfw::{Key, Action};
    /// #     let mut context = init(WindowProperties::new(1024, 720, "title here", WinMode::Windowed))?;
    ///     while !context.window_close_requested() {
    ///         context.poll_events();
    ///         context.handle_events(|window, (_, event)| {
    ///             match event {
    ///                 glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
    ///                     window.set_should_close(true)
    ///                 }
    ///                 _ => {}
    ///             }
    ///         });
    ///         context.swap_buffers();
    ///     }
    /// #     Ok(())
    /// # }
    /// ```
    pub fn handle_events<F>(&mut self, mut handler: F)
    where
        F: FnMut(Event),
    {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(key, scan_code, action, modifiers) => {
                    match action {
                        glfw::Action::Press => handler(Event::KeyPressEvent(
                            EventTypes::KeyPressContainer { key, count: 0 },
                        )),
                        glfw::Action::Release => {
                            handler(Event::KeyReleaseEvent(key))
                        }
                        _ => {}
                    }
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
