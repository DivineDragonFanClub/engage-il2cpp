
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/qualitysettings/QualitySettings.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "QualitySettings")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct QualitySettings {}

#[cfg(feature = "unity_engine-qualitysettings")]
#[::unity2::methods]
impl QualitySettings {
    #[method(name = "get_pixelLightCount", args = 0)]
    pub fn get_pixel_light_count() -> i32;

    #[method(name = "set_pixelLightCount", args = 1)]
    pub fn set_pixel_light_count(value: i32) -> ();

    #[method(name = "get_shadowCascades", args = 0)]
    pub fn get_shadow_cascades() -> i32;

    #[method(name = "set_shadowCascades", args = 1)]
    pub fn set_shadow_cascades(value: i32) -> ();

    #[method(name = "get_shadowResolution", args = 0)]
    pub fn get_shadow_resolution() -> crate::unity_engine::shadowresolution::ShadowResolution;

    #[method(name = "set_shadowResolution", args = 1)]
    pub fn set_shadow_resolution(
        value: crate::unity_engine::shadowresolution::ShadowResolution,
    ) -> ();

    #[method(name = "get_shadowmaskMode", args = 0)]
    pub fn get_shadowmask_mode() -> crate::unity_engine::shadowmaskmode::ShadowmaskMode;

    #[method(name = "set_shadowmaskMode", args = 1)]
    pub fn set_shadowmask_mode(value: crate::unity_engine::shadowmaskmode::ShadowmaskMode) -> ();

    #[method(name = "set_lodBias", args = 1)]
    pub fn set_lod_bias(value: f32) -> ();

    #[method(name = "get_maximumLODLevel", args = 0)]
    pub fn get_maximum_lod_level() -> i32;

    #[method(name = "set_maximumLODLevel", args = 1)]
    pub fn set_maximum_lod_level(value: i32) -> ();

    #[method(name = "set_softParticles", args = 1)]
    pub fn set_soft_particles(value: bool) -> ();

    #[method(name = "get_vSyncCount", args = 0)]
    pub fn get_v_sync_count() -> i32;

    #[method(name = "set_vSyncCount", args = 1)]
    pub fn set_v_sync_count(value: i32) -> ();

    #[method(name = "get_antiAliasing", args = 0)]
    pub fn get_anti_aliasing() -> i32;

    #[method(name = "set_antiAliasing", args = 1)]
    pub fn set_anti_aliasing(value: i32) -> ();

    #[method(name = "set_asyncUploadTimeSlice", args = 1)]
    pub fn set_async_upload_time_slice(value: i32) -> ();

    #[method(name = "set_skinWeights", args = 1)]
    pub fn set_skin_weights(value: crate::unity_engine::skinweights::SkinWeights) -> ();

    #[method(name = "get_desiredColorSpace", args = 0)]
    pub fn get_desired_color_space() -> crate::unity_engine::colorspace::ColorSpace;

    #[method(name = "get_activeColorSpace", args = 0)]
    pub fn get_active_color_space() -> crate::unity_engine::colorspace::ColorSpace;
}
