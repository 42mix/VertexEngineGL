use crate::events::Event;

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
    internal_window: glfw::Window,
}

impl Window {
    pub fn new(properties: WindowProperties) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ClientApi(
            glfw::ClientApiHint::NoApi,
        ));

        let (mut window, _) = glfw
            .create_window(
                300,
                300,
                "Hello this is window",
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        Self {
            internal_window: window,
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
