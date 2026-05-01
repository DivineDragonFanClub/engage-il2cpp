
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemrefinedata/ItemRefineData.md")))]
#[::unity2::class(namespace = "App", name = "ItemRefineData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: itemrefinedata :: ItemRefineData >)]
pub struct ItemRefineData {}

#[cfg(feature = "app-itemrefinedata")]
#[::unity2::methods]
impl ItemRefineData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "TryGetFromIid", args = 1)]
    pub fn try_get_from_iid(
        iid: ::unity2::Il2CppString,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::itemrefinedata::ItemRefineData,
    >;

    #[method(name = "TryGetFromItem", args = 1)]
    pub fn try_get_from_item(
        item: crate::app::itemdata::ItemData,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::itemrefinedata::ItemRefineData,
    >;

    #[method(name = "Iid2Rid", args = 1)]
    pub fn iid2_rid(iid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "get_Iron", args = 0)]
    pub fn get_iron(self) -> u16;

    #[method(name = "set_Iron", args = 1)]
    pub fn set_iron(self, value: u16) -> ();

    #[method(name = "get_Steel", args = 0)]
    pub fn get_steel(self) -> u16;

    #[method(name = "set_Steel", args = 1)]
    pub fn set_steel(self, value: u16) -> ();

    #[method(name = "get_Silver", args = 0)]
    pub fn get_silver(self) -> u16;

    #[method(name = "set_Silver", args = 1)]
    pub fn set_silver(self, value: u16) -> ();

    #[method(name = "get_Price", args = 0)]
    pub fn get_price(self) -> u16;

    #[method(name = "set_Price", args = 1)]
    pub fn set_price(self, value: u16) -> ();

    #[method(name = "get_Power", args = 0)]
    pub fn get_power(self) -> i8;

    #[method(name = "set_Power", args = 1)]
    pub fn set_power(self, value: i8) -> ();

    #[method(name = "get_Weight", args = 0)]
    pub fn get_weight(self) -> i8;

    #[method(name = "set_Weight", args = 1)]
    pub fn set_weight(self, value: i8) -> ();

    #[method(name = "get_Hit", args = 0)]
    pub fn get_hit(self) -> i8;

    #[method(name = "set_Hit", args = 1)]
    pub fn set_hit(self, value: i8) -> ();

    #[method(name = "get_Critical", args = 0)]
    pub fn get_critical(self) -> i8;

    #[method(name = "set_Critical", args = 1)]
    pub fn set_critical(self, value: i8) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-itemrefinedata")]
impl ItemRefineData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemRefineData),
                ::core::stringify!(new),
            )
        });
        <Self as IItemRefineDataMethods>::ctor(this);
        this
    }
}
