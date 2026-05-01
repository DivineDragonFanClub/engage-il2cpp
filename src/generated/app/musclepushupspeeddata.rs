
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/musclepushupspeeddata/MusclePushUpSpeedData.md")))]
#[::unity2::class(namespace = "App", name = "MusclePushUpSpeedData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: musclepushupspeeddata :: MusclePushUpSpeedData >)]
pub struct MusclePushUpSpeedData {}

#[cfg(feature = "app-musclepushupspeeddata")]
#[::unity2::methods]
impl MusclePushUpSpeedData {
    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> i32;

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, value: i32) -> ();

    #[method(name = "get_SpeedMin", args = 0)]
    pub fn get_speed_min(self) -> f32;

    #[method(name = "set_SpeedMin", args = 1)]
    pub fn set_speed_min(self, value: f32) -> ();

    #[method(name = "get_SpeedMax", args = 0)]
    pub fn get_speed_max(self) -> f32;

    #[method(name = "set_SpeedMax", args = 1)]
    pub fn set_speed_max(self, value: f32) -> ();

    #[method(name = "get_LevelUpCount", args = 0)]
    pub fn get_level_up_count(self) -> i32;

    #[method(name = "set_LevelUpCount", args = 1)]
    pub fn set_level_up_count(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetSpeedList", args = 2)]
    pub fn get_speed_list(
        level: i32,
        set_list: crate::system::collections::generic::list_1::List_1<
            crate::app::musclepushupspeeddata::MusclePushUpSpeedData,
        >,
    ) -> ();
}

#[cfg(feature = "app-musclepushupspeeddata")]
impl MusclePushUpSpeedData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MusclePushUpSpeedData),
                ::core::stringify!(new),
            )
        });
        <Self as IMusclePushUpSpeedDataMethods>::ctor(this);
        this
    }
}
