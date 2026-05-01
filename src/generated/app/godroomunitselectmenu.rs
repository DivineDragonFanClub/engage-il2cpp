
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godroomunitselectmenu/GodRoomUnitSelectMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "GodRoomUnitSelectMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct GodRoomUnitSelectMenu_DecideEventHandler {}

#[cfg(feature = "app-godroomunitselectmenu")]
#[::unity2::methods]
impl GodRoomUnitSelectMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        unit: crate::app::unit::Unit,
        god_list: crate::system::collections::generic::list_1::List_1<crate::app::godunit::GodUnit>,
    ) -> ();
}

#[cfg(feature = "app-godroomunitselectmenu")]
impl GodRoomUnitSelectMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRoomUnitSelectMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRoomUnitSelectMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godroomunitselectmenu/GodRoomUnitSelectMenu_GodRoomUnitSelectMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GodRoomUnitSelectMenu.GodRoomUnitSelectMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct GodRoomUnitSelectMenu_GodRoomUnitSelectMenuItem {
    #[rename(name = "m_Index")]
    pub m_index: i32,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::godroomunitselectmenu::GodRoomUnitSelectMenu_DecideEventHandler,
}

#[cfg(feature = "app-godroomunitselectmenu")]
#[::unity2::methods]
impl GodRoomUnitSelectMenu_GodRoomUnitSelectMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        index: i32,
        unit: crate::app::unit::Unit,
        decide_event_handler : crate :: app :: godroomunitselectmenu :: GodRoomUnitSelectMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;
}

#[cfg(feature = "app-godroomunitselectmenu")]
impl GodRoomUnitSelectMenu_GodRoomUnitSelectMenuItem {
    pub fn new(
        index: i32,
        unit: crate::app::unit::Unit,
        decide_event_handler : crate :: app :: godroomunitselectmenu :: GodRoomUnitSelectMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRoomUnitSelectMenu_GodRoomUnitSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRoomUnitSelectMenu_GodRoomUnitSelectMenuItemMethods>::ctor(
            this,
            index,
            unit,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godroomunitselectmenu/GodRoomUnitSelectMenu_GodRoomUnitSelectEmptyMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "GodRoomUnitSelectMenu.GodRoomUnitSelectEmptyMenuItem"
)]
#[parent(crate::app::godroomunitselectmenu::GodRoomUnitSelectMenu_GodRoomUnitSelectMenuItem)]
pub struct GodRoomUnitSelectMenu_GodRoomUnitSelectEmptyMenuItem {}

#[cfg(feature = "app-godroomunitselectmenu")]
#[::unity2::methods]
impl GodRoomUnitSelectMenu_GodRoomUnitSelectEmptyMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-godroomunitselectmenu")]
impl GodRoomUnitSelectMenu_GodRoomUnitSelectEmptyMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRoomUnitSelectMenu_GodRoomUnitSelectEmptyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRoomUnitSelectMenu_GodRoomUnitSelectEmptyMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godroomunitselectmenu/GodRoomUnitSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "GodRoomUnitSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct GodRoomUnitSelectMenu {
    #[static_field]
    #[rename(name = "ForceMask")]
    pub force_mask: u32,
    #[static_field]
    #[rename(name = "s_ScrollIndex")]
    pub s_scroll_index: i32,
}

#[cfg(feature = "app-godroomunitselectmenu")]
#[::unity2::methods]
impl GodRoomUnitSelectMenu {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        decide_event_handler : crate :: app :: godroomunitselectmenu :: GodRoomUnitSelectMenu_DecideEventHandler,
        select_unit: crate::app::unit::Unit,
    ) -> crate::app::godroomunitselectmenu::GodRoomUnitSelectMenu;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::godroomunitselectmenucontent::GodRoomUnitSelectMenuContent,
        default_menu_item_index: i32,
    ) -> ();

    #[method(name = "GetGodListActive", args = 0)]
    pub fn get_god_list_active(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::godunit::GodUnit>;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "UpdateUnit", args = 1)]
    pub fn update_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "CloseStatus", args = 0)]
    pub fn close_status(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-godroomunitselectmenu")]
impl GodRoomUnitSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::godroomunitselectmenucontent::GodRoomUnitSelectMenuContent,
        default_menu_item_index: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodRoomUnitSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IGodRoomUnitSelectMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            default_menu_item_index,
        );
        this
    }
}
