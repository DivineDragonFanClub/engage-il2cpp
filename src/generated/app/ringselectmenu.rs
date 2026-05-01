
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringselectmenu/RingSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "RingSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RingSelectMenu {
    #[static_field]
    #[rename(name = "GidMax")]
    pub gid_max: i32,
    #[rename(name = "m_unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_statusSetter")]
    pub m_status_setter: crate::app::unitstatussetter::UnitStatusSetter,
    #[rename(name = "m_ringSelectRoot")]
    pub m_ring_select_root: crate::app::ringselectroot::RingSelectRoot,
    #[rename(name = "m_menuSelect")]
    pub m_menu_select: crate::system::collections::generic::list_1::List_1<
        crate::app::basicmenuselect::BasicMenuSelect,
    >,
    #[rename(name = "m_selectPage")]
    pub m_select_page: i32,
    #[static_field]
    #[rename(name = "m_gidPageName")]
    pub m_gid_page_name:
        crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "m_isUnitChangeAnim")]
    pub m_is_unit_change_anim: bool,
    #[static_field]
    #[rename(name = "m_isUnitChangeEquip")]
    pub m_is_unit_change_equip: bool,
    #[static_field]
    #[rename(name = "m_ringUnitList")]
    pub m_ring_unit_list: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::app::unit::Unit,
        ::unity2::Il2CppString,
    >,
}

#[cfg(feature = "app-ringselectmenu")]
#[::unity2::methods]
impl RingSelectMenu {
    #[method(name = "GetPoolGodNum", args = 0)]
    pub fn get_pool_god_num() -> i32;

    #[method(name = "GetPoolGod", args = 1)]
    pub fn get_pool_god(index: i32) -> crate::app::godunit::GodUnit;

    #[method(name = "IsIgnoreForRelay", args = 1)]
    pub fn is_ignore_for_relay(god: crate::app::godunit::GodUnit) -> bool;

    #[method(name = "IsIgnoreForRelay", args = 1)]
    pub fn is_ignore_for_relay_2(unit_ring: crate::app::unitring::UnitRing) -> bool;

    #[method(name = "Create", args = 5)]
    pub fn create(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        menu_content: crate::app::ringselectmenucontent::RingSelectMenuContent,
        status_setter: crate::app::unitstatussetter::UnitStatusSetter,
        ring_select_root: crate::app::ringselectroot::RingSelectRoot,
    ) -> crate::app::ringselectmenu::RingSelectMenu;

    #[method(name = "CreateRingCategory", args = 1)]
    pub fn create_ring_category(unit: crate::app::unit::Unit) -> i32;

    #[method(name = "CreateMenuItemList", args = 4)]
    pub fn create_menu_item_list(
        unit: crate::app::unit::Unit,
        page: i32,
        ring_select_root: crate::app::ringselectroot::RingSelectRoot,
        default_menu_item_index: i32,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = "CreateMenuItemListForVersus", args = 2)]
    pub fn create_menu_item_list_for_versus(
        unit: crate::app::unit::Unit,
        default_index: i32,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = "RebuildMenu", args = 2)]
    pub fn rebuild_menu(self, display_index: i32, equip_unit: crate::app::unit::Unit) -> ();

    #[method(name = "SaveSelect", args = 0)]
    pub fn save_select(self) -> ();

    #[method(name = "IsEmptyCommon", args = 0)]
    pub fn is_empty_common(self) -> bool;

    #[method(name = "GetDisplayIndex", args = 0)]
    pub fn get_display_index(self) -> i32;

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::ringselectmenucontent::RingSelectMenuContent,
        unit: crate::app::unit::Unit,
        status_setter: crate::app::unitstatussetter::UnitStatusSetter,
        ring_select_root: crate::app::ringselectroot::RingSelectRoot,
        page: i32,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "CustomCall", args = 0)]
    pub fn custom_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "UpdateStatus", args = 0)]
    pub fn update_status(self) -> ();

    #[method(name = "GetPageNum", args = 0)]
    pub fn get_page_num(self) -> i32;

    #[method(name = "GetPageGodGid", args = 1)]
    pub fn get_page_god_gid(self, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetSelectPage", args = 0)]
    pub fn get_select_page(self) -> i32;

    #[method(name = "GetSelectRing", args = 0)]
    pub fn get_select_ring(self) -> crate::app::unitring::UnitRing;

    #[method(name = "GetSelectGod", args = 0)]
    pub fn get_select_god(self) -> crate::app::godunit::GodUnit;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-ringselectmenu")]
impl RingSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::ringselectmenucontent::RingSelectMenuContent,
        unit: crate::app::unit::Unit,
        status_setter: crate::app::unitstatussetter::UnitStatusSetter,
        ring_select_root: crate::app::ringselectroot::RingSelectRoot,
        page: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRingSelectMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            unit,
            status_setter,
            ring_select_root,
            page,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringselectmenu/RingSelectMenu_RingMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RingSelectMenu.RingMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RingSelectMenu_RingMenuItem {
    #[rename(name = "m_isGod")]
    pub m_is_god: bool,
    #[rename(name = "m_god")]
    pub m_god: crate::app::godunit::GodUnit,
    #[rename(name = "m_ring")]
    pub m_ring: crate::app::unitring::UnitRing,
}

#[cfg(feature = "app-ringselectmenu")]
#[::unity2::methods]
impl RingSelectMenu_RingMenuItem {
    #[method(name = "get_RingSortId", args = 0)]
    pub fn get_ring_sort_id(self) -> i32;

    #[method(name = "set_RingSortId", args = 1)]
    pub fn set_ring_sort_id(self, value: i32) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        is_god: bool,
        god: crate::app::godunit::GodUnit,
        ring: crate::app::unitring::UnitRing,
        ring_sort_id: i32,
    ) -> ();

    #[method(name = "IsGod", args = 0)]
    pub fn is_god(self) -> bool;

    #[method(name = "GetGod", args = 0)]
    pub fn get_god(self) -> crate::app::godunit::GodUnit;

    #[method(name = "GetCommon", args = 0)]
    pub fn get_common(self) -> crate::app::unitring::UnitRing;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "CalcAttribute", args = 3)]
    pub fn calc_attribute(
        unit: crate::app::unit::Unit,
        god: crate::app::godunit::GodUnit,
        ring: crate::app::unitring::UnitRing,
    ) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-ringselectmenu")]
impl RingSelectMenu_RingMenuItem {
    pub fn new(
        is_god: bool,
        god: crate::app::godunit::GodUnit,
        ring: crate::app::unitring::UnitRing,
        ring_sort_id: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingSelectMenu_RingMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRingSelectMenu_RingMenuItemMethods>::ctor(this, is_god, god, ring, ring_sort_id);
        this
    }
}
