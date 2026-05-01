
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentruststockdata/SortieEntrustStockData.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrustStockData")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrustStockData {
    #[rename(name = "m_UnitItem")]
    pub m_unit_item: crate::app::unititem::UnitItem,
    #[rename(name = "m_ItemData")]
    pub m_item_data: crate::app::itemdata::ItemData,
    #[rename(name = "m_StockCount")]
    pub m_stock_count: i32,
    #[rename(name = "m_AssignedCount")]
    pub m_assigned_count: i32,
}

#[cfg(feature = "app-sortieentruststockdata")]
#[::unity2::methods]
impl SortieEntrustStockData {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit_item: crate::app::unititem::UnitItem, count: i32) -> ();

    #[method(name = "Assigned", args = 0)]
    pub fn assigned(self) -> ();

    #[method(name = "AddStock", args = 1)]
    pub fn add_stock(self, count: i32) -> ();

    #[method(name = "get_UnitItem", args = 0)]
    pub fn get_unit_item(self) -> crate::app::unititem::UnitItem;

    #[method(name = "get_ItemData", args = 0)]
    pub fn get_item_data(self) -> crate::app::itemdata::ItemData;

    #[method(name = "get_StockCount", args = 0)]
    pub fn get_stock_count(self) -> i32;

    #[method(name = "get_AssignedCount", args = 0)]
    pub fn get_assigned_count(self) -> i32;
}

#[cfg(feature = "app-sortieentruststockdata")]
impl SortieEntrustStockData {
    pub fn new(unit_item: crate::app::unititem::UnitItem, count: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrustStockData),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrustStockDataMethods>::ctor(this, unit_item, count);
        this
    }
}
