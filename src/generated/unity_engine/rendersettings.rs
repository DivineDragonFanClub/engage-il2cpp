
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendersettings/RenderSettings.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "RenderSettings")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct RenderSettings {}

#[cfg(feature = "unity_engine-rendersettings")]
#[::unity2::methods]
impl RenderSettings {
    #[method(name = "get_ambientSkyboxAmount", args = 0)]
    pub fn get_ambient_skybox_amount() -> f32;

    #[method(name = "set_ambientSkyboxAmount", args = 1)]
    pub fn set_ambient_skybox_amount(value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_fog", args = 0)]
    pub fn get_fog() -> bool;

    #[method(name = "set_fog", args = 1)]
    pub fn set_fog(value: bool) -> ();

    #[method(name = "get_fogStartDistance", args = 0)]
    pub fn get_fog_start_distance() -> f32;

    #[method(name = "set_fogStartDistance", args = 1)]
    pub fn set_fog_start_distance(value: f32) -> ();

    #[method(name = "get_fogEndDistance", args = 0)]
    pub fn get_fog_end_distance() -> f32;

    #[method(name = "set_fogEndDistance", args = 1)]
    pub fn set_fog_end_distance(value: f32) -> ();

    #[method(name = "get_fogMode", args = 0)]
    pub fn get_fog_mode() -> crate::unity_engine::fogmode::FogMode;

    #[method(name = "set_fogMode", args = 1)]
    pub fn set_fog_mode(value: crate::unity_engine::fogmode::FogMode) -> ();

    #[method(name = "get_fogColor", args = 0)]
    pub fn get_fog_color() -> crate::unity_engine::color::Color;

    #[method(name = "set_fogColor", args = 1)]
    pub fn set_fog_color(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_fogDensity", args = 0)]
    pub fn get_fog_density() -> f32;

    #[method(name = "set_fogDensity", args = 1)]
    pub fn set_fog_density(value: f32) -> ();

    #[method(name = "get_ambientMode", args = 0)]
    pub fn get_ambient_mode() -> crate::unity_engine::rendering::ambientmode::AmbientMode;

    #[method(name = "set_ambientMode", args = 1)]
    pub fn set_ambient_mode(value: crate::unity_engine::rendering::ambientmode::AmbientMode) -> ();

    #[method(name = "get_ambientSkyColor", args = 0)]
    pub fn get_ambient_sky_color() -> crate::unity_engine::color::Color;

    #[method(name = "set_ambientSkyColor", args = 1)]
    pub fn set_ambient_sky_color(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_ambientEquatorColor", args = 0)]
    pub fn get_ambient_equator_color() -> crate::unity_engine::color::Color;

    #[method(name = "set_ambientEquatorColor", args = 1)]
    pub fn set_ambient_equator_color(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_ambientGroundColor", args = 0)]
    pub fn get_ambient_ground_color() -> crate::unity_engine::color::Color;

    #[method(name = "set_ambientGroundColor", args = 1)]
    pub fn set_ambient_ground_color(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_ambientIntensity", args = 0)]
    pub fn get_ambient_intensity() -> f32;

    #[method(name = "set_ambientIntensity", args = 1)]
    pub fn set_ambient_intensity(value: f32) -> ();

    #[method(name = "get_ambientLight", args = 0)]
    pub fn get_ambient_light() -> crate::unity_engine::color::Color;

    #[method(name = "set_ambientLight", args = 1)]
    pub fn set_ambient_light(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_subtractiveShadowColor", args = 0)]
    pub fn get_subtractive_shadow_color() -> crate::unity_engine::color::Color;

    #[method(name = "set_subtractiveShadowColor", args = 1)]
    pub fn set_subtractive_shadow_color(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_skybox", args = 0)]
    pub fn get_skybox() -> crate::unity_engine::material::Material;

    #[method(name = "set_skybox", args = 1)]
    pub fn set_skybox(value: crate::unity_engine::material::Material) -> ();

    #[method(name = "get_sun", args = 0)]
    pub fn get_sun() -> crate::unity_engine::light::Light;

    #[method(name = "set_sun", args = 1)]
    pub fn set_sun(value: crate::unity_engine::light::Light) -> ();

    #[method(name = "get_ambientProbe", args = 0)]
    pub fn get_ambient_probe(
    ) -> crate::unity_engine::rendering::sphericalharmonicsl2::SphericalHarmonicsL2;

    #[method(name = "set_ambientProbe", args = 1)]
    pub fn set_ambient_probe(
        value: crate::unity_engine::rendering::sphericalharmonicsl2::SphericalHarmonicsL2,
    ) -> ();

    #[method(name = "get_customReflection", args = 0)]
    pub fn get_custom_reflection() -> crate::unity_engine::cubemap::Cubemap;

    #[method(name = "set_customReflection", args = 1)]
    pub fn set_custom_reflection(value: crate::unity_engine::cubemap::Cubemap) -> ();

    #[method(name = "get_reflectionIntensity", args = 0)]
    pub fn get_reflection_intensity() -> f32;

    #[method(name = "set_reflectionIntensity", args = 1)]
    pub fn set_reflection_intensity(value: f32) -> ();

    #[method(name = "get_reflectionBounces", args = 0)]
    pub fn get_reflection_bounces() -> i32;

    #[method(name = "set_reflectionBounces", args = 1)]
    pub fn set_reflection_bounces(value: i32) -> ();

    #[method(name = "get_defaultReflectionMode", args = 0)]
    pub fn get_default_reflection_mode(
    ) -> crate::unity_engine::rendering::defaultreflectionmode::DefaultReflectionMode;

    #[method(name = "set_defaultReflectionMode", args = 1)]
    pub fn set_default_reflection_mode(
        value: crate::unity_engine::rendering::defaultreflectionmode::DefaultReflectionMode,
    ) -> ();

    #[method(name = "get_defaultReflectionResolution", args = 0)]
    pub fn get_default_reflection_resolution() -> i32;

    #[method(name = "set_defaultReflectionResolution", args = 1)]
    pub fn set_default_reflection_resolution(value: i32) -> ();

    #[method(name = "get_haloStrength", args = 0)]
    pub fn get_halo_strength() -> f32;

    #[method(name = "set_haloStrength", args = 1)]
    pub fn set_halo_strength(value: f32) -> ();

    #[method(name = "get_flareStrength", args = 0)]
    pub fn get_flare_strength() -> f32;

    #[method(name = "set_flareStrength", args = 1)]
    pub fn set_flare_strength(value: f32) -> ();

    #[method(name = "get_flareFadeSpeed", args = 0)]
    pub fn get_flare_fade_speed() -> f32;

    #[method(name = "set_flareFadeSpeed", args = 1)]
    pub fn set_flare_fade_speed(value: f32) -> ();

    #[method(name = "GetRenderSettings", args = 0)]
    pub fn get_render_settings() -> crate::unity_engine::object_2::Object_2;

    #[method(name = "Reset", args = 0)]
    pub fn reset() -> ();

    #[method(name = "get_fogColor_Injected", args = 1)]
    pub fn get_fog_color_injected(ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_fogColor_Injected", args = 1)]
    pub fn set_fog_color_injected(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_ambientSkyColor_Injected", args = 1)]
    pub fn get_ambient_sky_color_injected(ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_ambientSkyColor_Injected", args = 1)]
    pub fn set_ambient_sky_color_injected(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_ambientEquatorColor_Injected", args = 1)]
    pub fn get_ambient_equator_color_injected(ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_ambientEquatorColor_Injected", args = 1)]
    pub fn set_ambient_equator_color_injected(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_ambientGroundColor_Injected", args = 1)]
    pub fn get_ambient_ground_color_injected(ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_ambientGroundColor_Injected", args = 1)]
    pub fn set_ambient_ground_color_injected(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_ambientLight_Injected", args = 1)]
    pub fn get_ambient_light_injected(ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_ambientLight_Injected", args = 1)]
    pub fn set_ambient_light_injected(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_subtractiveShadowColor_Injected", args = 1)]
    pub fn get_subtractive_shadow_color_injected(ret: crate::unity_engine::color::Color) -> ();

    #[method(name = "set_subtractiveShadowColor_Injected", args = 1)]
    pub fn set_subtractive_shadow_color_injected(value: crate::unity_engine::color::Color) -> ();

    #[method(name = "get_ambientProbe_Injected", args = 1)]
    pub fn get_ambient_probe_injected(
        ret: crate::unity_engine::rendering::sphericalharmonicsl2::SphericalHarmonicsL2,
    ) -> ();

    #[method(name = "set_ambientProbe_Injected", args = 1)]
    pub fn set_ambient_probe_injected(
        value: crate::unity_engine::rendering::sphericalharmonicsl2::SphericalHarmonicsL2,
    ) -> ();
}

#[cfg(feature = "unity_engine-rendersettings")]
impl RenderSettings {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RenderSettings),
                ::core::stringify!(new),
            )
        });
        <Self as IRenderSettingsMethods>::ctor(this);
        this
    }
}
