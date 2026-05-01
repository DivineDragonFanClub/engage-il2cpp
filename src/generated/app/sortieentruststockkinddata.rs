
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentruststockkinddata/SortieEntrustStockKindData.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrustStockKindData")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrustStockKindData {
    #[rename(name = "m_Data")]
    pub m_data: crate::system::collections::generic::list_1::List_1<
        crate::app::sortieentruststockdata::SortieEntrustStockData,
    >,
}

#[cfg(feature = "app-sortieentruststockkinddata")]
#[::unity2::methods]
impl SortieEntrustStockKindData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, unit_item: crate::app::unititem::UnitItem) -> ();

    #[method(name = "get_Data", args = 0)]
    pub fn get_data(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::sortieentruststockdata::SortieEntrustStockData,
    >;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;
}

#[cfg(feature = "app-sortieentruststockkinddata")]
impl SortieEntrustStockKindData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrustStockKindData),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrustStockKindDataMethods>::ctor(this);
        this
    }
}
