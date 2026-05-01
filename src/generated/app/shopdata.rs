
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopdata/ShopData.md")))]
#[::unity2::class(namespace = "App", name = "ShopData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: shopdata :: ShopData >)]
pub struct ShopData {}

#[cfg(feature = "app-shopdata")]
#[::unity2::methods]
impl ShopData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Iid", args = 0)]
    pub fn get_iid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Iid", args = 1)]
    pub fn set_iid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Stock", args = 0)]
    pub fn get_stock(self) -> i32;

    #[method(name = "set_Stock", args = 1)]
    pub fn set_stock(self, value: i32) -> ();

    #[method(name = "get_Condition", args = 0)]
    pub fn get_condition(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Condition", args = 1)]
    pub fn set_condition(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetWithoutPrefix", args = 0)]
    pub fn get_without_prefix(self) -> ::unity2::Il2CppString;

    #[method(name = "get_StockKey", args = 0)]
    pub fn get_stock_key(self) -> ::unity2::Il2CppString;

    #[method(name = "set_StockKey", args = 1)]
    pub fn set_stock_key(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PurchasedKey", args = 0)]
    pub fn get_purchased_key(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PurchasedKey", args = 1)]
    pub fn set_purchased_key(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetPurchased", args = 0)]
    pub fn get_purchased(self) -> i32;

    #[method(name = "SetPurchased", args = 1)]
    pub fn set_purchased(self, value: i32) -> ();

    #[method(name = "AddPurchased", args = 1)]
    pub fn add_purchased(self, value: i32) -> ();

    #[method(name = "IsCondition", args = 0)]
    pub fn is_condition(self) -> bool;

    #[method(name = "RegistImpl", args = 1)]
    pub fn regist_impl(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "Regist", args = 0)]
    pub fn regist() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-shopdata")]
impl ShopData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopData),
                ::core::stringify!(new),
            )
        });
        <Self as IShopDataMethods>::ctor(this);
        this
    }
}
