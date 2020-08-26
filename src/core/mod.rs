pub mod errors;

use crate::core::errors::InitError;
use crate::windowing::{WinMode, WindowProperties};
use glfw::{Context, WindowEvent, WindowMode};
use std::sync::mpsc::Receiver;

pub struct VxCtx {
    glfw_ctx: glfw::Glfw,
    window: glfw::Window,
    window_events: Receiver<(f64, WindowEvent)>,
}

impl VxCtx {
    pub fn new(
        glfw_ctx: glfw::Glfw,
        window: glfw::Window,
        window_events: Receiver<(f64, WindowEvent)>,
    ) -> Self {
        Self {
            glfw_ctx,
            window,
            window_events,
        }
    }

    pub fn window_close_requested(&self) -> bool {
        self.window.should_close()
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers()
    }

    pub fn poll_events(&mut self) {
        self.glfw_ctx.poll_events();
    }
}

pub fn init(win_properties: WindowProperties) -> Result<VxCtx, InitError> {
    let mut glfw = glfw::init::<()>(None)?;

    // Create a windowed mode window and its OpenGL context
    let result = match win_properties.win_mode {
        WinMode::Windowed => glfw.create_window(
            win_properties.width,
            win_properties.height,
            win_properties.title,
            WindowMode::Windowed,
        ),
        WinMode::Fullscreen => glfw.with_primary_monitor(|param_glfw, monitor| {
            param_glfw.create_window(
                win_properties.width,
                win_properties.height,
                win_properties.title,
                WindowMode::FullScreen(monitor.unwrap()),
            )
        }),
    };
    let (mut window, events) = match result {
        Some(v) => v,
        None => return Err(InitError::WindowCreationFailed),
    };

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    Ok(VxCtx::new(glfw, window, events))
}
