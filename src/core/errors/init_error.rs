/// The errors that can happen during [`crate::init`]
#[derive(Debug)]
pub enum InitError {
    /// When [`glfw::init`] failed, with the [`glfw::InitError`] that caused it
    GlfwInitError(glfw::InitError),
    /// When [`glfw::Glfw::create_window`] failed
    WindowCreationFailed,
}

impl From<glfw::InitError> for InitError {
    fn from(err: glfw::InitError) -> Self {
        Self::GlfwInitError(err)
    }
}

impl Display for InitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for InitError {}
