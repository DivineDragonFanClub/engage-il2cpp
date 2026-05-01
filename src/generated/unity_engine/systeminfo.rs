
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/systeminfo/SystemInfo.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "SystemInfo")]
#[parent(crate::system::object::Object)]
pub struct SystemInfo {}

#[cfg(feature = "unity_engine-systeminfo")]
#[::unity2::methods]
impl SystemInfo {
    #[method(name = "get_operatingSystemFamily", args = 0)]
    pub fn get_operating_system_family(
    ) -> crate::unity_engine::operatingsystemfamily::OperatingSystemFamily;

    #[method(name = "get_processorType", args = 0)]
    pub fn get_processor_type() -> ::unity2::Il2CppString;

    #[method(name = "get_deviceType", args = 0)]
    pub fn get_device_type() -> crate::unity_engine::devicetype::DeviceType;

    #[method(name = "get_graphicsDeviceVendor", args = 0)]
    pub fn get_graphics_device_vendor() -> ::unity2::Il2CppString;

    #[method(name = "get_graphicsDeviceType", args = 0)]
    pub fn get_graphics_device_type(
    ) -> crate::unity_engine::rendering::graphicsdevicetype::GraphicsDeviceType;

    #[method(name = "get_graphicsUVStartsAtTop", args = 0)]
    pub fn get_graphics_uv_starts_at_top() -> bool;

    #[method(name = "get_graphicsShaderLevel", args = 0)]
    pub fn get_graphics_shader_level() -> i32;

    #[method(name = "get_hasHiddenSurfaceRemovalOnGPU", args = 0)]
    pub fn get_has_hidden_surface_removal_on_gpu() -> bool;

    #[method(name = "get_supportsShadows", args = 0)]
    pub fn get_supports_shadows() -> bool;

    #[method(name = "get_copyTextureSupport", args = 0)]
    pub fn get_copy_texture_support(
    ) -> crate::unity_engine::rendering::copytexturesupport::CopyTextureSupport;

    #[method(name = "get_supportsRenderTargetArrayIndexFromVertexShader", args = 0)]
    pub fn get_supports_render_target_array_index_from_vertex_shader() -> bool;

    #[method(name = "get_supportedRenderTargetCount", args = 0)]
    pub fn get_supported_render_target_count() -> i32;

    #[method(name = "get_supportsMultisampledTextures", args = 0)]
    pub fn get_supports_multisampled_textures() -> i32;

    #[method(name = "get_supportsMultisampleAutoResolve", args = 0)]
    pub fn get_supports_multisample_auto_resolve() -> bool;

    #[method(name = "get_usesReversedZBuffer", args = 0)]
    pub fn get_uses_reversed_z_buffer() -> bool;

    #[method(name = "IsValidEnumValue", args = 1)]
    pub fn is_valid_enum_value(value: crate::system::r#enum::Enum) -> bool;

    #[method(name = "SupportsRenderTextureFormat", args = 1)]
    pub fn supports_render_texture_format(
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
    ) -> bool;

    #[method(name = "SupportsTextureFormat", args = 1)]
    pub fn supports_texture_format(
        format: crate::unity_engine::textureformat::TextureFormat,
    ) -> bool;

    #[method(name = "get_supportsGraphicsFence", args = 0)]
    pub fn get_supports_graphics_fence() -> bool;

    #[method(name = "get_supportsMultiview", args = 0)]
    pub fn get_supports_multiview() -> bool;

    #[method(name = "HasHiddenSurfaceRemovalOnGPU", args = 0)]
    pub fn has_hidden_surface_removal_on_gpu() -> bool;

    #[method(name = "SupportsShadows", args = 0)]
    pub fn supports_shadows() -> bool;

    #[method(name = "SupportsRenderTargetArrayIndexFromVertexShader", args = 0)]
    pub fn supports_render_target_array_index_from_vertex_shader() -> bool;

    #[method(name = "SupportedRenderTargetCount", args = 0)]
    pub fn supported_render_target_count() -> i32;

    #[method(name = "SupportsMultisampledTextures", args = 0)]
    pub fn supports_multisampled_textures() -> i32;

    #[method(name = "SupportsMultisampleAutoResolve", args = 0)]
    pub fn supports_multisample_auto_resolve() -> bool;

    #[method(name = "UsesReversedZBuffer", args = 0)]
    pub fn uses_reversed_z_buffer() -> bool;

    #[method(name = "HasRenderTextureNative", args = 1)]
    pub fn has_render_texture_native(
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
    ) -> bool;

    #[method(name = "SupportsTextureFormatNative", args = 1)]
    pub fn supports_texture_format_native(
        format: crate::unity_engine::textureformat::TextureFormat,
    ) -> bool;

    #[method(name = "SupportsGPUFence", args = 0)]
    pub fn supports_gpu_fence() -> bool;

    #[method(name = "IsFormatSupported", args = 2)]
    pub fn is_format_supported(
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        usage: crate::unity_engine::experimental::rendering::formatusage::FormatUsage,
    ) -> bool;

    #[method(name = "GetCompatibleFormat", args = 2)]
    pub fn get_compatible_format(
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        usage: crate::unity_engine::experimental::rendering::formatusage::FormatUsage,
    ) -> crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat;

    #[method(name = "GetGraphicsFormat", args = 1)]
    pub fn get_graphics_format(
        format: crate::unity_engine::experimental::rendering::defaultformat::DefaultFormat,
    ) -> crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat;

    #[method(name = "GetRenderTextureSupportedMSAASampleCount", args = 1)]
    pub fn get_render_texture_supported_msaa_sample_count(
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> i32;

    #[method(name = "SupportsMultiview", args = 0)]
    pub fn supports_multiview() -> bool;

    #[method(name = "GetRenderTextureSupportedMSAASampleCount_Injected", args = 1)]
    pub fn get_render_texture_supported_msaa_sample_count_injected(
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> i32;
}
