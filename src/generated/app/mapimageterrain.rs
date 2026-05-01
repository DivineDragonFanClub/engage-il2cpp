
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimageterrain/MapImageTerrain_MinimapInfo.md")))]
#[::unity2::class(namespace = "App", name = "MapImageTerrain.MinimapInfo")]
#[parent(crate::system::object::Object)]
pub struct MapImageTerrain_MinimapInfo {
    #[rename(name = "X")]
    pub x: i32,
    #[rename(name = "Z")]
    pub z: i32,
    #[rename(name = "Terrain")]
    pub terrain: crate::app::terraindata_2::TerrainData_2,
}

#[cfg(feature = "app-mapimageterrain")]
#[::unity2::methods]
impl MapImageTerrain_MinimapInfo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapimageterrain")]
impl MapImageTerrain_MinimapInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapImageTerrain_MinimapInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IMapImageTerrain_MinimapInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimageterrain/MapImageTerrain.md")))]
#[::unity2::class(namespace = "App", name = "MapImageTerrain")]
#[parent(crate::system::object::Object)]
pub struct MapImageTerrain {
    #[rename(name = "m_Original")]
    pub m_original: crate::app::mapimagecorebyte::MapImageCoreByte,
    #[rename(name = "m_Base")]
    pub m_base: crate::app::mapimagecorebyte::MapImageCoreByte,
    #[rename(name = "m_Result")]
    pub m_result: crate::app::mapimagecorebyte::MapImageCoreByte,
    #[rename(name = "m_MinimapInfos")]
    pub m_minimap_infos: crate::system::collections::generic::list_1::List_1<
        crate::app::mapimageterrain::MapImageTerrain_MinimapInfo,
    >,
    #[rename(name = "m_MinimapBuffer")]
    pub m_minimap_buffer: crate::system::collections::generic::list_1::List_1<
        crate::app::mapimageterrain::MapImageTerrain_MinimapInfo,
    >,
}

#[cfg(feature = "app-mapimageterrain")]
#[::unity2::methods]
impl MapImageTerrain {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "get_Original", args = 0)]
    pub fn get_original(self) -> crate::app::mapimagecorebyte::MapImageCoreByte;

    #[method(name = "get_Base", args = 0)]
    pub fn get_base(self) -> crate::app::mapimagecorebyte::MapImageCoreByte;

    #[method(name = "get_MinimapInfos", args = 0)]
    pub fn get_minimap_infos(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::mapimageterrain::MapImageTerrain_MinimapInfo,
    >;

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "UpdateMinimap", args = 0)]
    pub fn update_minimap(self) -> ();

    #[method(name = "UpdateForEvent", args = 0)]
    pub fn update_for_event(self) -> ();

    #[method(name = "Get", args = 2)]
    pub fn get(self, x: i32, z: i32) -> u8;

    #[method(name = "GetData", args = 2)]
    pub fn get_data(self, x: i32, z: i32) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}

#[cfg(feature = "app-mapimageterrain")]
impl MapImageTerrain {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapImageTerrain),
                ::core::stringify!(new),
            )
        });
        <Self as IMapImageTerrainMethods>::ctor(this);
        this
    }
}
