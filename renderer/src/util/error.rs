use std::fmt;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, ArashiError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ErrorCode {
    Ok = 0,

    Unknown = 1000,
    Internal = 1001,
    NotImplemented = 1002,
    InvalidState = 1003,
    InvalidParameter = 1004,
    InvalidHandle = 1005,
    NotInitialized = 1006,
    AlreadyInitialized = 1007,
    OutOfMemory = 1008,
    OutOfRange = 1009,
    Timeout = 1010,
    Cancelled = 1011,
    Interrupted = 1012,
    WouldBlock = 1013,
    PermissionDenied = 1014,
    ResourceBusy = 1015,
    ResourceExhausted = 1016,
    NotSupported = 1017,
    VersionMismatch = 1018,
    ChecksumMismatch = 1019,
    DataCorrupted = 1020,

    LoaderNotFound = 2000,
    LoaderLoadFailed = 2001,
    EntryPointMissing = 2002,
    ExtensionMissing = 2003,
    LayerMissing = 2004,
    InstanceCreationFailed = 2005,
    InstanceVersionTooLow = 2006,
    NoPhysicalDevice = 2007,
    PhysicalDeviceEnumerationFailed = 2008,
    PhysicalDeviceLost = 2009,
    PhysicalDeviceRemoved = 2010,
    NoQueueFamily = 2011,
    QueueFamilyIncompatible = 2012,
    DeviceCreationFailed = 2013,
    DeviceLost = 2014,
    QueueSubmitFailed = 2015,
    QueueWaitIdleFailed = 2016,
    DeviceWaitIdleFailed = 2017,
    FeatureNotPresent = 2018,
    FeatureNotEnabled = 2019,

    SurfaceCreationFailed = 3000,
    SurfaceLost = 3001,
    SurfaceOutOfDate = 3002,
    SurfaceQueryFailed = 3003,
    SurfaceNotSupported = 3004,
    SurfaceFormatUnsupported = 3005,
    SurfacePresentModeUnsupported = 3006,

    SwapchainCreationFailed = 4000,
    SwapchainOutOfDate = 4001,
    SwapchainSuboptimal = 4002,
    SwapchainEmpty = 4003,
    SwapchainImageAcquireFailed = 4004,
    SwapchainImageMissing = 4005,
    SwapchainRecreationFailed = 4006,
    SwapchainTooManyImages = 4007,
    SwapchainTooFewImages = 4008,
    PresentFailed = 4009,

    PipelineCreationFailed = 5000,
    PipelineLayoutCreationFailed = 5001,
    PipelineCacheCreationFailed = 5002,
    PipelineCompileFailed = 5003,
    RenderPassCreationFailed = 5004,
    FramebufferCreationFailed = 5005,
    SubpassDependencyInvalid = 5006,
    VertexInputInvalid = 5007,

    ShaderModuleCreationFailed = 6000,
    ShaderCompilationFailed = 6001,
    ShaderSourceNotFound = 6002,
    ShaderReflectionFailed = 6003,
    ShaderInterfaceMismatch = 6004,
    ShaderBinaryInvalid = 6005,

    DescriptorSetLayoutCreationFailed = 7000,
    DescriptorPoolCreationFailed = 7001,
    DescriptorSetAllocationFailed = 7002,
    DescriptorPoolExhausted = 7003,
    DescriptorBindingMissing = 7004,
    DescriptorUpdateFailed = 7005,

    CommandPoolCreationFailed = 8000,
    CommandBufferAllocationFailed = 8001,
    CommandBufferBeginFailed = 8002,
    CommandBufferEndFailed = 8003,
    CommandBufferRecordingFailed = 8004,
    CommandBufferInUse = 8005,

    SemaphoreCreationFailed = 9000,
    FenceCreationFailed = 9001,
    FenceWaitFailed = 9002,
    FenceTimeout = 9003,
    EventCreationFailed = 9004,

    BufferCreationFailed = 10000,
    BufferAllocationFailed = 10001,
    BufferBindingFailed = 10002,
    BufferTooLarge = 10003,
    BufferAlignmentError = 10004,
    BufferMapFailed = 10005,
    BufferUnmapFailed = 10006,
    BufferFlushFailed = 10007,
    BufferInvalidateFailed = 10008,

    ImageCreationFailed = 11000,
    ImageViewCreationFailed = 11001,
    ImageLayoutTransitionFailed = 11002,
    ImageFormatUnsupported = 11003,
    ImageSamplerCreationFailed = 11004,

    MemoryTypeNotFound = 12000,
    MemoryAllocationFailed = 12001,
    MemoryMappingFailed = 12002,
    MemoryBindingFailed = 12003,
    MemoryOutOfDeviceLocal = 12004,
    MemoryFragmentation = 12005,

    AtlasFull = 13000,
    AtlasAllocationFailed = 13001,
    AtlasEvictionFailed = 13002,
    AtlasPackingFailed = 13003,
    GlyphNotFound = 13004,
    GlyphRasterizationFailed = 13005,
    FontLoadFailed = 13006,
    FontParseFailed = 13007,
    FontFaceMissing = 13008,
    FontFallbackExhausted = 13009,

    GlesContextCreationFailed = 14000,
    GlesExtensionMissing = 14001,
    GlesShaderCompileFailed = 14002,
    GlesProgramLinkFailed = 14003,
    GlesFramebufferIncomplete = 14004,

    JniEnvMissing = 15000,
    JniClassNotFound = 15001,
    JniMethodNotFound = 15002,
    JniFieldNotFound = 15003,
    JniExceptionPending = 15004,
    JniStringConversionFailed = 15005,
    JniArrayAccessFailed = 15006,

    IoFailed = 16000,
    FileNotFound = 16001,
    FileAccessDenied = 16002,
    FileAlreadyExists = 16003,
    FileReadFailed = 16004,
    FileWriteFailed = 16005,
    DirectoryNotFound = 16006,

    InvalidFormatString = 17000,
    InvalidLayout = 17001,
    InvalidViewport = 17002,
    InvalidScissor = 17003,
    InvalidCellCoordinate = 17004,
}

impl ErrorCode {
    pub const fn as_u32(self) -> u32 {
        self as u32
    }

    pub const fn from_u32(v: u32) -> Option<Self> {
        match v {
            0 => Some(ErrorCode::Ok),
            1000 => Some(ErrorCode::Unknown),
            1001 => Some(ErrorCode::Internal),
            1002 => Some(ErrorCode::NotImplemented),
            1003 => Some(ErrorCode::InvalidState),
            1004 => Some(ErrorCode::InvalidParameter),
            1005 => Some(ErrorCode::InvalidHandle),
            1006 => Some(ErrorCode::NotInitialized),
            1007 => Some(ErrorCode::AlreadyInitialized),
            1008 => Some(ErrorCode::OutOfMemory),
            1009 => Some(ErrorCode::OutOfRange),
            1010 => Some(ErrorCode::Timeout),
            1011 => Some(ErrorCode::Cancelled),
            1012 => Some(ErrorCode::Interrupted),
            1013 => Some(ErrorCode::WouldBlock),
            1014 => Some(ErrorCode::PermissionDenied),
            1015 => Some(ErrorCode::ResourceBusy),
            1016 => Some(ErrorCode::ResourceExhausted),
            1017 => Some(ErrorCode::NotSupported),
            1018 => Some(ErrorCode::VersionMismatch),
            1019 => Some(ErrorCode::ChecksumMismatch),
            1020 => Some(ErrorCode::DataCorrupted),
            2000 => Some(ErrorCode::LoaderNotFound),
            2001 => Some(ErrorCode::LoaderLoadFailed),
            2002 => Some(ErrorCode::EntryPointMissing),
            2003 => Some(ErrorCode::ExtensionMissing),
            2004 => Some(ErrorCode::LayerMissing),
            2005 => Some(ErrorCode::InstanceCreationFailed),
            2006 => Some(ErrorCode::InstanceVersionTooLow),
            2007 => Some(ErrorCode::NoPhysicalDevice),
            2008 => Some(ErrorCode::PhysicalDeviceEnumerationFailed),
            2009 => Some(ErrorCode::PhysicalDeviceLost),
            2010 => Some(ErrorCode::PhysicalDeviceRemoved),
            2011 => Some(ErrorCode::NoQueueFamily),
            2012 => Some(ErrorCode::QueueFamilyIncompatible),
            2013 => Some(ErrorCode::DeviceCreationFailed),
            2014 => Some(ErrorCode::DeviceLost),
            2015 => Some(ErrorCode::QueueSubmitFailed),
            2016 => Some(ErrorCode::QueueWaitIdleFailed),
            2017 => Some(ErrorCode::DeviceWaitIdleFailed),
            2018 => Some(ErrorCode::FeatureNotPresent),
            2019 => Some(ErrorCode::FeatureNotEnabled),
            3000 => Some(ErrorCode::SurfaceCreationFailed),
            3001 => Some(ErrorCode::SurfaceLost),
            3002 => Some(ErrorCode::SurfaceOutOfDate),
            3003 => Some(ErrorCode::SurfaceQueryFailed),
            3004 => Some(ErrorCode::SurfaceNotSupported),
            3005 => Some(ErrorCode::SurfaceFormatUnsupported),
            3006 => Some(ErrorCode::SurfacePresentModeUnsupported),
            4000 => Some(ErrorCode::SwapchainCreationFailed),
            4001 => Some(ErrorCode::SwapchainOutOfDate),
            4002 => Some(ErrorCode::SwapchainSuboptimal),
            4003 => Some(ErrorCode::SwapchainEmpty),
            4004 => Some(ErrorCode::SwapchainImageAcquireFailed),
            4005 => Some(ErrorCode::SwapchainImageMissing),
            4006 => Some(ErrorCode::SwapchainRecreationFailed),
            4007 => Some(ErrorCode::SwapchainTooManyImages),
            4008 => Some(ErrorCode::SwapchainTooFewImages),
            4009 => Some(ErrorCode::PresentFailed),
            5000 => Some(ErrorCode::PipelineCreationFailed),
            5001 => Some(ErrorCode::PipelineLayoutCreationFailed),
            5002 => Some(ErrorCode::PipelineCacheCreationFailed),
            5003 => Some(ErrorCode::PipelineCompileFailed),
            5004 => Some(ErrorCode::RenderPassCreationFailed),
            5005 => Some(ErrorCode::FramebufferCreationFailed),
            5006 => Some(ErrorCode::SubpassDependencyInvalid),
            5007 => Some(ErrorCode::VertexInputInvalid),
            6000 => Some(ErrorCode::ShaderModuleCreationFailed),
            6001 => Some(ErrorCode::ShaderCompilationFailed),
            6002 => Some(ErrorCode::ShaderSourceNotFound),
            6003 => Some(ErrorCode::ShaderReflectionFailed),
            6004 => Some(ErrorCode::ShaderInterfaceMismatch),
            6005 => Some(ErrorCode::ShaderBinaryInvalid),
            7000 => Some(ErrorCode::DescriptorSetLayoutCreationFailed),
            7001 => Some(ErrorCode::DescriptorPoolCreationFailed),
            7002 => Some(ErrorCode::DescriptorSetAllocationFailed),
            7003 => Some(ErrorCode::DescriptorPoolExhausted),
            7004 => Some(ErrorCode::DescriptorBindingMissing),
            7005 => Some(ErrorCode::DescriptorUpdateFailed),
            8000 => Some(ErrorCode::CommandPoolCreationFailed),
            8001 => Some(ErrorCode::CommandBufferAllocationFailed),
            8002 => Some(ErrorCode::CommandBufferBeginFailed),
            8003 => Some(ErrorCode::CommandBufferEndFailed),
            8004 => Some(ErrorCode::CommandBufferRecordingFailed),
            8005 => Some(ErrorCode::CommandBufferInUse),
            9000 => Some(ErrorCode::SemaphoreCreationFailed),
            9001 => Some(ErrorCode::FenceCreationFailed),
            9002 => Some(ErrorCode::FenceWaitFailed),
            9003 => Some(ErrorCode::FenceTimeout),
            9004 => Some(ErrorCode::EventCreationFailed),
            10000 => Some(ErrorCode::BufferCreationFailed),
            10001 => Some(ErrorCode::BufferAllocationFailed),
            10002 => Some(ErrorCode::BufferBindingFailed),
            10003 => Some(ErrorCode::BufferTooLarge),
            10004 => Some(ErrorCode::BufferAlignmentError),
            10005 => Some(ErrorCode::BufferMapFailed),
            10006 => Some(ErrorCode::BufferUnmapFailed),
            10007 => Some(ErrorCode::BufferFlushFailed),
            10008 => Some(ErrorCode::BufferInvalidateFailed),
            11000 => Some(ErrorCode::ImageCreationFailed),
            11001 => Some(ErrorCode::ImageViewCreationFailed),
            11002 => Some(ErrorCode::ImageLayoutTransitionFailed),
            11003 => Some(ErrorCode::ImageFormatUnsupported),
            11004 => Some(ErrorCode::ImageSamplerCreationFailed),
            12000 => Some(ErrorCode::MemoryTypeNotFound),
            12001 => Some(ErrorCode::MemoryAllocationFailed),
            12002 => Some(ErrorCode::MemoryMappingFailed),
            12003 => Some(ErrorCode::MemoryBindingFailed),
            12004 => Some(ErrorCode::MemoryOutOfDeviceLocal),
            12005 => Some(ErrorCode::MemoryFragmentation),
            13000 => Some(ErrorCode::AtlasFull),
            13001 => Some(ErrorCode::AtlasAllocationFailed),
            13002 => Some(ErrorCode::AtlasEvictionFailed),
            13003 => Some(ErrorCode::AtlasPackingFailed),
            13004 => Some(ErrorCode::GlyphNotFound),
            13005 => Some(ErrorCode::GlyphRasterizationFailed),
            13006 => Some(ErrorCode::FontLoadFailed),
            13007 => Some(ErrorCode::FontParseFailed),
            13008 => Some(ErrorCode::FontFaceMissing),
            13009 => Some(ErrorCode::FontFallbackExhausted),
            14000 => Some(ErrorCode::GlesContextCreationFailed),
            14001 => Some(ErrorCode::GlesExtensionMissing),
            14002 => Some(ErrorCode::GlesShaderCompileFailed),
            14003 => Some(ErrorCode::GlesProgramLinkFailed),
            14004 => Some(ErrorCode::GlesFramebufferIncomplete),
            15000 => Some(ErrorCode::JniEnvMissing),
            15001 => Some(ErrorCode::JniClassNotFound),
            15002 => Some(ErrorCode::JniMethodNotFound),
            15003 => Some(ErrorCode::JniFieldNotFound),
            15004 => Some(ErrorCode::JniExceptionPending),
            15005 => Some(ErrorCode::JniStringConversionFailed),
            15006 => Some(ErrorCode::JniArrayAccessFailed),
            16000 => Some(ErrorCode::IoFailed),
            16001 => Some(ErrorCode::FileNotFound),
            16002 => Some(ErrorCode::FileAccessDenied),
            16003 => Some(ErrorCode::FileAlreadyExists),
            16004 => Some(ErrorCode::FileReadFailed),
            16005 => Some(ErrorCode::FileWriteFailed),
            16006 => Some(ErrorCode::DirectoryNotFound),
            17000 => Some(ErrorCode::InvalidFormatString),
            17001 => Some(ErrorCode::InvalidLayout),
            17002 => Some(ErrorCode::InvalidViewport),
            17003 => Some(ErrorCode::InvalidScissor),
            17004 => Some(ErrorCode::InvalidCellCoordinate),
            _ => None,
        }
    }

    pub const fn category(self) -> ErrorCategory {
        match self as u32 {
            0..=1020 => ErrorCategory::General,
            2000..=2019 => ErrorCategory::VulkanInit,
            3000..=3006 => ErrorCategory::Surface,
            4000..=4009 => ErrorCategory::Swapchain,
            5000..=5007 => ErrorCategory::Pipeline,
            6000..=6005 => ErrorCategory::Shader,
            7000..=7005 => ErrorCategory::Descriptor,
            8000..=8005 => ErrorCategory::Command,
            9000..=9004 => ErrorCategory::Sync,
            10000..=10008 => ErrorCategory::Buffer,
            11000..=11004 => ErrorCategory::Image,
            12000..=12005 => ErrorCategory::Memory,
            13000..=13009 => ErrorCategory::Text,
            14000..=14004 => ErrorCategory::Gles,
            15000..=15006 => ErrorCategory::Jni,
            16000..=16006 => ErrorCategory::Io,
            17000..=17004 => ErrorCategory::Validation,
            _ => ErrorCategory::General,
        }
    }

    pub const fn is_recoverable(self) -> bool {
        matches!(
            self,
            ErrorCode::Timeout
                | ErrorCode::WouldBlock
                | ErrorCode::Interrupted
                | ErrorCode::SwapchainOutOfDate
                | ErrorCode::SwapchainSuboptimal
                | ErrorCode::ResourceBusy
                | ErrorCode::SurfaceOutOfDate
                | ErrorCode::FenceTimeout
                | ErrorCode::DescriptorPoolExhausted
                | ErrorCode::CommandBufferInUse
        )
    }

    pub const fn is_fatal(self) -> bool {
        matches!(
            self,
            ErrorCode::DeviceLost
                | ErrorCode::PhysicalDeviceLost
                | ErrorCode::PhysicalDeviceRemoved
                | ErrorCode::OutOfMemory
                | ErrorCode::MemoryAllocationFailed
                | ErrorCode::DataCorrupted
        )
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}({})", self, *self as u32)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    General,
    VulkanInit,
    Surface,
    Swapchain,
    Pipeline,
    Shader,
    Descriptor,
    Command,
    Sync,
    Buffer,
    Image,
    Memory,
    Text,
    Gles,
    Jni,
    Io,
    Validation,
}

impl ErrorCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            ErrorCategory::General => "general",
            ErrorCategory::VulkanInit => "vulkan-init",
            ErrorCategory::Surface => "surface",
            ErrorCategory::Swapchain => "swapchain",
            ErrorCategory::Pipeline => "pipeline",
            ErrorCategory::Shader => "shader",
            ErrorCategory::Descriptor => "descriptor",
            ErrorCategory::Command => "command",
            ErrorCategory::Sync => "sync",
            ErrorCategory::Buffer => "buffer",
            ErrorCategory::Image => "image",
            ErrorCategory::Memory => "memory",
            ErrorCategory::Text => "text",
            ErrorCategory::Gles => "gles",
            ErrorCategory::Jni => "jni",
            ErrorCategory::Io => "io",
            ErrorCategory::Validation => "validation",
        }
    }
}

impl fmt::Display for ErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug)]
pub enum ArashiError {
    LoaderNotFound,
    LoaderLoadFailed { detail: String },

    EntryPointMissing { name: String },
    ExtensionMissing { names: Vec<String> },
    LayerMissing { names: Vec<String> },

    InstanceCreationFailed { result: i32, detail: String },
    InstanceVersionTooLow { required: u32, available: u32 },

    NoPhysicalDevice,
    PhysicalDeviceEnumerationFailed { result: i32, detail: String },
    PhysicalDeviceLost { name: String },
    PhysicalDeviceRemoved { name: String },
    NoQueueFamily { flags: String },
    QueueFamilyIncompatible { index: u32, reason: String },

    DeviceCreationFailed { result: i32, detail: String },
    DeviceLost { detail: String },
    QueueSubmitFailed { result: i32, detail: String },
    QueueWaitIdleFailed { result: i32, detail: String },
    DeviceWaitIdleFailed { result: i32, detail: String },

    FeatureNotPresent { name: String },
    FeatureNotEnabled { name: String },

    SurfaceCreationFailed { detail: String },
    SurfaceLost,
    SurfaceOutOfDate,
    SurfaceQueryFailed { result: i32, detail: String },
    SurfaceNotSupported,
    SurfaceFormatUnsupported { format: u32 },
    SurfacePresentModeUnsupported { mode: u32 },

    SwapchainCreationFailed { result: i32, detail: String },
    SwapchainOutOfDate,
    SwapchainSuboptimal,
    SwapchainEmpty,
    SwapchainImageAcquireFailed { result: i32, detail: String },
    SwapchainImageMissing { index: u32 },
    SwapchainRecreationFailed { detail: String },
    SwapchainTooManyImages { count: u32, max: u32 },
    SwapchainTooFewImages { count: u32, min: u32 },
    PresentFailed { result: i32, detail: String },

    PipelineCreationFailed { result: i32, detail: String },
    PipelineLayoutCreationFailed { result: i32, detail: String },
    PipelineCacheCreationFailed { result: i32, detail: String },
    PipelineCompileFailed { stage: String, log: String },
    RenderPassCreationFailed { result: i32, detail: String },
    FramebufferCreationFailed { result: i32, detail: String },
    SubpassDependencyInvalid { detail: String },
    VertexInputInvalid { detail: String },

    ShaderModuleCreationFailed { result: i32, detail: String },
    ShaderCompilationFailed { path: String, log: String },
    ShaderSourceNotFound { path: String },
    ShaderReflectionFailed { detail: String },
    ShaderInterfaceMismatch { expected: String, actual: String },
    ShaderBinaryInvalid { path: String, reason: String },

    DescriptorSetLayoutCreationFailed { result: i32, detail: String },
    DescriptorPoolCreationFailed { result: i32, detail: String },
    DescriptorSetAllocationFailed { result: i32, detail: String },
    DescriptorPoolExhausted { pool_size: u32 },
    DescriptorBindingMissing { set: u32, binding: u32 },
    DescriptorUpdateFailed { detail: String },

    CommandPoolCreationFailed { result: i32, detail: String },
    CommandBufferAllocationFailed { result: i32, detail: String },
    CommandBufferBeginFailed { result: i32, detail: String },
    CommandBufferEndFailed { result: i32, detail: String },
    CommandBufferRecordingFailed { detail: String },
    CommandBufferInUse { index: u32 },

    SemaphoreCreationFailed { result: i32, detail: String },
    FenceCreationFailed { result: i32, detail: String },
    FenceWaitFailed { result: i32, detail: String },
    FenceTimeout { waited_ms: u64 },
    EventCreationFailed { result: i32, detail: String },

    BufferCreationFailed { result: i32, detail: String },
    BufferAllocationFailed { size: u64, detail: String },
    BufferBindingFailed { detail: String },
    BufferTooLarge { requested: u64, max: u64 },
    BufferAlignmentError { offset: u64, alignment: u64 },
    BufferMapFailed { detail: String },
    BufferUnmapFailed { detail: String },
    BufferFlushFailed { detail: String },
    BufferInvalidateFailed { detail: String },

    ImageCreationFailed { result: i32, detail: String },
    ImageViewCreationFailed { result: i32, detail: String },
    ImageLayoutTransitionFailed { detail: String },
    ImageFormatUnsupported { format: u32 },
    ImageSamplerCreationFailed { result: i32, detail: String },

    MemoryTypeNotFound { required_flags: u32 },
    MemoryAllocationFailed { size: u64, detail: String },
    MemoryMappingFailed { detail: String },
    MemoryBindingFailed { detail: String },
    MemoryOutOfDeviceLocal { requested: u64, available: u64 },
    MemoryFragmentation { largest_free: u64, requested: u64 },

    AtlasFull { used: u32, capacity: u32 },
    AtlasAllocationFailed { glyph_count: u32 },
    AtlasEvictionFailed { detail: String },
    AtlasPackingFailed { detail: String },
    GlyphNotFound { codepoint: u32, font: String },
    GlyphRasterizationFailed { codepoint: u32, detail: String },
    FontLoadFailed { path: PathBuf, detail: String },
    FontParseFailed { path: PathBuf, detail: String },
    FontFaceMissing { family: String, style: String },
    FontFallbackExhausted { codepoint: u32 },

    GlesContextCreationFailed { detail: String },
    GlesExtensionMissing { name: String },
    GlesShaderCompileFailed { stage: String, log: String },
    GlesProgramLinkFailed { log: String },
    GlesFramebufferIncomplete { status: u32 },

    JniEnvMissing,
    JniClassNotFound { name: String },
    JniMethodNotFound { class: String, method: String, signature: String },
    JniFieldNotFound { class: String, field: String },
    JniExceptionPending { detail: String },
    JniStringConversionFailed { detail: String },
    JniArrayAccessFailed { detail: String },

    IoFailed { path: Option<PathBuf>, detail: String, kind: io::ErrorKind },
    FileNotFound { path: PathBuf },
    FileAccessDenied { path: PathBuf },
    FileAlreadyExists { path: PathBuf },
    FileReadFailed { path: PathBuf, detail: String },
    FileWriteFailed { path: PathBuf, detail: String },
    DirectoryNotFound { path: PathBuf },

    InvalidFormatString { fmt: String, reason: String },
    InvalidLayout { detail: String },
    InvalidViewport { x: i32, y: i32, w: u32, h: u32 },
    InvalidScissor { x: i32, y: i32, w: u32, h: u32 },
    InvalidCellCoordinate { x: i32, y: i32, cols: u32, rows: u32 },

    Unknown { code: u32, detail: String },
    Internal { detail: String },
    NotImplemented { what: String },
    InvalidState { expected: String, actual: String },
    InvalidParameter { name: String, reason: String },
    InvalidHandle { kind: String },
    NotInitialized,
    AlreadyInitialized,
    OutOfMemory { detail: String },
    OutOfRange { value: i64, min: i64, max: i64 },
    Timeout { operation: String, ms: u64 },
    Cancelled { operation: String },
    Interrupted { syscall: String },
    WouldBlock { operation: String },
    PermissionDenied { detail: String },
    ResourceBusy { name: String },
    ResourceExhausted { kind: String, limit: u64 },
    NotSupported { feature: String },
    VersionMismatch { expected: String, actual: String },
    ChecksumMismatch { expected: u64, actual: u64 },
    DataCorrupted { detail: String },
}

impl ArashiError {
    pub fn code(&self) -> ErrorCode {
        match self {
            ArashiError::LoaderNotFound => ErrorCode::LoaderNotFound,
            ArashiError::LoaderLoadFailed { .. } => ErrorCode::LoaderLoadFailed,
            ArashiError::EntryPointMissing { .. } => ErrorCode::EntryPointMissing,
            ArashiError::ExtensionMissing { .. } => ErrorCode::ExtensionMissing,
            ArashiError::LayerMissing { .. } => ErrorCode::LayerMissing,
            ArashiError::InstanceCreationFailed { .. } => ErrorCode::InstanceCreationFailed,
            ArashiError::InstanceVersionTooLow { .. } => ErrorCode::InstanceVersionTooLow,
            ArashiError::NoPhysicalDevice => ErrorCode::NoPhysicalDevice,
            ArashiError::PhysicalDeviceEnumerationFailed { .. } => ErrorCode::PhysicalDeviceEnumerationFailed,
            ArashiError::PhysicalDeviceLost { .. } => ErrorCode::PhysicalDeviceLost,
            ArashiError::PhysicalDeviceRemoved { .. } => ErrorCode::PhysicalDeviceRemoved,
            ArashiError::NoQueueFamily { .. } => ErrorCode::NoQueueFamily,
            ArashiError::QueueFamilyIncompatible { .. } => ErrorCode::QueueFamilyIncompatible,
            ArashiError::DeviceCreationFailed { .. } => ErrorCode::DeviceCreationFailed,
            ArashiError::DeviceLost { .. } => ErrorCode::DeviceLost,
            ArashiError::QueueSubmitFailed { .. } => ErrorCode::QueueSubmitFailed,
            ArashiError::QueueWaitIdleFailed { .. } => ErrorCode::QueueWaitIdleFailed,
            ArashiError::DeviceWaitIdleFailed { .. } => ErrorCode::DeviceWaitIdleFailed,
            ArashiError::FeatureNotPresent { .. } => ErrorCode::FeatureNotPresent,
            ArashiError::FeatureNotEnabled { .. } => ErrorCode::FeatureNotEnabled,
            ArashiError::SurfaceCreationFailed { .. } => ErrorCode::SurfaceCreationFailed,
            ArashiError::SurfaceLost => ErrorCode::SurfaceLost,
            ArashiError::SurfaceOutOfDate => ErrorCode::SurfaceOutOfDate,
            ArashiError::SurfaceQueryFailed { .. } => ErrorCode::SurfaceQueryFailed,
            ArashiError::SurfaceNotSupported => ErrorCode::SurfaceNotSupported,
            ArashiError::SurfaceFormatUnsupported { .. } => ErrorCode::SurfaceFormatUnsupported,
            ArashiError::SurfacePresentModeUnsupported { .. } => ErrorCode::SurfacePresentModeUnsupported,
            ArashiError::SwapchainCreationFailed { .. } => ErrorCode::SwapchainCreationFailed,
            ArashiError::SwapchainOutOfDate => ErrorCode::SwapchainOutOfDate,
            ArashiError::SwapchainSuboptimal => ErrorCode::SwapchainSuboptimal,
            ArashiError::SwapchainEmpty => ErrorCode::SwapchainEmpty,
            ArashiError::SwapchainImageAcquireFailed { .. } => ErrorCode::SwapchainImageAcquireFailed,
            ArashiError::SwapchainImageMissing { .. } => ErrorCode::SwapchainImageMissing,
            ArashiError::SwapchainRecreationFailed { .. } => ErrorCode::SwapchainRecreationFailed,
            ArashiError::SwapchainTooManyImages { .. } => ErrorCode::SwapchainTooManyImages,
            ArashiError::SwapchainTooFewImages { .. } => ErrorCode::SwapchainTooFewImages,
            ArashiError::PresentFailed { .. } => ErrorCode::PresentFailed,
            ArashiError::PipelineCreationFailed { .. } => ErrorCode::PipelineCreationFailed,
            ArashiError::PipelineLayoutCreationFailed { .. } => ErrorCode::PipelineLayoutCreationFailed,
            ArashiError::PipelineCacheCreationFailed { .. } => ErrorCode::PipelineCacheCreationFailed,
            ArashiError::PipelineCompileFailed { .. } => ErrorCode::PipelineCompileFailed,
            ArashiError::RenderPassCreationFailed { .. } => ErrorCode::RenderPassCreationFailed,
            ArashiError::FramebufferCreationFailed { .. } => ErrorCode::FramebufferCreationFailed,
            ArashiError::SubpassDependencyInvalid { .. } => ErrorCode::SubpassDependencyInvalid,
            ArashiError::VertexInputInvalid { .. } => ErrorCode::VertexInputInvalid,
            ArashiError::ShaderModuleCreationFailed { .. } => ErrorCode::ShaderModuleCreationFailed,
            ArashiError::ShaderCompilationFailed { .. } => ErrorCode::ShaderCompilationFailed,
            ArashiError::ShaderSourceNotFound { .. } => ErrorCode::ShaderSourceNotFound,
            ArashiError::ShaderReflectionFailed { .. } => ErrorCode::ShaderReflectionFailed,
            ArashiError::ShaderInterfaceMismatch { .. } => ErrorCode::ShaderInterfaceMismatch,
            ArashiError::ShaderBinaryInvalid { .. } => ErrorCode::ShaderBinaryInvalid,
            ArashiError::DescriptorSetLayoutCreationFailed { .. } => ErrorCode::DescriptorSetLayoutCreationFailed,
            ArashiError::DescriptorPoolCreationFailed { .. } => ErrorCode::DescriptorPoolCreationFailed,
            ArashiError::DescriptorSetAllocationFailed { .. } => ErrorCode::DescriptorSetAllocationFailed,
            ArashiError::DescriptorPoolExhausted { .. } => ErrorCode::DescriptorPoolExhausted,
            ArashiError::DescriptorBindingMissing { .. } => ErrorCode::DescriptorBindingMissing,
            ArashiError::DescriptorUpdateFailed { .. } => ErrorCode::DescriptorUpdateFailed,
            ArashiError::CommandPoolCreationFailed { .. } => ErrorCode::CommandPoolCreationFailed,
            ArashiError::CommandBufferAllocationFailed { .. } => ErrorCode::CommandBufferAllocationFailed,
            ArashiError::CommandBufferBeginFailed { .. } => ErrorCode::CommandBufferBeginFailed,
            ArashiError::CommandBufferEndFailed { .. } => ErrorCode::CommandBufferEndFailed,
            ArashiError::CommandBufferRecordingFailed { .. } => ErrorCode::CommandBufferRecordingFailed,
            ArashiError::CommandBufferInUse { .. } => ErrorCode::CommandBufferInUse,
            ArashiError::SemaphoreCreationFailed { .. } => ErrorCode::SemaphoreCreationFailed,
            ArashiError::FenceCreationFailed { .. } => ErrorCode::FenceCreationFailed,
            ArashiError::FenceWaitFailed { .. } => ErrorCode::FenceWaitFailed,
            ArashiError::FenceTimeout { .. } => ErrorCode::FenceTimeout,
            ArashiError::EventCreationFailed { .. } => ErrorCode::EventCreationFailed,
            ArashiError::BufferCreationFailed { .. } => ErrorCode::BufferCreationFailed,
            ArashiError::BufferAllocationFailed { .. } => ErrorCode::BufferAllocationFailed,
            ArashiError::BufferBindingFailed { .. } => ErrorCode::BufferBindingFailed,
            ArashiError::BufferTooLarge { .. } => ErrorCode::BufferTooLarge,
            ArashiError::BufferAlignmentError { .. } => ErrorCode::BufferAlignmentError,
            ArashiError::BufferMapFailed { .. } => ErrorCode::BufferMapFailed,
            ArashiError::BufferUnmapFailed { .. } => ErrorCode::BufferUnmapFailed,
            ArashiError::BufferFlushFailed { .. } => ErrorCode::BufferFlushFailed,
            ArashiError::BufferInvalidateFailed { .. } => ErrorCode::BufferInvalidateFailed,
            ArashiError::ImageCreationFailed { .. } => ErrorCode::ImageCreationFailed,
            ArashiError::ImageViewCreationFailed { .. } => ErrorCode::ImageViewCreationFailed,
            ArashiError::ImageLayoutTransitionFailed { .. } => ErrorCode::ImageLayoutTransitionFailed,
            ArashiError::ImageFormatUnsupported { .. } => ErrorCode::ImageFormatUnsupported,
            ArashiError::ImageSamplerCreationFailed { .. } => ErrorCode::ImageSamplerCreationFailed,
            ArashiError::MemoryTypeNotFound { .. } => ErrorCode::MemoryTypeNotFound,
            ArashiError::MemoryAllocationFailed { .. } => ErrorCode::MemoryAllocationFailed,
            ArashiError::MemoryMappingFailed { .. } => ErrorCode::MemoryMappingFailed,
            ArashiError::MemoryBindingFailed { .. } => ErrorCode::MemoryBindingFailed,
            ArashiError::MemoryOutOfDeviceLocal { .. } => ErrorCode::MemoryOutOfDeviceLocal,
            ArashiError::MemoryFragmentation { .. } => ErrorCode::MemoryFragmentation,
            ArashiError::AtlasFull { .. } => ErrorCode::AtlasFull,
            ArashiError::AtlasAllocationFailed { .. } => ErrorCode::AtlasAllocationFailed,
            ArashiError::AtlasEvictionFailed { .. } => ErrorCode::AtlasEvictionFailed,
            ArashiError::AtlasPackingFailed { .. } => ErrorCode::AtlasPackingFailed,
            ArashiError::GlyphNotFound { .. } => ErrorCode::GlyphNotFound,
            ArashiError::GlyphRasterizationFailed { .. } => ErrorCode::GlyphRasterizationFailed,
            ArashiError::FontLoadFailed { .. } => ErrorCode::FontLoadFailed,
            ArashiError::FontParseFailed { .. } => ErrorCode::FontParseFailed,
            ArashiError::FontFaceMissing { .. } => ErrorCode::FontFaceMissing,
            ArashiError::FontFallbackExhausted { .. } => ErrorCode::FontFallbackExhausted,
            ArashiError::GlesContextCreationFailed { .. } => ErrorCode::GlesContextCreationFailed,
            ArashiError::GlesExtensionMissing { .. } => ErrorCode::GlesExtensionMissing,
            ArashiError::GlesShaderCompileFailed { .. } => ErrorCode::GlesShaderCompileFailed,
            ArashiError::GlesProgramLinkFailed { .. } => ErrorCode::GlesProgramLinkFailed,
            ArashiError::GlesFramebufferIncomplete { .. } => ErrorCode::GlesFramebufferIncomplete,
            ArashiError::JniEnvMissing => ErrorCode::JniEnvMissing,
            ArashiError::JniClassNotFound { .. } => ErrorCode::JniClassNotFound,
            ArashiError::JniMethodNotFound { .. } => ErrorCode::JniMethodNotFound,
            ArashiError::JniFieldNotFound { .. } => ErrorCode::JniFieldNotFound,
            ArashiError::JniExceptionPending { .. } => ErrorCode::JniExceptionPending,
            ArashiError::JniStringConversionFailed { .. } => ErrorCode::JniStringConversionFailed,
            ArashiError::JniArrayAccessFailed { .. } => ErrorCode::JniArrayAccessFailed,
            ArashiError::IoFailed { .. } => ErrorCode::IoFailed,
            ArashiError::FileNotFound { .. } => ErrorCode::FileNotFound,
            ArashiError::FileAccessDenied { .. } => ErrorCode::FileAccessDenied,
            ArashiError::FileAlreadyExists { .. } => ErrorCode::FileAlreadyExists,
            ArashiError::FileReadFailed { .. } => ErrorCode::FileReadFailed,
            ArashiError::FileWriteFailed { .. } => ErrorCode::FileWriteFailed,
            ArashiError::DirectoryNotFound { .. } => ErrorCode::DirectoryNotFound,
            ArashiError::InvalidFormatString { .. } => ErrorCode::InvalidFormatString,
            ArashiError::InvalidLayout { .. } => ErrorCode::InvalidLayout,
            ArashiError::InvalidViewport { .. } => ErrorCode::InvalidViewport,
            ArashiError::InvalidScissor { .. } => ErrorCode::InvalidScissor,
            ArashiError::InvalidCellCoordinate { .. } => ErrorCode::InvalidCellCoordinate,
            ArashiError::Unknown { .. } => ErrorCode::Unknown,
            ArashiError::Internal { .. } => ErrorCode::Internal,
            ArashiError::NotImplemented { .. } => ErrorCode::NotImplemented,
            ArashiError::InvalidState { .. } => ErrorCode::InvalidState,
            ArashiError::InvalidParameter { .. } => ErrorCode::InvalidParameter,
            ArashiError::InvalidHandle { .. } => ErrorCode::InvalidHandle,
            ArashiError::NotInitialized => ErrorCode::NotInitialized,
            ArashiError::AlreadyInitialized => ErrorCode::AlreadyInitialized,
            ArashiError::OutOfMemory { .. } => ErrorCode::OutOfMemory,
            ArashiError::OutOfRange { .. } => ErrorCode::OutOfRange,
            ArashiError::Timeout { .. } => ErrorCode::Timeout,
            ArashiError::Cancelled { .. } => ErrorCode::Cancelled,
            ArashiError::Interrupted { .. } => ErrorCode::Interrupted,
            ArashiError::WouldBlock { .. } => ErrorCode::WouldBlock,
            ArashiError::PermissionDenied { .. } => ErrorCode::PermissionDenied,
            ArashiError::ResourceBusy { .. } => ErrorCode::ResourceBusy,
            ArashiError::ResourceExhausted { .. } => ErrorCode::ResourceExhausted,
            ArashiError::NotSupported { .. } => ErrorCode::NotSupported,
            ArashiError::VersionMismatch { .. } => ErrorCode::VersionMismatch,
            ArashiError::ChecksumMismatch { .. } => ErrorCode::ChecksumMismatch,
            ArashiError::DataCorrupted { .. } => ErrorCode::DataCorrupted,
        }
    }

    pub fn category(&self) -> ErrorCategory {
        self.code().category()
    }

    pub fn is_recoverable(&self) -> bool {
        self.code().is_recoverable()
    }

    pub fn is_fatal(&self) -> bool {
        self.code().is_fatal()
    }

    pub fn recovery_hint(&self) -> &'static str {
        match self {
            ArashiError::LoaderNotFound => "Install the Vulkan loader or fall back to GLES.",
            ArashiError::ExtensionMissing { .. } => "Enable the missing instance extensions or use GLES.",
            ArashiError::LayerMissing { .. } => "Install the validation layers or disable validation.",
            ArashiError::NoPhysicalDevice => "No Vulkan-capable GPU found. Falling back to GLES.",
            ArashiError::NoQueueFamily { .. } => "GPU lacks required queue families. Falling back to GLES.",
            ArashiError::DeviceLost { .. } => "Device lost. Reinitialize the renderer.",
            ArashiError::SurfaceLost => "Surface lost. Recreate the surface and swapchain.",
            ArashiError::SurfaceOutOfDate => "Surface resized. Recreate the swapchain.",
            ArashiError::SwapchainOutOfDate => "Swapchain out of date. Recreate it.",
            ArashiError::SwapchainSuboptimal => "Swapchain suboptimal. Recreate when convenient.",
            ArashiError::SwapchainEmpty => "Swapchain has no images. Recreate it.",
            ArashiError::AtlasFull { .. } => "Grow the atlas or evict unused glyphs.",
            ArashiError::GlyphNotFound { .. } => "Add the codepoint to a fallback font.",
            ArashiError::FontFallbackExhausted { .. } => "Add a CJK or emoji fallback font.",
            ArashiError::FenceTimeout { .. } => "Increase the timeout or check for GPU hang.",
            ArashiError::OutOfMemory { .. } => "Reduce scrollback or close sessions.",
            ArashiError::DescriptorPoolExhausted { .. } => "Grow the descriptor pool.",
            ArashiError::WouldBlock { .. } => "Retry the operation.",
            ArashiError::Timeout { .. } => "Retry the operation with a longer timeout.",
            ArashiError::Interrupted { .. } => "Retry the syscall.",
            ArashiError::IoFailed { .. } => "Check filesystem permissions and free space.",
            ArashiError::FileNotFound { .. } => "Verify the file path.",
            ArashiError::FileAccessDenied { .. } => "Check file permissions.",
            _ => "No specific recovery action available.",
        }
    }

    pub fn context(self, ctx: impl Into<String>) -> ErrorContext {
        ErrorContext {
            error: Arc::new(self),
            chain: vec![ctx.into()],
        }
    }
}

impl fmt::Display for ArashiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = self.code();
        write!(f, "[{}:{}] ", code.category(), code.as_u32())?;

        match self {
            ArashiError::LoaderNotFound => write!(f, "Vulkan loader not found"),

            ArashiError::LoaderLoadFailed { detail } => {
                write!(f, "Vulkan loader failed to load: {detail}")
            }

            ArashiError::EntryPointMissing { name } => {
                write!(f, "Vulkan entry point missing: {name}")
            }

            ArashiError::ExtensionMissing { names } => {
                write!(f, "missing Vulkan extensions: {}", names.join(", "))
            }

            ArashiError::LayerMissing { names } => {
                write!(f, "missing Vulkan layers: {}", names.join(", "))
            }

            ArashiError::InstanceCreationFailed { result, detail } => {
                write!(f, "instance creation failed ({result:#x}): {detail}")
            }

            ArashiError::InstanceVersionTooLow { required, available } => {
                write!(
                    f,
                    "Vulkan version too low: need {:#x}, have {:#x}",
                    required, available
                )
            }

            ArashiError::NoPhysicalDevice => write!(f, "no Vulkan-capable GPU found"),

            ArashiError::PhysicalDeviceEnumerationFailed { result, detail } => {
                write!(f, "physical device enumeration failed ({result:#x}): {detail}")
            }

            ArashiError::PhysicalDeviceLost { name } => {
                write!(f, "physical device lost: {name}")
            }

            ArashiError::PhysicalDeviceRemoved { name } => {
                write!(f, "physical device removed: {name}")
            }

            ArashiError::NoQueueFamily { flags } => {
                write!(f, "no queue family with flags {flags}")
            }

            ArashiError::QueueFamilyIncompatible { index, reason } => {
                write!(f, "queue family {index} incompatible: {reason}")
            }

            ArashiError::DeviceCreationFailed { result, detail } => {
                write!(f, "logical device creation failed ({result:#x}): {detail}")
            }

            ArashiError::DeviceLost { detail } => {
                write!(f, "device lost: {detail}")
            }

            ArashiError::QueueSubmitFailed { result, detail } => {
                write!(f, "queue submit failed ({result:#x}): {detail}")
            }

            ArashiError::QueueWaitIdleFailed { result, detail } => {
                write!(f, "queue wait idle failed ({result:#x}): {detail}")
            }

            ArashiError::DeviceWaitIdleFailed { result, detail } => {
                write!(f, "device wait idle failed ({result:#x}): {detail}")
            }

            ArashiError::FeatureNotPresent { name } => {
                write!(f, "feature not present on device: {name}")
            }

            ArashiError::FeatureNotEnabled { name } => {
                write!(f, "feature not enabled: {name}")
            }

            ArashiError::SurfaceCreationFailed { detail } => {
                write!(f, "surface creation failed: {detail}")
            }

            ArashiError::SurfaceLost => write!(f, "surface lost"),

            ArashiError::SurfaceOutOfDate => write!(f, "surface out of date"),

            ArashiError::SurfaceQueryFailed { result, detail } => {
                write!(f, "surface query failed ({result:#x}): {detail}")
            }

            ArashiError::SurfaceNotSupported => {
                write!(f, "surface not supported by this device")
            }

            ArashiError::SurfaceFormatUnsupported { format } => {
                write!(f, "surface format unsupported: {format}")
            }

            ArashiError::SurfacePresentModeUnsupported { mode } => {
                write!(f, "surface present mode unsupported: {mode}")
            }

            ArashiError::SwapchainCreationFailed { result, detail } => {
                write!(f, "swapchain creation failed ({result:#x}): {detail}")
            }

            ArashiError::SwapchainOutOfDate => write!(f, "swapchain out of date"),

            ArashiError::SwapchainSuboptimal => write!(f, "swapchain suboptimal"),

            ArashiError::SwapchainEmpty => write!(f, "swapchain contains no images"),

            ArashiError::SwapchainImageAcquireFailed { result, detail } => {
                write!(f, "swapchain image acquire failed ({result:#x}): {detail}")
            }

            ArashiError::SwapchainImageMissing { index } => {
                write!(f, "swapchain image missing at index {index}")
            }

            ArashiError::SwapchainRecreationFailed { detail } => {
                write!(f, "swapchain recreation failed: {detail}")
            }

            ArashiError::SwapchainTooManyImages { count, max } => {
                write!(f, "swapchain has {count} images, max is {max}")
            }

            ArashiError::SwapchainTooFewImages { count, min } => {
                write!(f, "swapchain has {count} images, min is {min}")
            }

            ArashiError::PresentFailed { result, detail } => {
                write!(f, "present failed ({result:#x}): {detail}")
            }

            ArashiError::PipelineCreationFailed { result, detail } => {
                write!(f, "pipeline creation failed ({result:#x}): {detail}")
            }

            ArashiError::PipelineLayoutCreationFailed { result, detail } => {
                write!(f, "pipeline layout creation failed ({result:#x}): {detail}")
            }

            ArashiError::PipelineCacheCreationFailed { result, detail } => {
                write!(f, "pipeline cache creation failed ({result:#x}): {detail}")
            }

            ArashiError::PipelineCompileFailed { stage, log } => {
                write!(f, "pipeline compile failed at {stage}: {log}")
            }

            ArashiError::RenderPassCreationFailed { result, detail } => {
                write!(f, "render pass creation failed ({result:#x}): {detail}")
            }

            ArashiError::FramebufferCreationFailed { result, detail } => {
                write!(f, "framebuffer creation failed ({result:#x}): {detail}")
            }

            ArashiError::SubpassDependencyInvalid { detail } => {
                write!(f, "invalid subpass dependency: {detail}")
            }

            ArashiError::VertexInputInvalid { detail } => {
                write!(f, "invalid vertex input: {detail}")
            }

            ArashiError::ShaderModuleCreationFailed { result, detail } => {
                write!(f, "shader module creation failed ({result:#x}): {detail}")
            }

            ArashiError::ShaderCompilationFailed { path, log } => {
                write!(f, "shader compilation failed for {path}: {log}")
            }

            ArashiError::ShaderSourceNotFound { path } => {
                write!(f, "shader source not found: {path}")
            }

            ArashiError::ShaderReflectionFailed { detail } => {
                write!(f, "shader reflection failed: {detail}")
            }

            ArashiError::ShaderInterfaceMismatch { expected, actual } => {
                write!(f, "shader interface mismatch: expected {expected}, got {actual}")
            }

            ArashiError::ShaderBinaryInvalid { path, reason } => {
                write!(f, "shader binary invalid at {path}: {reason}")
            }

            ArashiError::DescriptorSetLayoutCreationFailed { result, detail } => {
                write!(f, "descriptor set layout creation failed ({result:#x}): {detail}")
            }

            ArashiError::DescriptorPoolCreationFailed { result, detail } => {
                write!(f, "descriptor pool creation failed ({result:#x}): {detail}")
            }

            ArashiError::DescriptorSetAllocationFailed { result, detail } => {
                write!(f, "descriptor set allocation failed ({result:#x}): {detail}")
            }

            ArashiError::DescriptorPoolExhausted { pool_size } => {
                write!(f, "descriptor pool exhausted (size {pool_size})")
            }

            ArashiError::DescriptorBindingMissing { set, binding } => {
                write!(f, "descriptor binding missing: set {set}, binding {binding}")
            }

            ArashiError::DescriptorUpdateFailed { detail } => {
                write!(f, "descriptor update failed: {detail}")
            }

            ArashiError::CommandPoolCreationFailed { result, detail } => {
                write!(f, "command pool creation failed ({result:#x}): {detail}")
            }

            ArashiError::CommandBufferAllocationFailed { result, detail } => {
                write!(f, "command buffer allocation failed ({result:#x}): {detail}")
            }

            ArashiError::CommandBufferBeginFailed { result, detail } => {
                write!(f, "command buffer begin failed ({result:#x}): {detail}")
            }

            ArashiError::CommandBufferEndFailed { result, detail } => {
                write!(f, "command buffer end failed ({result:#x}): {detail}")
            }

            ArashiError::CommandBufferRecordingFailed { detail } => {
                write!(f, "command buffer recording failed: {detail}")
            }

            ArashiError::CommandBufferInUse { index } => {
                write!(f, "command buffer {index} still in use")
            }

            ArashiError::SemaphoreCreationFailed { result, detail } => {
                write!(f, "semaphore creation failed ({result:#x}): {detail}")
            }

            ArashiError::FenceCreationFailed { result, detail } => {
                write!(f, "fence creation failed ({result:#x}): {detail}")
            }

            ArashiError::FenceWaitFailed { result, detail } => {
                write!(f, "fence wait failed ({result:#x}): {detail}")
            }

            ArashiError::FenceTimeout { waited_ms } => {
                write!(f, "fence wait timed out after {waited_ms} ms")
            }

            ArashiError::EventCreationFailed { result, detail } => {
                write!(f, "event creation failed ({result:#x}): {detail}")
            }

            ArashiError::BufferCreationFailed { result, detail } => {
                write!(f, "buffer creation failed ({result:#x}): {detail}")
            }

            ArashiError::BufferAllocationFailed { size, detail } => {
                write!(f, "buffer allocation failed for {size} bytes: {detail}")
            }

            ArashiError::BufferBindingFailed { detail } => {
                write!(f, "buffer binding failed: {detail}")
            }

            ArashiError::BufferTooLarge { requested, max } => {
                write!(f, "buffer too large: requested {requested}, max {max}")
            }

            ArashiError::BufferAlignmentError { offset, alignment } => {
                write!(f, "buffer offset {offset} not aligned to {alignment}")
            }

            ArashiError::BufferMapFailed { detail } => {
                write!(f, "buffer map failed: {detail}")
            }

            ArashiError::BufferUnmapFailed { detail } => {
                write!(f, "buffer unmap failed: {detail}")
            }

            ArashiError::BufferFlushFailed { detail } => {
                write!(f, "buffer flush failed: {detail}")
            }

            ArashiError::BufferInvalidateFailed { detail } => {
                write!(f, "buffer invalidate failed: {detail}")
            }

            ArashiError::ImageCreationFailed { result, detail } => {
                write!(f, "image creation failed ({result:#x}): {detail}")
            }

            ArashiError::ImageViewCreationFailed { result, detail } => {
                write!(f, "image view creation failed ({result:#x}): {detail}")
            }

            ArashiError::ImageLayoutTransitionFailed { detail } => {
                write!(f, "image layout transition failed: {detail}")
            }

            ArashiError::ImageFormatUnsupported { format } => {
                write!(f, "image format unsupported: {format}")
            }

            ArashiError::ImageSamplerCreationFailed { result, detail } => {
                write!(f, "image sampler creation failed ({result:#x}): {detail}")
            }

            ArashiError::MemoryTypeNotFound { required_flags } => {
                write!(f, "no memory type matches flags {required_flags:#x}")
            }

            ArashiError::MemoryAllocationFailed { size, detail } => {
                write!(f, "memory allocation failed for {size} bytes: {detail}")
            }

            ArashiError::MemoryMappingFailed { detail } => {
                write!(f, "memory mapping failed: {detail}")
            }

            ArashiError::MemoryBindingFailed { detail } => {
                write!(f, "memory binding failed: {detail}")
            }

            ArashiError::MemoryOutOfDeviceLocal { requested, available } => {
                write!(
                    f,
                    "device-local memory exhausted: requested {requested}, available {available}"
                )
            }

            ArashiError::AtlasFull { used, capacity } => {
                write!(f, "atlas full: {used} used of {capacity}")
            }

            ArashiError::AtlasAllocationFailed { glyph_count } => {
                write!(f, "atlas allocation failed for {glyph_count} glyphs")
            }

            ArashiError::AtlasEvictionFailed { detail } => {
                write!(f, "atlas eviction failed: {detail}")
            }

            ArashiError::AtlasPackingFailed { detail } => {
                write!(f, "atlas packing failed: {detail}")
            }

            ArashiError::GlyphNotFound { codepoint, font } => {
                write!(f, "glyph U+{codepoint:04X} not found in {font}")
            }

            ArashiError::GlyphRasterizationFailed { codepoint, detail } => {
                write!(f, "glyph rasterization failed for U+{codepoint:04X}: {detail}")
            }

            ArashiError::FontLoadFailed { path, detail } => {
                write!(f, "font load failed at {}: {detail}", path.display())
            }

            ArashiError::FontParseFailed { path, detail } => {
                write!(f, "font parse failed at {}: {detail}", path.display())
            }

            ArashiError::FontFaceMissing { family, style } => {
                write!(f, "font face missing: {family} {style}")
            }

            ArashiError::FontFallbackExhausted { codepoint } => {
                write!(f, "no fallback font for U+{codepoint:04X}")
            }

            ArashiError::GlesContextCreationFailed { detail } => {
                write!(f, "GLES context creation failed: {detail}")
            }

            ArashiError::GlesExtensionMissing { name } => {
                write!(f, "missing GLES extension: {name}")
            }

            ArashiError::GlesShaderCompileFailed { stage, log } => {
                write!(f, "GLES shader compile failed at {stage}: {log}")
            }

            ArashiError::GlesProgramLinkFailed { log } => {
                write!(f, "GLES program link failed: {log}")
            }

            ArashiError::GlesFramebufferIncomplete { status } => {
                write!(f, "GLES framebuffer incomplete (status {status:#x})")
            }

            ArashiError::JniEnvMissing => write!(f, "JNI environment missing"),

            ArashiError::JniClassNotFound { name } => {
                write!(f, "JNI class not found: {name}")
            }

            ArashiError::JniMethodNotFound {
                class,
                method,
                signature,
            } => {
                write!(f, "JNI method not found: {class}.{method}{signature}")
            }

            ArashiError::JniFieldNotFound { class, field } => {
                write!(f, "JNI field not found: {class}.{field}")
            }

            ArashiError::JniExceptionPending { detail } => {
                write!(f, "pending JNI exception: {detail}")
            }

            ArashiError::JniStringConversionFailed { detail } => {
                write!(f, "JNI string conversion failed: {detail}")
            }

            ArashiError::JniArrayAccessFailed { detail } => {
                write!(f, "JNI array access failed: {detail}")
            }

            ArashiError::IoFailed {
                path,
                detail,
                kind,
            } => match path {
                Some(p) => write!(f, "I/O {kind:?} at {}: {detail}", p.display()),
                None => write!(f, "I/O {kind:?}: {detail}"),
            },

            ArashiError::FileNotFound { path } => {
                write!(f, "file not found: {}", path.display())
            }

            ArashiError::FileAccessDenied { path } => {
                write!(f, "access denied: {}", path.display())
            }

            ArashiError::FileAlreadyExists { path } => {
                write!(f, "file already exists: {}", path.display())
            }

            ArashiError::FileReadFailed { path, detail } => {
                write!(f, "read failed at {}: {detail}", path.display())
            }

            ArashiError::FileWriteFailed { path, detail } => {
                write!(f, "write failed at {}: {detail}", path.display())
            }

            ArashiError::DirectoryNotFound { path } => {
                write!(f, "directory not found: {}", path.display())
            }

            ArashiError::InvalidFormatString { fmt, reason } => {
                write!(f, "invalid format string {fmt:?}: {reason}")
            }

            ArashiError::InvalidLayout { detail } => {
                write!(f, "invalid layout: {detail}")
            }

            ArashiError::InvalidViewport { x, y, w, h } => {
                write!(f, "invalid viewport ({x}, {y}, {w}x{h})")
            }

            ArashiError::InvalidScissor { x, y, w, h } => {
                write!(f, "invalid scissor ({x}, {y}, {w}x{h})")
            }

            ArashiError::InvalidCellCoordinate {
                x,
                y,
                cols,
                rows,
            } => {
                write!(f, "cell coordinate ({x}, {y}) outside grid {cols}x{rows}")
            }

            ArashiError::Unknown { code, detail } => {
                write!(f, "unknown error {code}: {detail}")
            }

            ArashiError::Internal { detail } => write!(f, "internal error: {detail}"),

            ArashiError::NotImplemented { what } => {
                write!(f, "not implemented: {what}")
            }

            ArashiError::InvalidState { expected, actual } => {
                write!(f, "invalid state: expected {expected}, got {actual}")
            }

            ArashiError::InvalidParameter { name, reason } => {
                write!(f, "invalid parameter {name}: {reason}")
            }

            ArashiError::InvalidHandle { kind } => {
                write!(f, "invalid handle: {kind}")
            }

            ArashiError::NotInitialized => write!(f, "renderer not initialized"),

            ArashiError::AlreadyInitialized => write!(f, "renderer already initialized"),

            ArashiError::OutOfMemory { detail } => {
                write!(f, "out of memory: {detail}")
            }

            ArashiError::OutOfRange { value, min, max } => {
                write!(f, "value {value} out of range [{min}, {max}]")
            }

            ArashiError::Timeout { operation, ms } => {
                write!(f, "timeout after {ms} ms during {operation}")
            }

            ArashiError::Cancelled { operation } => {
                write!(f, "cancelled: {operation}")
            }

            ArashiError::Interrupted { syscall } => {
                write!(f, "interrupted: {syscall}")
            }

            ArashiError::WouldBlock { operation } => {
                write!(f, "would block: {operation}")
            }

            ArashiError::PermissionDenied { detail } => {
                write!(f, "permission denied: {detail}")
            }

            ArashiError::ResourceBusy { name } => {
                write!(f, "resource busy: {name}")
            }

            ArashiError::ResourceExhausted { kind, limit } => {
                write!(f, "{kind} exhausted (limit {limit})")
            }

            ArashiError::NotSupported { feature } => {
                write!(f, "not supported: {feature}")
            }

            ArashiError::VersionMismatch { expected, actual } => {
                write!(f, "version mismatch: expected {expected}, got {actual}")
            }

            ArashiError::ChecksumMismatch { expected, actual } => {
                write!(f, "checksum mismatch: expected {expected:#x}, got {actual:#x}")
            }

            ArashiError::DataCorrupted { detail } => {
                write!(f, "data corrupted: {detail}")
            }
        }
    }
}

impl std::error::Error for ArashiError {}

impl From<io::Error> for ArashiError {
    fn from(e: io::Error) -> Self {
        let kind = e.kind();
        let detail = e.to_string();
        match kind {
            io::ErrorKind::NotFound => ArashiError::FileNotFound {
                path: PathBuf::new(),
            },
            io::ErrorKind::PermissionDenied => ArashiError::FileAccessDenied {
                path: PathBuf::new(),
            },
            io::ErrorKind::AlreadyExists => ArashiError::FileAlreadyExists {
                path: PathBuf::new(),
            },
            io::ErrorKind::WouldBlock => ArashiError::WouldBlock {
                operation: "io".into(),
            },
            io::ErrorKind::Interrupted => ArashiError::Interrupted {
                syscall: "io".into(),
            },
            io::ErrorKind::TimedOut => ArashiError::Timeout {
                operation: "io".into(),
                ms: 0,
            },
            io::ErrorKind::OutOfMemory => ArashiError::OutOfMemory { detail },
            _ => ArashiError::IoFailed {
                path: None,
                detail,
                kind,
            },
        }
    }
}

#[derive(Clone)]
pub struct ErrorContext {
    error: Arc<ArashiError>,
    chain: Vec<String>,
}

impl ErrorContext {
    pub fn new(error: ArashiError) -> Self {
        Self {
            error: Arc::new(error),
            chain: Vec::new(),
        }
    }

    pub fn with(mut self, ctx: impl Into<String>) -> Self {
        self.chain.push(ctx.into());
        self
    }

    pub fn error(&self) -> &ArashiError {
        &self.error
    }

    pub fn code(&self) -> ErrorCode {
        self.error.code()
    }

    pub fn category(&self) -> ErrorCategory {
        self.error.category()
    }

    pub fn is_recoverable(&self) -> bool {
        self.error.is_recoverable()
    }

    pub fn is_fatal(&self) -> bool {
        self.error.is_fatal()
    }

    pub fn hint(&self) -> &'static str {
        self.error.recovery_hint()
    }

    pub fn depth(&self) -> usize {
        self.chain.len()
    }

    pub fn chain(&self) -> &[String] {
        &self.chain
    }

    pub fn formatted(&self) -> String {
        let mut s = self.error.to_string();
        for (i, ctx) in self.chain.iter().rev().enumerate() {
            s.push_str(&format!("\n  {} {}", i, ctx));
        }
        s
    }
}

impl fmt::Debug for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ErrorContext {{")?;
        writeln!(f, "  error: {:?}", self.error)?;
        writeln!(f, "  code: {}", self.error.code())?;
        writeln!(f, "  category: {}", self.error.category())?;
        writeln!(f, "  recoverable: {}", self.error.is_recoverable())?;
        writeln!(f, "  fatal: {}", self.error.is_fatal())?;
        if !self.chain.is_empty() {
            writeln!(f, "  context chain:")?;
            for (i, c) in self.chain.iter().rev().enumerate() {
                writeln!(f, "    {}: {}", i, c)?;
            }
        }
        write!(f, "}}")
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.formatted())
    }
}

#[macro_export]
macro_rules! err_ctx {
    ($e:expr, $($arg:tt)*) => {
        $e.context(format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_as_u32() {
        assert_eq!(ErrorCode::Ok.as_u32(), 0);
        assert_eq!(ErrorCode::Unknown.as_u32(), 1000);
        assert_eq!(ErrorCode::LoaderNotFound.as_u32(), 2000);
    }

    #[test]
    fn code_from_u32_known() {
        assert_eq!(ErrorCode::from_u32(0), Some(ErrorCode::Ok));
        assert_eq!(ErrorCode::from_u32(1000), Some(ErrorCode::Unknown));
        assert_eq!(ErrorCode::from_u32(2000), Some(ErrorCode::LoaderNotFound));
        assert_eq!(ErrorCode::from_u32(13000), Some(ErrorCode::AtlasFull));
    }

    #[test]
    fn code_from_u32_unknown() {
        assert_eq!(ErrorCode::from_u32(9999), None);
        assert_eq!(ErrorCode::from_u32(u32::MAX), None);
    }

    #[test]
    fn code_category_mapping() {
        assert_eq!(ErrorCode::Ok.category(), ErrorCategory::General);
        assert_eq!(ErrorCode::LoaderNotFound.category(), ErrorCategory::VulkanInit);
        assert_eq!(ErrorCode::SurfaceLost.category(), ErrorCategory::Surface);
        assert_eq!(ErrorCode::SwapchainOutOfDate.category(), ErrorCategory::Swapchain);
        assert_eq!(ErrorCode::PipelineCreationFailed.category(), ErrorCategory::Pipeline);
        assert_eq!(ErrorCode::ShaderSourceNotFound.category(), ErrorCategory::Shader);
        assert_eq!(ErrorCode::DescriptorPoolExhausted.category(), ErrorCategory::Descriptor);
        assert_eq!(ErrorCode::CommandPoolCreationFailed.category(), ErrorCategory::Command);
        assert_eq!(ErrorCode::FenceTimeout.category(), ErrorCategory::Sync);
        assert_eq!(ErrorCode::BufferTooLarge.category(), ErrorCategory::Buffer);
        assert_eq!(ErrorCode::ImageCreationFailed.category(), ErrorCategory::Image);
        assert_eq!(ErrorCode::MemoryFragmentation.category(), ErrorCategory::Memory);
        assert_eq!(ErrorCode::AtlasFull.category(), ErrorCategory::Text);
        assert_eq!(ErrorCode::GlesContextCreationFailed.category(), ErrorCategory::Gles);
        assert_eq!(ErrorCode::JniEnvMissing.category(), ErrorCategory::Jni);
        assert_eq!(ErrorCode::FileNotFound.category(), ErrorCategory::Io);
        assert_eq!(ErrorCode::InvalidViewport.category(), ErrorCategory::Validation);
    }

    #[test]
    fn error_recoverable_flags() {
        assert!(ErrorCode::Timeout.is_recoverable());
        assert!(ErrorCode::SwapchainOutOfDate.is_recoverable());
        assert!(ErrorCode::WouldBlock.is_recoverable());
        assert!(!ErrorCode::LoaderNotFound.is_recoverable());
        assert!(!ErrorCode::OutOfMemory.is_recoverable());
    }

    #[test]
    fn error_fatal_flags() {
        assert!(ErrorCode::DeviceLost.is_fatal());
        assert!(ErrorCode::PhysicalDeviceRemoved.is_fatal());
        assert!(ErrorCode::OutOfMemory.is_fatal());
        assert!(!ErrorCode::Timeout.is_fatal());
        assert!(!ErrorCode::SwapchainOutOfDate.is_fatal());
    }

    #[test]
    fn error_display_loader() {
        let e = ArashiError::LoaderNotFound;
        let s = e.to_string();
        assert!(s.contains("vulkan-init"));
        assert!(s.contains("2000"));
        assert!(s.contains("not found"));
    }

    #[test]
    fn error_display_extension_missing() {
        let e = ArashiError::ExtensionMissing {
            names: vec!["VK_KHR_surface".into(), "VK_KHR_swapchain".into()],
        };
        let s = e.to_string();
        assert!(s.contains("VK_KHR_surface"));
        assert!(s.contains("VK_KHR_swapchain"));
    }

    #[test]
    fn error_display_file_not_found() {
        let e = ArashiError::FileNotFound {
            path: PathBuf::from("/tmp/x.ttf"),
        };
        let s = e.to_string();
        assert!(s.contains("/tmp/x.ttf"));
    }

    #[test]
    fn error_code_roundtrip() {
        for code in [
            ErrorCode::Ok,
            ErrorCode::LoaderNotFound,
            ErrorCode::SurfaceLost,
            ErrorCode::SwapchainOutOfDate,
            ErrorCode::PipelineCreationFailed,
            ErrorCode::ShaderCompilationFailed,
            ErrorCode::DescriptorPoolExhausted,
            ErrorCode::CommandPoolCreationFailed,
            ErrorCode::FenceTimeout,
            ErrorCode::BufferTooLarge,
            ErrorCode::ImageCreationFailed,
            ErrorCode::MemoryAllocationFailed,
            ErrorCode::AtlasFull,
            ErrorCode::GlesContextCreationFailed,
            ErrorCode::JniEnvMissing,
            ErrorCode::FileNotFound,
            ErrorCode::InvalidViewport,
        ] {
            let n = code.as_u32();
            assert_eq!(ErrorCode::from_u32(n), Some(code));
        }
    }

    #[test]
    fn recovery_hint_present() {
        for e in [
            ArashiError::LoaderNotFound,
            ArashiError::NoPhysicalDevice,
            ArashiError::DeviceLost {
                detail: "x".into(),
            },
            ArashiError::SwapchainOutOfDate,
            ArashiError::AtlasFull {
                used: 1,
                capacity: 2,
            },
            ArashiError::OutOfMemory {
                detail: "x".into(),
            },
        ] {
            let h = e.recovery_hint();
            assert!(!h.is_empty());
        }
    }

    #[test]
    fn error_from_io_not_found() {
        let io = io::Error::new(io::ErrorKind::NotFound, "missing");
        let e: ArashiError = io.into();
        assert_eq!(e.code(), ErrorCode::FileNotFound);
    }

    #[test]
    fn error_from_io_permission_denied() {
        let io = io::Error::new(io::ErrorKind::PermissionDenied, "nope");
        let e: ArashiError = io.into();
        assert_eq!(e.code(), ErrorCode::FileAccessDenied);
    }

    #[test]
    fn error_from_io_already_exists() {
        let io = io::Error::new(io::ErrorKind::AlreadyExists, "dup");
        let e: ArashiError = io.into();
        assert_eq!(e.code(), ErrorCode::FileAlreadyExists);
    }

    #[test]
    fn error_from_io_would_block() {
        let io = io::Error::new(io::ErrorKind::WouldBlock, "wait");
        let e: ArashiError = io.into();
        assert_eq!(e.code(), ErrorCode::WouldBlock);
    }

    #[test]
    fn error_from_io_interrupted() {
        let io = io::Error::new(io::ErrorKind::Interrupted, "sig");
        let e: ArashiError = io.into();
        assert_eq!(e.code(), ErrorCode::Interrupted);
    }

    #[test]
    fn error_from_io_timed_out() {
        let io = io::Error::new(io::ErrorKind::TimedOut, "late");
        let e: ArashiError = io.into();
        assert_eq!(e.code(), ErrorCode::Timeout);
    }

    #[test]
    fn error_from_io_out_of_memory() {
        let io = io::Error::new(io::ErrorKind::OutOfMemory, "oom");
        let e: ArashiError = io.into();
        assert_eq!(e.code(), ErrorCode::OutOfMemory);
    }

    #[test]
    fn error_from_io_other() {
        let io = io::Error::new(io::ErrorKind::Other, "misc");
        let e: ArashiError = io.into();
        assert_eq!(e.code(), ErrorCode::IoFailed);
    }

    #[test]
    fn error_context_empty_chain() {
        let ctx = ErrorContext::new(ArashiError::NotInitialized);
        assert_eq!(ctx.depth(), 0);
        assert_eq!(ctx.code(), ErrorCode::NotInitialized);
    }

    #[test]
    fn error_context_with_one() {
        let ctx = ErrorContext::new(ArashiError::LoaderNotFound).with("initializing");
        assert_eq!(ctx.depth(), 1);
        assert_eq!(ctx.chain()[0], "initializing");
    }

    #[test]
    fn error_context_with_many() {
        let ctx = ErrorContext::new(ArashiError::DeviceLost {
            detail: "gpu reset".into(),
        })
        .with("frame 42")
        .with("drawing cells")
        .with("main loop");
        assert_eq!(ctx.depth(), 3);
        assert_eq!(ctx.chain()[0], "frame 42");
        assert_eq!(ctx.chain()[1], "drawing cells");
        assert_eq!(ctx.chain()[2], "main loop");
    }

    #[test]
    fn error_context_formatted() {
        let ctx = ErrorContext::new(ArashiError::SwapchainOutOfDate)
            .with("resize to 1080x2400");
        let s = ctx.formatted();
        assert!(s.contains("swapchain out of date"));
        assert!(s.contains("resize to 1080x2400"));
    }

    #[test]
    fn error_context_method_context() {
        let e = ArashiError::NoPhysicalDevice;
        let ctx = e.context("initializing renderer");
        assert_eq!(ctx.depth(), 1);
    }

    #[test]
    fn error_context_debug_impl() {
        let ctx = ErrorContext::new(ArashiError::AtlasFull {
            used: 100,
            capacity: 100,
        })
        .with("packing glyph");
        let s = format!("{ctx:?}");
        assert!(s.contains("AtlasFull"));
        assert!(s.contains("packing glyph"));
    }

    #[test]
    fn error_context_recoverable_via_code() {
        let ctx = ErrorContext::new(ArashiError::SwapchainOutOfDate);
        assert!(ctx.is_recoverable());
    }

    #[test]
    fn error_context_fatal_via_code() {
        let ctx = ErrorContext::new(ArashiError::DeviceLost {
            detail: "x".into(),
        });
        assert!(ctx.is_fatal());
    }

    #[test]
    fn error_context_hint_via_code() {
        let ctx = ErrorContext::new(ArashiError::SwapchainOutOfDate);
        assert!(!ctx.hint().is_empty());
    }

    #[test]
    fn error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ArashiError>();
    }

    #[test]
    fn error_context_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ErrorContext>();
    }

    #[test]
    fn error_code_display() {
        let s = format!("{}", ErrorCode::LoaderNotFound);
        assert!(s.contains("LoaderNotFound"));
        assert!(s.contains("2000"));
    }

    #[test]
    fn error_category_display() {
        assert_eq!(ErrorCategory::General.to_string(), "general");
        assert_eq!(ErrorCategory::VulkanInit.to_string(), "vulkan-init");
        assert_eq!(ErrorCategory::Text.to_string(), "text");
    }

    #[test]
    fn error_code_all_categories_have_names() {
        for code in [
            ErrorCode::Ok,
            ErrorCode::LoaderNotFound,
            ErrorCode::SurfaceLost,
            ErrorCode::SwapchainOutOfDate,
            ErrorCode::PipelineCreationFailed,
            ErrorCode::ShaderCompilationFailed,
            ErrorCode::DescriptorPoolExhausted,
            ErrorCode::CommandPoolCreationFailed,
            ErrorCode::FenceTimeout,
            ErrorCode::BufferTooLarge,
            ErrorCode::ImageCreationFailed,
            ErrorCode::MemoryAllocationFailed,
            ErrorCode::AtlasFull,
            ErrorCode::GlesContextCreationFailed,
            ErrorCode::JniEnvMissing,
            ErrorCode::FileNotFound,
            ErrorCode::InvalidViewport,
        ] {
            assert!(!code.category().as_str().is_empty());
        }
    }

    #[test]
    fn error_code_unique_values() {
        let codes = [
            ErrorCode::Ok,
            ErrorCode::Unknown,
            ErrorCode::Internal,
            ErrorCode::NotImplemented,
            ErrorCode::InvalidState,
            ErrorCode::LoaderNotFound,
            ErrorCode::SurfaceLost,
            ErrorCode::SwapchainOutOfDate,
            ErrorCode::AtlasFull,
            ErrorCode::JniEnvMissing,
        ];
        let mut seen = std::collections::HashSet::new();
        for c in codes {
            assert!(seen.insert(c.as_u32()), "duplicate code {c:?}");
        }
    }

    #[test]
    fn error_display_never_empty() {
        let errors = [
            ArashiError::LoaderNotFound,
            ArashiError::NoPhysicalDevice,
            ArashiError::SurfaceLost,
            ArashiError::SwapchainEmpty,
            ArashiError::NotInitialized,
            ArashiError::AlreadyInitialized,
            ArashiError::JniEnvMissing,
            ArashiError::OutOfMemory {
                detail: "oom".into(),
            },
        ];
        for e in errors {
            let s = e.to_string();
            assert!(!s.is_empty(), "empty display for {e:?}");
        }
    }
}
