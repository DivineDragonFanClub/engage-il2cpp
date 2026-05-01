
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentruststockdataholder/SortieEntrustStockDataHolder.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrustStockDataHolder")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrustStockDataHolder {
    #[rename(name = "m_Kinds")]
    pub m_kinds:
        ::unity2::Array<crate::app::sortieentruststockkinddata::SortieEntrustStockKindData>,
}

#[cfg(feature = "app-sortieentruststockdataholder")]
#[::unity2::methods]
impl SortieEntrustStockDataHolder {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, unit_item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "get_Kinds", args = 0)]
    pub fn get_kinds(
        self,
    ) -> ::unity2::Array<crate::app::sortieentruststockkinddata::SortieEntrustStockKindData>;
}

#[cfg(feature = "app-sortieentruststockdataholder")]
impl SortieEntrustStockDataHolder {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrustStockDataHolder),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrustStockDataHolderMethods>::ctor(this);
        this
    }
}
