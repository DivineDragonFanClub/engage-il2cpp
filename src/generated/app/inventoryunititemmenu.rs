
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inventoryunititemmenu/InventoryUnitItemMenu.md")))]
#[::unity2::class(namespace = "App", name = "InventoryUnitItemMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct InventoryUnitItemMenu {
    #[static_field]
    #[rename(name = "m_unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_firstSelect")]
    pub m_first_select: i32,
    #[rename(name = "m_secondSelect")]
    pub m_second_select: i32,
}

#[cfg(feature = "app-inventoryunititemmenu")]
#[::unity2::methods]
impl InventoryUnitItemMenu {
    #[method(name = "get_m_CommonDisplayIndex", args = 0)]
    pub fn get_m_common_display_index(self) -> i32;

    #[method(name = "set_m_CommonDisplayIndex", args = 1)]
    pub fn set_m_common_display_index(self, value: i32) -> ();

    #[method(name = "Create", args = 4)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        default_select: i32,
        menu_content: crate::app::inventoryunititemmenucontent::InventoryUnitItemMenuContent,
    ) -> crate::app::inventoryunititemmenu::InventoryUnitItemMenu;

    #[method(name = "CreateMenuItemList", args = 1)]
    pub fn create_menu_item_list(
        unit: crate::app::unit::Unit,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::inventoryunititemmenucontent::InventoryUnitItemMenuContent,
    ) -> ();

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "UpdateUnit", args = 1)]
    pub fn update_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetSelectItem", args = 0)]
    pub fn get_select_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "GetMenuItemIndexEquipped", args = 0)]
    pub fn get_menu_item_index_equipped(self) -> i32;

    #[method(name = "ClearSelect", args = 0)]
    pub fn clear_select(self) -> ();

    #[method(name = "SetSelect", args = 1)]
    pub fn set_select(self, select_no: i32) -> ();

    #[method(name = "GetFirstSelect", args = 0)]
    pub fn get_first_select(self) -> i32;

    #[method(name = "GetSecondSelect", args = 0)]
    pub fn get_second_select(self) -> i32;

    #[method(name = "ShowCursor", args = 1)]
    pub fn show_cursor(self, is_show: bool) -> ();

    #[method(name = "IsShowCursor", args = 0)]
    pub fn is_show_cursor(self) -> bool;

    #[method(name = "EnableInput", args = 1)]
    pub fn enable_input(self, is_enable: bool) -> ();

    #[method(name = "IsEnableInput", args = 0)]
    pub fn is_enable_input(self) -> bool;

    #[method(name = "SetSuspend", args = 1)]
    pub fn set_suspend(self, is_active: bool) -> ();

    #[method(name = "SetFirstSelection", args = 1)]
    pub fn set_first_selection(self, update_common_display_index: bool) -> ();

    #[method(name = "ResetFirstSelection", args = 0)]
    pub fn reset_first_selection(self) -> ();

    #[method(name = "SetSelectIndexOnChangeMenu", args = 1)]
    pub fn set_select_index_on_change_menu(self, common_display_index: i32) -> ();

    #[method(name = "HoldSelection", args = 0)]
    pub fn hold_selection(self) -> ();

    #[method(name = "UpdateMenu", args = 1)]
    pub fn update_menu(self, is_rebuild: bool) -> ();

    #[method(name = "GetSelectableItemCount", args = 0)]
    pub fn get_selectable_item_count(self) -> i32;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "ClampMenuItemIndex", args = 1)]
    pub fn clamp_menu_item_index(self, item_index: i32) -> i32;

    #[method(name = "KeyUp", args = 1)]
    pub fn key_up(self, is_trigger: bool) -> ();

    #[method(name = "KeyDown", args = 1)]
    pub fn key_down(self, is_trigger: bool) -> ();

    #[method(name = "KeyLeft", args = 1)]
    pub fn key_left(self, is_trigger: bool) -> ();

    #[method(name = "KeyRight", args = 1)]
    pub fn key_right(self, is_trigger: bool) -> ();

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-inventoryunititemmenu")]
impl InventoryUnitItemMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::inventoryunititemmenucontent::InventoryUnitItemMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InventoryUnitItemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IInventoryUnitItemMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
