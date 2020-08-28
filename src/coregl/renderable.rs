/// Anything that can be rendered
pub trait Renderable {
    /// The Graphics API used to render this
    type API: GraphicsAPI;
    /// Render self given the graphics API
    fn render(&mut self, api: &mut Self::API);
}

/// When something can be rendered in a batch.
///
/// Examples in opengl: vao
/// Multiple instances of the same vao appearing in
/// the world can be rendered at the same time, requiring
/// the VAO to be only bound and unbound once, instead of
/// for each object.
///
/// The PartialEq is required to determine whether 2 renderables
/// should be in the same batch or not.
pub trait BatchRenderable: PartialEq {
    /// The Graphics API used to render this
    type API: GraphicsAPI;
    /// Start by binding some resources
    fn batch_start(&self, api: &mut Self::API);
    /// Actually render all the items
    fn batch_render(&self, api: &mut Self::API);
    /// End by unbinding the resources bound in [`Self::batch_start`]
    fn batch_end(&self, api: &mut Self::API);
}
