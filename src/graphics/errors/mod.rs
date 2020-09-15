//! The errors that can occur due to actions in the `graphics` module.
use std::error::Error;
use std::fmt::{Display, Formatter};

/// The errors that can happen during creation of a window.
#[derive(Debug)]
pub enum WindowInitError {
    /// When [glfw::init] failed, with the [glfw::InitError] that caused it.
    GlfwInitError(glfw::InitError),
    /// When [glfw::Glfw::create_window] failed.
    WindowCreationFailed,
}

impl From<glfw::InitError> for WindowInitError {
    fn from(err: glfw::InitError) -> Self {
        Self::GlfwInitError(err)
    }
}

impl Display for WindowInitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for WindowInitError {}
