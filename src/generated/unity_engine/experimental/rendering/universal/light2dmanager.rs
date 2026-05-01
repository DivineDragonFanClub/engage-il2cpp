
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/light2dmanager/Light2DManager.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal",
    name = "Light2DManager"
)]
#[parent(crate::system::object::Object)]
pub struct Light2DManager {
    #[static_field]
    #[rename(name = "s_SortingLayers")]
    pub s_sorting_layers: ::unity2::Array<crate::unity_engine::sortinglayer::SortingLayer>,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-light2dmanager")]
#[::unity2::methods]
impl Light2DManager {
    #[method(name = "get_lights", args = 0)]
    pub fn get_lights() -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::experimental::rendering::universal::light2d_2::Light2D_2,
    >;

    #[method(name = "RegisterLight", args = 1)]
    pub fn register_light(
        light: crate::unity_engine::experimental::rendering::universal::light2d_2::Light2D_2,
    ) -> ();

    #[method(name = "DeregisterLight", args = 1)]
    pub fn deregister_light(
        light: crate::unity_engine::experimental::rendering::universal::light2d_2::Light2D_2,
    ) -> ();

    #[method(name = "ErrorIfDuplicateGlobalLight", args = 1)]
    pub fn error_if_duplicate_global_light(
        light: crate::unity_engine::experimental::rendering::universal::light2d_2::Light2D_2,
    ) -> ();

    #[method(name = "GetGlobalColor", args = 3)]
    pub fn get_global_color(
        sorting_layer_index: i32,
        blend_style_index: i32,
        color: crate::unity_engine::color::Color,
    ) -> bool;

    #[method(name = "ContainsDuplicateGlobalLight", args = 2)]
    pub fn contains_duplicate_global_light(
        sorting_layer_index: i32,
        blend_style_index: i32,
    ) -> bool;

    #[method(name = "GetCachedSortingLayer", args = 0)]
    pub fn get_cached_sorting_layer(
    ) -> ::unity2::Array<crate::unity_engine::sortinglayer::SortingLayer>;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
