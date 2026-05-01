
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimagehistory/MapImageHistory_HeatMap.md")))]
#[::unity2::class(namespace = "App", name = "MapImageHistory.HeatMap")]
#[parent(crate::system::object::Object)]
pub struct MapImageHistory_HeatMap {
    #[rename(name = "m_Maps")]
    pub m_maps: ::unity2::Array<crate::app::mapimagecorebyte::MapImageCoreByte>,
}

#[cfg(feature = "app-mapimagehistory")]
#[::unity2::methods]
impl MapImageHistory_HeatMap {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(self, index: i32) -> crate::app::mapimagecorebyte::MapImageCoreByte;

    #[method(name = "Get", args = 1)]
    pub fn get_2(
        self,
        r#type: crate::app::force::Force_Type,
    ) -> crate::app::mapimagecorebyte::MapImageCoreByte;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();
}

#[cfg(feature = "app-mapimagehistory")]
impl MapImageHistory_HeatMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapImageHistory_HeatMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapImageHistory_HeatMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapimagehistory/MapImageHistory.md")))]
#[::unity2::class(namespace = "App", name = "MapImageHistory")]
#[parent(crate::system::object::Object)]
pub struct MapImageHistory {
    #[rename(name = "UnitMap")]
    pub unit_map: crate::app::mapimagehistory::MapImageHistory_HeatMap,
    #[rename(name = "BattleMap")]
    pub battle_map: crate::app::mapimagehistory::MapImageHistory_HeatMap,
    #[rename(name = "DeadMap")]
    pub dead_map: crate::app::mapimagehistory::MapImageHistory_HeatMap,
    #[rename(name = "m_Cells")]
    pub m_cells: ::unity2::Array<crate::app::mappos::MapPos>,
}

#[cfg(feature = "app-mapimagehistory")]
#[::unity2::methods]
impl MapImageHistory {
    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "UpdatePhase", args = 0)]
    pub fn update_phase(self) -> ();

    #[method(name = "AddDead", args = 1)]
    pub fn add_dead(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "AddBattle", args = 1)]
    pub fn add_battle(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapimagehistory")]
impl MapImageHistory {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapImageHistory),
                ::core::stringify!(new),
            )
        });
        <Self as IMapImageHistoryMethods>::ctor(this);
        this
    }
}
