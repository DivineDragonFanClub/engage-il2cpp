
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitselectmenu/RefreshUnitSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "RefreshUnitSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RefreshUnitSelectMenu {
    #[static_field]
    #[rename(name = "m_MenuItemIndexNone")]
    pub m_menu_item_index_none: i32,
    #[static_field]
    #[rename(name = "m_MenuItemIndexEmpty")]
    pub m_menu_item_index_empty: i32,
    #[rename(name = "m_DecidedMenuItemIndex")]
    pub m_decided_menu_item_index: i32,
    #[rename(name = "m_UnselectableUnit")]
    pub m_unselectable_unit: crate::app::unit::Unit,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::refreshunitselectmenu::RefreshUnitSelectMenu_DecideEventHandler,
    #[rename(name = "m_CloseEventHandler")]
    pub m_close_event_handler:
        crate::app::refreshunitselectmenu::RefreshUnitSelectMenu_CloseEventHandler,
    #[rename(name = "m_DisposeEventHandler")]
    pub m_dispose_event_handler:
        crate::app::refreshunitselectmenu::RefreshUnitSelectMenu_DisposeEventHandler,
}

#[cfg(feature = "app-refreshunitselectmenu")]
#[::unity2::methods]
impl RefreshUnitSelectMenu {
    #[method(name = "CreateBind", args = 10)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::refreshunitselectmenucontent::RefreshUnitSelectMenuContent,
        initial_selected_unit: crate::app::unit::Unit,
        unselectable_unit: crate::app::unit::Unit,
        selected_entrust: bool,
        initial_scroll_index: i32,
        select_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_DecideEventHandler,
        close_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_CloseEventHandler,
        dispose_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_DisposeEventHandler,
    ) -> crate::app::refreshunitselectmenu::RefreshUnitSelectMenu;

    #[method(name = "CreateMenuItem", args = 6)]
    pub fn create_menu_item(
        initial_selected_unit: crate::app::unit::Unit,
        unselectable_unit: crate::app::unit::Unit,
        selected_entrust: bool,
        select_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_DecideEventHandler,
        initial_select_index: i32,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 8)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::refreshunitselectmenucontent::RefreshUnitSelectMenuContent,
        initial_select_index: i32,
        unselectable_unit: crate::app::unit::Unit,
        initial_scroll_index: i32,
        decide_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_DecideEventHandler,
        close_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_CloseEventHandler,
        dispose_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_DisposeEventHandler,
    ) -> ();

    #[method(name = "OnBuild", args = 1)]
    pub fn on_build(self, is_first_build: bool) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "AdjustScrollIndex", args = 0)]
    pub fn adjust_scroll_index(self) -> ();

    #[method(name = "GetUnselectableUnit", args = 0)]
    pub fn get_unselectable_unit(self) -> crate::app::unit::Unit;

    #[method(name = "SetDecidedMenuItem", args = 1)]
    pub fn set_decided_menu_item(self, menu_item_index: i32) -> bool;

    #[method(name = "CancelSetDecidedMenuItem", args = 0)]
    pub fn cancel_set_decided_menu_item(self) -> ();

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-refreshunitselectmenu")]
impl RefreshUnitSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::refreshunitselectmenucontent::RefreshUnitSelectMenuContent,
        initial_select_index: i32,
        unselectable_unit: crate::app::unit::Unit,
        initial_scroll_index: i32,
        decide_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_DecideEventHandler,
        close_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_CloseEventHandler,
        dispose_event_handler : crate :: app :: refreshunitselectmenu :: RefreshUnitSelectMenu_DisposeEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshUnitSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshUnitSelectMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            initial_select_index,
            unselectable_unit,
            initial_scroll_index,
            decide_event_handler,
            close_event_handler,
            dispose_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitselectmenu/RefreshUnitSelectMenu_CloseEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "RefreshUnitSelectMenu.CloseEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefreshUnitSelectMenu_CloseEventHandler {}

#[cfg(feature = "app-refreshunitselectmenu")]
#[::unity2::methods]
impl RefreshUnitSelectMenu_CloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refreshunitselectmenu")]
impl RefreshUnitSelectMenu_CloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshUnitSelectMenu_CloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshUnitSelectMenu_CloseEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitselectmenu/RefreshUnitSelectMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "RefreshUnitSelectMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefreshUnitSelectMenu_DecideEventHandler {}

#[cfg(feature = "app-refreshunitselectmenu")]
#[::unity2::methods]
impl RefreshUnitSelectMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(
        self,
        unit: crate::app::unit::Unit,
        selected_entrust: bool,
        scroll_index: i32,
    ) -> ();
}

#[cfg(feature = "app-refreshunitselectmenu")]
impl RefreshUnitSelectMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshUnitSelectMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshUnitSelectMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitselectmenu/RefreshUnitSelectMenu_SelectEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "RefreshUnitSelectMenu.SelectEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefreshUnitSelectMenu_SelectEventHandler {}

#[cfg(feature = "app-refreshunitselectmenu")]
#[::unity2::methods]
impl RefreshUnitSelectMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(
        self,
        unit: crate::app::unit::Unit,
        caption_mid: ::unity2::Il2CppString,
        message_mid: ::unity2::Il2CppString,
    ) -> ();
}

#[cfg(feature = "app-refreshunitselectmenu")]
impl RefreshUnitSelectMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshUnitSelectMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshUnitSelectMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshunitselectmenu/RefreshUnitSelectMenu_DisposeEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "RefreshUnitSelectMenu.DisposeEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefreshUnitSelectMenu_DisposeEventHandler {}

#[cfg(feature = "app-refreshunitselectmenu")]
#[::unity2::methods]
impl RefreshUnitSelectMenu_DisposeEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refreshunitselectmenu")]
impl RefreshUnitSelectMenu_DisposeEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshUnitSelectMenu_DisposeEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshUnitSelectMenu_DisposeEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
