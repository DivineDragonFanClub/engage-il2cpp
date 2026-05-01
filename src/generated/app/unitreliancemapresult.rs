
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitreliancemapresult/UnitRelianceMapResult.md")))]
#[::unity2::class(namespace = "App", name = "UnitRelianceMapResult")]
#[parent(crate::system::object::Object)]
pub struct UnitRelianceMapResult {}

#[cfg(feature = "app-unitreliancemapresult")]
#[::unity2::methods]
impl UnitRelianceMapResult {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        pid_a: ::unity2::Il2CppString,
        pid_b: ::unity2::Il2CppString,
        unit_data: crate::app::unitreliancedata::UnitRelianceData,
    ) -> ();

    #[method(name = "get_PidA", args = 0)]
    pub fn get_pid_a(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PidA", args = 1)]
    pub fn set_pid_a(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PersonIndexA", args = 0)]
    pub fn get_person_index_a(self) -> i32;

    #[method(name = "set_PersonIndexA", args = 1)]
    pub fn set_person_index_a(self, value: i32) -> ();

    #[method(name = "get_PidB", args = 0)]
    pub fn get_pid_b(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PidB", args = 1)]
    pub fn set_pid_b(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PersonIndexB", args = 0)]
    pub fn get_person_index_b(self) -> i32;

    #[method(name = "set_PersonIndexB", args = 1)]
    pub fn set_person_index_b(self, value: i32) -> ();

    #[method(name = "get_UnitData", args = 0)]
    pub fn get_unit_data(self) -> crate::app::unitreliancedata::UnitRelianceData;

    #[method(name = "set_UnitData", args = 1)]
    pub fn set_unit_data(self, value: crate::app::unitreliancedata::UnitRelianceData) -> ();

    #[method(name = "Compare", args = 2)]
    pub fn compare(
        a: crate::app::unitreliancemapresult::UnitRelianceMapResult,
        b: crate::app::unitreliancemapresult::UnitRelianceMapResult,
    ) -> i32;
}

#[cfg(feature = "app-unitreliancemapresult")]
impl UnitRelianceMapResult {
    pub fn new(
        pid_a: ::unity2::Il2CppString,
        pid_b: ::unity2::Il2CppString,
        unit_data: crate::app::unitreliancedata::UnitRelianceData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitRelianceMapResult),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitRelianceMapResultMethods>::ctor(this, pid_a, pid_b, unit_data);
        this
    }
}
