
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshfacilityselectmenu/RefreshFacilitySelectMenu_CloseEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefreshFacilitySelectMenu.CloseEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefreshFacilitySelectMenu_CloseEventHandler {}

#[cfg(feature = "app-refreshfacilityselectmenu")]
#[::unity2::methods]
impl RefreshFacilitySelectMenu_CloseEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refreshfacilityselectmenu")]
impl RefreshFacilitySelectMenu_CloseEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshFacilitySelectMenu_CloseEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshFacilitySelectMenu_CloseEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshfacilityselectmenu/RefreshFacilitySelectMenu_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefreshFacilitySelectMenu.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefreshFacilitySelectMenu_DecideEventHandler {}

#[cfg(feature = "app-refreshfacilityselectmenu")]
#[::unity2::methods]
impl RefreshFacilitySelectMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, facility_data: crate::app::hubfacilitydata::HubFacilityData) -> ();
}

#[cfg(feature = "app-refreshfacilityselectmenu")]
impl RefreshFacilitySelectMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshFacilitySelectMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshFacilitySelectMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshfacilityselectmenu/RefreshFacilitySelectMenu.md")))]
#[::unity2::class(namespace = "App", name = "RefreshFacilitySelectMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RefreshFacilitySelectMenu {
    #[static_field]
    #[rename(name = "m_FacilityAid")]
    pub m_facility_aid: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler:
        crate::app::refreshfacilityselectmenu::RefreshFacilitySelectMenu_DecideEventHandler,
    #[rename(name = "m_CloseEventHandler")]
    pub m_close_event_handler:
        crate::app::refreshfacilityselectmenu::RefreshFacilitySelectMenu_CloseEventHandler,
    #[rename(name = "m_DisposeEventHandler")]
    pub m_dispose_event_handler:
        crate::app::refreshfacilityselectmenu::RefreshFacilitySelectMenu_DisposeEventHandler,
}

#[cfg(feature = "app-refreshfacilityselectmenu")]
#[::unity2::methods]
impl RefreshFacilitySelectMenu {
    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        menu_content : crate :: app :: refreshfacilityselectmenucontent :: RefreshFacilitySelectMenuContent,
        select_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_DecideEventHandler,
        close_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_CloseEventHandler,
        dispose_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_DisposeEventHandler,
    ) -> crate::app::refreshfacilityselectmenu::RefreshFacilitySelectMenu;

    #[method(name = "CreateMenuItem", args = 2)]
    pub fn create_menu_item(
        select_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_SelectEventHandler,
        decide_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_DecideEventHandler,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::basicmenuitem::BasicMenuItem>;

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content : crate :: app :: refreshfacilityselectmenucontent :: RefreshFacilitySelectMenuContent,
        decide_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_DecideEventHandler,
        close_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_CloseEventHandler,
        dispose_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_DisposeEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-refreshfacilityselectmenu")]
impl RefreshFacilitySelectMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content : crate :: app :: refreshfacilityselectmenucontent :: RefreshFacilitySelectMenuContent,
        decide_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_DecideEventHandler,
        close_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_CloseEventHandler,
        dispose_event_handler : crate :: app :: refreshfacilityselectmenu :: RefreshFacilitySelectMenu_DisposeEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshFacilitySelectMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshFacilitySelectMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            decide_event_handler,
            close_event_handler,
            dispose_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshfacilityselectmenu/RefreshFacilitySelectMenu_DisposeEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefreshFacilitySelectMenu.DisposeEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefreshFacilitySelectMenu_DisposeEventHandler {}

#[cfg(feature = "app-refreshfacilityselectmenu")]
#[::unity2::methods]
impl RefreshFacilitySelectMenu_DisposeEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refreshfacilityselectmenu")]
impl RefreshFacilitySelectMenu_DisposeEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshFacilitySelectMenu_DisposeEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshFacilitySelectMenu_DisposeEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshfacilityselectmenu/RefreshFacilitySelectMenu_SelectEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefreshFacilitySelectMenu.SelectEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefreshFacilitySelectMenu_SelectEventHandler {}

#[cfg(feature = "app-refreshfacilityselectmenu")]
#[::unity2::methods]
impl RefreshFacilitySelectMenu_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, facility_data: crate::app::hubfacilitydata::HubFacilityData) -> ();
}

#[cfg(feature = "app-refreshfacilityselectmenu")]
impl RefreshFacilitySelectMenu_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshFacilitySelectMenu_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshFacilitySelectMenu_SelectEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
