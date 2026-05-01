
use crate::app::terrainmap::ITerrainMap;
use crate::app::terrainmap::TerrainMap;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/terrainlayermap/TerrainLayerMap.md")))]
#[::unity2::class(namespace = "App", name = "TerrainLayerMap")]
#[parent(crate::app::terrainmap::TerrainMap)]
pub struct TerrainLayerMap {}

#[cfg(feature = "app-terrainlayermap")]
#[::unity2::methods]
impl TerrainLayerMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, terrain: crate::unity_engine::terrain::Terrain) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, data: crate::unity_engine::terraindata::TerrainData) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_4(self, width: i32, height: i32, layers: i32) -> ();

    #[method(name = "ReadData", args = 1)]
    pub fn read_data(self, data: crate::unity_engine::terraindata::TerrainData) -> ();

    #[method(name = "WriteData", args = 1)]
    pub fn write_data(self, data: crate::unity_engine::terraindata::TerrainData) -> ();

    #[method(name = "GetCurrentLayer", args = 2)]
    pub fn get_current_layer(self, x: i32, y: i32) -> i32;

    #[method(name = "GetMaxAlpha", args = 2)]
    pub fn get_max_alpha(self, x: i32, y: i32) -> f32;

    #[method(name = "Normalize", args = 0)]
    pub fn normalize(self) -> ();

    #[method(name = "Normalize", args = 1)]
    pub fn normalize_2(self, layer: i32) -> ();

    #[method(name = "FillLayer", args = 1)]
    pub fn fill_layer(self, layer: i32) -> ();

    #[method(name = "SlopLayer", args = 2)]
    pub fn slop_layer(self, data: crate::unity_engine::terraindata::TerrainData, layer: i32) -> ();

    #[method(name = "ClearLayer", args = 1)]
    pub fn clear_layer(self, layer: i32) -> ();

    #[method(name = "UpdateSoft", args = 0)]
    pub fn update_soft(self) -> ();

    #[method(name = "UpdateSoft", args = 1)]
    pub fn update_soft_2(self, layer: i32) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "set_Count", args = 1)]
    pub fn set_count(self, value: i32) -> ();

    #[method(name = "get_Item", args = 3)]
    pub fn get_item(self, x: i32, y: i32, layer: i32) -> f32;

    #[method(name = "set_Item", args = 4)]
    pub fn set_item(self, x: i32, y: i32, layer: i32, value: f32) -> ();
}

#[cfg(feature = "app-terrainlayermap")]
impl TerrainLayerMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TerrainLayerMap),
                ::core::stringify!(new),
            )
        });
        <Self as ITerrainLayerMapMethods>::ctor(this);
        this
    }

    pub fn new_2(terrain: crate::unity_engine::terrain::Terrain) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TerrainLayerMap),
                ::core::stringify!(new_2),
            )
        });
        <Self as ITerrainLayerMapMethods>::ctor_2(this, terrain);
        this
    }

    pub fn new_3(data: crate::unity_engine::terraindata::TerrainData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TerrainLayerMap),
                ::core::stringify!(new_3),
            )
        });
        <Self as ITerrainLayerMapMethods>::ctor_3(this, data);
        this
    }

    pub fn new_4(width: i32, height: i32, layers: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TerrainLayerMap),
                ::core::stringify!(new_4),
            )
        });
        <Self as ITerrainLayerMapMethods>::ctor_4(this, width, height, layers);
        this
    }
}
