/// The graphics api
pub trait GraphicsAPI {
    /// The context
    type Context: GraphicsContext;

    fn get_ctx(&self) -> &Self::Context;
    fn get_ctx_mut(&mut self) -> &mut Self::Context;
}
