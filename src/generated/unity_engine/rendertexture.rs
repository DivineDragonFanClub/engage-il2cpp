
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::texture::ITexture;
use crate::unity_engine::texture::Texture;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendertexture/RenderTexture.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "RenderTexture")]
#[parent(crate::unity_engine::texture::Texture)]
pub struct RenderTexture {}

#[cfg(feature = "unity_engine-rendertexture")]
#[::unity2::methods]
impl RenderTexture {
    #[method(name = "get_width", args = 0)]
    pub fn get_width(self) -> i32;

    #[method(name = "set_width", args = 1)]
    pub fn set_width(self, value: i32) -> ();

    #[method(name = "get_height", args = 0)]
    pub fn get_height(self) -> i32;

    #[method(name = "set_height", args = 1)]
    pub fn set_height(self, value: i32) -> ();

    #[method(name = "get_dimension", args = 0)]
    pub fn get_dimension(
        self,
    ) -> crate::unity_engine::rendering::texturedimension::TextureDimension;

    #[method(name = "set_dimension", args = 1)]
    pub fn set_dimension(
        self,
        value: crate::unity_engine::rendering::texturedimension::TextureDimension,
    ) -> ();

    #[method(name = "get_graphicsFormat", args = 0)]
    pub fn get_graphics_format(
        self,
    ) -> crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat;

    #[method(name = "set_graphicsFormat", args = 1)]
    pub fn set_graphics_format(
        self,
        value: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
    ) -> ();

    #[method(name = "get_useMipMap", args = 0)]
    pub fn get_use_mip_map(self) -> bool;

    #[method(name = "set_useMipMap", args = 1)]
    pub fn set_use_mip_map(self, value: bool) -> ();

    #[method(name = "get_sRGB", args = 0)]
    pub fn get_s_rgb(self) -> bool;

    #[method(name = "get_vrUsage", args = 0)]
    pub fn get_vr_usage(self) -> crate::unity_engine::vrtextureusage::VRTextureUsage;

    #[method(name = "set_vrUsage", args = 1)]
    pub fn set_vr_usage(self, value: crate::unity_engine::vrtextureusage::VRTextureUsage) -> ();

    #[method(name = "get_memorylessMode", args = 0)]
    pub fn get_memoryless_mode(
        self,
    ) -> crate::unity_engine::rendertexturememoryless::RenderTextureMemoryless;

    #[method(name = "set_memorylessMode", args = 1)]
    pub fn set_memoryless_mode(
        self,
        value: crate::unity_engine::rendertexturememoryless::RenderTextureMemoryless,
    ) -> ();

    #[method(name = "get_format", args = 0)]
    pub fn get_format(self) -> crate::unity_engine::rendertextureformat::RenderTextureFormat;

    #[method(name = "set_format", args = 1)]
    pub fn set_format(
        self,
        value: crate::unity_engine::rendertextureformat::RenderTextureFormat,
    ) -> ();

    #[method(name = "get_stencilFormat", args = 0)]
    pub fn get_stencil_format(
        self,
    ) -> crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat;

    #[method(name = "set_stencilFormat", args = 1)]
    pub fn set_stencil_format(
        self,
        value: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
    ) -> ();

    #[method(name = "get_autoGenerateMips", args = 0)]
    pub fn get_auto_generate_mips(self) -> bool;

    #[method(name = "set_autoGenerateMips", args = 1)]
    pub fn set_auto_generate_mips(self, value: bool) -> ();

    #[method(name = "get_volumeDepth", args = 0)]
    pub fn get_volume_depth(self) -> i32;

    #[method(name = "set_volumeDepth", args = 1)]
    pub fn set_volume_depth(self, value: i32) -> ();

    #[method(name = "get_antiAliasing", args = 0)]
    pub fn get_anti_aliasing(self) -> i32;

    #[method(name = "set_antiAliasing", args = 1)]
    pub fn set_anti_aliasing(self, value: i32) -> ();

    #[method(name = "get_bindTextureMS", args = 0)]
    pub fn get_bind_texture_ms(self) -> bool;

    #[method(name = "set_bindTextureMS", args = 1)]
    pub fn set_bind_texture_ms(self, value: bool) -> ();

    #[method(name = "get_enableRandomWrite", args = 0)]
    pub fn get_enable_random_write(self) -> bool;

    #[method(name = "set_enableRandomWrite", args = 1)]
    pub fn set_enable_random_write(self, value: bool) -> ();

    #[method(name = "get_useDynamicScale", args = 0)]
    pub fn get_use_dynamic_scale(self) -> bool;

    #[method(name = "set_useDynamicScale", args = 1)]
    pub fn set_use_dynamic_scale(self, value: bool) -> ();

    #[method(name = "GetIsPowerOfTwo", args = 0)]
    pub fn get_is_power_of_two(self) -> bool;

    #[method(name = "set_isPowerOfTwo", args = 1)]
    pub fn set_is_power_of_two(self, value: bool) -> ();

    #[method(name = "GetActive", args = 0)]
    pub fn get_active() -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "SetActive", args = 1)]
    pub fn set_active(rt: crate::unity_engine::rendertexture::RenderTexture) -> ();

    #[method(name = "GetColorBuffer", args = 0)]
    pub fn get_color_buffer(self) -> crate::unity_engine::renderbuffer::RenderBuffer;

    #[method(name = "GetDepthBuffer", args = 0)]
    pub fn get_depth_buffer(self) -> crate::unity_engine::renderbuffer::RenderBuffer;

    #[method(name = "GetNativeDepthBufferPtr", args = 0)]
    pub fn get_native_depth_buffer_ptr(self) -> ::unity2::IntPtr;

    #[method(name = "DiscardContents", args = 2)]
    pub fn discard_contents(self, discard_color: bool, discard_depth: bool) -> ();

    #[method(name = "MarkRestoreExpected", args = 0)]
    pub fn mark_restore_expected(self) -> ();

    #[method(name = "DiscardContents", args = 0)]
    pub fn discard_contents_2(self) -> ();

    #[method(name = "ResolveAA", args = 0)]
    pub fn resolve_aa(self) -> ();

    #[method(name = "ResolveAATo", args = 1)]
    pub fn resolve_aa_to(self, rt: crate::unity_engine::rendertexture::RenderTexture) -> ();

    #[method(name = "ResolveAntiAliasedSurface", args = 0)]
    pub fn resolve_anti_aliased_surface(self) -> ();

    #[method(name = "ResolveAntiAliasedSurface", args = 1)]
    pub fn resolve_anti_aliased_surface_2(
        self,
        target: crate::unity_engine::rendertexture::RenderTexture,
    ) -> ();

    #[method(name = "SetGlobalShaderProperty", args = 1)]
    pub fn set_global_shader_property(self, property_name: ::unity2::Il2CppString) -> ();

    #[method(name = "Create", args = 0)]
    pub fn create(self) -> bool;

    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = "IsCreated", args = 0)]
    pub fn is_created(self) -> bool;

    #[method(name = "GenerateMips", args = 0)]
    pub fn generate_mips(self) -> ();

    #[method(name = "ConvertToEquirect", args = 2)]
    pub fn convert_to_equirect(
        self,
        equirect: crate::unity_engine::rendertexture::RenderTexture,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
    ) -> ();

    #[method(name = "SetSRGBReadWrite", args = 1)]
    pub fn set_srgb_read_write(self, srgb: bool) -> ();

    #[method(name = "Internal_Create", args = 1)]
    pub fn internal_create(rt: crate::unity_engine::rendertexture::RenderTexture) -> ();

    #[method(name = "SupportsStencil", args = 1)]
    pub fn supports_stencil(rt: crate::unity_engine::rendertexture::RenderTexture) -> bool;

    #[method(name = "SetRenderTextureDescriptor", args = 1)]
    pub fn set_render_texture_descriptor(
        self,
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> ();

    #[method(name = "GetDescriptor", args = 0)]
    pub fn get_descriptor(
        self,
    ) -> crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor;

    #[method(name = "GetTemporary_Internal", args = 1)]
    pub fn get_temporary_internal(
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "ReleaseTemporary", args = 1)]
    pub fn release_temporary(temp: crate::unity_engine::rendertexture::RenderTexture) -> ();

    #[method(name = "get_depth", args = 0)]
    pub fn get_depth(self) -> i32;

    #[method(name = "set_depth", args = 1)]
    pub fn set_depth(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, texture_to_copy: crate::unity_engine::rendertexture::RenderTexture) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_4(
        self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::experimental::rendering::defaultformat::DefaultFormat,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_5(
        self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor_6(
        self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        mip_count: i32,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor_7(
        self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        read_write: crate::unity_engine::rendertexturereadwrite::RenderTextureReadWrite,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_8(
        self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_9(self, width: i32, height: i32, depth: i32) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor_10(
        self,
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        mip_count: i32,
    ) -> ();

    #[method(name = "set_descriptor", args = 1)]
    pub fn set_descriptor(
        self,
        value: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> ();

    #[method(name = "ValidateRenderTextureDesc", args = 1)]
    pub fn validate_render_texture_desc(
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> ();

    #[method(name = "GetCompatibleFormat", args = 2)]
    pub fn get_compatible_format(
        render_texture_format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        read_write: crate::unity_engine::rendertexturereadwrite::RenderTextureReadWrite,
    ) -> crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat;

    #[method(name = "GetTemporary", args = 1)]
    pub fn get_temporary(
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetTemporaryImpl", args = 8)]
    pub fn get_temporary_impl(
        width: i32,
        height: i32,
        depth_buffer: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        anti_aliasing: i32,
        memoryless_mode: crate::unity_engine::rendertexturememoryless::RenderTextureMemoryless,
        vr_usage: crate::unity_engine::vrtextureusage::VRTextureUsage,
        use_dynamic_scale: bool,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetTemporary", args = 8)]
    pub fn get_temporary_2(
        width: i32,
        height: i32,
        depth_buffer: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        anti_aliasing: i32,
        memoryless_mode: crate::unity_engine::rendertexturememoryless::RenderTextureMemoryless,
        vr_usage: crate::unity_engine::vrtextureusage::VRTextureUsage,
        use_dynamic_scale: bool,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetTemporary", args = 7)]
    pub fn get_temporary_3(
        width: i32,
        height: i32,
        depth_buffer: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        anti_aliasing: i32,
        memoryless_mode: crate::unity_engine::rendertexturememoryless::RenderTextureMemoryless,
        vr_usage: crate::unity_engine::vrtextureusage::VRTextureUsage,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetTemporary", args = 6)]
    pub fn get_temporary_4(
        width: i32,
        height: i32,
        depth_buffer: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        anti_aliasing: i32,
        memoryless_mode: crate::unity_engine::rendertexturememoryless::RenderTextureMemoryless,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetTemporary", args = 5)]
    pub fn get_temporary_5(
        width: i32,
        height: i32,
        depth_buffer: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        anti_aliasing: i32,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetTemporary", args = 4)]
    pub fn get_temporary_6(
        width: i32,
        height: i32,
        depth_buffer: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetTemporary", args = 9)]
    pub fn get_temporary_7(
        width: i32,
        height: i32,
        depth_buffer: i32,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        read_write: crate::unity_engine::rendertexturereadwrite::RenderTextureReadWrite,
        anti_aliasing: i32,
        memoryless_mode: crate::unity_engine::rendertexturememoryless::RenderTextureMemoryless,
        vr_usage: crate::unity_engine::vrtextureusage::VRTextureUsage,
        use_dynamic_scale: bool,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetTemporary", args = 8)]
    pub fn get_temporary_8(
        width: i32,
        height: i32,
        depth_buffer: i32,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        read_write: crate::unity_engine::rendertexturereadwrite::RenderTextureReadWrite,
        anti_aliasing: i32,
        memoryless_mode: crate::unity_engine::rendertexturememoryless::RenderTextureMemoryless,
        vr_usage: crate::unity_engine::vrtextureusage::VRTextureUsage,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetTemporary", args = 7)]
    pub fn get_temporary_9(
        width: i32,
        height: i32,
        depth_buffer: i32,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        read_write: crate::unity_engine::rendertexturereadwrite::RenderTextureReadWrite,
        anti_aliasing: i32,
        memoryless_mode: crate::unity_engine::rendertexturememoryless::RenderTextureMemoryless,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetTemporary", args = 6)]
    pub fn get_temporary_10(
        width: i32,
        height: i32,
        depth_buffer: i32,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        read_write: crate::unity_engine::rendertexturereadwrite::RenderTextureReadWrite,
        anti_aliasing: i32,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetTemporary", args = 5)]
    pub fn get_temporary_11(
        width: i32,
        height: i32,
        depth_buffer: i32,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        read_write: crate::unity_engine::rendertexturereadwrite::RenderTextureReadWrite,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetTemporary", args = 4)]
    pub fn get_temporary_12(
        width: i32,
        height: i32,
        depth_buffer: i32,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetTemporary", args = 3)]
    pub fn get_temporary_13(
        width: i32,
        height: i32,
        depth_buffer: i32,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "GetTemporary", args = 2)]
    pub fn get_temporary_14(
        width: i32,
        height: i32,
    ) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "get_isCubemap", args = 0)]
    pub fn get_is_cubemap(self) -> bool;

    #[method(name = "set_isCubemap", args = 1)]
    pub fn set_is_cubemap(self, value: bool) -> ();

    #[method(name = "get_isVolume", args = 0)]
    pub fn get_is_volume(self) -> bool;

    #[method(name = "set_isVolume", args = 1)]
    pub fn set_is_volume(self, value: bool) -> ();

    #[method(name = "get_enabled", args = 0)]
    pub fn get_enabled() -> bool;

    #[method(name = "set_enabled", args = 1)]
    pub fn set_enabled(value: bool) -> ();

    #[method(name = "GetTexelOffset", args = 0)]
    pub fn get_texel_offset(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetColorBuffer_Injected", args = 1)]
    pub fn get_color_buffer_injected(
        self,
        ret: crate::unity_engine::renderbuffer::RenderBuffer,
    ) -> ();

    #[method(name = "GetDepthBuffer_Injected", args = 1)]
    pub fn get_depth_buffer_injected(
        self,
        ret: crate::unity_engine::renderbuffer::RenderBuffer,
    ) -> ();

    #[method(name = "SetRenderTextureDescriptor_Injected", args = 1)]
    pub fn set_render_texture_descriptor_injected(
        self,
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> ();

    #[method(name = "GetDescriptor_Injected", args = 1)]
    pub fn get_descriptor_injected(
        self,
        ret: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> ();

    #[method(name = "GetTemporary_Internal_Injected", args = 1)]
    pub fn get_temporary_internal_injected(
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> crate::unity_engine::rendertexture::RenderTexture;
}

#[cfg(feature = "unity_engine-rendertexture")]
impl RenderTexture {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderTexture),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderTextureMethods>::ctor(this);
        this
    }

    pub fn new_2(
        desc: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderTexture),
                ::core::stringify!(new_2),
            )
        });
        <Self as IRenderTextureMethods>::ctor_2(this, desc);
        this
    }

    pub fn new_3(texture_to_copy: crate::unity_engine::rendertexture::RenderTexture) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderTexture),
                ::core::stringify!(new_3),
            )
        });
        <Self as IRenderTextureMethods>::ctor_3(this, texture_to_copy);
        this
    }

    pub fn new_4(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::experimental::rendering::defaultformat::DefaultFormat,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderTexture),
                ::core::stringify!(new_4),
            )
        });
        <Self as IRenderTextureMethods>::ctor_4(this, width, height, depth, format);
        this
    }

    pub fn new_5(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderTexture),
                ::core::stringify!(new_5),
            )
        });
        <Self as IRenderTextureMethods>::ctor_5(this, width, height, depth, format);
        this
    }

    pub fn new_6(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::experimental::rendering::graphicsformat::GraphicsFormat,
        mip_count: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderTexture),
                ::core::stringify!(new_6),
            )
        });
        <Self as IRenderTextureMethods>::ctor_6(this, width, height, depth, format, mip_count);
        this
    }

    pub fn new_7(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        read_write: crate::unity_engine::rendertexturereadwrite::RenderTextureReadWrite,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderTexture),
                ::core::stringify!(new_7),
            )
        });
        <Self as IRenderTextureMethods>::ctor_7(this, width, height, depth, format, read_write);
        this
    }

    pub fn new_8(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderTexture),
                ::core::stringify!(new_8),
            )
        });
        <Self as IRenderTextureMethods>::ctor_8(this, width, height, depth, format);
        this
    }

    pub fn new_9(width: i32, height: i32, depth: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderTexture),
                ::core::stringify!(new_9),
            )
        });
        <Self as IRenderTextureMethods>::ctor_9(this, width, height, depth);
        this
    }

    pub fn new_10(
        width: i32,
        height: i32,
        depth: i32,
        format: crate::unity_engine::rendertextureformat::RenderTextureFormat,
        mip_count: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderTexture),
                ::core::stringify!(new_10),
            )
        });
        <Self as IRenderTextureMethods>::ctor_10(this, width, height, depth, format, mip_count);
        this
    }
}
