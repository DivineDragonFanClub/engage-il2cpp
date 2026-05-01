
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/tilelayoututils/TileLayoutUtils.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "TileLayoutUtils")]
#[parent(crate::system::object::Object)]
pub struct TileLayoutUtils {}

#[cfg(feature = "unity_engine-rendering-tilelayoututils")]
#[::unity2::methods]
impl TileLayoutUtils {
    #[method(name = "TryLayoutByTiles", args = 6)]
    pub fn try_layout_by_tiles(
        src: crate::unity_engine::rectint::RectInt,
        tile_size: u32,
        main: crate::unity_engine::rectint::RectInt,
        top_row: crate::unity_engine::rectint::RectInt,
        right_col: crate::unity_engine::rectint::RectInt,
        top_right: crate::unity_engine::rectint::RectInt,
    ) -> bool;

    #[method(name = "TryLayoutByRow", args = 4)]
    pub fn try_layout_by_row(
        src: crate::unity_engine::rectint::RectInt,
        tile_size: u32,
        main: crate::unity_engine::rectint::RectInt,
        other: crate::unity_engine::rectint::RectInt,
    ) -> bool;

    #[method(name = "TryLayoutByCol", args = 4)]
    pub fn try_layout_by_col(
        src: crate::unity_engine::rectint::RectInt,
        tile_size: u32,
        main: crate::unity_engine::rectint::RectInt,
        other: crate::unity_engine::rectint::RectInt,
    ) -> bool;
}
