
use crate::app::basicitemmenuitem::BasicItemMenuItem;
use crate::app::basicitemmenuitem::IBasicItemMenuItem;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/wellitemselectmenuitem/WellItemSelectMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "WellItemSelectMenuItem")]
#[parent(crate::app::basicitemmenuitem::BasicItemMenuItem)]
pub struct WellItemSelectMenuItem {
    #[rename(name = "m_OwnerItemIndex")]
    pub m_owner_item_index: i32,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_SortIndex")]
    pub m_sort_index: i32,
    #[rename(name = "m_IsMarking")]
    pub m_is_marking: bool,
    #[rename(name = "m_Expected")]
    pub m_expected: i32,
}

#[cfg(feature = "app-wellitemselectmenuitem")]
#[::unity2::methods]
impl WellItemSelectMenuItem {
    #[method(name = "IsMarking", args = 0)]
    pub fn is_marking(self) -> bool;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(self, unit: crate::app::unit::Unit, owner_item_index: i32, sort_index: i32) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "Exchange", args = 0)]
    pub fn exchange(self) -> ();

    #[method(name = "CreateDialog", args = 0)]
    pub fn create_dialog(self) -> ();

    #[method(name = "TryPutOff", args = 0)]
    pub fn try_put_off(self) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "PlusCall", args = 0)]
    pub fn plus_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetUnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetSortIndex", args = 0)]
    pub fn get_sort_index(self) -> i32;

    #[method(name = "OnCursorMoveEnd", args = 0)]
    pub fn on_cursor_move_end(self) -> ();

    #[method(name = "SetMarking", args = 1)]
    pub fn set_marking(self, is_marking: bool) -> ();

    #[method(name = "SetMarkingCursor", args = 1)]
    pub fn set_marking_cursor(self, is_marking: bool) -> ();

    #[method(name = "HoldSelection", args = 0)]
    pub fn hold_selection(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "SetTextColor", args = 2)]
    pub fn set_text_color(
        self,
        active_color: crate::unity_engine::color::Color,
        inactive_color: crate::unity_engine::color::Color,
    ) -> ();

    #[method(name = "UpdateTextColor", args = 1)]
    pub fn update_text_color(self, is_active: bool) -> ();

    #[method(name = "SetInitialColor", args = 0)]
    pub fn set_initial_color(self) -> ();
}

#[cfg(feature = "app-wellitemselectmenuitem")]
impl WellItemSelectMenuItem {
    pub fn new(unit: crate::app::unit::Unit, owner_item_index: i32, sort_index: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WellItemSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IWellItemSelectMenuItemMethods>::ctor(this, unit, owner_item_index, sort_index);
        this
    }
}
