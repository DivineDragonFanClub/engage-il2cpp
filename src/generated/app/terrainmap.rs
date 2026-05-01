
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/terrainmap/TerrainMap.md")))]
#[::unity2::class(namespace = "App", name = "TerrainMap")]
#[parent(crate::system::object::Object)]
pub struct TerrainMap {}

#[cfg(feature = "app-terrainmap")]
#[::unity2::methods]
impl TerrainMap {
    #[method(name = "get_Width", args = 0)]
    pub fn get_width(self) -> i32;

    #[method(name = "set_Width", args = 1)]
    pub fn set_width(self, value: i32) -> ();

    #[method(name = "get_Height", args = 0)]
    pub fn get_height(self) -> i32;

    #[method(name = "set_Height", args = 1)]
    pub fn set_height(self, value: i32) -> ();

    #[method(name = "IsInside", args = 2)]
    pub fn is_inside(self, x: i32, y: i32) -> bool;

    #[method(name = "IsOutside", args = 2)]
    pub fn is_outside(self, x: i32, y: i32) -> bool;

    #[method(name = "Read", args = 1)]
    pub fn read(self, terrain: crate::unity_engine::terrain::Terrain) -> ();

    #[method(name = "Read", args = 1)]
    pub fn read_2(self, data: crate::unity_engine::terraindata::TerrainData) -> ();

    #[method(name = "Write", args = 1)]
    pub fn write(self, terrain: crate::unity_engine::terrain::Terrain) -> ();

    #[method(name = "Write", args = 1)]
    pub fn write_2(self, data: crate::unity_engine::terraindata::TerrainData) -> ();

    #[method(name = "ReadData", args = 1)]
    pub fn read_data(self, data: crate::unity_engine::terraindata::TerrainData) -> ();

    #[method(name = "WriteData", args = 1)]
    pub fn write_data(self, data: crate::unity_engine::terraindata::TerrainData) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-terrainmap")]
impl TerrainMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TerrainMap),
                ::core::stringify!(new),
            )
        });
        <Self as ITerrainMapMethods>::ctor(this);
        this
    }
}
