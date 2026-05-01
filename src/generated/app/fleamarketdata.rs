
use crate::app::shopdatabase_1::IShopDataBase_1;
use crate::app::shopdatabase_1::ShopDataBase_1;
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fleamarketdata/FleaMarketData.md")))]
#[::unity2::class(namespace = "App", name = "FleaMarketData")]
# [parent (crate :: app :: shopdatabase_1 :: ShopDataBase_1 < crate :: app :: fleamarketdata :: FleaMarketData >)]
pub struct FleaMarketData {}

#[cfg(feature = "app-fleamarketdata")]
#[::unity2::methods]
impl FleaMarketData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "GetStockAddedKeyImpl", args = 1)]
    pub fn get_stock_added_key_impl(condition: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetStockKeyImpl", args = 1)]
    pub fn get_stock_key_impl(iid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "Regist", args = 0)]
    pub fn regist() -> ();

    #[method(name = "SetupContentList", args = 0)]
    pub fn setup_content_list() -> ::unity2::Array<crate::app::shopcontent::ShopContent>;

    #[method(name = "IsExistAdditionalStock", args = 0)]
    pub fn is_exist_additional_stock() -> bool;

    #[method(name = "Purchase", args = 2)]
    pub fn purchase(iid: ::unity2::Il2CppString, value: i32) -> ();

    #[method(name = "GetStockNum", args = 1)]
    pub fn get_stock_num(iid: ::unity2::Il2CppString) -> i32;

    #[method(name = "IsInfinity", args = 1)]
    pub fn is_infinity(iid: ::unity2::Il2CppString) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-fleamarketdata")]
impl FleaMarketData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FleaMarketData),
                ::core::stringify!(new),
            )
        });
        <Self as IFleaMarketDataMethods>::ctor(this);
        this
    }
}
