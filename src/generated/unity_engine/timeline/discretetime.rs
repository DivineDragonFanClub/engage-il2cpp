
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/discretetime/DiscreteTime.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct DiscreteTime {
    pub m_discrete_time: i64,
}

impl ::unity2::ClassIdentity for DiscreteTime {
    const NAMESPACE: &'static str = "UnityEngine.Timeline";

    const NAME: &'static str = "DiscreteTime";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DiscreteTime {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-timeline-discretetime")]
#[::unity2::methods(value)]
impl DiscreteTime {
    #[method(name = "get_tickValue", args = 0)]
    pub fn get_tick_value() -> f64;

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, time: i64) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, time: f64) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, time: i32) -> ();

    #[method(name = "OneTickBefore", args = 0)]
    pub fn one_tick_before(self) -> crate::unity_engine::timeline::discretetime::DiscreteTime;

    #[method(name = "OneTickAfter", args = 0)]
    pub fn one_tick_after(self) -> crate::unity_engine::timeline::discretetime::DiscreteTime;

    #[method(name = "FromTicks", args = 1)]
    pub fn from_ticks(ticks: i64) -> crate::unity_engine::timeline::discretetime::DiscreteTime;

    #[method(name = "CompareTo", args = 1)]
    pub fn compare_to(self, obj: crate::system::object::Object) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::unity_engine::timeline::discretetime::DiscreteTime) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "DoubleToDiscreteTime", args = 1)]
    pub fn double_to_discrete_time(time: f64) -> i64;

    #[method(name = "IntToDiscreteTime", args = 1)]
    pub fn int_to_discrete_time(time: i32) -> i64;

    #[method(name = "ToDouble", args = 1)]
    pub fn to_double(time: i64) -> f64;

    #[method(name = "op_Explicit", args = 1)]
    pub fn op_explicit(b: crate::unity_engine::timeline::discretetime::DiscreteTime) -> f64;

    #[method(name = "op_Explicit", args = 1)]
    pub fn op_explicit_2(time: f64) -> crate::unity_engine::timeline::discretetime::DiscreteTime;

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(time: i32) -> crate::unity_engine::timeline::discretetime::DiscreteTime;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::unity_engine::timeline::discretetime::DiscreteTime,
        rhs: crate::unity_engine::timeline::discretetime::DiscreteTime,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        lhs: crate::unity_engine::timeline::discretetime::DiscreteTime,
        rhs: crate::unity_engine::timeline::discretetime::DiscreteTime,
    ) -> bool;

    #[method(name = "op_LessThanOrEqual", args = 2)]
    pub fn op_less_than_or_equal(
        lhs: crate::unity_engine::timeline::discretetime::DiscreteTime,
        rhs: crate::unity_engine::timeline::discretetime::DiscreteTime,
    ) -> bool;

    #[method(name = "op_GreaterThanOrEqual", args = 2)]
    pub fn op_greater_than_or_equal(
        lhs: crate::unity_engine::timeline::discretetime::DiscreteTime,
        rhs: crate::unity_engine::timeline::discretetime::DiscreteTime,
    ) -> bool;

    #[method(name = "op_Subtraction", args = 2)]
    pub fn op_subtraction(
        lhs: crate::unity_engine::timeline::discretetime::DiscreteTime,
        rhs: crate::unity_engine::timeline::discretetime::DiscreteTime,
    ) -> crate::unity_engine::timeline::discretetime::DiscreteTime;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Max", args = 2)]
    pub fn max(
        lhs: crate::unity_engine::timeline::discretetime::DiscreteTime,
        rhs: crate::unity_engine::timeline::discretetime::DiscreteTime,
    ) -> crate::unity_engine::timeline::discretetime::DiscreteTime;

    #[method(name = "GetNearestTick", args = 1)]
    pub fn get_nearest_tick(time: f64) -> i64;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
