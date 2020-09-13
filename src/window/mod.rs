use crate::errors::InitError;
use crate::events::event_types::{self, Event};

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
    pub fn window_close_requested(&self) -> bool {
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
                glfw::WindowEvent::Key(key, scan_code, action, modifiers) => {
                    match action {
                        glfw::Action::Press => {
                            events.push(Event::KeyPressEvent(
                                event_types::KeyPressContainer {
                                    key,
                                    count: 0,
                                },
                            ))
                        }
                        glfw::Action::Release => {
                            events.push(Event::KeyReleaseEvent(key))
                        }
                        _ => {}
                    }
                }
                glfw::WindowEvent::Char(c) => {
                    events.push(Event::KeyCharInputEvent(c))
                }
                glfw::WindowEvent::MouseButton(button, action, modifiers) => {
                    match action {
                        glfw::Action::Press => {
                            events.push(Event::MouseClickEvent(button))
                        }
                        glfw::Action::Release => {
                            events.push(Event::MouseReleaseEvent(button))
                        }
                        _ => {}
                    }
                }
                glfw::WindowEvent::Scroll(offset_x, offset_y) => {
                    events.push(Event::MouseScrollEvent(
                        event_types::MouseScrollContainer {
                            offset_x,
                            offset_y,
                        },
                    ))
                }
                glfw::WindowEvent::CursorPos(pos_x, pos_y) => {
                    events.push(Event::MouseMoveEvent(
                        event_types::CursorPositionContainer { pos_x, pos_y },
                    ))
                }
                glfw::WindowEvent::Size(width, height) => {
                    events.push(Event::WindowResizeEvent(
                        event_types::WindowSizeContainer { width, height },
                    ))
                }
                glfw::WindowEvent::Close => {
                    events.push(Event::WindowCloseEvent)
                }
                glfw::WindowEvent::Focus(state) => {
                    if state == true {
                        events.push(Event::WindowGainedFocusEvent);
                    } else {
                        events.push(Event::WindowLostFocusEvent);
                    }
                }
                glfw::WindowEvent::CursorEnter(state) => {
                    if state == true {
                        events.push(Event::WindowCursorEnteredEvent);
                    } else {
                        events.push(Event::WindowCursorLeftEvent);
                    }
                }
                _ => {}
            }
        }
        events
    }
}

/// The window mode to use for creating the window
pub enum WinMode {
    /// As a "windowed" window
    Windowed,
    /// As a fullscreen window
    Fullscreen,
}
