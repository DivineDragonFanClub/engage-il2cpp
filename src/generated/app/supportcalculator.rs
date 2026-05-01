
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/supportcalculator/SupportCalculator.md")))]
#[::unity2::class(namespace = "App", name = "SupportCalculator")]
#[parent(crate::system::object::Object)]
pub struct SupportCalculator {
    #[static_field]
    #[rename(name = "Range")]
    pub range: i32,
    #[static_field]
    #[rename(name = "MaxShowUnits")]
    pub max_show_units: i32,
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_X")]
    pub m_x: i32,
    #[rename(name = "m_Z")]
    pub m_z: i32,
    #[rename(name = "m_Hit")]
    pub m_hit: i32,
    #[rename(name = "m_Avoid")]
    pub m_avoid: i32,
    #[rename(name = "m_Critical")]
    pub m_critical: i32,
    #[rename(name = "m_Secure")]
    pub m_secure: i32,
    #[rename(name = "m_ShowUnits")]
    pub m_show_units: ::unity2::Array<crate::app::unit::Unit>,
    #[rename(name = "m_ShowUnitLevels")]
    pub m_show_unit_levels: ::unity2::Array<crate::app::reliancedata::RelianceData_Level>,
    #[rename(name = "m_RangeFunction")]
    pub m_range_function: crate::app::mapfor::MapFor_RangeFunction,
}

#[cfg(feature = "app-supportcalculator")]
#[::unity2::methods]
impl SupportCalculator {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Setup", args = 3)]
    pub fn setup(self, unit: crate::app::unit::Unit, x: i32, z: i32) -> ();

    #[method(name = "RangeFunction", args = 3)]
    pub fn range_function(self, x: i32, z: i32, range: i32) -> ();

    #[method(name = "Calc", args = 0)]
    pub fn calc(self) -> ();

    #[method(name = "get_Hit", args = 0)]
    pub fn get_hit(self) -> i32;

    #[method(name = "get_Avoid", args = 0)]
    pub fn get_avoid(self) -> i32;

    #[method(name = "get_Critical", args = 0)]
    pub fn get_critical(self) -> i32;

    #[method(name = "get_Secure", args = 0)]
    pub fn get_secure(self) -> i32;

    #[method(name = "get_ShowUnits", args = 0)]
    pub fn get_show_units(self) -> ::unity2::Array<crate::app::unit::Unit>;

    #[method(name = "RegisterShowUnit", args = 2)]
    pub fn register_show_unit(
        self,
        unit: crate::app::unit::Unit,
        reliance_level: crate::app::reliancedata::RelianceData_Level,
    ) -> ();

    #[method(name = "InsertShowUnit", args = 3)]
    pub fn insert_show_unit(
        self,
        index: i32,
        unit: crate::app::unit::Unit,
        reliance_level: crate::app::reliancedata::RelianceData_Level,
    ) -> ();
}

#[cfg(feature = "app-supportcalculator")]
impl SupportCalculator {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SupportCalculator),
                ::core::stringify!(new),
            )
        });
        <Self as ISupportCalculatorMethods>::ctor(this);
        this
    }
}
