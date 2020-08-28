/// A vulkan context, holding the instance,
/// holding the instance, physical and logical devices,
/// among a few other things.
pub struct VulkanContext {
    instance: Instance,
}

impl GraphicsContext for VulkanContext {}
