
use crate::app::terrainmap::ITerrainMap;
use crate::app::terrainmap::TerrainMap;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/terrainheightmap/TerrainHeightMap.md")))]
#[::unity2::class(namespace = "App", name = "TerrainHeightMap")]
#[parent(crate::app::terrainmap::TerrainMap)]
pub struct TerrainHeightMap {}

#[cfg(feature = "app-terrainheightmap")]
#[::unity2::methods]
impl TerrainHeightMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, terrain: crate::unity_engine::terrain::Terrain) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, data: crate::unity_engine::terraindata::TerrainData) -> ();

    #[method(name = "ReadData", args = 1)]
    pub fn read_data(self, data: crate::unity_engine::terraindata::TerrainData) -> ();

    #[method(name = "WriteData", args = 1)]
    pub fn write_data(self, data: crate::unity_engine::terraindata::TerrainData) -> ();

    #[method(name = "FillHeight", args = 1)]
    pub fn fill_height(self, height: f32) -> ();

    #[method(name = "MoveHeight", args = 1)]
    pub fn move_height(self, height: f32) -> ();

    #[method(name = "get_Item", args = 2)]
    pub fn get_item(self, x: i32, y: i32) -> f32;

    #[method(name = "set_Item", args = 3)]
    pub fn set_item(self, x: i32, y: i32, value: f32) -> ();
}

#[cfg(feature = "app-terrainheightmap")]
impl TerrainHeightMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TerrainHeightMap),
                ::core::stringify!(new),
            )
        });
        <Self as ITerrainHeightMapMethods>::ctor(this);
        this
    }

    pub fn new_2(terrain: crate::unity_engine::terrain::Terrain) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TerrainHeightMap),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITerrainHeightMapMethods>::ctor_2(this, terrain);
        this
    }

    pub fn new_3(data: crate::unity_engine::terraindata::TerrainData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TerrainHeightMap),
                ::core::stringify!(new_3),
            )
        });
        <Self as ITerrainHeightMapMethods>::ctor_3(this, data);
        this
    }
}
