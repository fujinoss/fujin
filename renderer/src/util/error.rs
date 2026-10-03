use thiserror::Error;

#[derive(Debug, Error)]
pub enum ArashiError {
    #[error("Vulkan loader not found")]
    LoaderNotFound,

    #[error("Vulkan entry point '{0}' failed to load")]
    EntryLoad(&'static str),

    #[error("Vulkan instance creation failed: {0:?}")]
    InstanceCreation(ash::vk::Result),

    #[error("Vulkan extension missing: {0}")]
    MissingExtension(String),

    #[error("Vulkan layer missing: {0}")]
    MissingLayer(String),

    #[error("No suitable physical device found")]
    NoPhysicalDevice,

    #[error("Physical device enumeration failed: {0:?}")]
    PhysicalDeviceEnum(ash::vk::Result),

    #[error("No suitable queue family")]
    NoQueueFamily,

    #[error("Device creation failed: {0:?}")]
    DeviceCreation(ash::vk::Result),

    #[error("Surface creation failed: {0}")]
    SurfaceCreation(String),

    #[error("Surface query failed: {0:?}")]
    SurfaceQuery(ash::vk::Result),

    #[error("Swapchain creation failed: {0:?}")]
    SwapchainCreation(ash::vk::Result),

    #[error("Swapchain out of date")]
    SwapchainOutOfDate,

    #[error("Swapchain suboptimal")]
    SwapchainSuboptimal,

    #[error("Swapchain has no images")]
    SwapchainEmpty,

    #[error("Pipeline creation failed: {0:?}")]
    PipelineCreation(ash::vk::Result),

    #[error("Pipeline layout creation failed: {0:?}")]
    PipelineLayoutCreation(ash::vk::Result),

    #[error("Render pass creation failed: {0:?}")]
    RenderPassCreation(ash::vk::Result),

    #[error("Framebuffer creation failed: {0:?}")]
    FramebufferCreation(ash::vk::Result),

    #[error("Shader module creation failed: {0:?}")]
    ShaderModuleCreation(ash::vk::Result),

    #[error("Descriptor set layout creation failed: {0:?}")]
    DescriptorSetLayoutCreation(ash::vk::Result),

    #[error("Descriptor pool creation failed: {0:?}")]
    DescriptorPoolCreation(ash::vk::Result),

    #[error("Descriptor set allocation failed: {0:?}")]
    DescriptorSetAllocation(ash::vk::Result),

    #[error("Command pool creation failed: {0:?}")]
    CommandPoolCreation(ash::vk::Result),

    #[error("Command buffer allocation failed: {0:?}")]
    CommandBufferAllocation(ash::vk::Result),

    #[error("Semaphore creation failed: {0:?}")]
    SemaphoreCreation(ash::vk::Result),

    #[error("Fence creation failed: {0:?}")]
    FenceCreation(ash::vk::Result),

    #[error("Image creation failed: {0:?}")]
    ImageCreation(ash::vk::Result),

    #[error("Image view creation failed: {0:?}")]
    ImageViewCreation(ash::vk::Result),

    #[error("Sampler creation failed: {0:?}")]
    SamplerCreation(ash::vk::Result),

    #[error("Buffer creation failed: {0:?}")]
    BufferCreation(ash::vk::Result),

    #[error("Memory allocation failed: {0}")]
    MemoryAllocation(String),

    #[error("Image acquisition failed: {0:?}")]
    AcquireImage(ash::vk::Result),

    #[error("Queue submit failed: {0:?}")]
    QueueSubmit(ash::vk::Result),

    #[error("Present failed: {0:?}")]
    Present(ash::vk::Result),

    #[error("Fence wait failed: {0:?}")]
    FenceWait(ash::vk::Result),

    #[error("Shader source not found: {0}")]
    ShaderNotFound(String),

    #[error("Shader compilation failed: {0}")]
    ShaderCompile(String),

    #[error("Feature unsupported: {0}")]
    Unsupported(String),

    #[error("Invalid handle: {0}")]
    InvalidHandle(String),

    #[error("Not initialized")]
    NotInitialized,

    #[error("Already initialized")]
    AlreadyInitialized,

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ArashiError>;
