
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::gridmenu::GridMenu;
use crate::app::gridmenu::IGridMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/trademenu/TradeMenu.md")))]
#[::unity2::class(namespace = "App", name = "TradeMenu")]
#[parent(crate::app::gridmenu::GridMenu)]
pub struct TradeMenu {
    #[rename(name = "m_selectItemIndex")]
    pub m_select_item_index: i32,
    #[rename(name = "m_CanLeftUnitAutoEquipBefore")]
    pub m_can_left_unit_auto_equip_before: bool,
    #[rename(name = "m_CanRightUnitAutoEquipBefore")]
    pub m_can_right_unit_auto_equip_before: bool,
}

#[cfg(feature = "app-trademenu")]
#[::unity2::methods]
impl TradeMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::trademenucontent::TradeMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetShowRowMax", args = 0)]
    pub fn get_show_row_max(self) -> i32;

    #[method(name = "GetBuildRowNum", args = 0)]
    pub fn get_build_row_num(self) -> i32;

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "InitLeftUnitInfo", args = 1)]
    pub fn init_left_unit_info(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "InitRightUnitInfo", args = 1)]
    pub fn init_right_unit_info(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "SetLeftUnitName", args = 1)]
    pub fn set_left_unit_name(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetRightUnitName", args = 1)]
    pub fn set_right_unit_name(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "CanLeftUnitAutoEquip", args = 0)]
    pub fn can_left_unit_auto_equip(self) -> bool;

    #[method(name = "CanRightUnitAutoEquip", args = 0)]
    pub fn can_right_unit_auto_equip(self) -> bool;

    #[method(name = "ShowSelectCursor", args = 0)]
    pub fn show_select_cursor(self) -> ();

    #[method(name = "HideSelectCursor", args = 0)]
    pub fn hide_select_cursor(self) -> ();

    #[method(name = "SetSelectItemIndex", args = 1)]
    pub fn set_select_item_index(self, item_index: i32) -> ();

    #[method(name = "GetSelectItemIndex", args = 0)]
    pub fn get_select_item_index(self) -> i32;
}

#[cfg(feature = "app-trademenu")]
impl TradeMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::trademenucontent::TradeMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TradeMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ITradeMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
