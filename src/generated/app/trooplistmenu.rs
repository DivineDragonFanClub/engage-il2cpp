
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/trooplistmenu/TroopListMenu_ConfirmBattleSequence.md")))]
#[::unity2::class(namespace = "App", name = "TroopListMenu.ConfirmBattleSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct TroopListMenu_ConfirmBattleSequence {
    #[rename(name = "m_ParentMenu")]
    pub m_parent_menu: crate::app::basicmenu::BasicMenu,
}

#[cfg(feature = "app-trooplistmenu")]
#[::unity2::methods]
impl TroopListMenu_ConfirmBattleSequence {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(parent_menu: crate::app::basicmenu::BasicMenu) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, parent_menu: crate::app::basicmenu::BasicMenu) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "CreateDialog", args = 0)]
    pub fn create_dialog(self) -> ();
}

#[cfg(feature = "app-trooplistmenu")]
impl TroopListMenu_ConfirmBattleSequence {
    pub fn new(parent_menu: crate::app::basicmenu::BasicMenu) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TroopListMenu_ConfirmBattleSequence),
                ::core::stringify!(new),
            )
        });
        <Self as ITroopListMenu_ConfirmBattleSequenceMethods>::ctor(this, parent_menu);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/trooplistmenu/TroopListMenu.md")))]
#[::unity2::class(namespace = "App", name = "TroopListMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct TroopListMenu {
    #[static_field]
    #[rename(name = "m_mode")]
    pub m_mode: crate::app::sortiesequencetrooplist::SortieSequenceTroopList_Mode,
    #[static_field]
    #[rename(name = "m_page")]
    pub m_page: i32,
    #[static_field]
    #[rename(name = "ListUnitNum")]
    pub list_unit_num: i32,
    #[static_field]
    #[rename(name = "m_ListUnitNum")]
    pub m_list_unit_num: i32,
    #[static_field]
    #[rename(name = "m_ListUnit")]
    pub m_list_unit: ::unity2::Array<crate::app::unit::Unit>,
    #[static_field]
    #[rename(name = "m_TmpListUnit")]
    pub m_tmp_list_unit: ::unity2::Array<crate::app::unit::Unit>,
    #[static_field]
    #[rename(name = "m_sortMenu")]
    pub m_sort_menu: crate::app::trooplistsortmenu::TroopListSortMenu,
    #[rename(name = "m_SelectSort")]
    pub m_select_sort: bool,
}

#[cfg(feature = "app-trooplistmenu")]
#[::unity2::methods]
impl TroopListMenu {
    #[method(name = "Create", args = 4)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::trooplistmenucontent::TroopListMenuContent,
        sort_menu: crate::app::trooplistsortmenu::TroopListSortMenu,
        mode: crate::app::sortiesequencetrooplist::SortieSequenceTroopList_Mode,
    ) -> crate::app::trooplistmenu::TroopListMenu;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::trooplistmenucontent::TroopListMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "MoveUp", args = 1)]
    pub fn move_up(self, is_trigger: bool) -> ();

    #[method(name = "MoveDown", args = 1)]
    pub fn move_down(self, is_trigger: bool) -> ();

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetCurrentPage", args = 0)]
    pub fn get_current_page(self) -> i32;

    #[method(name = "IncPage", args = 1)]
    pub fn inc_page(self, is_trigger: bool) -> ();

    #[method(name = "DecPage", args = 1)]
    pub fn dec_page(self, is_trigger: bool) -> ();

    #[method(name = "IncSort", args = 1)]
    pub fn inc_sort(self, is_trigger: bool) -> ();

    #[method(name = "DecSort", args = 1)]
    pub fn dec_sort(self, is_trigger: bool) -> ();

    #[method(name = "InitUnitOrder", args = 0)]
    pub fn init_unit_order() -> ();

    #[method(name = "AddListUnit", args = 1)]
    pub fn add_list_unit(unit: crate::app::unit::Unit) -> ();

    #[method(name = "InitUnitOrderNormal", args = 0)]
    pub fn init_unit_order_normal() -> ();

    #[method(name = "InitUnitOrderRelay", args = 0)]
    pub fn init_unit_order_relay() -> ();

    #[method(name = "SortUnitOrder", args = 0)]
    pub fn sort_unit_order() -> ();

    #[method(name = "GetSortValue", args = 2)]
    pub fn get_sort_value(
        unit: crate::app::unit::Unit,
        r#type: crate::app::trooplistsortmenu::TroopListSortMenu_SortType,
    ) -> i32;

    #[method(name = "GetSortUnit", args = 1)]
    pub fn get_sort_unit(self, index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetSortTmpUnit", args = 1)]
    pub fn get_sort_tmp_unit(self, index: i32) -> crate::app::unit::Unit;

    #[method(name = "IsSelectSort", args = 0)]
    pub fn is_select_sort(self) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-trooplistmenu")]
impl TroopListMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::trooplistmenucontent::TroopListMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TroopListMenu),
                ::core::stringify!(new),
            )
        });
        <Self as ITroopListMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/trooplistmenu/TroopListMenu_TroopListMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "TroopListMenu.TroopListMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct TroopListMenu_TroopListMenuItem {
    #[rename(name = "m_index")]
    pub m_index: i32,
    #[rename(name = "m_mode")]
    pub m_mode: crate::app::sortiesequencetrooplist::SortieSequenceTroopList_Mode,
}

#[cfg(feature = "app-trooplistmenu")]
#[::unity2::methods]
impl TroopListMenu_TroopListMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        index: i32,
        mode: crate::app::sortiesequencetrooplist::SortieSequenceTroopList_Mode,
    ) -> ();

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetTmpUnit", args = 0)]
    pub fn get_tmp_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetMode", args = 0)]
    pub fn get_mode(self) -> crate::app::sortiesequencetrooplist::SortieSequenceTroopList_Mode;

    #[method(name = "GetRectTransform", args = 0)]
    pub fn get_rect_transform(self) -> crate::unity_engine::recttransform::RectTransform;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "RCall", args = 0)]
    pub fn r_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "PlusCall", args = 0)]
    pub fn plus_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "SetPage", args = 1)]
    pub fn set_page(self, page: i32) -> ();
}

#[cfg(feature = "app-trooplistmenu")]
impl TroopListMenu_TroopListMenuItem {
    pub fn new(
        index: i32,
        mode: crate::app::sortiesequencetrooplist::SortieSequenceTroopList_Mode,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TroopListMenu_TroopListMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as ITroopListMenu_TroopListMenuItemMethods>::ctor(this, index, mode);
        this
    }
}
