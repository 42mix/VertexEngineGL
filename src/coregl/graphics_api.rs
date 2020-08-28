/// The graphics api
pub trait GraphicsAPI {
    /// The context
    type Context: GraphicsContext;

    /// Get the context
    fn get_ctx(&self) -> &Self::Context;
    /// Get the context mutably
    fn get_ctx_mut(&mut self) -> &mut Self::Context;
}
