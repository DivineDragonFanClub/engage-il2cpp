
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mappanelsortie/MapPanelSortie_DontPosChangeMap.md")))]
#[::unity2::class(namespace = "App", name = "MapPanelSortie.DontPosChangeMap")]
#[parent(crate::system::object::Object)]
pub struct MapPanelSortie_DontPosChangeMap {
    #[rename(name = "m_Bits")]
    pub m_bits: crate::system::collections::bitarray::BitArray,
}

#[cfg(feature = "app-mappanelsortie")]
#[::unity2::methods]
impl MapPanelSortie_DontPosChangeMap {
    #[method(name = "Add", args = 2)]
    pub fn add(self, x: i32, z: i32) -> ();

    #[method(name = "Test", args = 2)]
    pub fn test(self, x: i32, z: i32) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "GetIndex", args = 2)]
    pub fn get_index(self, x: i32, z: i32) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mappanelsortie")]
impl MapPanelSortie_DontPosChangeMap {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapPanelSortie_DontPosChangeMap),
                ::core::stringify!(new),
            )
        });
        <Self as IMapPanelSortie_DontPosChangeMapMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mappanelsortie/MapPanelSortie.md")))]
#[::unity2::class(namespace = "App", name = "MapPanelSortie")]
pub struct MapPanelSortie {
    #[rename(name = "m_DontPosChangeMap")]
    pub m_dont_pos_change_map: crate::app::mappanelsortie::MapPanelSortie_DontPosChangeMap,
    #[rename(name = "m_SelectX")]
    pub m_select_x: i32,
    #[rename(name = "m_SelectZ")]
    pub m_select_z: i32,
}

#[cfg(feature = "app-mappanelsortie")]
#[::unity2::methods]
impl MapPanelSortie {
    #[method(name = "SetActive", args = 1)]
    pub fn set_active(is_active: bool) -> ();

    #[method(name = "Renew", args = 0)]
    pub fn renew(self) -> ();

    #[method(name = "SetSelect", args = 2)]
    pub fn set_select(self, x: i32, z: i32) -> ();

    #[method(name = "ClearSelect", args = 0)]
    pub fn clear_select(self) -> ();

    #[method(name = "get_SelectX", args = 0)]
    pub fn get_select_x(self) -> i32;

    #[method(name = "set_SelectX", args = 1)]
    pub fn set_select_x(self, value: i32) -> ();

    #[method(name = "get_SelectZ", args = 0)]
    pub fn get_select_z(self) -> i32;

    #[method(name = "set_SelectZ", args = 1)]
    pub fn set_select_z(self, value: i32) -> ();

    #[method(name = "get_IsSelected", args = 0)]
    pub fn get_is_selected(self) -> bool;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "SetVertex", args = 0)]
    pub fn set_vertex(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mappanelsortie")]
impl MapPanelSortie {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapPanelSortie),
                ::core::stringify!(new),
            )
        });
        <Self as IMapPanelSortieMethods>::ctor(this);
        this
    }
}
