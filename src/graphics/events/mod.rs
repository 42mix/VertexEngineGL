//! The events that can be registered by a `glfw::Window`.

/// A struct containing details of a key press.
#[derive(Debug)]
pub struct KeyPressContainer {
    /// Represents the key that was pressed. Variant of `glfw::Key`.
    pub key: glfw::Key,
    /// Number of times the key was pressed.
    pub count: u32,
}

/// A struct containing details of a mouse scroll.
#[derive(Debug)]
pub struct MouseScrollContainer {
    /// Increase in x offset.
    pub offset_x: f64,
    /// Increase in y offset.
    pub offset_y: f64,
}

/// A struct containing details of the cursor's position relative to a window.
#[derive(Debug)]
pub struct CursorPositionContainer {
    /// Horizontal position of the cursor.
    pub pos_x: f64,
    /// Vertical position of the cursor.
    pub pos_y: f64,
}

/// A struct containing details of a window's dimensions.
#[derive(Debug)]
pub struct WindowSizeContainer {
    /// Horizontal dimensions of the window.
    pub width: i32,
    /// Vertical dimensions of the window.
    pub height: i32,
}

/// Possible window events that can occur.
#[derive(Debug)]
pub enum Event {
    KeyPress(KeyPressContainer),
    KeyRelease(glfw::Key),
    /// A sequence of characters was input.
    KeyCharInput(char),

    MouseClick(glfw::MouseButton),
    MouseRelease(glfw::MouseButton),
    MouseScroll(MouseScrollContainer),
    MouseMove(CursorPositionContainer),

    WindowResize(WindowSizeContainer),
    WindowClose,
    WindowGainedFocus,
    WindowLostFocus,
    WindowCursorLeft,
    WindowCursorEntered,
}
