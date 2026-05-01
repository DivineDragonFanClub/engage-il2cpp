
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/photographspotdata/PhotographSpotData.md")))]
#[::unity2::class(namespace = "App", name = "PhotographSpotData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: photographspotdata :: PhotographSpotData >)]
pub struct PhotographSpotData {}

#[cfg(feature = "app-photographspotdata")]
#[::unity2::methods]
impl PhotographSpotData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MID", args = 0)]
    pub fn get_mid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MID", args = 1)]
    pub fn set_mid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ConditionCid", args = 0)]
    pub fn get_condition_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ConditionCid", args = 1)]
    pub fn set_condition_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LocatorCount", args = 0)]
    pub fn get_locator_count(self) -> i32;

    #[method(name = "set_LocatorCount", args = 1)]
    pub fn set_locator_count(self, value: i32) -> ();

    #[method(name = "get_PauseGroupNameList1", args = 0)]
    pub fn get_pause_group_name_list1(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_PauseGroupNameList1", args = 1)]
    pub fn set_pause_group_name_list1(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_PauseGroupNameList2", args = 0)]
    pub fn get_pause_group_name_list2(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_PauseGroupNameList2", args = 1)]
    pub fn set_pause_group_name_list2(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_PauseGroupNameList3", args = 0)]
    pub fn get_pause_group_name_list3(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_PauseGroupNameList3", args = 1)]
    pub fn set_pause_group_name_list3(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_PauseGroupNameList4", args = 0)]
    pub fn get_pause_group_name_list4(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_PauseGroupNameList4", args = 1)]
    pub fn set_pause_group_name_list4(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "GetPauseGroupNameList", args = 1)]
    pub fn get_pause_group_name_list(
        self,
        locator_no: i32,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "IsNew", args = 0)]
    pub fn is_new(self) -> bool;

    #[method(name = "RegistGlobalFlags", args = 0)]
    pub fn regist_global_flags() -> ();

    #[method(name = "UpdateStoryProgress", args = 0)]
    pub fn update_story_progress() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-photographspotdata")]
impl PhotographSpotData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhotographSpotData),
                ::core::stringify!(new),
            )
        });
        <Self as IPhotographSpotDataMethods>::ctor(this);
        this
    }
}
