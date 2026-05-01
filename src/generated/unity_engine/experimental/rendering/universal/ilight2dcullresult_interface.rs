
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/ilight2dcullresult_interface/ILight2DCullResult_Interface.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal",
    name = "ILight2DCullResult"
)]
pub struct ILight2DCullResult_Interface {}

#[cfg(feature = "unity_engine-experimental-rendering-universal-ilight2dcullresult_interface")]
#[::unity2::methods]
impl ILight2DCullResult_Interface {
    #[method(name = "get_visibleLights", args = 0)]
    pub fn get_visible_lights(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::experimental::rendering::universal::light2d_2::Light2D_2,
    >;

    #[method(name = "GetLightStatsByLayer", args = 1)]
    pub fn get_light_stats_by_layer(
        self,
        layer: i32,
    ) -> crate::unity_engine::experimental::rendering::universal::lightstats::LightStats;

    #[method(name = "IsSceneLit", args = 0)]
    pub fn is_scene_lit(self) -> bool;
}
