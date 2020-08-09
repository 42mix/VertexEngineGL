#pragma once

#include "Core/Core.h"

#include "GL/GraphicsContext.h"

#include "VulkanGraphicsAPI.h"

namespace Vertex
{
    class VulkanContext : public GraphicsContext
    {
    public:
        VulkanContext(GLFWwindow* window);

        void SwapBuffers() override;

        void SetViewPort(uint32_t x, uint32_t y, uint32_t w, uint32_t h) const override {};

    public:
        struct QueueFamilyIndices
        {
            std::optional<uint32_t> graphics_family;
            std::optional<uint32_t> present_family;

            bool IsComplete() { return graphics_family.has_value() && present_family.has_value(); }
        };

        struct SwapChainSupportDetails
        {
            VkSurfaceCapabilitiesKHR        capabilities;
            std::vector<VkSurfaceFormatKHR> formats;
            std::vector<VkPresentModeKHR>   present_modes;
        };

    private:
        GLFWwindow* m_WindowHandle;

    private:
        void InitVulkan();

    private:
        void CreateInstance();

        bool CheckValidationLayerSupport();

        void CreateSurface();

        void CreateLogicalDevice();

        // ------------------------------------------------------
        // ------------------- Physical Device ------------------

        void PickPhysicalDevice();

        bool                    IsDeviceSuitable(VkPhysicalDevice device);
        QueueFamilyIndices      FindQueueFamilies(VkPhysicalDevice device);
        bool                    CheckDeviceExtensionSupport(VkPhysicalDevice device);
        SwapChainSupportDetails QuerySwapChainSupportDetails(VkPhysicalDevice device);

        // ------------------------------------------------------
        // --------------------- Swapchain ----------------------

        VkSurfaceFormatKHR ChooseSwapChainFormat(const std::vector<VkSurfaceFormatKHR>& available_formats);
        VkPresentModeKHR   ChooseSwapPresentMode(const std::vector<VkPresentModeKHR>& available_present_modes);
        VkExtent2D         ChooseSwapExtent(const VkSurfaceCapabilitiesKHR& capabilities);

        void CreateSwapChain();
        void CreateImageViews();
        void CleanupSwapchain();
        void RecreateSwapchain();

        // ----------------- Graphics Pipeline ------------------
        // ------------------------------------------------------

        void CreateRenderPass();
        void CreateDescriptorSetLayout();
        void CreateGraphicsPipelineLayout();

        // ------------------------------------------------------
        // ------------------------------------------------------

        void CreateFramebuffers();

        void CreateCommandPool();
        void CreateCommandBuffers();

        void CreateUniformBuffers();

        void InitDebugMessenger();

        std::vector<const char*> GetRequiredExtensions();

    private:
        VkInstance m_Instance;

        VkPhysicalDevice m_PhysicalDevice;
        VkDevice         m_LogicalDevice;

        VkSurfaceKHR m_Surface;

        VkQueue m_GraphicsQueue;
        VkQueue m_PresentQueue;

        // --------------------- Swapchain ----------------------

        VkSwapchainKHR             m_SwapChain;
        std::vector<VkImage>       m_SwapChainImages;
        VkFormat                   m_SwapChainImageFormat;
        VkExtent2D                 m_SwapChainExtent;
        std::vector<VkImageView>   m_SwapChainImageViews;
        std::vector<VkFramebuffer> m_SwapChainFramebuffers;

        // ---------------- Graphics Pipeline -------------------

        VkDescriptorSetLayout m_DescriptorSetLayout;
        VkPipelineLayout      m_PipelineLayout;
        VkRenderPass          m_RenderPass;

        // std::vector<std::reference_wrapper<VulkanShaderPipeline>> m_Pipelines;

        // ------------------------------------------------------

        VkCommandPool                m_CommandPool;
        VkCommandBuffer              m_LoadCommandBuffer;
        std::vector<VkCommandBuffer> m_CommandBuffers;

        std::vector<VkBuffer>       m_UniformBuffers;
        std::vector<VkDeviceMemory> m_UniformBuffersMemory;

        VkDebugUtilsMessengerEXT m_DebugMessenger;

        bool m_NeedSwapChainRecreate = false;

    private:
        // clang-format off
        // ---------------------- Helpers -----------------------

        uint32_t FindMemoryType(uint32_t type_filter, VkMemoryPropertyFlags properties);

        void CreateBuffer(VkDeviceSize          size,
                          VkBufferUsageFlags    usage,
                          VkMemoryPropertyFlags properties,
                          VkBuffer&             buffer,
                          VkDeviceMemory&       buffer_memory);

        void CopyBuffer(VkBuffer src_buffer, VkBuffer dst_buffer, VkDeviceSize size);

        // ------------------------------------------------------
        // clang-format on
    };
} // namespace Vertex