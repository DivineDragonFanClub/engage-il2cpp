
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/accessoryshopdata/AccessoryShopData.md")))]
#[::unity2::class(namespace = "App", name = "AccessoryShopData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: accessoryshopdata :: AccessoryShopData >)]
pub struct AccessoryShopData {}

#[cfg(feature = "app-accessoryshopdata")]
#[::unity2::methods]
impl AccessoryShopData {
    #[method(name = "get_Aid", args = 0)]
    pub fn get_aid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Aid", args = 1)]
    pub fn set_aid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "GetStockAddedKey", args = 1)]
    pub fn get_stock_added_key(condition: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "Regist", args = 0)]
    pub fn regist() -> ();

    #[method(name = "SetupContentList", args = 0)]
    pub fn setup_content_list(
    ) -> ::unity2::Array<crate::app::accessoryshopcontent::AccessoryShopContent>;

    #[method(name = "IsExistAdditionalStock", args = 0)]
    pub fn is_exist_additional_stock() -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-accessoryshopdata")]
impl AccessoryShopData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AccessoryShopData),
                ::core::stringify!(new),
            )
        });
        <Self as IAccessoryShopDataMethods>::ctor(this);
        this
    }
}
