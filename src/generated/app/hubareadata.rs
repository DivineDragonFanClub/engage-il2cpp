
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubareadata/HubAreaData.md")))]
#[::unity2::class(namespace = "App", name = "HubAreaData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: hubareadata :: HubAreaData >)]
pub struct HubAreaData {}

#[cfg(feature = "app-hubareadata")]
#[::unity2::methods]
impl HubAreaData {
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

    #[method(name = "get_SceneName", args = 0)]
    pub fn get_scene_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SceneName", args = 1)]
    pub fn set_scene_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LocatorName", args = 0)]
    pub fn get_locator_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LocatorName", args = 1)]
    pub fn set_locator_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MID_H", args = 0)]
    pub fn get_mid_h(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MID_H", args = 1)]
    pub fn set_mid_h(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MapPointNo", args = 0)]
    pub fn get_map_point_no(self) -> u8;

    #[method(name = "set_MapPointNo", args = 1)]
    pub fn set_map_point_no(self, value: u8) -> ();

    #[method(name = "get_FacilityAidList", args = 0)]
    pub fn get_facility_aid_list(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_FacilityAidList", args = 1)]
    pub fn set_facility_aid_list(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubareadata")]
impl HubAreaData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubAreaData),
                ::core::stringify!(new),
            )
        });
        <Self as IHubAreaDataMethods>::ctor(this);
        this
    }
}
