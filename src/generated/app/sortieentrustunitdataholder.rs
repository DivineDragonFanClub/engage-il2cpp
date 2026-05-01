
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentrustunitdataholder/SortieEntrustUnitDataHolder.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrustUnitDataHolder")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrustUnitDataHolder {
    #[rename(name = "m_Data")]
    pub m_data: crate::system::collections::generic::list_1::List_1<
        crate::app::sortieentrustunitdata::SortieEntrustUnitData,
    >,
}

#[cfg(feature = "app-sortieentrustunitdataholder")]
#[::unity2::methods]
impl SortieEntrustUnitDataHolder {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "RegisterUnit", args = 1)]
    pub fn register_unit(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(self, unit: crate::app::unit::Unit, unit_item: crate::app::unititem::UnitItem)
        -> ();

    #[method(name = "get_Data", args = 0)]
    pub fn get_data(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::sortieentrustunitdata::SortieEntrustUnitData,
    >;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;
}

#[cfg(feature = "app-sortieentrustunitdataholder")]
impl SortieEntrustUnitDataHolder {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrustUnitDataHolder),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrustUnitDataHolderMethods>::ctor(this);
        this
    }
}
