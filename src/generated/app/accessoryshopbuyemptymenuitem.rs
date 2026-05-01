
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopbuyemptymenuitem/AccessoryShopBuyEmptyMenuItem.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryShopBuyEmptyMenuItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct AccessoryShopBuyEmptyMenuItem {
    #[rename(name = "m_SelectEventHandler")]
    pub m_select_event_handler:
        crate::app::accessoryshopbuymenu::AccessoryShopBuyMenu_SelectEventHandler,
}

#[cfg(feature = "app-accessoryshopbuyemptymenuitem")]
#[::unity2::methods]
impl AccessoryShopBuyEmptyMenuItem {
    #[method(name = "get_m_AccessoryKind", args = 0)]
    pub fn get_m_accessory_kind(self) -> crate::app::accessorydata::AccessoryData_Kinds;

    #[method(name = "set_m_AccessoryKind", args = 1)]
    pub fn set_m_accessory_kind(self, value: crate::app::accessorydata::AccessoryData_Kinds) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        accessory_kind: crate::app::accessorydata::AccessoryData_Kinds,
        select_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_SelectEventHandler,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-accessoryshopbuyemptymenuitem")]
impl AccessoryShopBuyEmptyMenuItem {
    pub fn new(
        accessory_kind: crate::app::accessorydata::AccessoryData_Kinds,
        select_event_handler : crate :: app :: accessoryshopbuymenu :: AccessoryShopBuyMenu_SelectEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopBuyEmptyMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopBuyEmptyMenuItemMethods>::ctor(
            this,
            accessory_kind,
            select_event_handler,
        );
        this
    }
}
