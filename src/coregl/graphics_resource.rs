/// A graphics resource
///
/// Needs to implement the `Drop` trait, cleaning up any resources it allocated,
/// for example via the `glDelete*` functions in OpenGL.
pub trait GraphicsResource: Drop {}
