#[derive(Debug)]
pub enum InitError {
    GlfwInitError(glfw::InitError),
}

impl From<glfw::InitError> for InitError {
    fn from(err: glfw::InitError) -> Self {
        Self::GlfwInitError(err)
    }
}
