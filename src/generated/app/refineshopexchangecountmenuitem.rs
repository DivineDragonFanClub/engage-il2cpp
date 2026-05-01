
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopexchangecountmenuitem/RefineShopExchangeCountMenuItem_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopExchangeCountMenuItem.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopExchangeCountMenuItem_DecideEventHandler {}

#[cfg(feature = "app-refineshopexchangecountmenuitem")]
#[::unity2::methods]
impl RefineShopExchangeCountMenuItem_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(self, source_count: i32, target_count: i32, target_overflow: i32) -> ();
}

#[cfg(feature = "app-refineshopexchangecountmenuitem")]
impl RefineShopExchangeCountMenuItem_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopExchangeCountMenuItem_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopExchangeCountMenuItem_DecideEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopexchangecountmenuitem/RefineShopExchangeCountMenuItem_SelectEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopExchangeCountMenuItem.SelectEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopExchangeCountMenuItem_SelectEventHandler {}

#[cfg(feature = "app-refineshopexchangecountmenuitem")]
#[::unity2::methods]
impl RefineShopExchangeCountMenuItem_SelectEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-refineshopexchangecountmenuitem")]
impl RefineShopExchangeCountMenuItem_SelectEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopExchangeCountMenuItem_SelectEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopExchangeCountMenuItem_SelectEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopexchangecountmenuitem/RefineShopExchangeCountMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopExchangeCountMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RefineShopExchangeCountMenuItem {
# [rename (name = "m_AttributeEnable")] pub m_attribute_enable : bool ,
# [rename (name = "m_SelectEventHandler")] pub m_select_event_handler : crate :: app :: refineshopexchangecountmenuitem :: RefineShopExchangeCountMenuItem_SelectEventHandler ,
# [rename (name = "m_DecideEventHandler")] pub m_decide_event_handler : crate :: app :: refineshopexchangecountmenuitem :: RefineShopExchangeCountMenuItem_DecideEventHandler ,
}

#[cfg(feature = "app-refineshopexchangecountmenuitem")]
#[::unity2::methods]
impl RefineShopExchangeCountMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        select_event_handler : crate :: app :: refineshopexchangecountmenuitem :: RefineShopExchangeCountMenuItem_SelectEventHandler,
        decide_event_handler : crate :: app :: refineshopexchangecountmenuitem :: RefineShopExchangeCountMenuItem_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "SetColor", args = 0)]
    pub fn set_color(self) -> ();

    #[method(name = "SetAttributeEnable", args = 1)]
    pub fn set_attribute_enable(self, enabled: bool) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refineshopexchangecountmenuitem")]
impl RefineShopExchangeCountMenuItem {
    pub fn new(
        select_event_handler : crate :: app :: refineshopexchangecountmenuitem :: RefineShopExchangeCountMenuItem_SelectEventHandler,
        decide_event_handler : crate :: app :: refineshopexchangecountmenuitem :: RefineShopExchangeCountMenuItem_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopExchangeCountMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopExchangeCountMenuItemMethods>::ctor(
            this,
            select_event_handler,
            decide_event_handler,
        );
        this
    }
}
