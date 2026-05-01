
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/belongdata/BelongData.md")))]
#[::unity2::class(namespace = "App", name = "BelongData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: belongdata :: BelongData >)]
pub struct BelongData {}

#[cfg(feature = "app-belongdata")]
#[::unity2::methods]
impl BelongData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Bid", args = 0)]
    pub fn get_bid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Bid", args = 1)]
    pub fn set_bid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DefeatAchieve", args = 0)]
    pub fn get_defeat_achieve(self) -> crate::app::achievedata::AchieveData_Kinds;

    #[method(name = "set_DefeatAchieve", args = 1)]
    pub fn set_defeat_achieve(self, value: crate::app::achievedata::AchieveData_Kinds) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-belongdata")]
impl BelongData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BelongData),
                ::core::stringify!(new),
            )
        });
        <Self as IBelongDataMethods>::ctor(this);
        this
    }
}
