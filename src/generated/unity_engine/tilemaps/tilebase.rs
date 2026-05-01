
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/tilemaps/tilebase/TileBase.md")))]
#[::unity2::class(namespace = "UnityEngine.Tilemaps", name = "TileBase")]
#[parent(crate::unity_engine::scriptableobject::ScriptableObject)]
pub struct TileBase {}

#[cfg(feature = "unity_engine-tilemaps-tilebase")]
#[::unity2::methods]
impl TileBase {
    #[method(name = "RefreshTile", args = 2)]
    pub fn refresh_tile(
        self,
        position: crate::unity_engine::vector3int::Vector3Int,
        tilemap: crate::unity_engine::tilemaps::itilemap_interface::ITilemap_Interface,
    ) -> ();

    #[method(name = "GetTileData", args = 3)]
    pub fn get_tile_data(
        self,
        position: crate::unity_engine::vector3int::Vector3Int,
        tilemap: crate::unity_engine::tilemaps::itilemap_interface::ITilemap_Interface,
        tile_data: crate::unity_engine::tilemaps::tiledata::TileData,
    ) -> ();

    #[method(name = "GetTileDataNoRef", args = 2)]
    pub fn get_tile_data_no_ref(
        self,
        position: crate::unity_engine::vector3int::Vector3Int,
        tilemap: crate::unity_engine::tilemaps::itilemap_interface::ITilemap_Interface,
    ) -> crate::unity_engine::tilemaps::tiledata::TileData;

    #[method(name = "GetTileAnimationData", args = 3)]
    pub fn get_tile_animation_data(
        self,
        position: crate::unity_engine::vector3int::Vector3Int,
        tilemap: crate::unity_engine::tilemaps::itilemap_interface::ITilemap_Interface,
        tile_animation_data: crate::unity_engine::tilemaps::tileanimationdata::TileAnimationData,
    ) -> bool;

    #[method(name = "GetTileAnimationDataNoRef", args = 2)]
    pub fn get_tile_animation_data_no_ref(
        self,
        position: crate::unity_engine::vector3int::Vector3Int,
        tilemap: crate::unity_engine::tilemaps::itilemap_interface::ITilemap_Interface,
    ) -> crate::unity_engine::tilemaps::tileanimationdata::TileAnimationData;

    #[method(name = "StartUp", args = 3)]
    pub fn start_up(
        self,
        position: crate::unity_engine::vector3int::Vector3Int,
        tilemap: crate::unity_engine::tilemaps::itilemap_interface::ITilemap_Interface,
        go: crate::unity_engine::gameobject::GameObject,
    ) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-tilemaps-tilebase")]
impl TileBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TileBase),
                ::core::stringify!(new),
            )
        });
        <Self as ITileBaseMethods>::ctor(this);
        this
    }
}
