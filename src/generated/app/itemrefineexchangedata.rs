
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemrefineexchangedata/ItemRefineExchangeData.md")))]
#[::unity2::class(namespace = "App", name = "ItemRefineExchangeData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: itemrefineexchangedata :: ItemRefineExchangeData >)]
pub struct ItemRefineExchangeData {}

#[cfg(feature = "app-itemrefineexchangedata")]
#[::unity2::methods]
impl ItemRefineExchangeData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Operation", args = 0)]
    pub fn get_operation(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Operation", args = 1)]
    pub fn set_operation(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Icon", args = 0)]
    pub fn get_icon(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Icon", args = 1)]
    pub fn set_icon(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ToIron", args = 0)]
    pub fn get_to_iron(self) -> u16;

    #[method(name = "set_ToIron", args = 1)]
    pub fn set_to_iron(self, value: u16) -> ();

    #[method(name = "get_ToSteel", args = 0)]
    pub fn get_to_steel(self) -> u16;

    #[method(name = "set_ToSteel", args = 1)]
    pub fn set_to_steel(self, value: u16) -> ();

    #[method(name = "get_ToSilver", args = 0)]
    pub fn get_to_silver(self) -> u16;

    #[method(name = "set_ToSilver", args = 1)]
    pub fn set_to_silver(self, value: u16) -> ();

    #[method(name = "get_ForIron", args = 0)]
    pub fn get_for_iron(self) -> u16;

    #[method(name = "set_ForIron", args = 1)]
    pub fn set_for_iron(self, value: u16) -> ();

    #[method(name = "get_ForSteel", args = 0)]
    pub fn get_for_steel(self) -> u16;

    #[method(name = "set_ForSteel", args = 1)]
    pub fn set_for_steel(self, value: u16) -> ();

    #[method(name = "get_ForSilver", args = 0)]
    pub fn get_for_silver(self) -> u16;

    #[method(name = "set_ForSilver", args = 1)]
    pub fn set_for_silver(self, value: u16) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetExchangeCost", args = 3)]
    pub fn get_exchange_cost(
        self,
        material_id: ::unity2::Il2CppString,
        source_count: i32,
        target_count: i32,
    ) -> ();

    #[method(name = "GetExchangeCostTo", args = 1)]
    pub fn get_exchange_cost_to(self, material_id: ::unity2::Il2CppString) -> u16;

    #[method(name = "GetExchangeCostFor", args = 1)]
    pub fn get_exchange_cost_for(self, material_id: ::unity2::Il2CppString) -> u16;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-itemrefineexchangedata")]
impl ItemRefineExchangeData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemRefineExchangeData),
                ::core::stringify!(new),
            )
        });
        <Self as IItemRefineExchangeDataMethods>::ctor(this);
        this
    }
}
