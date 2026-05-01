
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingtargetlistdata/FishingTargetListData.md")))]
#[::unity2::class(namespace = "App", name = "FishingTargetListData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: fishingtargetlistdata :: FishingTargetListData >)]
pub struct FishingTargetListData {}

#[cfg(feature = "app-fishingtargetlistdata")]
#[::unity2::methods]
impl FishingTargetListData {
    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_FishID", args = 0)]
    pub fn get_fish_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FishID", args = 1)]
    pub fn set_fish_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Priority", args = 0)]
    pub fn get_priority(self) -> i32;

    #[method(name = "set_Priority", args = 1)]
    pub fn set_priority(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetTargetList", args = 5)]
    pub fn get_target_list(
        enable_stick_a: bool,
        enable_stick_b: bool,
        enable_stick_c: bool,
        out_fish_list: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
        out_priority_list: crate::system::collections::generic::list_1::List_1<i32>,
    ) -> ();
}

#[cfg(feature = "app-fishingtargetlistdata")]
impl FishingTargetListData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingTargetListData),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingTargetListDataMethods>::ctor(this);
        this
    }
}
