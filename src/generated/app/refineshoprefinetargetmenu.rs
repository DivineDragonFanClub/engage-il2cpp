
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoprefinetargetmenu/RefineShopRefineTargetMenu_SelectEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopRefineTargetMenu.SelectEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopRefineTargetMenu_SelectEventHandler {}

#[cfg(feature = "app-refineshoprefinetargetmenu")]
#[::unity2::methods]
impl RefineShopRefineTargetMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, unit_item: crate::app::unititem::UnitItem, revealed: bool) -> ();
}

#[cfg(feature = "app-refineshoprefinetargetmenu")]
impl RefineShopRefineTargetMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopRefineTargetMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopRefineTargetMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoprefinetargetmenu/RefineShopRefineTargetMenu_DecideToEvolveEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopRefineTargetMenu.DecideToEvolveEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopRefineTargetMenu_DecideToEvolveEventHandler {}

#[cfg(feature = "app-refineshoprefinetargetmenu")]
#[::unity2::methods]
impl RefineShopRefineTargetMenu_DecideToEvolveEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, evolved_unit_item: crate::app::unititem::UnitItem, evolve_index: i32)
        -> ();
}

#[cfg(feature = "app-refineshoprefinetargetmenu")]
impl RefineShopRefineTargetMenu_DecideToEvolveEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopRefineTargetMenu_DecideToEvolveEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopRefineTargetMenu_DecideToEvolveEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoprefinetargetmenu/RefineShopRefineTargetMenu_RequestCloseEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopRefineTargetMenu.RequestCloseEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopRefineTargetMenu_RequestCloseEventHandler {}

#[cfg(feature = "app-refineshoprefinetargetmenu")]
#[::unity2::methods]
impl RefineShopRefineTargetMenu_RequestCloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refineshoprefinetargetmenu")]
impl RefineShopRefineTargetMenu_RequestCloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopRefineTargetMenu_RequestCloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopRefineTargetMenu_RequestCloseEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoprefinetargetmenu/RefineShopRefineTargetMenu_DecideToRefineEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopRefineTargetMenu.DecideToRefineEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopRefineTargetMenu_DecideToRefineEventHandler {}

#[cfg(feature = "app-refineshoprefinetargetmenu")]
#[::unity2::methods]
impl RefineShopRefineTargetMenu_DecideToRefineEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 6)]
    pub fn invoke(
        self,
        refine_level: i32,
        refined_unit_item: crate::app::unititem::UnitItem,
        iron_num: i32,
        steel_num: i32,
        silver_num: i32,
        price: i32,
    ) -> ();
}

#[cfg(feature = "app-refineshoprefinetargetmenu")]
impl RefineShopRefineTargetMenu_DecideToRefineEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopRefineTargetMenu_DecideToRefineEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopRefineTargetMenu_DecideToRefineEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoprefinetargetmenu/RefineShopRefineTargetMenu.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopRefineTargetMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RefineShopRefineTargetMenu {
    #[rename(name = "m_RequestCloseEventHandler")]
    pub m_request_close_event_handler:
        crate::app::refineshoprefinetargetmenu::RefineShopRefineTargetMenu_RequestCloseEventHandler,
}

#[cfg(feature = "app-refineshoprefinetargetmenu")]
#[::unity2::methods]
impl RefineShopRefineTargetMenu {
    #[method(name = "get_m_Unit", args = 0)]
    pub fn get_m_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_m_Unit", args = 1)]
    pub fn set_m_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_m_UnitItem", args = 0)]
    pub fn get_m_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "set_m_UnitItem", args = 1)]
    pub fn set_m_unit_item(self, value: crate::app::unititem::UnitItem) -> ();

    #[method(name = "CreateBind", args = 8)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_object: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
        item_index: i32,
        select_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_SelectEventHandler,
        decide_to_refine_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_DecideToRefineEventHandler,
        decide_to_evolve_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_DecideToEvolveEventHandler,
        request_close_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_RequestCloseEventHandler,
    ) -> crate::app::refineshoprefinetargetmenu::RefineShopRefineTargetMenu;

    #[method(name = "CreateMenuItemList", args = 5)]
    pub fn create_menu_item_list(
        show_row_num: i32,
        unit_item: crate::app::unititem::UnitItem,
        select_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_SelectEventHandler,
        decide_to_refine_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_DecideToRefineEventHandler,
        decide_to_evolve_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_DecideToEvolveEventHandler,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        owner_unit: crate::app::unit::Unit,
        base_unit_item: crate::app::unititem::UnitItem,
        request_close_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_RequestCloseEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refineshoprefinetargetmenu")]
impl RefineShopRefineTargetMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        owner_unit: crate::app::unit::Unit,
        base_unit_item: crate::app::unititem::UnitItem,
        request_close_event_handler : crate :: app :: refineshoprefinetargetmenu :: RefineShopRefineTargetMenu_RequestCloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopRefineTargetMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopRefineTargetMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            owner_unit,
            base_unit_item,
            request_close_event_handler,
        );
        this
    }
}
