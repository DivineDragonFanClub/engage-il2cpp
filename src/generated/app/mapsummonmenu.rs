
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::mapbasicmenu::IMapBasicMenu;
use crate::app::mapbasicmenu::MapBasicMenu;
use crate::app::mapbasicmenuitem::IMapBasicMenuItem;
use crate::app::mapbasicmenuitem::MapBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsummonmenu/MapSummonMenu_SummonColorMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapSummonMenu.SummonColorMenuItem")]
#[parent(crate::app::mapbasicmenuitem::MapBasicMenuItem)]
pub struct MapSummonMenu_SummonColorMenuItem {
    #[rename(name = "m_Color")]
    pub m_color: crate::app::persondata::PersonData_Colors,
}

#[cfg(feature = "app-mapsummonmenu")]
#[::unity2::methods]
impl MapSummonMenu_SummonColorMenuItem {
    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "OnCursorMoveEnd", args = 0)]
    pub fn on_cursor_move_end(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, color: crate::app::persondata::PersonData_Colors) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCommandHelp", args = 0)]
    pub fn get_command_help(self) -> ::unity2::Il2CppString;

    #[method(name = "SetupHelpText", args = 0)]
    pub fn setup_help_text(self) -> ();

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapsummonmenu")]
impl MapSummonMenu_SummonColorMenuItem {
    pub fn new(color: crate::app::persondata::PersonData_Colors) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSummonMenu_SummonColorMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSummonMenu_SummonColorMenuItemMethods>::ctor(this, color);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsummonmenu/MapSummonMenu.md")))]
#[::unity2::class(namespace = "App", name = "MapSummonMenu")]
#[parent(crate::app::mapbasicmenu::MapBasicMenu)]
pub struct MapSummonMenu {
    #[static_field]
    #[rename(name = "s_SelectIndex")]
    pub s_select_index: i32,
    #[rename(name = "m_MapUnitCommandMenuContent")]
    pub m_map_unit_command_menu_content:
        crate::app::mapunitcommandmenucontent::MapUnitCommandMenuContent,
}

#[cfg(feature = "app-mapsummonmenu")]
#[::unity2::methods]
impl MapSummonMenu {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::mapunitcommandmenucontent::MapUnitCommandMenuContent,
    ) -> ();

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "CreateSummonBind", args = 1)]
    pub fn create_summon_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "ResetSelectIndex", args = 0)]
    pub fn reset_select_index() -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapsummonmenu")]
impl MapSummonMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::mapunitcommandmenucontent::MapUnitCommandMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSummonMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSummonMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}
