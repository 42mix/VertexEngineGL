use crate::graphics::errors::WindowInitError;
use crate::graphics::events::{self, Event};

use std::sync::mpsc::Receiver;

use glfw::Context;

/// The window mode to use for creating the window.
pub enum Mode {
    /// As a "windowed" window
    Windowed,
    /// As a fullscreen window
    Fullscreen,
}

/// The properties to open the window with.
pub struct Properties {
    width: u32,
    height: u32,

    title: String,
    win_mode: Mode,
}

impl Properties {
    /// Create a `Properties` object, given the fields.
    pub fn new(width: u32, height: u32, title: &str, win_mode: Mode) -> Self {
        Self {
            width,
            height,
            title: String::from(title),
            win_mode,
        }
    }
}

/// A struct containing all details and methods relevant to a window.
pub struct Window {
    glfw: glfw::Glfw,
    events: Receiver<(f64, glfw::WindowEvent)>,
    internal_window: glfw::Window,
}

impl Window {
    /// Create a new window with the given properties.
    ///
    /// # Errors
    /// Returns a `crate::graphics::errors::WindowInitError` if glfw initialization or window creation fails.
    pub fn new(properties: &Properties) -> Result<Self, WindowInitError> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ClientApi(
            glfw::ClientApiHint::NoApi,
        ));

        // Create a windowed mode window and its OpenGL context
        let result = match properties.win_mode {
            Mode::Windowed => glfw.create_window(
                properties.width,
                properties.height,
                &properties.title[..],
                glfw::WindowMode::Windowed,
            ),
            Mode::Fullscreen => {
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
            None => return Err(WindowInitError::WindowCreationFailed),
        };

        window.set_all_polling(true);

        Ok(Self {
            glfw,
            events,
            internal_window: window,
        })
    }

    /// Returns whether the user requested the window to close(for example clicking on the X button).
    pub fn should_close(&self) -> bool {
        self.internal_window.should_close()
    }

    /// Swaps the window buffers. This function should be called every frame.
    pub fn swap_buffers(&mut self) {
        self.internal_window.swap_buffers()
    }

    /// Polls and returns events that occurred since the last poll from glfw.
    pub fn flush_events(&mut self) -> Vec<Event> {
        self.glfw.poll_events();
        let mut events = Vec::new();
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Key(key, _, action, _) => match action {
                    glfw::Action::Press => events.push(Event::KeyPress(
                        events::KeyPressContainer { key, count: 0 },
                    )),
                    glfw::Action::Release => {
                        events.push(Event::KeyRelease(key))
                    }
                    glfw::Action::Repeat => {}
                },
                glfw::WindowEvent::Char(c) => {
                    events.push(Event::KeyCharInput(c))
                }
                glfw::WindowEvent::MouseButton(button, action, _) => {
                    match action {
                        glfw::Action::Press => {
                            events.push(Event::MouseClick(button))
                        }
                        glfw::Action::Release => {
                            events.push(Event::MouseRelease(button))
                        }
                        glfw::Action::Repeat => {}
                    }
                }
                glfw::WindowEvent::Scroll(offset_x, offset_y) => {
                    events.push(Event::MouseScroll(
                        events::MouseScrollContainer { offset_x, offset_y },
                    ))
                }
                glfw::WindowEvent::CursorPos(pos_x, pos_y) => {
                    events.push(Event::MouseMove(
                        events::CursorPositionContainer { pos_x, pos_y },
                    ))
                }
                glfw::WindowEvent::Size(width, height) => {
                    events.push(Event::WindowResize(
                        events::WindowSizeContainer { width, height },
                    ))
                }
                glfw::WindowEvent::Close => events.push(Event::WindowClose),
                glfw::WindowEvent::Focus(state) => {
                    if state {
                        events.push(Event::WindowGainedFocus);
                    } else {
                        events.push(Event::WindowLostFocus);
                    }
                }
                glfw::WindowEvent::CursorEnter(state) => {
                    if state {
                        events.push(Event::WindowCursorEntered);
                    } else {
                        events.push(Event::WindowCursorLeft);
                    }
                }
                _ => {}
            }
        }
        events
    }
}
