
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cooking_menu/unitselectmenu_2/UnitSelectMenu_UnitSelectMenuItem_SelectEventHandler.md")))]
#[::unity2::class(
    namespace = "App.CookingMenu",
    name = "UnitSelectMenu.UnitSelectMenuItem.SelectEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct UnitSelectMenu_UnitSelectMenuItem_SelectEventHandler {}

#[cfg(feature = "app-cooking_menu-unitselectmenu_2")]
#[::unity2::methods]
impl UnitSelectMenu_UnitSelectMenuItem_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, unit: crate::app::unit::Unit) -> ();
}

#[cfg(feature = "app-cooking_menu-unitselectmenu_2")]
impl UnitSelectMenu_UnitSelectMenuItem_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectMenu_UnitSelectMenuItem_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectMenu_UnitSelectMenuItem_SelectEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cooking_menu/unitselectmenu_2/UnitSelectMenu_UnitSelectMenuItem.md")))]
#[::unity2::class(
    namespace = "App.CookingMenu",
    name = "UnitSelectMenu.UnitSelectMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct UnitSelectMenu_UnitSelectMenuItem {
# [rename (name = "m_Unit")] pub m_unit : crate :: app :: unit :: Unit ,
# [rename (name = "m_IsDecided")] pub m_is_decided : bool ,
# [rename (name = "m_SelectEventHander")] pub m_select_event_hander : crate :: app :: cooking_menu :: unitselectmenu_2 :: UnitSelectMenu_UnitSelectMenuItem_SelectEventHandler ,
}

#[cfg(feature = "app-cooking_menu-unitselectmenu_2")]
#[::unity2::methods]
impl UnitSelectMenu_UnitSelectMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        select_event_handler : crate :: app :: cooking_menu :: unitselectmenu_2 :: UnitSelectMenu_UnitSelectMenuItem_SelectEventHandler,
    ) -> ();

    #[method(name = "Decide", args = 0)]
    pub fn decide(self) -> crate::app::unit::Unit;

    #[method(name = "Cancel", args = 0)]
    pub fn cancel(self) -> ();

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "IsDecided", args = 0)]
    pub fn is_decided(self) -> bool;
}

#[cfg(feature = "app-cooking_menu-unitselectmenu_2")]
impl UnitSelectMenu_UnitSelectMenuItem {
    pub fn new(
        unit: crate::app::unit::Unit,
        select_event_handler : crate :: app :: cooking_menu :: unitselectmenu_2 :: UnitSelectMenu_UnitSelectMenuItem_SelectEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectMenu_UnitSelectMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectMenu_UnitSelectMenuItemMethods>::ctor(this, unit, select_event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cooking_menu/unitselectmenu_2/UnitSelectMenu_2.md")))]
#[::unity2::class(namespace = "App.CookingMenu", name = "UnitSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct UnitSelectMenu_2 {
    #[static_field]
    #[rename(name = "s_SelectUnitList")]
    pub s_select_unit_list:
        crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
    #[static_field]
    #[rename(name = "s_Content")]
    pub s_content: crate::app::cooking_menu::unitselectmenucontent_2::UnitSelectMenuContent_2,
    #[static_field]
    #[rename(name = "s_SelectIndex")]
    pub s_select_index: i32,
    #[static_field]
    #[rename(name = "s_ScrollIndex")]
    pub s_scroll_index: i32,
    #[static_field]
    #[rename(name = "s_IsFirstSelect")]
    pub s_is_first_select: bool,
    #[rename(name = "m_CloseCallback")]
    pub m_close_callback: crate::system::action::Action,
}

#[cfg(feature = "app-cooking_menu-unitselectmenu_2")]
#[::unity2::methods]
impl UnitSelectMenu_2 {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::cooking_menu::unitselectmenucontent_2::UnitSelectMenuContent_2,
        select_unit_list: crate::system::collections::generic::list_1::List_1<
            crate::app::unit::Unit,
        >,
        close_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        select_unit_list: crate::system::collections::generic::list_1::List_1<
            crate::app::unit::Unit,
        >,
        all_content: crate::app::cooking_menu::dishallmenucontent::DishAllMenuContent,
        close_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = "OnSelectMenuItem", args = 1)]
    pub fn on_select_menu_item(unit: crate::app::unit::Unit) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "NextItem", args = 0)]
    pub fn next_item(self) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetUnitMenuItemCurrent", args = 0)]
    pub fn get_unit_menu_item_current(
        self,
    ) -> crate::app::cooking_menu::unitselectmenu_2::UnitSelectMenu_UnitSelectMenuItem;

    #[method(name = "GetUnitMenuItem", args = 1)]
    pub fn get_unit_menu_item(
        self,
        index: i32,
    ) -> crate::app::cooking_menu::unitselectmenu_2::UnitSelectMenu_UnitSelectMenuItem;

    #[method(name = "GetUnitMenuItem", args = 1)]
    pub fn get_unit_menu_item_2(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::app::cooking_menu::unitselectmenu_2::UnitSelectMenu_UnitSelectMenuItem;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "RestoreMenu", args = 2)]
    pub fn restore_menu(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::cooking_menu::unitselectmenu_2::UnitSelectMenu_UnitSelectMenuItem,
        >,
        selected_unit_list: crate::system::collections::generic::list_1::List_1<
            crate::app::unit::Unit,
        >,
    ) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-cooking_menu-unitselectmenu_2")]
impl UnitSelectMenu_2 {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::cooking_menu::unitselectmenucontent_2::UnitSelectMenuContent_2,
        select_unit_list: crate::system::collections::generic::list_1::List_1<
            crate::app::unit::Unit,
        >,
        close_callback: crate::system::action::Action,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitSelectMenu_2),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitSelectMenu_2Methods>::ctor(
            this,
            menu_item_list,
            menu_content,
            select_unit_list,
            close_callback,
        );
        this
    }
}
