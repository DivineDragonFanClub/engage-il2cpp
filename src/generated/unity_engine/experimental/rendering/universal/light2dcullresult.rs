
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/light2dcullresult/Light2DCullResult.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal",
    name = "Light2DCullResult"
)]
#[parent(crate::system::object::Object)]
pub struct Light2DCullResult {
    #[rename(name = "m_VisibleLights")]
    pub m_visible_lights: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::experimental::rendering::universal::light2d_2::Light2D_2,
    >,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-light2dcullresult")]
#[::unity2::methods]
impl Light2DCullResult {
    #[method(name = "get_visibleLights", args = 0)]
    pub fn get_visible_lights(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::experimental::rendering::universal::light2d_2::Light2D_2,
    >;

    #[method(name = "IsSceneLit", args = 0)]
    pub fn is_scene_lit(self) -> bool;

    #[method(name = "GetLightStatsByLayer", args = 1)]
    pub fn get_light_stats_by_layer(
        self,
        layer: i32,
    ) -> crate::unity_engine::experimental::rendering::universal::lightstats::LightStats;

    #[method(name = "SetupCulling", args = 2)]
    pub fn setup_culling(
        self,
        culling_parameters : crate :: unity_engine :: rendering :: scriptablecullingparameters :: ScriptableCullingParameters,
        camera: crate::unity_engine::camera::Camera,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-light2dcullresult")]
impl Light2DCullResult {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Light2DCullResult),
                ::core::stringify!(new),
            )
        });
        <Self as ILight2DCullResultMethods>::ctor(this);
        this
    }
}
