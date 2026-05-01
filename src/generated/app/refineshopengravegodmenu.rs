
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodmenu/RefineShopEngraveGodMenu_RequestCloseEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopEngraveGodMenu.RequestCloseEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopEngraveGodMenu_RequestCloseEventHandler {}

#[cfg(feature = "app-refineshopengravegodmenu")]
#[::unity2::methods]
impl RefineShopEngraveGodMenu_RequestCloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refineshopengravegodmenu")]
impl RefineShopEngraveGodMenu_RequestCloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodMenu_RequestCloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodMenu_RequestCloseEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodmenu/RefineShopEngraveGodMenu_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopEngraveGodMenu.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopEngraveGodMenu_DecideEventHandler {}

#[cfg(feature = "app-refineshopengravegodmenu")]
#[::unity2::methods]
impl RefineShopEngraveGodMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, god_data: crate::app::goddata::GodData) -> ();
}

#[cfg(feature = "app-refineshopengravegodmenu")]
impl RefineShopEngraveGodMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodmenu/RefineShopEngraveGodMenu_SelectEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopEngraveGodMenu.SelectEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopEngraveGodMenu_SelectEventHandler {}

#[cfg(feature = "app-refineshopengravegodmenu")]
#[::unity2::methods]
impl RefineShopEngraveGodMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        god_data: crate::app::goddata::GodData,
        engraved_unit_item: crate::app::unititem::UnitItem,
    ) -> ();
}

#[cfg(feature = "app-refineshopengravegodmenu")]
impl RefineShopEngraveGodMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopengravegodmenu/RefineShopEngraveGodMenu.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopEngraveGodMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RefineShopEngraveGodMenu {
    #[rename(name = "m_RequestCloseEventHandler")]
    pub m_request_close_event_handler:
        crate::app::refineshopengravegodmenu::RefineShopEngraveGodMenu_RequestCloseEventHandler,
}

#[cfg(feature = "app-refineshopengravegodmenu")]
#[::unity2::methods]
impl RefineShopEngraveGodMenu {
    #[method(name = "get_m_Unit", args = 0)]
    pub fn get_m_unit(self) -> crate::app::unit::Unit;

    #[method(name = "set_m_Unit", args = 1)]
    pub fn set_m_unit(self, value: crate::app::unit::Unit) -> ();

    #[method(name = "get_m_BaseUnitItem", args = 0)]
    pub fn get_m_base_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "set_m_BaseUnitItem", args = 1)]
    pub fn set_m_base_unit_item(self, value: crate::app::unititem::UnitItem) -> ();

    #[method(name = "CreateBind", args = 7)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_object: crate::unity_engine::gameobject::GameObject,
        unit: crate::app::unit::Unit,
        item_index: i32,
        select_event_handler : crate :: app :: refineshopengravegodmenu :: RefineShopEngraveGodMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refineshopengravegodmenu :: RefineShopEngraveGodMenu_DecideEventHandler,
        request_close_event_handler : crate :: app :: refineshopengravegodmenu :: RefineShopEngraveGodMenu_RequestCloseEventHandler,
    ) -> crate::app::refineshopengravegodmenu::RefineShopEngraveGodMenu;

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        unit: crate::app::unit::Unit,
        base_unit_item: crate::app::unititem::UnitItem,
        request_close_event_handler : crate :: app :: refineshopengravegodmenu :: RefineShopEngraveGodMenu_RequestCloseEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refineshopengravegodmenu")]
impl RefineShopEngraveGodMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        unit: crate::app::unit::Unit,
        base_unit_item: crate::app::unititem::UnitItem,
        request_close_event_handler : crate :: app :: refineshopengravegodmenu :: RefineShopEngraveGodMenu_RequestCloseEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopEngraveGodMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopEngraveGodMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            unit,
            base_unit_item,
            request_close_event_handler,
        );
        this
    }
}
