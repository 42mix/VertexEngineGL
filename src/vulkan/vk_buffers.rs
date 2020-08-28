/// A vertex buffer
pub struct VulkanVertexBuffer {}

impl Drop for VulkanVertexBuffer {
    fn drop(&mut self) {}
}

impl GraphicsResource for VulkanVertexBuffer {}

impl GraphicsBuffer for VulkanVertexBuffer {}

/// A index buffer
pub struct VulkanIndexBuffer {}

impl Drop for VulkanIndexBuffer {
    fn drop(&mut self) {}
}

impl GraphicsResource for VulkanIndexBuffer {}

impl GraphicsBuffer for VulkanIndexBuffer {}
