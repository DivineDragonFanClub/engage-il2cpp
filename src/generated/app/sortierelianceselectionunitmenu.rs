
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortierelianceselectionunitmenu/SortieRelianceSelectionUnitMenu_UnitMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SortieRelianceSelectionUnitMenu.UnitMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct SortieRelianceSelectionUnitMenu_UnitMenuItem {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_CanTalk")]
    pub m_can_talk: bool,
}

#[cfg(feature = "app-sortierelianceselectionunitmenu")]
#[::unity2::methods]
impl SortieRelianceSelectionUnitMenu_UnitMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-sortierelianceselectionunitmenu")]
impl SortieRelianceSelectionUnitMenu_UnitMenuItem {
    pub fn new(unit: crate::app::unit::Unit) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieRelianceSelectionUnitMenu_UnitMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieRelianceSelectionUnitMenu_UnitMenuItemMethods>::ctor(this, unit);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortierelianceselectionunitmenu/SortieRelianceSelectionUnitMenu.md")))]
#[::unity2::class(namespace = "App", name = "SortieRelianceSelectionUnitMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct SortieRelianceSelectionUnitMenu {}

#[cfg(feature = "app-sortierelianceselectionunitmenu")]
#[::unity2::methods]
impl SortieRelianceSelectionUnitMenu {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> ();
}

#[cfg(feature = "app-sortierelianceselectionunitmenu")]
impl SortieRelianceSelectionUnitMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieRelianceSelectionUnitMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieRelianceSelectionUnitMenuMethods>::ctor(this, menu_item_list);
        this
    }
}
