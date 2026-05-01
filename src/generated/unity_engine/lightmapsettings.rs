
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/lightmapsettings/LightmapSettings.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "LightmapSettings")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct LightmapSettings {}

#[cfg(feature = "unity_engine-lightmapsettings")]
#[::unity2::methods]
impl LightmapSettings {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_lightmaps", args = 0)]
    pub fn get_lightmaps() -> ::unity2::Array<crate::unity_engine::lightmapdata::LightmapData>;

    #[method(name = "set_lightmaps", args = 1)]
    pub fn set_lightmaps(
        value: ::unity2::Array<crate::unity_engine::lightmapdata::LightmapData>,
    ) -> ();

    #[method(name = "get_lightmapsMode", args = 0)]
    pub fn get_lightmaps_mode() -> crate::unity_engine::lightmapsmode::LightmapsMode;

    #[method(name = "set_lightmapsMode", args = 1)]
    pub fn set_lightmaps_mode(value: crate::unity_engine::lightmapsmode::LightmapsMode) -> ();

    #[method(name = "get_lightProbes", args = 0)]
    pub fn get_light_probes() -> crate::unity_engine::lightprobes::LightProbes;

    #[method(name = "set_lightProbes", args = 1)]
    pub fn set_light_probes(value: crate::unity_engine::lightprobes::LightProbes) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset() -> ();

    #[method(name = "get_lightmapsModeLegacy", args = 0)]
    pub fn get_lightmaps_mode_legacy(
    ) -> crate::unity_engine::lightmapsmodelegacy::LightmapsModeLegacy;

    #[method(name = "set_lightmapsModeLegacy", args = 1)]
    pub fn set_lightmaps_mode_legacy(
        value: crate::unity_engine::lightmapsmodelegacy::LightmapsModeLegacy,
    ) -> ();

    #[method(name = "get_bakedColorSpace", args = 0)]
    pub fn get_baked_color_space() -> crate::unity_engine::colorspace::ColorSpace;

    #[method(name = "set_bakedColorSpace", args = 1)]
    pub fn set_baked_color_space(value: crate::unity_engine::colorspace::ColorSpace) -> ();
}

#[cfg(feature = "unity_engine-lightmapsettings")]
impl LightmapSettings {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LightmapSettings),
                ::core::stringify!(new),
            )
        });
        <Self as ILightmapSettingsMethods>::ctor(this);
        this
    }
}
