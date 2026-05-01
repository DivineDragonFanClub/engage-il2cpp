
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/weaponshoptopmenu/WeaponShopTopMenu.md")))]
#[::unity2::class(namespace = "App", name = "WeaponShopTopMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct WeaponShopTopMenu {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::weaponshoptopmenu::WeaponShopTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-weaponshoptopmenu")]
#[::unity2::methods]
impl WeaponShopTopMenu {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        initial_selected: crate::app::weaponshoptopmenu::WeaponShopTopMenu_Result2,
        decide_event_handler: crate::app::weaponshoptopmenu::WeaponShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::shoptopmenucontent::ShopTopMenuContent,
        initial_selected: crate::app::weaponshoptopmenu::WeaponShopTopMenu_Result2,
        decide_event_handler: crate::app::weaponshoptopmenu::WeaponShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-weaponshoptopmenu")]
impl WeaponShopTopMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::shoptopmenucontent::ShopTopMenuContent,
        initial_selected: crate::app::weaponshoptopmenu::WeaponShopTopMenu_Result2,
        decide_event_handler: crate::app::weaponshoptopmenu::WeaponShopTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WeaponShopTopMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IWeaponShopTopMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
            initial_selected,
            decide_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/weaponshoptopmenu/WeaponShopTopMenu_DecideEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "WeaponShopTopMenu.DecideEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct WeaponShopTopMenu_DecideEventHandler {}

#[cfg(feature = "app-weaponshoptopmenu")]
#[::unity2::methods]
impl WeaponShopTopMenu_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, result: crate::app::weaponshoptopmenu::WeaponShopTopMenu_Result2) -> ();
}

#[cfg(feature = "app-weaponshoptopmenu")]
impl WeaponShopTopMenu_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WeaponShopTopMenu_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IWeaponShopTopMenu_DecideEventHandlerMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/weaponshoptopmenu/WeaponShopTopMenu_BuyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "WeaponShopTopMenu.BuyMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct WeaponShopTopMenu_BuyMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::weaponshoptopmenu::WeaponShopTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-weaponshoptopmenu")]
#[::unity2::methods]
impl WeaponShopTopMenu_BuyMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        decide_event_handler: crate::app::weaponshoptopmenu::WeaponShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-weaponshoptopmenu")]
impl WeaponShopTopMenu_BuyMenuItem {
    pub fn new(
        decide_event_handler: crate::app::weaponshoptopmenu::WeaponShopTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WeaponShopTopMenu_BuyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IWeaponShopTopMenu_BuyMenuItemMethods>::ctor(this, decide_event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/weaponshoptopmenu/WeaponShopTopMenu_Result2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct WeaponShopTopMenu_Result2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for WeaponShopTopMenu_Result2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "WeaponShopTopMenu.Result2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for WeaponShopTopMenu_Result2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl WeaponShopTopMenu_Result2 {
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/weaponshoptopmenu/WeaponShopTopMenu_SellMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "WeaponShopTopMenu.SellMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct WeaponShopTopMenu_SellMenuItem {
    #[rename(name = "m_DecideEventHandler")]
    pub m_decide_event_handler: crate::app::weaponshoptopmenu::WeaponShopTopMenu_DecideEventHandler,
}

#[cfg(feature = "app-weaponshoptopmenu")]
#[::unity2::methods]
impl WeaponShopTopMenu_SellMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        decide_event_handler: crate::app::weaponshoptopmenu::WeaponShopTopMenu_DecideEventHandler,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-weaponshoptopmenu")]
impl WeaponShopTopMenu_SellMenuItem {
    pub fn new(
        decide_event_handler: crate::app::weaponshoptopmenu::WeaponShopTopMenu_DecideEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WeaponShopTopMenu_SellMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IWeaponShopTopMenu_SellMenuItemMethods>::ctor(this, decide_event_handler);
        this
    }
}
