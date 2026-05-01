
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopcontent/AccessoryShopContent.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryShopContent")]
#[parent(crate::system::object::Object)]
pub struct AccessoryShopContent {
    #[rename(name = "m_Aid")]
    pub m_aid: ::unity2::Il2CppString,
    #[rename(name = "m_NewArrival")]
    pub m_new_arrival: bool,
}

#[cfg(feature = "app-accessoryshopcontent")]
#[::unity2::methods]
impl AccessoryShopContent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-accessoryshopcontent")]
impl AccessoryShopContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopContent),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopContentMethods>::ctor(this);
        this
    }
}
