
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/musclesitupfalldata/MuscleSitUpFallData.md")))]
#[::unity2::class(namespace = "App", name = "MuscleSitUpFallData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: musclesitupfalldata :: MuscleSitUpFallData >)]
pub struct MuscleSitUpFallData {}

#[cfg(feature = "app-musclesitupfalldata")]
#[::unity2::methods]
impl MuscleSitUpFallData {
    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> i32;

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, value: i32) -> ();

    #[method(name = "get_PerfectLimit", args = 0)]
    pub fn get_perfect_limit(self) -> f32;

    #[method(name = "set_PerfectLimit", args = 1)]
    pub fn set_perfect_limit(self, value: f32) -> ();

    #[method(name = "get_GainPower", args = 0)]
    pub fn get_gain_power(self) -> f32;

    #[method(name = "set_GainPower", args = 1)]
    pub fn set_gain_power(self, value: f32) -> ();

    #[method(name = "get_FallSpeed", args = 0)]
    pub fn get_fall_speed(self) -> f32;

    #[method(name = "set_FallSpeed", args = 1)]
    pub fn set_fall_speed(self, value: f32) -> ();

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
            crate::app::musclesitupfalldata::MuscleSitUpFallData,
        >,
    ) -> ();
}

#[cfg(feature = "app-musclesitupfalldata")]
impl MuscleSitUpFallData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MuscleSitUpFallData),
                ::core::stringify!(new),
            )
        });
        <Self as IMuscleSitUpFallDataMethods>::ctor(this);
        this
    }
}
