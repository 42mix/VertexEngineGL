pub mod EventTypes {
    pub struct KeyPressContainer {
        pub key: glfw::Key,
        pub count: u32,
    }

    pub struct MouseScrollContainer {
        pub offset_x: f64,
        pub offset_y: f64,
    }

    pub struct CursorPositionContainer {
        pub pos_x: f64,
        pub pos_y: f64,
    }

    pub struct WindowDimensionsContainer {
        pub width: i32,
        pub height: i32,
    }
}

pub enum Event {
    KeyPressEvent(EventTypes::KeyPressContainer),
    KeyReleaseEvent(glfw::Key),
    KeyCharInputEvent(u32),

    MouseClickEvent(glfw::MouseButton),
    MouseReleaseEvent(glfw::MouseButton),
    MouseScrollEvent(EventTypes::MouseScrollContainer),
    MouseMoveEvent(EventTypes::CursorPositionContainer),

    WindowResizeEvent(EventTypes::WindowDimensionsContainer),
    WindowCloseEvent,
    WindowGainedFocusEvent,
    WindowLostFocusEvent,
    WindowCursorLeftEvent,
    WindowCursorEnteredEvent,
}