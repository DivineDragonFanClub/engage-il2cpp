
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondunitselectmenu/ArenaBondUnitSelectMenu_SelectEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondUnitSelectMenu.SelectEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaBondUnitSelectMenu_SelectEventHandler {}

#[cfg(feature = "app-arenabondunitselectmenu")]
#[::unity2::methods]
impl ArenaBondUnitSelectMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, unit: crate::app::unit::Unit) -> ();
}

#[cfg(feature = "app-arenabondunitselectmenu")]
impl ArenaBondUnitSelectMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondUnitSelectMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondUnitSelectMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondunitselectmenu/ArenaBondUnitSelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondUnitSelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ArenaBondUnitSelectMenu {
    #[rename(name = "m_HelpEventHandler")]
    pub m_help_event_handler:
        crate::app::arenabondunitselectmenu::ArenaBondUnitSelectMenu_HelpEventHandler,
}

#[cfg(feature = "app-arenabondunitselectmenu")]
#[::unity2::methods]
impl ArenaBondUnitSelectMenu {
    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content: crate::app::arenabondunitselectmenucontent::ArenaBondUnitSelectMenuContent,
        default_unit: crate::app::unit::Unit,
        decide_event_handler : crate :: app :: arenabondunitselectmenu :: ArenaBondUnitSelectMenu_DecideEventHandler,
        select_event_handler : crate :: app :: arenabondunitselectmenu :: ArenaBondUnitSelectMenu_SelectEventHandler,
        help_event_handler : crate :: app :: arenabondunitselectmenu :: ArenaBondUnitSelectMenu_HelpEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::arenabondunitselectmenucontent::ArenaBondUnitSelectMenuContent,
        default_menu_item_index: i32,
        help_event_handler : crate :: app :: arenabondunitselectmenu :: ArenaBondUnitSelectMenu_HelpEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "XCall", args = 0)]
    pub fn x_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-arenabondunitselectmenu")]
impl ArenaBondUnitSelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::arenabondunitselectmenucontent::ArenaBondUnitSelectMenuContent,
        default_menu_item_index: i32,
        help_event_handler : crate :: app :: arenabondunitselectmenu :: ArenaBondUnitSelectMenu_HelpEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondUnitSelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondUnitSelectMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            default_menu_item_index,
            help_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondunitselectmenu/ArenaBondUnitSelectMenu_HelpEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondUnitSelectMenu.HelpEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaBondUnitSelectMenu_HelpEventHandler {}

#[cfg(feature = "app-arenabondunitselectmenu")]
#[::unity2::methods]
impl ArenaBondUnitSelectMenu_HelpEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, parent: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-arenabondunitselectmenu")]
impl ArenaBondUnitSelectMenu_HelpEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondUnitSelectMenu_HelpEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondUnitSelectMenu_HelpEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/arenabondunitselectmenu/ArenaBondUnitSelectMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ArenaBondUnitSelectMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ArenaBondUnitSelectMenu_DecideEventHandler {}

#[cfg(feature = "app-arenabondunitselectmenu")]
#[::unity2::methods]
impl ArenaBondUnitSelectMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, unit: crate::app::unit::Unit) -> ();
}

#[cfg(feature = "app-arenabondunitselectmenu")]
impl ArenaBondUnitSelectMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaBondUnitSelectMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaBondUnitSelectMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
