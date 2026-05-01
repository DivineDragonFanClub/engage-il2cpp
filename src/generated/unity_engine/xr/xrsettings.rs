
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/xr/xrsettings/XRSettings_StereoRenderingMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct XRSettings_StereoRenderingMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for XRSettings_StereoRenderingMode {
    const NAMESPACE: &'static str = "UnityEngine.XR";

    const NAME: &'static str = "XRSettings.StereoRenderingMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for XRSettings_StereoRenderingMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl XRSettings_StereoRenderingMode {
    pub fn multi_pass() -> Self {
        Self { value: 0 }
    }

    pub fn single_pass() -> Self {
        Self { value: 1 }
    }

    pub fn single_pass_instanced() -> Self {
        Self { value: 2 }
    }

    pub fn single_pass_multiview() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/xr/xrsettings/XRSettings.md")))]
#[::unity2::class(namespace = "UnityEngine.XR", name = "XRSettings")]
#[parent(crate::system::object::Object)]
pub struct XRSettings {}

#[cfg(feature = "unity_engine-xr-xrsettings")]
#[::unity2::methods]
impl XRSettings {
    #[method(name = "get_enabled", args = 0)]
    pub fn get_enabled() -> bool;

    #[method(name = "get_isDeviceActive", args = 0)]
    pub fn get_is_device_active() -> bool;

    #[method(name = "get_eyeTextureResolutionScale", args = 0)]
    pub fn get_eye_texture_resolution_scale() -> f32;

    #[method(name = "set_eyeTextureResolutionScale", args = 1)]
    pub fn set_eye_texture_resolution_scale(value: f32) -> ();

    #[method(name = "get_eyeTextureWidth", args = 0)]
    pub fn get_eye_texture_width() -> i32;

    #[method(name = "get_eyeTextureHeight", args = 0)]
    pub fn get_eye_texture_height() -> i32;

    #[method(name = "get_eyeTextureDesc", args = 0)]
    pub fn get_eye_texture_desc(
    ) -> crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor;

    #[method(name = "get_renderViewportScale", args = 0)]
    pub fn get_render_viewport_scale() -> f32;

    #[method(name = "get_renderViewportScaleInternal", args = 0)]
    pub fn get_render_viewport_scale_internal() -> f32;

    #[method(name = "get_loadedDeviceName", args = 0)]
    pub fn get_loaded_device_name() -> ::unity2::Il2CppString;

    #[method(name = "get_supportedDevices", args = 0)]
    pub fn get_supported_devices() -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "get_stereoRenderingMode", args = 0)]
    pub fn get_stereo_rendering_mode(
    ) -> crate::unity_engine::xr::xrsettings::XRSettings_StereoRenderingMode;

    #[method(name = "get_eyeTextureDesc_Injected", args = 1)]
    pub fn get_eye_texture_desc_injected(
        ret: crate::unity_engine::rendertexturedescriptor::RenderTextureDescriptor,
    ) -> ();
}
