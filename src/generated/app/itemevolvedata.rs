
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemevolvedata/ItemEvolveData.md")))]
#[::unity2::class(namespace = "App", name = "ItemEvolveData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: itemevolvedata :: ItemEvolveData >)]
pub struct ItemEvolveData {}

#[cfg(feature = "app-itemevolvedata")]
#[::unity2::methods]
impl ItemEvolveData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "TryGetFromIid", args = 1)]
    pub fn try_get_from_iid(
        iid: ::unity2::Il2CppString,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::itemevolvedata::ItemEvolveData,
    >;

    #[method(name = "TryGetFromItem", args = 1)]
    pub fn try_get_from_item(
        item: crate::app::itemdata::ItemData,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::itemevolvedata::ItemEvolveData,
    >;

    #[method(name = "Iid2Eid", args = 1)]
    pub fn iid2_eid(iid: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "get_Iid", args = 0)]
    pub fn get_iid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Iid", args = 1)]
    pub fn set_iid(self, value: ::unity2::Il2CppString) -> ();

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

    #[method(name = "get_RefineLevel", args = 0)]
    pub fn get_refine_level(self) -> u8;

    #[method(name = "set_RefineLevel", args = 1)]
    pub fn set_refine_level(self, value: u8) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetFlagName", args = 0)]
    pub fn get_flag_name(self) -> ::unity2::Il2CppString;

    #[method(name = "RegistGlobalFlags", args = 0)]
    pub fn regist_global_flags() -> ();

    #[method(name = "IsOnceEvolved", args = 0)]
    pub fn is_once_evolved(self) -> bool;

    #[method(name = "SetEvolved", args = 1)]
    pub fn set_evolved(self, evolved: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-itemevolvedata")]
impl ItemEvolveData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemEvolveData),
                ::core::stringify!(new),
            )
        });
        <Self as IItemEvolveDataMethods>::ctor(this);
        this
    }
}
