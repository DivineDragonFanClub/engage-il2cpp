
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/terrainexporter/TerrainExporter.md")))]
#[::unity2::class(namespace = "App", name = "TerrainExporter")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct TerrainExporter {
    #[rename(name = "m_TerrainSize")]
    pub m_terrain_size: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_OriginalTerrainGuid")]
    pub m_original_terrain_guid: ::unity2::Il2CppString,
}

#[cfg(feature = "app-terrainexporter")]
#[::unity2::methods]
impl TerrainExporter {
    #[method(name = "get_OriginalTerrainData", args = 0)]
    pub fn get_original_terrain_data(self) -> crate::unity_engine::terraindata::TerrainData;

    #[method(name = "set_OriginalTerrainData", args = 1)]
    pub fn set_original_terrain_data(
        self,
        value: crate::unity_engine::terraindata::TerrainData,
    ) -> ();

    #[method(name = "get_TerrainSize", args = 0)]
    pub fn get_terrain_size(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-terrainexporter")]
impl TerrainExporter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TerrainExporter),
                ::core::stringify!(new),
            )
        });
        <Self as ITerrainExporterMethods>::ctor(this);
        this
    }
}
