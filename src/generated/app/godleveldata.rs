
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godleveldata/GodLevelData.md")))]
#[::unity2::class(namespace = "App", name = "GodLevelData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: godleveldata :: GodLevelData >)]
pub struct GodLevelData {}

#[cfg(feature = "app-godleveldata")]
#[::unity2::methods]
impl GodLevelData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Exp", args = 0)]
    pub fn get_exp(self) -> i32;

    #[method(name = "set_Exp", args = 1)]
    pub fn set_exp(self, value: i32) -> ();

    #[method(name = "get_RelianceLevel", args = 0)]
    pub fn get_reliance_level(self) -> ::unity2::Il2CppString;

    #[method(name = "set_RelianceLevel", args = 1)]
    pub fn set_reliance_level(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Cost", args = 0)]
    pub fn get_cost(self) -> i32;

    #[method(name = "set_Cost", args = 1)]
    pub fn set_cost(self, value: i32) -> ();

    #[method(name = "GetMaxLevel", args = 0)]
    pub fn get_max_level() -> i32;

    #[method(name = "GetMaxLevelData", args = 0)]
    pub fn get_max_level_data() -> crate::app::godleveldata::GodLevelData;

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-godleveldata")]
impl GodLevelData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodLevelData),
                ::core::stringify!(new),
            )
        });
        <Self as IGodLevelDataMethods>::ctor(this);
        this
    }
}
