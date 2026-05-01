
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
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoptopmenu/RefineShopTopMenu.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopTopMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct RefineShopTopMenu {}

#[cfg(feature = "app-refineshoptopmenu")]
#[::unity2::methods]
impl RefineShopTopMenu {
    #[method(name = "get_m_DecideEventHandler", args = 0)]
    pub fn get_m_decide_event_handler(
        self,
    ) -> crate::app::refineshoptopmenu::RefineShopTopMenu_DecideEventHandler;

    #[method(name = "set_m_DecideEventHandler", args = 1)]
    pub fn set_m_decide_event_handler(
        self,
        value: crate::app::refineshoptopmenu::RefineShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        initial_selected: crate::app::refineshoptopmenu::RefineShopTopMenu_Result2,
        decide_event_handler: crate::app::refineshoptopmenu::RefineShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::shoptopmenucontent::ShopTopMenuContent,
        initial_selected: crate::app::refineshoptopmenu::RefineShopTopMenu_Result2,
        decide_event_handler: crate::app::refineshoptopmenu::RefineShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refineshoptopmenu")]
impl RefineShopTopMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::shoptopmenucontent::ShopTopMenuContent,
        initial_selected: crate::app::refineshoptopmenu::RefineShopTopMenu_Result2,
        decide_event_handler: crate::app::refineshoptopmenu::RefineShopTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopTopMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopTopMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            initial_selected,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoptopmenu/RefineShopTopMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopTopMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct RefineShopTopMenu_DecideEventHandler {}

#[cfg(feature = "app-refineshoptopmenu")]
#[::unity2::methods]
impl RefineShopTopMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, result: crate::app::refineshoptopmenu::RefineShopTopMenu_Result2) -> ();
}

#[cfg(feature = "app-refineshoptopmenu")]
impl RefineShopTopMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopTopMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopTopMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoptopmenu/RefineShopTopMenu_Result2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RefineShopTopMenu_Result2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RefineShopTopMenu_Result2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RefineShopTopMenu.Result2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RefineShopTopMenu_Result2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RefineShopTopMenu_Result2 {
    pub fn refine() -> Self {
        Self { value: 0 }
    }

    pub fn engrave() -> Self {
        Self { value: 1 }
    }

    pub fn exchange() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoptopmenu/RefineShopTopMenu_RefineShopTopExchangeMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopTopMenu.RefineShopTopExchangeMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RefineShopTopMenu_RefineShopTopExchangeMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::refineshoptopmenu::RefineShopTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-refineshoptopmenu")]
#[::unity2::methods]
impl RefineShopTopMenu_RefineShopTopExchangeMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        decide_event_handler: crate::app::refineshoptopmenu::RefineShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refineshoptopmenu")]
impl RefineShopTopMenu_RefineShopTopExchangeMenuItem {
    pub fn new(
        decide_event_handler: crate::app::refineshoptopmenu::RefineShopTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopTopMenu_RefineShopTopExchangeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopTopMenu_RefineShopTopExchangeMenuItemMethods>::ctor(
            this,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoptopmenu/RefineShopTopMenu_RefineShopTopEngraveMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopTopMenu.RefineShopTopEngraveMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RefineShopTopMenu_RefineShopTopEngraveMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::refineshoptopmenu::RefineShopTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-refineshoptopmenu")]
#[::unity2::methods]
impl RefineShopTopMenu_RefineShopTopEngraveMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        decide_event_handler: crate::app::refineshoptopmenu::RefineShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refineshoptopmenu")]
impl RefineShopTopMenu_RefineShopTopEngraveMenuItem {
    pub fn new(
        decide_event_handler: crate::app::refineshoptopmenu::RefineShopTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopTopMenu_RefineShopTopEngraveMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopTopMenu_RefineShopTopEngraveMenuItemMethods>::ctor(
            this,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshoptopmenu/RefineShopTopMenu_RefineShopTopRefineMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "RefineShopTopMenu.RefineShopTopRefineMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct RefineShopTopMenu_RefineShopTopRefineMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::refineshoptopmenu::RefineShopTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-refineshoptopmenu")]
#[::unity2::methods]
impl RefineShopTopMenu_RefineShopTopRefineMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        decide_event_handler: crate::app::refineshoptopmenu::RefineShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-refineshoptopmenu")]
impl RefineShopTopMenu_RefineShopTopRefineMenuItem {
    pub fn new(
        decide_event_handler: crate::app::refineshoptopmenu::RefineShopTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopTopMenu_RefineShopTopRefineMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopTopMenu_RefineShopTopRefineMenuItemMethods>::ctor(
            this,
            decide_event_handler,
        );
        this
    }
}
