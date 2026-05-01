
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubfacilitydata/HubFacilityData.md")))]
#[::unity2::class(namespace = "App", name = "HubFacilityData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: hubfacilitydata :: HubFacilityData >)]
pub struct HubFacilityData {}

#[cfg(feature = "app-hubfacilitydata")]
#[::unity2::methods]
impl HubFacilityData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_AID", args = 0)]
    pub fn get_aid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AID", args = 1)]
    pub fn set_aid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MID", args = 0)]
    pub fn get_mid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MID", args = 1)]
    pub fn set_mid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ConditionCID", args = 0)]
    pub fn get_condition_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ConditionCID", args = 1)]
    pub fn set_condition_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IconName", args = 0)]
    pub fn get_icon_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_IconName", args = 1)]
    pub fn set_icon_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetPrefixlessAid", args = 0)]
    pub fn get_prefixless_aid(self) -> ::unity2::Il2CppString;

    #[method(name = "GetFirstAccessFlag", args = 0)]
    pub fn get_first_access_flag(self) -> ::unity2::Il2CppString;

    #[method(name = "IsFirstAccessFlag", args = 0)]
    pub fn is_first_access_flag(self) -> bool;

    #[method(name = "SetFirstAccessFlag", args = 0)]
    pub fn set_first_access_flag(self) -> ();

    #[method(name = "ResetFirstAccessFlag", args = 0)]
    pub fn reset_first_access_flag(self) -> ();

    #[method(name = "IsComplete", args = 0)]
    pub fn is_complete(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubfacilitydata")]
impl HubFacilityData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubFacilityData),
                ::core::stringify!(new),
            )
        });
        <Self as IHubFacilityDataMethods>::ctor(this);
        this
    }
}
