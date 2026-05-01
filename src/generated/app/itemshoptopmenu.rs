
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemshoptopmenu/ItemShopTopMenu_Result2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ItemShopTopMenu_Result2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ItemShopTopMenu_Result2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ItemShopTopMenu.Result2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ItemShopTopMenu_Result2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ItemShopTopMenu_Result2 {
    pub fn buy() -> Self {
        Self { value: 0 }
    }

    pub fn sell() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemshoptopmenu/ItemShopTopMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ItemShopTopMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ItemShopTopMenu_DecideEventHandler {}

#[cfg(feature = "app-itemshoptopmenu")]
#[::unity2::methods]
impl ItemShopTopMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, result: crate::app::itemshoptopmenu::ItemShopTopMenu_Result2) -> ();
}

#[cfg(feature = "app-itemshoptopmenu")]
impl ItemShopTopMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemShopTopMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IItemShopTopMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemshoptopmenu/ItemShopTopMenu_SellMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ItemShopTopMenu.SellMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ItemShopTopMenu_SellMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::itemshoptopmenu::ItemShopTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-itemshoptopmenu")]
#[::unity2::methods]
impl ItemShopTopMenu_SellMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        decide_event_handler: crate::app::itemshoptopmenu::ItemShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-itemshoptopmenu")]
impl ItemShopTopMenu_SellMenuItem {
    pub fn new(
        decide_event_handler: crate::app::itemshoptopmenu::ItemShopTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemShopTopMenu_SellMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IItemShopTopMenu_SellMenuItemMethods>::ctor(this, decide_event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemshoptopmenu/ItemShopTopMenu_BuyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "ItemShopTopMenu.BuyMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct ItemShopTopMenu_BuyMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::itemshoptopmenu::ItemShopTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-itemshoptopmenu")]
#[::unity2::methods]
impl ItemShopTopMenu_BuyMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        decide_event_handler: crate::app::itemshoptopmenu::ItemShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-itemshoptopmenu")]
impl ItemShopTopMenu_BuyMenuItem {
    pub fn new(
        decide_event_handler: crate::app::itemshoptopmenu::ItemShopTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemShopTopMenu_BuyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IItemShopTopMenu_BuyMenuItemMethods>::ctor(this, decide_event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemshoptopmenu/ItemShopTopMenu.md")))]
#[::unity2::class(namespace = "App", name = "ItemShopTopMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct ItemShopTopMenu {}

#[cfg(feature = "app-itemshoptopmenu")]
#[::unity2::methods]
impl ItemShopTopMenu {
    #[method(name = "get_m_DecideEventHandler", args = 0)]
    pub fn get_m_decide_event_handler(
        self,
    ) -> crate::app::itemshoptopmenu::ItemShopTopMenu_DecideEventHandler;

    #[method(name = "set_m_DecideEventHandler", args = 1)]
    pub fn set_m_decide_event_handler(
        self,
        value: crate::app::itemshoptopmenu::ItemShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        initial_selected: crate::app::itemshoptopmenu::ItemShopTopMenu_Result2,
        decide_event_handler: crate::app::itemshoptopmenu::ItemShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        initial_selected: crate::app::itemshoptopmenu::ItemShopTopMenu_Result2,
        decide_event_handler: crate::app::itemshoptopmenu::ItemShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-itemshoptopmenu")]
impl ItemShopTopMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicmenucontent::BasicMenuContent,
        initial_selected: crate::app::itemshoptopmenu::ItemShopTopMenu_Result2,
        decide_event_handler: crate::app::itemshoptopmenu::ItemShopTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemShopTopMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IItemShopTopMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            initial_selected,
            decide_event_handler,
        );
        this
    }
}
