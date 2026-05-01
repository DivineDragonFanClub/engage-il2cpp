
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopcontent/ShopContent.md")))]
#[::unity2::class(namespace = "App", name = "ShopContent")]
#[parent(crate::system::object::Object)]
pub struct ShopContent {
    #[rename(name = "m_Iid")]
    pub m_iid: ::unity2::Il2CppString,
    #[rename(name = "m_StockNum")]
    pub m_stock_num: i32,
    #[rename(name = "m_NewArrival")]
    pub m_new_arrival: bool,
}

#[cfg(feature = "app-shopcontent")]
#[::unity2::methods]
impl ShopContent {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-shopcontent")]
impl ShopContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopContent),
                ::core::stringify!(new),
            )
        });
        <Self as IShopContentMethods>::ctor(this);
        this
    }
}
