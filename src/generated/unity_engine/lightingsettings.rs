
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/lightingsettings/LightingSettings.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "LightingSettings")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct LightingSettings {}

#[cfg(feature = "unity_engine-lightingsettings")]
#[::unity2::methods]
impl LightingSettings {
    #[method(name = "LightingSettingsDontStripMe", args = 0)]
    pub fn lighting_settings_dont_strip_me(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Internal_Create", args = 1)]
    pub fn internal_create(self_: crate::unity_engine::lightingsettings::LightingSettings) -> ();

    #[method(name = "get_bakedGI", args = 0)]
    pub fn get_baked_gi(self) -> bool;

    #[method(name = "set_bakedGI", args = 1)]
    pub fn set_baked_gi(self, value: bool) -> ();

    #[method(name = "get_realtimeGI", args = 0)]
    pub fn get_realtime_gi(self) -> bool;

    #[method(name = "set_realtimeGI", args = 1)]
    pub fn set_realtime_gi(self, value: bool) -> ();

    #[method(name = "get_realtimeEnvironmentLighting", args = 0)]
    pub fn get_realtime_environment_lighting(self) -> bool;

    #[method(name = "set_realtimeEnvironmentLighting", args = 1)]
    pub fn set_realtime_environment_lighting(self, value: bool) -> ();
}

#[cfg(feature = "unity_engine-lightingsettings")]
impl LightingSettings {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LightingSettings),
                ::core::stringify!(new),
            )
        });
        <Self as ILightingSettingsMethods>::ctor(this);
        this
    }
}
