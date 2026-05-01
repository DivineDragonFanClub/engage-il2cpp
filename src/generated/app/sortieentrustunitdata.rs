
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentrustunitdata/SortieEntrustUnitData_WeaponKindExp.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrustUnitData.WeaponKindExp")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrustUnitData_WeaponKindExp {
    #[rename(name = "m_ItemKind")]
    pub m_item_kind: crate::app::itemdata::ItemData_Kinds,
    #[rename(name = "m_Exp")]
    pub m_exp: i32,
}

#[cfg(feature = "app-sortieentrustunitdata")]
#[::unity2::methods]
impl SortieEntrustUnitData_WeaponKindExp {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, item_kind: crate::app::itemdata::ItemData_Kinds, exp: i32) -> ();

    #[method(name = "get_ItemKind", args = 0)]
    pub fn get_item_kind(self) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "get_Exp", args = 0)]
    pub fn get_exp(self) -> i32;
}

#[cfg(feature = "app-sortieentrustunitdata")]
impl SortieEntrustUnitData_WeaponKindExp {
    pub fn new(item_kind: crate::app::itemdata::ItemData_Kinds, exp: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrustUnitData_WeaponKindExp),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrustUnitData_WeaponKindExpMethods>::ctor(this, item_kind, exp);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentrustunitdata/SortieEntrustUnitData.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrustUnitData")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrustUnitData {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_UnitItems")]
    pub m_unit_items: ::unity2::Array<crate::app::unititem::UnitItem>,
    #[rename(name = "m_Count")]
    pub m_count: i32,
    #[rename(name = "m_Progress")]
    pub m_progress: crate::app::sortieentrustprogress::SortieEntrustProgress,
    #[rename(name = "m_SortedWeaponKind")]
    pub m_sorted_weapon_kind: crate::system::collections::generic::list_1::List_1<
        crate::app::sortieentrustunitdata::SortieEntrustUnitData_WeaponKindExp,
    >,
}

#[cfg(feature = "app-sortieentrustunitdata")]
#[::unity2::methods]
impl SortieEntrustUnitData {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, unit_item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "NextProgress", args = 0)]
    pub fn next_progress(self) -> ();

    #[method(name = "EndProgress", args = 0)]
    pub fn end_progress(self) -> ();

    #[method(name = "GetSortedWeaponKind", args = 1)]
    pub fn get_sorted_weapon_kind(self, index: i32) -> crate::app::itemdata::ItemData_Kinds;

    #[method(name = "get_Unit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "get_UnitItems", args = 0)]
    pub fn get_unit_items(self) -> ::unity2::Array<crate::app::unititem::UnitItem>;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Progress", args = 0)]
    pub fn get_progress(self) -> crate::app::sortieentrustprogress::SortieEntrustProgress;
}

#[cfg(feature = "app-sortieentrustunitdata")]
impl SortieEntrustUnitData {
    pub fn new(unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrustUnitData),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrustUnitDataMethods>::ctor(this, unit);
        this
    }
}
