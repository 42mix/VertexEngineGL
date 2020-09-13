#[derive(Debug)]
pub struct KeyPressContainer {
    pub key: glfw::Key,
    pub count: u32,
}

#[derive(Debug)]
pub struct MouseScrollContainer {
    pub offset_x: f64,
    pub offset_y: f64,
}

#[derive(Debug)]
pub struct CursorPositionContainer {
    pub pos_x: f64,
    pub pos_y: f64,
}

#[derive(Debug)]
pub struct WindowSizeContainer {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug)]
pub enum Event {
    KeyPressEvent(KeyPressContainer),
    KeyReleaseEvent(glfw::Key),
    KeyCharInputEvent(char),

    MouseClickEvent(glfw::MouseButton),
    MouseReleaseEvent(glfw::MouseButton),
    MouseScrollEvent(MouseScrollContainer),
    MouseMoveEvent(CursorPositionContainer),

    WindowResizeEvent(WindowSizeContainer),
    WindowCloseEvent,
    WindowGainedFocusEvent,
    WindowLostFocusEvent,
    WindowCursorLeftEvent,
    WindowCursorEnteredEvent,
}
