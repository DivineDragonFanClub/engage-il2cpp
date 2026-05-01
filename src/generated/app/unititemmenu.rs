
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unititemmenu/UnitItemMenu.md")))]
#[::unity2::class(namespace = "App", name = "UnitItemMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct UnitItemMenu {
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler: crate::app::unititemmenu::UnitItemMenu_SelectEventHandler,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::unititemmenu::UnitItemMenu_DecideEventHandler,
    #[rename(name = "m_RequestCloseEventHandler")]
    pub m_request_close_event_handler:
        crate::app::unititemmenu::UnitItemMenu_RequestCloseEventHandler,
}

#[cfg(feature = "app-unititemmenu")]
#[::unity2::methods]
impl UnitItemMenu {
    #[method(name = "get_m_Unit", args = 0)]
    pub fn get_m_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_m_Unit", args = 1)]
    pub fn set_m_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::unititemmenucontent::UnitItemMenuContent,
        unit: crate::app::unit::Unit,
        select_event_handler: crate::app::unititemmenu::UnitItemMenu_SelectEventHandler,
        decide_event_handler: crate::app::unititemmenu::UnitItemMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: unititemmenu :: UnitItemMenu_RequestCloseEventHandler,
    ) -> crate::app::unititemmenu::UnitItemMenu;

    #[method(name = "CreateMenuItem", args = 4)]
    pub fn create_menu_item(
        unit: crate::app::unit::Unit,
        select_event_handler: crate::app::unititemmenu::UnitItemMenu_SelectEventHandler,
        decide_event_handler: crate::app::unititemmenu::UnitItemMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: unititemmenu :: UnitItemMenu_RequestCloseEventHandler,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::unititemmenucontent::UnitItemMenuContent,
        unit: crate::app::unit::Unit,
        select_event_handler: crate::app::unititemmenu::UnitItemMenu_SelectEventHandler,
        decide_event_handler: crate::app::unititemmenu::UnitItemMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: unititemmenu :: UnitItemMenu_RequestCloseEventHandler,
    ) -> ();

    #[method(name = "RebuildMenu", args = 1)]
    pub fn rebuild_menu(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "EnableMenu", args = 0)]
    pub fn enable_menu(self) -> ();

    #[method(name = "DisableMenu", args = 0)]
    pub fn disable_menu(self) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();
}

#[cfg(feature = "app-unititemmenu")]
impl UnitItemMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::unititemmenucontent::UnitItemMenuContent,
        unit: crate::app::unit::Unit,
        select_event_handler: crate::app::unititemmenu::UnitItemMenu_SelectEventHandler,
        decide_event_handler: crate::app::unititemmenu::UnitItemMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: unititemmenu :: UnitItemMenu_RequestCloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitItemMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitItemMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            unit,
            select_event_handler,
            decide_event_handler,
            request_close_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unititemmenu/UnitItemMenu_RequestCloseEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "UnitItemMenu.RequestCloseEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct UnitItemMenu_RequestCloseEventHandler {}

#[cfg(feature = "app-unititemmenu")]
#[::unity2::methods]
impl UnitItemMenu_RequestCloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-unititemmenu")]
impl UnitItemMenu_RequestCloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitItemMenu_RequestCloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitItemMenu_RequestCloseEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unititemmenu/UnitItemMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "UnitItemMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct UnitItemMenu_DecideEventHandler {}

#[cfg(feature = "app-unititemmenu")]
#[::unity2::methods]
impl UnitItemMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, unit_item: crate::app::unititem::UnitItem) -> ();
}

#[cfg(feature = "app-unititemmenu")]
impl UnitItemMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitItemMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitItemMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unititemmenu/UnitItemMenu_SelectEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "UnitItemMenu.SelectEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct UnitItemMenu_SelectEventHandler {}

#[cfg(feature = "app-unititemmenu")]
#[::unity2::methods]
impl UnitItemMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, unit_item: crate::app::unititem::UnitItem) -> ();
}

#[cfg(feature = "app-unititemmenu")]
impl UnitItemMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitItemMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitItemMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
