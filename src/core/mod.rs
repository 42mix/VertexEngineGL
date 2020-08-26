//! Module related to the core of the game engine, specifically
//! [the Core folder of the original VertexEngine](https://github.com/VertexEngine/VertexEngine/tree/master/Vertex/src/Core)
pub mod errors;

use crate::core::errors::InitError;
use crate::windowing::{WinMode, WindowProperties};
use glfw::{Context, Window, WindowEvent, WindowMode};
use std::sync::mpsc::Receiver;

/// The context used; this is perhaps the most important structure.
pub struct VxCtx {
    glfw_ctx: glfw::Glfw,
    window: glfw::Window,
    window_events: Receiver<(f64, WindowEvent)>,
}

impl VxCtx {
    /// Creates a new context from a raw glfw context,
    /// a raw glfw window and the glfw window events receiver.
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
        self.window.should_close()
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
        self.window.swap_buffers()
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
        self.glfw_ctx.poll_events();
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
        F: FnMut(&mut Window, (f64, WindowEvent)),
    {
        for event in glfw::flush_messages(&self.window_events) {
            handler(&mut self.window, event);
        }
    }
}

/// Initializes the engine and returns a context on success, on a new glfw window,
/// the properties of which come from the `win_properties` parameter
///
/// # Examples
/// Create a context with a 1024x720 window:
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// #     use vertex_engine::prelude::*;    
///     use vertex_engine::{
///         windowing::{WindowProperties, WinMode},
///         core::init
///     };
///     let ctx = init(WindowProperties::new(1024, 720, "title here", WinMode::Windowed))?;
/// #     Ok(())
/// # }
/// ```
///
/// Create a context with a fullscreen window, at 1600x1024:
/// ```no_run
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// #     use vertex_engine::prelude::*;    
///     use vertex_engine::{
///         windowing::{WindowProperties, WinMode},
///         core::init
///     };
///     let ctx = init(WindowProperties::new(1600, 1024, "title here", WinMode::Fullscreen))?;
/// #     Ok(())
/// # }
/// ```
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
        WinMode::Fullscreen => {
            glfw.with_primary_monitor(|param_glfw, monitor| {
                param_glfw.create_window(
                    win_properties.width,
                    win_properties.height,
                    win_properties.title,
                    WindowMode::FullScreen(monitor.unwrap()),
                )
            })
        }
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
