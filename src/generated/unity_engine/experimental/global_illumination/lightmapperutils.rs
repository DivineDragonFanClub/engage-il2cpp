
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/global_illumination/lightmapperutils/LightmapperUtils.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.GlobalIllumination",
    name = "LightmapperUtils"
)]
#[parent(crate::system::object::Object)]
pub struct LightmapperUtils {}

#[cfg(feature = "unity_engine-experimental-global_illumination-lightmapperutils")]
#[::unity2::methods]
impl LightmapperUtils {
    #[method(name = "Extract", args = 1)]
    pub fn extract(
        baketype: crate::unity_engine::lightmapbaketype::LightmapBakeType,
    ) -> crate::unity_engine::experimental::global_illumination::lightmode::LightMode;

    #[method(name = "ExtractIndirect", args = 1)]
    pub fn extract_indirect(
        l: crate::unity_engine::light::Light,
    ) -> crate::unity_engine::experimental::global_illumination::linearcolor::LinearColor;

    #[method(name = "ExtractInnerCone", args = 1)]
    pub fn extract_inner_cone(l: crate::unity_engine::light::Light) -> f32;

    #[method(name = "ExtractColorTemperature", args = 1)]
    pub fn extract_color_temperature(
        l: crate::unity_engine::light::Light,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "ApplyColorTemperature", args = 2)]
    pub fn apply_color_temperature(
        cct: crate::unity_engine::color::Color,
        light_color : crate :: unity_engine :: experimental :: global_illumination :: linearcolor :: LinearColor,
    ) -> ();

    #[method(name = "Extract", args = 2)]
    pub fn extract_2(
        l: crate::unity_engine::light::Light,
        dir : crate :: unity_engine :: experimental :: global_illumination :: directionallight :: DirectionalLight,
    ) -> ();

    #[method(name = "Extract", args = 2)]
    pub fn extract_3(
        l: crate::unity_engine::light::Light,
        point: crate::unity_engine::experimental::global_illumination::pointlight::PointLight,
    ) -> ();

    #[method(name = "Extract", args = 2)]
    pub fn extract_4(
        l: crate::unity_engine::light::Light,
        spot: crate::unity_engine::experimental::global_illumination::spotlight::SpotLight,
    ) -> ();

    #[method(name = "Extract", args = 2)]
    pub fn extract_5(
        l: crate::unity_engine::light::Light,
        rect : crate :: unity_engine :: experimental :: global_illumination :: rectanglelight :: RectangleLight,
    ) -> ();

    #[method(name = "Extract", args = 2)]
    pub fn extract_6(
        l: crate::unity_engine::light::Light,
        disc: crate::unity_engine::experimental::global_illumination::disclight::DiscLight,
    ) -> ();

    #[method(name = "Extract", args = 2)]
    pub fn extract_7(
        l: crate::unity_engine::light::Light,
        cookie: crate::unity_engine::experimental::global_illumination::cookie::Cookie,
    ) -> ();
}
