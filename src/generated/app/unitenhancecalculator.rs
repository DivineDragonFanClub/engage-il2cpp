
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/unitenhancecalculator/UnitEnhanceCalculator.md")))]
#[::unity2::class(namespace = "App", name = "UnitEnhanceCalculator")]
#[parent(crate::system::object::Object)]
pub struct UnitEnhanceCalculator {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[static_field]
    #[rename(name = "MinValue")]
    pub min_value: i32,
    #[static_field]
    #[rename(name = "MaxValue")]
    pub max_value: i32,
    #[rename(name = "m_Values")]
    pub m_values: crate::app::unitenhancevalues::UnitEnhanceValues,
    #[rename(name = "m_Temps")]
    pub m_temps: crate::app::unitenhancevalues::UnitEnhanceValues,
}

#[cfg(feature = "app-unitenhancecalculator")]
#[::unity2::methods]
impl UnitEnhanceCalculator {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Commit1st", args = 2)]
    pub fn commit1st(
        self,
        unit: crate::app::unit::Unit,
        equipped: crate::app::unititem::UnitItem,
    ) -> ();

    #[method(name = "Commit2nd", args = 1)]
    pub fn commit2nd(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Copy", args = 1)]
    pub fn copy(self, enhance: crate::app::unitenhancecalculator::UnitEnhanceCalculator) -> ();

    #[method(name = "Calculate", args = 2)]
    pub fn calculate(self, index: i32, value: i32) -> i32;

    #[method(name = "AddImpl", args = 2)]
    pub fn add_impl(
        self,
        capability: crate::app::capabilitysbyte::CapabilitySbyte,
        is_not_enhance: bool,
    ) -> ();

    #[method(name = "MergeImpl", args = 2)]
    pub fn merge_impl(
        self,
        values: crate::app::unitenhancevalues::UnitEnhanceValues,
        is_not_enhance: bool,
    ) -> ();

    #[method(name = "AddImpl", args = 4)]
    pub fn add_impl_2(
        self,
        unit: crate::app::unit::Unit,
        skills: crate::app::skillarray::SkillArray,
        mask: crate::app::skilldata::SkillData_Flags,
        is_not_enhance: bool,
    ) -> bool;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();
}

#[cfg(feature = "app-unitenhancecalculator")]
impl UnitEnhanceCalculator {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnitEnhanceCalculator),
                ::core::stringify!(new),
            )
        });
        <Self as IUnitEnhanceCalculatorMethods>::ctor(this);
        this
    }
}
