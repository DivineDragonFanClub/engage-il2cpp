
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/reflectionprobe/ReflectionProbe_ReflectionProbeEvent.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ReflectionProbe_ReflectionProbeEvent {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ReflectionProbe_ReflectionProbeEvent {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ReflectionProbe.ReflectionProbeEvent";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ReflectionProbe_ReflectionProbeEvent {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ReflectionProbe_ReflectionProbeEvent {
    pub fn reflection_probe_added() -> Self {
        Self { value: 0 }
    }

    pub fn reflection_probe_removed() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/reflectionprobe/ReflectionProbe.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "ReflectionProbe")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct ReflectionProbe {
    #[static_field]
    #[rename(name = "reflectionProbeChanged")]
    pub reflection_probe_changed: crate::system::action_2::Action_2<
        crate::unity_engine::reflectionprobe::ReflectionProbe,
        crate::unity_engine::reflectionprobe::ReflectionProbe_ReflectionProbeEvent,
    >,
    #[static_field]
    #[rename(name = "defaultReflectionSet")]
    pub default_reflection_set:
        crate::system::action_1::Action_1<crate::unity_engine::cubemap::Cubemap>,
}

#[cfg(feature = "unity_engine-reflectionprobe")]
#[::unity2::methods]
impl ReflectionProbe {
    #[method(name = "get_type", args = 0)]
    pub fn get_type(
        self,
    ) -> crate::unity_engine::rendering::reflectionprobetype::ReflectionProbeType;

    #[method(name = "set_type", args = 1)]
    pub fn set_type(
        self,
        value: crate::unity_engine::rendering::reflectionprobetype::ReflectionProbeType,
    ) -> ();

    #[method(name = "get_size", args = 0)]
    pub fn get_size(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_size", args = 1)]
    pub fn set_size(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_center", args = 0)]
    pub fn get_center(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_center", args = 1)]
    pub fn set_center(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_nearClipPlane", args = 0)]
    pub fn get_near_clip_plane(self) -> f32;

    #[method(name = "set_nearClipPlane", args = 1)]
    pub fn set_near_clip_plane(self, value: f32) -> ();

    #[method(name = "get_farClipPlane", args = 0)]
    pub fn get_far_clip_plane(self) -> f32;

    #[method(name = "set_farClipPlane", args = 1)]
    pub fn set_far_clip_plane(self, value: f32) -> ();

    #[method(name = "get_intensity", args = 0)]
    pub fn get_intensity(self) -> f32;

    #[method(name = "set_intensity", args = 1)]
    pub fn set_intensity(self, value: f32) -> ();

    #[method(name = "get_bounds", args = 0)]
    pub fn get_bounds(self) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "get_hdr", args = 0)]
    pub fn get_hdr(self) -> bool;

    #[method(name = "set_hdr", args = 1)]
    pub fn set_hdr(self, value: bool) -> ();

    #[method(name = "get_renderDynamicObjects", args = 0)]
    pub fn get_render_dynamic_objects(self) -> bool;

    #[method(name = "set_renderDynamicObjects", args = 1)]
    pub fn set_render_dynamic_objects(self, value: bool) -> ();

    #[method(name = "get_shadowDistance", args = 0)]
    pub fn get_shadow_distance(self) -> f32;

    #[method(name = "set_shadowDistance", args = 1)]
    pub fn set_shadow_distance(self, value: f32) -> ();

    #[method(name = "get_resolution", args = 0)]
    pub fn get_resolution(self) -> i32;

    #[method(name = "set_resolution", args = 1)]
    pub fn set_resolution(self, value: i32) -> ();

    #[method(name = "get_cullingMask", args = 0)]
    pub fn get_culling_mask(self) -> i32;

    #[method(name = "set_cullingMask", args = 1)]
    pub fn set_culling_mask(self, value: i32) -> ();

    #[method(name = "get_clearFlags", args = 0)]
    pub fn get_clear_flags(
        self,
    ) -> crate::unity_engine::rendering::reflectionprobeclearflags::ReflectionProbeClearFlags;

    #[method(name = "set_clearFlags", args = 1)]
    pub fn set_clear_flags(
        self,
        value: crate::unity_engine::rendering::reflectionprobeclearflags::ReflectionProbeClearFlags,
    ) -> ();

    #[method(name = "get_backgroundColor", args = 0)]
    pub fn get_background_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "set_backgroundColor", args = 1)]
    pub fn set_background_color(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_blendDistance", args = 0)]
    pub fn get_blend_distance(self) -> f32;

    #[method(name = "set_blendDistance", args = 1)]
    pub fn set_blend_distance(self, value: f32) -> ();

    #[method(name = "get_boxProjection", args = 0)]
    pub fn get_box_projection(self) -> bool;

    #[method(name = "set_boxProjection", args = 1)]
    pub fn set_box_projection(self, value: bool) -> ();

    #[method(name = "get_mode", args = 0)]
    pub fn get_mode(
        self,
    ) -> crate::unity_engine::rendering::reflectionprobemode::ReflectionProbeMode;

    #[method(name = "set_mode", args = 1)]
    pub fn set_mode(
        self,
        value: crate::unity_engine::rendering::reflectionprobemode::ReflectionProbeMode,
    ) -> ();

    #[method(name = "get_importance", args = 0)]
    pub fn get_importance(self) -> i32;

    #[method(name = "set_importance", args = 1)]
    pub fn set_importance(self, value: i32) -> ();

    #[method(name = "get_refreshMode", args = 0)]
    pub fn get_refresh_mode(
        self,
    ) -> crate::unity_engine::rendering::reflectionproberefreshmode::ReflectionProbeRefreshMode;

    #[method(name = "set_refreshMode", args = 1)]
    pub fn set_refresh_mode(
        self,
        value : crate :: unity_engine :: rendering :: reflectionproberefreshmode :: ReflectionProbeRefreshMode,
    ) -> ();

    #[method(name = "get_timeSlicingMode", args = 0)]
    pub fn get_time_slicing_mode (self ,) -> crate :: unity_engine :: rendering :: reflectionprobetimeslicingmode :: ReflectionProbeTimeSlicingMode ;

    #[method(name = "set_timeSlicingMode", args = 1)]
    pub fn set_time_slicing_mode(
        self,
        value : crate :: unity_engine :: rendering :: reflectionprobetimeslicingmode :: ReflectionProbeTimeSlicingMode,
    ) -> ();

    #[method(name = "get_bakedTexture", args = 0)]
    pub fn get_baked_texture(self) -> crate::unity_engine::texture::Texture;

    #[method(name = "set_bakedTexture", args = 1)]
    pub fn set_baked_texture(self, value: crate::unity_engine::texture::Texture) -> ();

    #[method(name = "get_customBakedTexture", args = 0)]
    pub fn get_custom_baked_texture(self) -> crate::unity_engine::texture::Texture;

    #[method(name = "set_customBakedTexture", args = 1)]
    pub fn set_custom_baked_texture(self, value: crate::unity_engine::texture::Texture) -> ();

    #[method(name = "get_realtimeTexture", args = 0)]
    pub fn get_realtime_texture(self) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "set_realtimeTexture", args = 1)]
    pub fn set_realtime_texture(
        self,
        value: crate::unity_engine::rendertexture::RenderTexture,
    ) -> ();

    #[method(name = "get_texture", args = 0)]
    pub fn get_texture(self) -> crate::unity_engine::texture::Texture;

    #[method(name = "get_textureHDRDecodeValues", args = 0)]
    pub fn get_texture_hdr_decode_values(self) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "RenderProbe", args = 0)]
    pub fn render_probe(self) -> i32;

    #[method(name = "RenderProbe", args = 1)]
    pub fn render_probe_2(
        self,
        target_texture: crate::unity_engine::rendertexture::RenderTexture,
    ) -> i32;

    #[method(name = "IsFinishedRendering", args = 1)]
    pub fn is_finished_rendering(self, render_id: i32) -> bool;

    #[method(name = "ScheduleRender", args = 2)]
    pub fn schedule_render(
        self,
        time_slicing_mode : crate :: unity_engine :: rendering :: reflectionprobetimeslicingmode :: ReflectionProbeTimeSlicingMode,
        target_texture: crate::unity_engine::rendertexture::RenderTexture,
    ) -> i32;

    #[method(name = "BlendCubemap", args = 4)]
    pub fn blend_cubemap(
        src: crate::unity_engine::texture::Texture,
        dst: crate::unity_engine::texture::Texture,
        blend: f32,
        target: crate::unity_engine::rendertexture::RenderTexture,
    ) -> bool;

    #[method(name = "get_minBakedCubemapResolution", args = 0)]
    pub fn get_min_baked_cubemap_resolution() -> i32;

    #[method(name = "get_maxBakedCubemapResolution", args = 0)]
    pub fn get_max_baked_cubemap_resolution() -> i32;

    #[method(name = "get_defaultTextureHDRDecodeValues", args = 0)]
    pub fn get_default_texture_hdr_decode_values() -> crate::unity_engine::vector4::Vector4;

    #[method(name = "get_defaultTexture", args = 0)]
    pub fn get_default_texture() -> crate::unity_engine::texture::Texture;

    #[method(name = "add_reflectionProbeChanged", args = 1)]
    pub fn add_reflection_probe_changed(
        value: crate::system::action_2::Action_2<
            crate::unity_engine::reflectionprobe::ReflectionProbe,
            crate::unity_engine::reflectionprobe::ReflectionProbe_ReflectionProbeEvent,
        >,
    ) -> ();

    #[method(name = "remove_reflectionProbeChanged", args = 1)]
    pub fn remove_reflection_probe_changed(
        value: crate::system::action_2::Action_2<
            crate::unity_engine::reflectionprobe::ReflectionProbe,
            crate::unity_engine::reflectionprobe::ReflectionProbe_ReflectionProbeEvent,
        >,
    ) -> ();

    #[method(name = "add_defaultReflectionSet", args = 1)]
    pub fn add_default_reflection_set(
        value: crate::system::action_1::Action_1<crate::unity_engine::cubemap::Cubemap>,
    ) -> ();

    #[method(name = "remove_defaultReflectionSet", args = 1)]
    pub fn remove_default_reflection_set(
        value: crate::system::action_1::Action_1<crate::unity_engine::cubemap::Cubemap>,
    ) -> ();

    #[method(name = "CallReflectionProbeEvent", args = 2)]
    pub fn call_reflection_probe_event(
        probe: crate::unity_engine::reflectionprobe::ReflectionProbe,
        probe_event: crate::unity_engine::reflectionprobe::ReflectionProbe_ReflectionProbeEvent,
    ) -> ();

    #[method(name = "CallSetDefaultReflection", args = 1)]
    pub fn call_set_default_reflection(
        default_reflection_cubemap: crate::unity_engine::cubemap::Cubemap,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_size_Injected", args = 1)]
    pub fn get_size_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_size_Injected", args = 1)]
    pub fn set_size_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_center_Injected", args = 1)]
    pub fn get_center_injected(self, ret: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "set_center_Injected", args = 1)]
    pub fn set_center_injected(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_bounds_Injected", args = 1)]
    pub fn get_bounds_injected(self, ret: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = "get_backgroundColor_Injected", args = 1)]
    pub fn get_background_color_injected(self, ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_backgroundColor_Injected", args = 1)]
    pub fn set_background_color_injected(self, value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_textureHDRDecodeValues_Injected", args = 1)]
    pub fn get_texture_hdr_decode_values_injected(
        self,
        ret: crate::unity_engine::vector4::Vector4,
    ) -> ();

    #[method(name = "get_defaultTextureHDRDecodeValues_Injected", args = 1)]
    pub fn get_default_texture_hdr_decode_values_injected(
        ret: crate::unity_engine::vector4::Vector4,
    ) -> ();
}

#[cfg(feature = "unity_engine-reflectionprobe")]
impl ReflectionProbe {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReflectionProbe),
                ::core::stringify!(new),
            )
        });
        <Self as IReflectionProbeMethods>::ctor(this);
        this
    }
}
