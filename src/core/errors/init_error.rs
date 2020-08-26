#[derive(Debug)]
pub enum InitError {
    GlfwInitError(glfw::InitError),
    WindowCreationFailed,
}

impl From<glfw::InitError> for InitError {
    fn from(err: glfw::InitError) -> Self {
        Self::GlfwInitError(err)
    }
}
