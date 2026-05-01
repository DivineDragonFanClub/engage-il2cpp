
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/camera/Camera_CameraCallback.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Camera.CameraCallback")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct Camera_CameraCallback {}

#[cfg(feature = "unity_engine-camera")]
#[::unity2::methods]
impl Camera_CameraCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, cam: crate::unity_engine::camera::Camera) -> ();
}

#[cfg(feature = "unity_engine-camera")]
impl Camera_CameraCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Camera_CameraCallback),
                ::core::stringify!(new),
            )
        });
        <Self as ICamera_CameraCallbackMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/camera/Camera_RenderRequest.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Camera_RenderRequest {
    pub m_camera_render_mode: crate::unity_engine::camera::Camera_RenderRequestMode,
    pub m_result_rt: crate::unity_engine::rendertexture::RenderTexture,
    pub m_output_space: crate::unity_engine::camera::Camera_RenderRequestOutputSpace,
}

impl ::unity2::ClassIdentity for Camera_RenderRequest {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Camera.RenderRequest";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Camera_RenderRequest {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/camera/Camera_RenderRequestMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Camera_RenderRequestMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Camera_RenderRequestMode {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Camera.RenderRequestMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Camera_RenderRequestMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Camera_RenderRequestMode {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn object_id() -> Self {
        Self { value: 1 }
    }

    pub fn depth() -> Self {
        Self { value: 2 }
    }

    pub fn vertex_normal() -> Self {
        Self { value: 3 }
    }

    pub fn world_position() -> Self {
        Self { value: 4 }
    }

    pub fn entity_id() -> Self {
        Self { value: 5 }
    }

    pub fn base_color() -> Self {
        Self { value: 6 }
    }

    pub fn specular_color() -> Self {
        Self { value: 7 }
    }

    pub fn metallic() -> Self {
        Self { value: 8 }
    }

    pub fn emission() -> Self {
        Self { value: 9 }
    }

    pub fn normal() -> Self {
        Self { value: 10 }
    }

    pub fn smoothness() -> Self {
        Self { value: 11 }
    }

    pub fn occlusion() -> Self {
        Self { value: 12 }
    }

    pub fn diffuse_color() -> Self {
        Self { value: 13 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/camera/Camera_MonoOrStereoscopicEye.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Camera_MonoOrStereoscopicEye {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Camera_MonoOrStereoscopicEye {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Camera.MonoOrStereoscopicEye";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Camera_MonoOrStereoscopicEye {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Camera_MonoOrStereoscopicEye {
    pub fn left() -> Self {
        Self { value: 0 }
    }

    pub fn right() -> Self {
        Self { value: 1 }
    }

    pub fn mono() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/camera/Camera_GateFitMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Camera_GateFitMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Camera_GateFitMode {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Camera.GateFitMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Camera_GateFitMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Camera_GateFitMode {
    pub fn vertical() -> Self {
        Self { value: 1 }
    }

    pub fn horizontal() -> Self {
        Self { value: 2 }
    }

    pub fn fill() -> Self {
        Self { value: 3 }
    }

    pub fn overscan() -> Self {
        Self { value: 4 }
    }

    pub fn none() -> Self {
        Self { value: 0 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/camera/Camera_RenderRequestOutputSpace.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Camera_RenderRequestOutputSpace {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Camera_RenderRequestOutputSpace {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Camera.RenderRequestOutputSpace";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Camera_RenderRequestOutputSpace {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Camera_RenderRequestOutputSpace {
    pub fn screen_space() -> Self {
        Self { value: -1 }
    }

    pub fn uv0() -> Self {
        Self { value: 0 }
    }

    pub fn uv1() -> Self {
        Self { value: 1 }
    }

    pub fn uv2() -> Self {
        Self { value: 2 }
    }

    pub fn uv3() -> Self {
        Self { value: 3 }
    }

    pub fn uv4() -> Self {
        Self { value: 4 }
    }

    pub fn uv5() -> Self {
        Self { value: 5 }
    }

    pub fn uv6() -> Self {
        Self { value: 6 }
    }

    pub fn uv7() -> Self {
        Self { value: 7 }
    }

    pub fn uv8() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/camera/Camera_GateFitParameters.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Camera_GateFitParameters {}

impl ::unity2::ClassIdentity for Camera_GateFitParameters {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Camera.GateFitParameters";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Camera_GateFitParameters {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-camera")]
#[::unity2::methods(value)]
impl Camera_GateFitParameters {
    #[method(name = "get_mode", args = 0)]
    pub fn get_mode(self) -> crate::unity_engine::camera::Camera_GateFitMode;

    #[method(name = "get_aspect", args = 0)]
    pub fn get_aspect(self) -> f32;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/camera/Camera.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Camera")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct Camera {
    #[static_field]
    #[rename(name = "onPreCull")]
    pub on_pre_cull: crate::unity_engine::camera::Camera_CameraCallback,
    #[static_field]
    #[rename(name = "onPreRender")]
    pub on_pre_render: crate::unity_engine::camera::Camera_CameraCallback,
    #[static_field]
    #[rename(name = "onPostRender")]
    pub on_post_render: crate::unity_engine::camera::Camera_CameraCallback,
}

#[cfg(feature = "unity_engine-camera")]
#[::unity2::methods]
impl Camera {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_nearClipPlane", args = 0)]
    pub fn get_near_clip_plane(self) -> f32;

    #[method(name = "set_nearClipPlane", args = 1)]
    pub fn set_near_clip_plane(self, value: f32) -> ();

    #[method(name = "get_farClipPlane", args = 0)]
    pub fn get_far_clip_plane(self) -> f32;

    #[method(name = "set_farClipPlane", args = 1)]
    pub fn set_far_clip_plane(self, value: f32) -> ();

    #[method(name = "get_fieldOfView", args = 0)]
    pub fn get_field_of_view(self) -> f32;

    #[method(name = "set_fieldOfView", args = 1)]
    pub fn set_field_of_view(self, value: f32) -> ();

    #[method(name = "get_renderingPath", args = 0)]
    pub fn get_rendering_path(self) -> crate::unity_engine::renderingpath::RenderingPath;

    #[method(name = "set_renderingPath", args = 1)]
    pub fn set_rendering_path(self, value: crate::unity_engine::renderingpath::RenderingPath)
        -> ();

    #[method(name = "get_actualRenderingPath", args = 0)]
    pub fn get_actual_rendering_path(self) -> crate::unity_engine::renderingpath::RenderingPath;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "get_allowHDR", args = 0)]
    pub fn get_allow_hdr(self) -> bool;

    #[method(name = "set_allowHDR", args = 1)]
    pub fn set_allow_hdr(self, value: bool) -> ();

    #[method(name = "get_allowMSAA", args = 0)]
    pub fn get_allow_msaa(self) -> bool;

    #[method(name = "set_allowMSAA", args = 1)]
    pub fn set_allow_msaa(self, value: bool) -> ();

    #[method(name = "get_allowDynamicResolution", args = 0)]
    pub fn get_allow_dynamic_resolution(self) -> bool;

    #[method(name = "set_allowDynamicResolution", args = 1)]
    pub fn set_allow_dynamic_resolution(self, value: bool) -> ();

    #[method(name = "get_forceIntoRenderTexture", args = 0)]
    pub fn get_force_into_render_texture(self) -> bool;

    #[method(name = "set_forceIntoRenderTexture", args = 1)]
    pub fn set_force_into_render_texture(self, value: bool) -> ();

    #[method(name = "get_orthographicSize", args = 0)]
    pub fn get_orthographic_size(self) -> f32;

    #[method(name = "set_orthographicSize", args = 1)]
    pub fn set_orthographic_size(self, value: f32) -> ();

    #[method(name = "get_orthographic", args = 0)]
    pub fn get_orthographic(self) -> bool;

    #[method(name = "set_orthographic", args = 1)]
    pub fn set_orthographic(self, value: bool) -> ();

    #[method(name = "get_opaqueSortMode", args = 0)]
    pub fn get_opaque_sort_mode(
        self,
    ) -> crate::unity_engine::rendering::opaquesortmode::OpaqueSortMode;

    #[method(name = "set_opaqueSortMode", args = 1)]
    pub fn set_opaque_sort_mode(
        self,
        value: crate::unity_engine::rendering::opaquesortmode::OpaqueSortMode,
    ) -> ();

    #[method(name = "get_transparencySortMode", args = 0)]
    pub fn get_transparency_sort_mode(
        self,
    ) -> crate::unity_engine::transparencysortmode::TransparencySortMode;

    #[method(name = "set_transparencySortMode", args = 1)]
    pub fn set_transparency_sort_mode(
        self,
        value: crate::unity_engine::transparencysortmode::TransparencySortMode,
    ) -> ();

    #[method(name = "get_transparencySortAxis", args = 0)]
    pub fn get_transparency_sort_axis(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_transparencySortAxis", args = 1)]
    pub fn set_transparency_sort_axis(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "ResetTransparencySortSettings", args = 0)]
    pub fn reset_transparency_sort_settings(self) -> ();

    #[method(name = "get_depth", args = 0)]
    pub fn get_depth(self) -> f32;

    #[method(name = "set_depth", args = 1)]
    pub fn set_depth(self, value: f32) -> ();

    #[method(name = "get_aspect", args = 0)]
    pub fn get_aspect(self) -> f32;

    #[method(name = "set_aspect", args = 1)]
    pub fn set_aspect(self, value: f32) -> ();

    #[method(name = "ResetAspect", args = 0)]
    pub fn reset_aspect(self) -> ();

    #[method(name = "get_velocity", args = 0)]
    pub fn get_velocity(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_cullingMask", args = 0)]
    pub fn get_culling_mask(self) -> i32;

    #[method(name = "set_cullingMask", args = 1)]
    pub fn set_culling_mask(self, value: i32) -> ();

    #[method(name = "get_eventMask", args = 0)]
    pub fn get_event_mask(self) -> i32;

    #[method(name = "set_eventMask", args = 1)]
    pub fn set_event_mask(self, value: i32) -> ();

    #[method(name = "get_layerCullSpherical", args = 0)]
    pub fn get_layer_cull_spherical(self) -> bool;

    #[method(name = "set_layerCullSpherical", args = 1)]
    pub fn set_layer_cull_spherical(self, value: bool) -> ();

    #[method(name = "get_cameraType", args = 0)]
    pub fn get_camera_type(self) -> crate::unity_engine::cameratype::CameraType;

    #[method(name = "set_cameraType", args = 1)]
    pub fn set_camera_type(self, value: crate::unity_engine::cameratype::CameraType) -> ();

    #[method(name = "get_overrideSceneCullingMask", args = 0)]
    pub fn get_override_scene_culling_mask(self) -> u64;

    #[method(name = "set_overrideSceneCullingMask", args = 1)]
    pub fn set_override_scene_culling_mask(self, value: u64) -> ();

    #[method(name = "get_sceneCullingMask", args = 0)]
    pub fn get_scene_culling_mask(self) -> u64;

    #[method(name = "GetLayerCullDistances", args = 0)]
    pub fn get_layer_cull_distances(self) -> ::unity2::Array<f32>;

    #[method(name = "SetLayerCullDistances", args = 1)]
    pub fn set_layer_cull_distances(self, d: ::unity2::Array<f32>) -> ();

    #[method(name = "get_PreviewCullingLayer", args = 0)]
    pub fn get_preview_culling_layer() -> i32;

    #[method(name = "get_useOcclusionCulling", args = 0)]
    pub fn get_use_occlusion_culling(self) -> bool;

    #[method(name = "set_useOcclusionCulling", args = 1)]
    pub fn set_use_occlusion_culling(self, value: bool) -> ();

    #[method(name = "get_cullingMatrix", args = 0)]
    pub fn get_culling_matrix(self) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "set_cullingMatrix", args = 1)]
    pub fn set_culling_matrix(self, value: crate::unity_engine::matrix4x4::Matrix4x4) -> ();

    #[method(name = "ResetCullingMatrix", args = 0)]
    pub fn reset_culling_matrix(self) -> ();

    #[method(name = "get_backgroundColor", args = 0)]
    pub fn get_background_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_backgroundColor", args = 1)]
    pub fn set_background_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_clearFlags", args = 0)]
    pub fn get_clear_flags(self) -> crate::unity_engine::cameraclearflags::CameraClearFlags;

    #[method(name = "set_clearFlags", args = 1)]
    pub fn set_clear_flags(
        self,
        value: crate::unity_engine::cameraclearflags::CameraClearFlags,
    ) -> ();

    #[method(name = "get_depthTextureMode", args = 0)]
    pub fn get_depth_texture_mode(self) -> crate::unity_engine::depthtexturemode::DepthTextureMode;

    #[method(name = "set_depthTextureMode", args = 1)]
    pub fn set_depth_texture_mode(
        self,
        value: crate::unity_engine::depthtexturemode::DepthTextureMode,
    ) -> ();

    #[method(name = "get_clearStencilAfterLightingPass", args = 0)]
    pub fn get_clear_stencil_after_lighting_pass(self) -> bool;

    #[method(name = "set_clearStencilAfterLightingPass", args = 1)]
    pub fn set_clear_stencil_after_lighting_pass(self, value: bool) -> ();

    #[method(name = "SetReplacementShader", args = 2)]
    pub fn set_replacement_shader(
        self,
        shader: crate::unity_engine::shader::Shader,
        replacement_tag: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ResetReplacementShader", args = 0)]
    pub fn reset_replacement_shader(self) -> ();

    #[method(name = "get_projectionMatrixMode", args = 0)]
    pub fn get_projection_matrix_mode(
        self,
    ) -> crate::unity_engine::camera::Camera_ProjectionMatrixMode;

    #[method(name = "get_usePhysicalProperties", args = 0)]
    pub fn get_use_physical_properties(self) -> bool;

    #[method(name = "set_usePhysicalProperties", args = 1)]
    pub fn set_use_physical_properties(self, value: bool) -> ();

    #[method(name = "get_sensorSize", args = 0)]
    pub fn get_sensor_size(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_sensorSize", args = 1)]
    pub fn set_sensor_size(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_lensShift", args = 0)]
    pub fn get_lens_shift(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "set_lensShift", args = 1)]
    pub fn set_lens_shift(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_focalLength", args = 0)]
    pub fn get_focal_length(self) -> f32;

    #[method(name = "set_focalLength", args = 1)]
    pub fn set_focal_length(self, value: f32) -> ();

    #[method(name = "get_gateFit", args = 0)]
    pub fn get_gate_fit(self) -> crate::unity_engine::camera::Camera_GateFitMode;

    #[method(name = "set_gateFit", args = 1)]
    pub fn set_gate_fit(self, value: crate::unity_engine::camera::Camera_GateFitMode) -> ();

    #[method(name = "GetGateFittedFieldOfView", args = 0)]
    pub fn get_gate_fitted_field_of_view(self) -> f32;

    #[method(name = "GetGateFittedLensShift", args = 0)]
    pub fn get_gate_fitted_lens_shift(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "GetLocalSpaceAim", args = 0)]
    pub fn get_local_space_aim(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_rect", args = 0)]
    pub fn get_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "set_rect", args = 1)]
    pub fn set_rect(self, value: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "get_pixelRect", args = 0)]
    pub fn get_pixel_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "set_pixelRect", args = 1)]
    pub fn set_pixel_rect(self, value: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "get_pixelWidth", args = 0)]
    pub fn get_pixel_width(self) -> i32;

    #[method(name = "get_pixelHeight", args = 0)]
    pub fn get_pixel_height(self) -> i32;

    #[method(name = "get_scaledPixelWidth", args = 0)]
    pub fn get_scaled_pixel_width(self) -> i32;

    #[method(name = "get_scaledPixelHeight", args = 0)]
    pub fn get_scaled_pixel_height(self) -> i32;

    #[method(name = "get_targetTexture", args = 0)]
    pub fn get_target_texture(self) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "set_targetTexture", args = 1)]
    pub fn set_target_texture(self, value: crate::unity_engine::rendertexture::RenderTexture)
        -> ();

    #[method(name = "get_activeTexture", args = 0)]
    pub fn get_active_texture(self) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "get_targetDisplay", args = 0)]
    pub fn get_target_display(self) -> i32;

    #[method(name = "set_targetDisplay", args = 1)]
    pub fn set_target_display(self, value: i32) -> ();

    #[method(name = "SetTargetBuffersImpl", args = 2)]
    pub fn set_target_buffers_impl(
        self,
        color: crate::unity_engine::renderbuffer::RenderBuffer,
        depth: crate::unity_engine::renderbuffer::RenderBuffer,
    ) -> ();

    #[method(name = "SetTargetBuffers", args = 2)]
    pub fn set_target_buffers(
        self,
        color_buffer: crate::unity_engine::renderbuffer::RenderBuffer,
        depth_buffer: crate::unity_engine::renderbuffer::RenderBuffer,
    ) -> ();

    #[method(name = "SetTargetBuffersMRTImpl", args = 2)]
    pub fn set_target_buffers_mrt_impl(
        self,
        color: ::unity2::Array<crate::unity_engine::renderbuffer::RenderBuffer>,
        depth: crate::unity_engine::renderbuffer::RenderBuffer,
    ) -> ();

    #[method(name = "SetTargetBuffers", args = 2)]
    pub fn set_target_buffers_2(
        self,
        color_buffer: ::unity2::Array<crate::unity_engine::renderbuffer::RenderBuffer>,
        depth_buffer: crate::unity_engine::renderbuffer::RenderBuffer,
    ) -> ();

    #[method(name = "GetCameraBufferWarnings", args = 0)]
    pub fn get_camera_buffer_warnings(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "get_cameraToWorldMatrix", args = 0)]
    pub fn get_camera_to_world_matrix(self) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "get_worldToCameraMatrix", args = 0)]
    pub fn get_world_to_camera_matrix(self) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "set_worldToCameraMatrix", args = 1)]
    pub fn set_world_to_camera_matrix(self, value: crate::unity_engine::matrix4x4::Matrix4x4)
        -> ();

    #[method(name = "get_projectionMatrix", args = 0)]
    pub fn get_projection_matrix(self) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "set_projectionMatrix", args = 1)]
    pub fn set_projection_matrix(self, value: crate::unity_engine::matrix4x4::Matrix4x4) -> ();

    #[method(name = "get_nonJitteredProjectionMatrix", args = 0)]
    pub fn get_non_jittered_projection_matrix(self) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "set_nonJitteredProjectionMatrix", args = 1)]
    pub fn set_non_jittered_projection_matrix(
        self,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(
        name = "get_useJitteredProjectionMatrixForTransparentRendering",
        args = 0
    )]
    pub fn get_use_jittered_projection_matrix_for_transparent_rendering(self) -> bool;

    #[method(
        name = "set_useJitteredProjectionMatrixForTransparentRendering",
        args = 1
    )]
    pub fn set_use_jittered_projection_matrix_for_transparent_rendering(self, value: bool) -> ();

    #[method(name = "get_previousViewProjectionMatrix", args = 0)]
    pub fn get_previous_view_projection_matrix(self) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "ResetWorldToCameraMatrix", args = 0)]
    pub fn reset_world_to_camera_matrix(self) -> ();

    #[method(name = "ResetProjectionMatrix", args = 0)]
    pub fn reset_projection_matrix(self) -> ();

    #[method(name = "CalculateObliqueMatrix", args = 1)]
    pub fn calculate_oblique_matrix(
        self,
        clip_plane: crate::unity_engine::vector4::Vector4,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "WorldToScreenPoint", args = 2)]
    pub fn world_to_screen_point(
        self,
        position: crate::unity_engine::vector3::Vector3,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "WorldToViewportPoint", args = 2)]
    pub fn world_to_viewport_point(
        self,
        position: crate::unity_engine::vector3::Vector3,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "ViewportToWorldPoint", args = 2)]
    pub fn viewport_to_world_point(
        self,
        position: crate::unity_engine::vector3::Vector3,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "ScreenToWorldPoint", args = 2)]
    pub fn screen_to_world_point(
        self,
        position: crate::unity_engine::vector3::Vector3,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "WorldToScreenPoint", args = 1)]
    pub fn world_to_screen_point_2(
        self,
        position: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "WorldToViewportPoint", args = 1)]
    pub fn world_to_viewport_point_2(
        self,
        position: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "ViewportToWorldPoint", args = 1)]
    pub fn viewport_to_world_point_2(
        self,
        position: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "ScreenToWorldPoint", args = 1)]
    pub fn screen_to_world_point_2(
        self,
        position: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "ScreenToViewportPoint", args = 1)]
    pub fn screen_to_viewport_point(
        self,
        position: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "ViewportToScreenPoint", args = 1)]
    pub fn viewport_to_screen_point(
        self,
        position: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetFrustumPlaneSizeAt", args = 1)]
    pub fn get_frustum_plane_size_at(self, distance: f32) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "ViewportPointToRay", args = 2)]
    pub fn viewport_point_to_ray(
        self,
        pos: crate::unity_engine::vector2::Vector2,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
    ) -> crate::unity_engine::ray::Ray;

    #[method(name = "ViewportPointToRay", args = 2)]
    pub fn viewport_point_to_ray_2(
        self,
        pos: crate::unity_engine::vector3::Vector3,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
    ) -> crate::unity_engine::ray::Ray;

    #[method(name = "ViewportPointToRay", args = 1)]
    pub fn viewport_point_to_ray_3(
        self,
        pos: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::ray::Ray;

    #[method(name = "ScreenPointToRay", args = 2)]
    pub fn screen_point_to_ray(
        self,
        pos: crate::unity_engine::vector2::Vector2,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
    ) -> crate::unity_engine::ray::Ray;

    #[method(name = "ScreenPointToRay", args = 2)]
    pub fn screen_point_to_ray_2(
        self,
        pos: crate::unity_engine::vector3::Vector3,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
    ) -> crate::unity_engine::ray::Ray;

    #[method(name = "ScreenPointToRay", args = 1)]
    pub fn screen_point_to_ray_3(
        self,
        pos: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::ray::Ray;

    #[method(name = "CalculateFrustumCornersInternal", args = 4)]
    pub fn calculate_frustum_corners_internal(
        self,
        viewport: crate::unity_engine::rect::Rect,
        z: f32,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
        out_corners: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    ) -> ();

    #[method(name = "CalculateFrustumCorners", args = 4)]
    pub fn calculate_frustum_corners(
        self,
        viewport: crate::unity_engine::rect::Rect,
        z: f32,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
        out_corners: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    ) -> ();

    #[method(
        name = "CalculateProjectionMatrixFromPhysicalPropertiesInternal",
        args = 8
    )]
    pub fn calculate_projection_matrix_from_physical_properties_internal(
        output: crate::unity_engine::matrix4x4::Matrix4x4,
        focal_length: f32,
        sensor_size: crate::unity_engine::vector2::Vector2,
        lens_shift: crate::unity_engine::vector2::Vector2,
        near_clip: f32,
        far_clip: f32,
        gate_aspect: f32,
        gate_fit_mode: crate::unity_engine::camera::Camera_GateFitMode,
    ) -> ();

    #[method(name = "CalculateProjectionMatrixFromPhysicalProperties", args = 7)]
    pub fn calculate_projection_matrix_from_physical_properties(
        output: crate::unity_engine::matrix4x4::Matrix4x4,
        focal_length: f32,
        sensor_size: crate::unity_engine::vector2::Vector2,
        lens_shift: crate::unity_engine::vector2::Vector2,
        near_clip: f32,
        far_clip: f32,
        gate_fit_parameters: crate::unity_engine::camera::Camera_GateFitParameters,
    ) -> ();

    #[method(name = "FocalLengthToFieldOfView", args = 2)]
    pub fn focal_length_to_field_of_view(focal_length: f32, sensor_size: f32) -> f32;

    #[method(name = "FieldOfViewToFocalLength", args = 2)]
    pub fn field_of_view_to_focal_length(field_of_view: f32, sensor_size: f32) -> f32;

    #[method(name = "HorizontalToVerticalFieldOfView", args = 2)]
    pub fn horizontal_to_vertical_field_of_view(
        horizontal_field_of_view: f32,
        aspect_ratio: f32,
    ) -> f32;

    #[method(name = "VerticalToHorizontalFieldOfView", args = 2)]
    pub fn vertical_to_horizontal_field_of_view(
        vertical_field_of_view: f32,
        aspect_ratio: f32,
    ) -> f32;

    #[method(name = "get_main", args = 0)]
    pub fn get_main() -> crate::unity_engine::camera::Camera;

    #[method(name = "get_current", args = 0)]
    pub fn get_current() -> crate::unity_engine::camera::Camera;

    #[method(name = "get_scene", args = 0)]
    pub fn get_scene(self) -> crate::unity_engine::scene_management::scene::Scene;

    #[method(name = "set_scene", args = 1)]
    pub fn set_scene(self, value: crate::unity_engine::scene_management::scene::Scene) -> ();

    #[method(name = "get_stereoEnabled", args = 0)]
    pub fn get_stereo_enabled(self) -> bool;

    #[method(name = "get_stereoSeparation", args = 0)]
    pub fn get_stereo_separation(self) -> f32;

    #[method(name = "set_stereoSeparation", args = 1)]
    pub fn set_stereo_separation(self, value: f32) -> ();

    #[method(name = "get_stereoConvergence", args = 0)]
    pub fn get_stereo_convergence(self) -> f32;

    #[method(name = "set_stereoConvergence", args = 1)]
    pub fn set_stereo_convergence(self, value: f32) -> ();

    #[method(
        name = "get_areVRStereoViewMatricesWithinSingleCullTolerance",
        args = 0
    )]
    pub fn get_are_vr_stereo_view_matrices_within_single_cull_tolerance(self) -> bool;

    #[method(name = "get_stereoTargetEye", args = 0)]
    pub fn get_stereo_target_eye(
        self,
    ) -> crate::unity_engine::stereotargeteyemask::StereoTargetEyeMask;

    #[method(name = "set_stereoTargetEye", args = 1)]
    pub fn set_stereo_target_eye(
        self,
        value: crate::unity_engine::stereotargeteyemask::StereoTargetEyeMask,
    ) -> ();

    #[method(name = "get_stereoActiveEye", args = 0)]
    pub fn get_stereo_active_eye(self)
        -> crate::unity_engine::camera::Camera_MonoOrStereoscopicEye;

    #[method(name = "GetStereoNonJitteredProjectionMatrix", args = 1)]
    pub fn get_stereo_non_jittered_projection_matrix(
        self,
        eye: crate::unity_engine::camera::Camera_StereoscopicEye,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "GetStereoViewMatrix", args = 1)]
    pub fn get_stereo_view_matrix(
        self,
        eye: crate::unity_engine::camera::Camera_StereoscopicEye,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "CopyStereoDeviceProjectionMatrixToNonJittered", args = 1)]
    pub fn copy_stereo_device_projection_matrix_to_non_jittered(
        self,
        eye: crate::unity_engine::camera::Camera_StereoscopicEye,
    ) -> ();

    #[method(name = "GetStereoProjectionMatrix", args = 1)]
    pub fn get_stereo_projection_matrix(
        self,
        eye: crate::unity_engine::camera::Camera_StereoscopicEye,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "SetStereoProjectionMatrix", args = 2)]
    pub fn set_stereo_projection_matrix(
        self,
        eye: crate::unity_engine::camera::Camera_StereoscopicEye,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "ResetStereoProjectionMatrices", args = 0)]
    pub fn reset_stereo_projection_matrices(self) -> ();

    #[method(name = "SetStereoViewMatrix", args = 2)]
    pub fn set_stereo_view_matrix(
        self,
        eye: crate::unity_engine::camera::Camera_StereoscopicEye,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "ResetStereoViewMatrices", args = 0)]
    pub fn reset_stereo_view_matrices(self) -> ();

    #[method(name = "GetAllCamerasCount", args = 0)]
    pub fn get_all_cameras_count() -> i32;

    #[method(name = "GetAllCamerasImpl", args = 1)]
    pub fn get_all_cameras_impl(cam: ::unity2::Array<crate::unity_engine::camera::Camera>) -> i32;

    #[method(name = "get_allCameras", args = 0)]
    pub fn get_all_cameras() -> ::unity2::Array<crate::unity_engine::camera::Camera>;

    #[method(name = "GetAllCameras", args = 1)]
    pub fn get_all_cameras_2(cameras: ::unity2::Array<crate::unity_engine::camera::Camera>) -> i32;

    #[method(name = "RenderToCubemapImpl", args = 2)]
    pub fn render_to_cubemap_impl(
        self,
        tex: crate::unity_engine::texture::Texture,
        face_mask: i32,
    ) -> bool;

    #[method(name = "RenderToCubemap", args = 2)]
    pub fn render_to_cubemap(
        self,
        cubemap: crate::unity_engine::cubemap::Cubemap,
        face_mask: i32,
    ) -> bool;

    #[method(name = "RenderToCubemap", args = 1)]
    pub fn render_to_cubemap_2(self, cubemap: crate::unity_engine::cubemap::Cubemap) -> bool;

    #[method(name = "RenderToCubemap", args = 2)]
    pub fn render_to_cubemap_3(
        self,
        cubemap: crate::unity_engine::rendertexture::RenderTexture,
        face_mask: i32,
    ) -> bool;

    #[method(name = "RenderToCubemap", args = 1)]
    pub fn render_to_cubemap_4(
        self,
        cubemap: crate::unity_engine::rendertexture::RenderTexture,
    ) -> bool;

    #[method(name = "RenderToCubemapEyeImpl", args = 3)]
    pub fn render_to_cubemap_eye_impl(
        self,
        cubemap: crate::unity_engine::rendertexture::RenderTexture,
        face_mask: i32,
        stereo_eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
    ) -> bool;

    #[method(name = "RenderToCubemap", args = 3)]
    pub fn render_to_cubemap_5(
        self,
        cubemap: crate::unity_engine::rendertexture::RenderTexture,
        face_mask: i32,
        stereo_eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
    ) -> bool;

    #[method(name = "Render", args = 0)]
    pub fn render(self) -> ();

    #[method(name = "RenderWithShader", args = 2)]
    pub fn render_with_shader(
        self,
        shader: crate::unity_engine::shader::Shader,
        replacement_tag: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "RenderDontRestore", args = 0)]
    pub fn render_dont_restore(self) -> ();

    #[method(name = "SubmitRenderRequests", args = 1)]
    pub fn submit_render_requests(
        self,
        render_requests: crate::system::collections::generic::list_1::List_1<
            crate::unity_engine::camera::Camera_RenderRequest,
        >,
    ) -> ();

    #[method(name = "SubmitRenderRequestsInternal", args = 1)]
    pub fn submit_render_requests_internal(self, requests: crate::system::object::Object) -> ();

    #[method(name = "SetupCurrent", args = 1)]
    pub fn setup_current(cur: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(self, other: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "get_commandBufferCount", args = 0)]
    pub fn get_command_buffer_count(self) -> i32;

    #[method(name = "RemoveCommandBuffers", args = 1)]
    pub fn remove_command_buffers(
        self,
        evt: crate::unity_engine::rendering::cameraevent::CameraEvent,
    ) -> ();

    #[method(name = "RemoveAllCommandBuffers", args = 0)]
    pub fn remove_all_command_buffers(self) -> ();

    #[method(name = "AddCommandBufferImpl", args = 2)]
    pub fn add_command_buffer_impl(
        self,
        evt: crate::unity_engine::rendering::cameraevent::CameraEvent,
        buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "AddCommandBufferAsyncImpl", args = 3)]
    pub fn add_command_buffer_async_impl(
        self,
        evt: crate::unity_engine::rendering::cameraevent::CameraEvent,
        buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        queue_type: crate::unity_engine::rendering::computequeuetype::ComputeQueueType,
    ) -> ();

    #[method(name = "RemoveCommandBufferImpl", args = 2)]
    pub fn remove_command_buffer_impl(
        self,
        evt: crate::unity_engine::rendering::cameraevent::CameraEvent,
        buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "AddCommandBuffer", args = 2)]
    pub fn add_command_buffer(
        self,
        evt: crate::unity_engine::rendering::cameraevent::CameraEvent,
        buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "AddCommandBufferAsync", args = 3)]
    pub fn add_command_buffer_async(
        self,
        evt: crate::unity_engine::rendering::cameraevent::CameraEvent,
        buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
        queue_type: crate::unity_engine::rendering::computequeuetype::ComputeQueueType,
    ) -> ();

    #[method(name = "RemoveCommandBuffer", args = 2)]
    pub fn remove_command_buffer(
        self,
        evt: crate::unity_engine::rendering::cameraevent::CameraEvent,
        buffer: crate::unity_engine::rendering::commandbuffer::CommandBuffer,
    ) -> ();

    #[method(name = "GetCommandBuffers", args = 1)]
    pub fn get_command_buffers(
        self,
        evt: crate::unity_engine::rendering::cameraevent::CameraEvent,
    ) -> ::unity2::Array<crate::unity_engine::rendering::commandbuffer::CommandBuffer>;

    #[method(name = "FireOnPreCull", args = 1)]
    pub fn fire_on_pre_cull(cam: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "FireOnPreRender", args = 1)]
    pub fn fire_on_pre_render(cam: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "FireOnPostRender", args = 1)]
    pub fn fire_on_post_render(cam: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "OnlyUsedForTesting1", args = 0)]
    pub fn only_used_for_testing1(self) -> ();

    #[method(name = "OnlyUsedForTesting2", args = 0)]
    pub fn only_used_for_testing2(self) -> ();

    #[method(name = "TryGetCullingParameters", args = 1)]
    pub fn try_get_culling_parameters(
        self,
        culling_parameters : crate :: unity_engine :: rendering :: scriptablecullingparameters :: ScriptableCullingParameters,
    ) -> bool;

    #[method(name = "TryGetCullingParameters", args = 2)]
    pub fn try_get_culling_parameters_2(
        self,
        stereo_aware: bool,
        culling_parameters : crate :: unity_engine :: rendering :: scriptablecullingparameters :: ScriptableCullingParameters,
    ) -> bool;

    #[method(name = "GetCullingParameters_Internal", args = 4)]
    pub fn get_culling_parameters_internal(
        camera: crate::unity_engine::camera::Camera,
        stereo_aware: bool,
        culling_parameters : crate :: unity_engine :: rendering :: scriptablecullingparameters :: ScriptableCullingParameters,
        managed_culling_parameters_size: i32,
    ) -> bool;

    #[method(name = "get_transparencySortAxis_Injected", args = 1)]
    pub fn get_transparency_sort_axis_injected(
        self,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "set_transparencySortAxis_Injected", args = 1)]
    pub fn set_transparency_sort_axis_injected(
        self,
        value: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "get_velocity_Injected", args = 1)]
    pub fn get_velocity_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_cullingMatrix_Injected", args = 1)]
    pub fn get_culling_matrix_injected(self, ret: crate::unity_engine::matrix4x4::Matrix4x4) -> ();

    #[method(name = "set_cullingMatrix_Injected", args = 1)]
    pub fn set_culling_matrix_injected(
        self,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "get_backgroundColor_Injected", args = 1)]
    pub fn get_background_color_injected(self, ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_backgroundColor_Injected", args = 1)]
    pub fn set_background_color_injected(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_sensorSize_Injected", args = 1)]
    pub fn get_sensor_size_injected(self, ret: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "set_sensorSize_Injected", args = 1)]
    pub fn set_sensor_size_injected(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "get_lensShift_Injected", args = 1)]
    pub fn get_lens_shift_injected(self, ret: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "set_lensShift_Injected", args = 1)]
    pub fn set_lens_shift_injected(self, value: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "GetGateFittedLensShift_Injected", args = 1)]
    pub fn get_gate_fitted_lens_shift_injected(
        self,
        ret: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "GetLocalSpaceAim_Injected", args = 1)]
    pub fn get_local_space_aim_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_rect_Injected", args = 1)]
    pub fn get_rect_injected(self, ret: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "set_rect_Injected", args = 1)]
    pub fn set_rect_injected(self, value: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "get_pixelRect_Injected", args = 1)]
    pub fn get_pixel_rect_injected(self, ret: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "set_pixelRect_Injected", args = 1)]
    pub fn set_pixel_rect_injected(self, value: crate::unity_engine::rect::Rect) -> ();

    #[method(name = "SetTargetBuffersImpl_Injected", args = 2)]
    pub fn set_target_buffers_impl_injected(
        self,
        color: crate::unity_engine::renderbuffer::RenderBuffer,
        depth: crate::unity_engine::renderbuffer::RenderBuffer,
    ) -> ();

    #[method(name = "SetTargetBuffersMRTImpl_Injected", args = 2)]
    pub fn set_target_buffers_mrt_impl_injected(
        self,
        color: ::unity2::Array<crate::unity_engine::renderbuffer::RenderBuffer>,
        depth: crate::unity_engine::renderbuffer::RenderBuffer,
    ) -> ();

    #[method(name = "get_cameraToWorldMatrix_Injected", args = 1)]
    pub fn get_camera_to_world_matrix_injected(
        self,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "get_worldToCameraMatrix_Injected", args = 1)]
    pub fn get_world_to_camera_matrix_injected(
        self,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "set_worldToCameraMatrix_Injected", args = 1)]
    pub fn set_world_to_camera_matrix_injected(
        self,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "get_projectionMatrix_Injected", args = 1)]
    pub fn get_projection_matrix_injected(
        self,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "set_projectionMatrix_Injected", args = 1)]
    pub fn set_projection_matrix_injected(
        self,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "get_nonJitteredProjectionMatrix_Injected", args = 1)]
    pub fn get_non_jittered_projection_matrix_injected(
        self,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "set_nonJitteredProjectionMatrix_Injected", args = 1)]
    pub fn set_non_jittered_projection_matrix_injected(
        self,
        value: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "get_previousViewProjectionMatrix_Injected", args = 1)]
    pub fn get_previous_view_projection_matrix_injected(
        self,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "CalculateObliqueMatrix_Injected", args = 2)]
    pub fn calculate_oblique_matrix_injected(
        self,
        clip_plane: crate::unity_engine::vector4::Vector4,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "WorldToScreenPoint_Injected", args = 3)]
    pub fn world_to_screen_point_injected(
        self,
        position: crate::unity_engine::vector3::Vector3,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "WorldToViewportPoint_Injected", args = 3)]
    pub fn world_to_viewport_point_injected(
        self,
        position: crate::unity_engine::vector3::Vector3,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "ViewportToWorldPoint_Injected", args = 3)]
    pub fn viewport_to_world_point_injected(
        self,
        position: crate::unity_engine::vector3::Vector3,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "ScreenToWorldPoint_Injected", args = 3)]
    pub fn screen_to_world_point_injected(
        self,
        position: crate::unity_engine::vector3::Vector3,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "ScreenToViewportPoint_Injected", args = 2)]
    pub fn screen_to_viewport_point_injected(
        self,
        position: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "ViewportToScreenPoint_Injected", args = 2)]
    pub fn viewport_to_screen_point_injected(
        self,
        position: crate::unity_engine::vector3::Vector3,
        ret: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "GetFrustumPlaneSizeAt_Injected", args = 2)]
    pub fn get_frustum_plane_size_at_injected(
        self,
        distance: f32,
        ret: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "ViewportPointToRay_Injected", args = 3)]
    pub fn viewport_point_to_ray_injected(
        self,
        pos: crate::unity_engine::vector2::Vector2,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
        ret: crate::unity_engine::ray::Ray,
    ) -> ();

    #[method(name = "ScreenPointToRay_Injected", args = 3)]
    pub fn screen_point_to_ray_injected(
        self,
        pos: crate::unity_engine::vector2::Vector2,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
        ret: crate::unity_engine::ray::Ray,
    ) -> ();

    #[method(name = "CalculateFrustumCornersInternal_Injected", args = 4)]
    pub fn calculate_frustum_corners_internal_injected(
        self,
        viewport: crate::unity_engine::rect::Rect,
        z: f32,
        eye: crate::unity_engine::camera::Camera_MonoOrStereoscopicEye,
        out_corners: ::unity2::Array<crate::unity_engine::vector3::Vector3>,
    ) -> ();

    #[method(
        name = "CalculateProjectionMatrixFromPhysicalPropertiesInternal_Injected",
        args = 8
    )]
    pub fn calculate_projection_matrix_from_physical_properties_internal_injected(
        output: crate::unity_engine::matrix4x4::Matrix4x4,
        focal_length: f32,
        sensor_size: crate::unity_engine::vector2::Vector2,
        lens_shift: crate::unity_engine::vector2::Vector2,
        near_clip: f32,
        far_clip: f32,
        gate_aspect: f32,
        gate_fit_mode: crate::unity_engine::camera::Camera_GateFitMode,
    ) -> ();

    #[method(name = "get_scene_Injected", args = 1)]
    pub fn get_scene_injected(self, ret: crate::unity_engine::scene_management::scene::Scene)
        -> ();

    #[method(name = "set_scene_Injected", args = 1)]
    pub fn set_scene_injected(
        self,
        value: crate::unity_engine::scene_management::scene::Scene,
    ) -> ();

    #[method(name = "GetStereoNonJitteredProjectionMatrix_Injected", args = 2)]
    pub fn get_stereo_non_jittered_projection_matrix_injected(
        self,
        eye: crate::unity_engine::camera::Camera_StereoscopicEye,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "GetStereoViewMatrix_Injected", args = 2)]
    pub fn get_stereo_view_matrix_injected(
        self,
        eye: crate::unity_engine::camera::Camera_StereoscopicEye,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "GetStereoProjectionMatrix_Injected", args = 2)]
    pub fn get_stereo_projection_matrix_injected(
        self,
        eye: crate::unity_engine::camera::Camera_StereoscopicEye,
        ret: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "SetStereoProjectionMatrix_Injected", args = 2)]
    pub fn set_stereo_projection_matrix_injected(
        self,
        eye: crate::unity_engine::camera::Camera_StereoscopicEye,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();

    #[method(name = "SetStereoViewMatrix_Injected", args = 2)]
    pub fn set_stereo_view_matrix_injected(
        self,
        eye: crate::unity_engine::camera::Camera_StereoscopicEye,
        matrix: crate::unity_engine::matrix4x4::Matrix4x4,
    ) -> ();
}

#[cfg(feature = "unity_engine-camera")]
impl Camera {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Camera),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/camera/Camera_StereoscopicEye.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Camera_StereoscopicEye {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Camera_StereoscopicEye {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Camera.StereoscopicEye";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Camera_StereoscopicEye {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Camera_StereoscopicEye {
    pub fn left() -> Self {
        Self { value: 0 }
    }

    pub fn right() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/camera/Camera_ProjectionMatrixMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Camera_ProjectionMatrixMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Camera_ProjectionMatrixMode {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Camera.ProjectionMatrixMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Camera_ProjectionMatrixMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Camera_ProjectionMatrixMode {
    pub fn explicit() -> Self {
        Self { value: 0 }
    }

    pub fn implicit() -> Self {
        Self { value: 1 }
    }

    pub fn physical_properties_based() -> Self {
        Self { value: 2 }
    }
}
