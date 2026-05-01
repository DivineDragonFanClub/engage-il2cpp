
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/double/Double.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Double {
    pub m_value: f64,
}

impl ::unity2::ClassIdentity for Double {
    const NAMESPACE: &'static str = "System";

    const NAME: &'static str = "Double";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Double {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-double")]
#[::unity2::methods(value)]
impl Double {
    #[method(name = "IsInfinity", args = 1)]
    pub fn is_infinity(d: f64) -> bool;

    #[method(name = "IsPositiveInfinity", args = 1)]
    pub fn is_positive_infinity(d: f64) -> bool;

    #[method(name = "IsNegativeInfinity", args = 1)]
    pub fn is_negative_infinity(d: f64) -> bool;

    #[method(name = "IsNegative", args = 1)]
    pub fn is_negative(d: f64) -> bool;

    #[method(name = "IsNaN", args = 1)]
    pub fn is_na_n(d: f64) -> bool;

    #[method(name = "CompareTo", args = 1)]
    pub fn compare_to(self, value: crate::system::object::Object) -> i32;

    #[method(name = "CompareTo", args = 1)]
    pub fn compare_to_2(self, value: f64) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: f64) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "ToString", args = 1)]
    pub fn to_string_2(self, format: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "Parse", args = 1)]
    pub fn parse(s: ::unity2::Il2CppString) -> f64;

    #[method(name = "TryParse", args = 2)]
    pub fn try_parse(s: ::unity2::Il2CppString, result: f64) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
