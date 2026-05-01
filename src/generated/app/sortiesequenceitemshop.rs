
use crate::app::hubitemshopsequence::HubItemShopSequence;
use crate::app::hubitemshopsequence::IHubItemShopSequence;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortiesequenceitemshop/SortieSequenceItemShop.md")))]
#[::unity2::class(namespace = "App", name = "SortieSequenceItemShop")]
#[parent(crate::app::hubitemshopsequence::HubItemShopSequence)]
pub struct SortieSequenceItemShop {}

#[cfg(feature = "app-sortiesequenceitemshop")]
#[::unity2::methods]
impl SortieSequenceItemShop {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateItemShopTopMenu", args = 0)]
    pub fn create_item_shop_top_menu(self) -> ();

    #[method(name = "CreateItemShopBuyMenu", args = 0)]
    pub fn create_item_shop_buy_menu(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortiesequenceitemshop")]
impl SortieSequenceItemShop {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieSequenceItemShop),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieSequenceItemShopMethods>::ctor(this);
        this
    }
}
