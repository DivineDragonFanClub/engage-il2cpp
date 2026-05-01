
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/terrain/Terrain.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Terrain")]
#[parent(crate::unity_engine::behaviour::Behaviour)]
pub struct Terrain {}

#[cfg(feature = "unity_engine-terrain")]
#[::unity2::methods]
impl Terrain {
    #[method(name = "get_terrainData", args = 0)]
    pub fn get_terrain_data(self) -> crate::unity_engine::terraindata::TerrainData;

    #[method(name = "get_lightmapIndex", args = 0)]
    pub fn get_lightmap_index(self) -> i32;

    #[method(name = "get_lightmapScaleOffset", args = 0)]
    pub fn get_lightmap_scale_offset(self) -> crate::unity_engine::vector4::Vector4;

    #[method(name = "get_allowAutoConnect", args = 0)]
    pub fn get_allow_auto_connect(self) -> bool;

    #[method(name = "get_groupingID", args = 0)]
    pub fn get_grouping_id(self) -> i32;

    #[method(name = "SetNeighbors", args = 4)]
    pub fn set_neighbors(
        self,
        left: crate::unity_engine::terrain::Terrain,
        top: crate::unity_engine::terrain::Terrain,
        right: crate::unity_engine::terrain::Terrain,
        bottom: crate::unity_engine::terrain::Terrain,
    ) -> ();

    #[method(name = "get_activeTerrains", args = 0)]
    pub fn get_active_terrains() -> ::unity2::Array<crate::unity_engine::terrain::Terrain>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_lightmapScaleOffset_Injected", args = 1)]
    pub fn get_lightmap_scale_offset_injected(
        self,
        ret: crate::unity_engine::vector4::Vector4,
    ) -> ();
}

#[cfg(feature = "unity_engine-terrain")]
impl Terrain {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Terrain),
                ::core::stringify!(new),
            )
        });
        <Self as ITerrainMethods>::ctor(this);
        this
    }
}
