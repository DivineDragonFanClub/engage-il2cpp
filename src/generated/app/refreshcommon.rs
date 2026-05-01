
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refreshcommon/RefreshCommon.md")))]
#[::unity2::class(namespace = "App", name = "RefreshCommon")]
#[parent(crate::system::object::Object)]
pub struct RefreshCommon {
    #[static_field]
    #[rename(name = "UnitCount")]
    pub unit_count: i32,
    #[static_field]
    #[rename(name = "FacilityAidArray")]
    pub facility_aid_array: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "FacilityCount")]
    pub facility_count: i32,
}

#[cfg(feature = "app-refreshcommon")]
#[::unity2::methods]
impl RefreshCommon {
    #[method(name = "GetFacilityIndex", args = 1)]
    pub fn get_facility_index(facility_aid: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetUnitRandom", args = 1)]
    pub fn get_unit_random(excepted_unit: crate::app::unit::Unit) -> crate::app::unit::Unit;

    #[method(name = "GetUnitEntrusted", args = 1)]
    pub fn get_unit_entrusted(another_unit: crate::app::unit::Unit) -> crate::app::unit::Unit;

    #[method(name = "GetUnitEntrusted", args = 1)]
    pub fn get_unit_entrusted_2(unit_array: ::unity2::Array<crate::app::unit::Unit>) -> ();

    #[method(name = "CalcUnitEntrusted", args = 1)]
    pub fn calc_unit_entrusted(another_unit: crate::app::unit::Unit) -> crate::app::unit::Unit;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-refreshcommon")]
impl RefreshCommon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefreshCommon),
                ::core::stringify!(new),
            )
        });
        <Self as IRefreshCommonMethods>::ctor(this);
        this
    }
}
