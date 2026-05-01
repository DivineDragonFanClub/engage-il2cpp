
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitaicommandmenu/MapUnitAICommandMenu.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitAICommandMenu")]
#[parent(crate::app::mapbasicmenu::MapBasicMenu)]
pub struct MapUnitAICommandMenu {
    #[rename(name = "m_MapUnitMenuContent")]
    pub m_map_unit_menu_content: crate::app::mapunitcommandmenucontent::MapUnitCommandMenuContent,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_OpenCallback")]
    pub m_open_callback: crate::system::action::Action,
    #[rename(name = "m_CloseCallback")]
    pub m_close_callback: crate::system::action::Action,
}

#[cfg(feature = "app-mapunitaicommandmenu")]
#[::unity2::methods]
impl MapUnitAICommandMenu {
    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::mapunitcommandmenucontent::MapUnitCommandMenuContent,
        unit: crate::app::unit::Unit,
        open_callback: crate::system::action::Action,
        close_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        open_callback: crate::system::action::Action,
        close_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();
}

#[cfg(feature = "app-mapunitaicommandmenu")]
impl MapUnitAICommandMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::mapunitcommandmenucontent::MapUnitCommandMenuContent,
        unit: crate::app::unit::Unit,
        open_callback: crate::system::action::Action,
        close_callback: crate::system::action::Action,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitAICommandMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitAICommandMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            unit,
            open_callback,
            close_callback,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapunitaicommandmenu/MapUnitAICommandMenu_AIMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "MapUnitAICommandMenu.AIMenuItem")]
#[parent(crate::app::mapbasicmenuitem::MapBasicMenuItem)]
pub struct MapUnitAICommandMenu_AIMenuItem {
    #[rename(name = "m_AIType")]
    pub m_ai_type: crate::app::unitai::UnitAI_VersusTypes,
}

#[cfg(feature = "app-mapunitaicommandmenu")]
#[::unity2::methods]
impl MapUnitAICommandMenu_AIMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, ai_type: crate::app::unitai::UnitAI_VersusTypes) -> ();

    #[method(name = "get_FlagID", args = 0)]
    pub fn get_flag_id(self) -> ::unity2::Il2CppString;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "SetupHelpText", args = 0)]
    pub fn setup_help_text(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHelpText", args = 0)]
    pub fn get_help_text(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMapAttribute", args = 0)]
    pub fn get_map_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-mapunitaicommandmenu")]
impl MapUnitAICommandMenu_AIMenuItem {
    pub fn new(ai_type: crate::app::unitai::UnitAI_VersusTypes) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapUnitAICommandMenu_AIMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IMapUnitAICommandMenu_AIMenuItemMethods>::ctor(this, ai_type);
        this
    }
}
