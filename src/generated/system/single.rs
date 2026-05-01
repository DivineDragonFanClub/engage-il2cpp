
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/single/Single.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Single {
    pub m_value: f32,
}

impl ::unity2::ClassIdentity for Single {
    const NAMESPACE: &'static str = "System";

    const NAME: &'static str = "Single";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Single {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-single")]
#[::unity2::methods(value)]
impl Single {
    #[method(name = "IsInfinity", args = 1)]
    pub fn is_infinity(f: f32) -> bool;

    #[method(name = "IsPositiveInfinity", args = 1)]
    pub fn is_positive_infinity(f: f32) -> bool;

    #[method(name = "IsNegativeInfinity", args = 1)]
    pub fn is_negative_infinity(f: f32) -> bool;

    #[method(name = "IsNaN", args = 1)]
    pub fn is_na_n(f: f32) -> bool;

    #[method(name = "CompareTo", args = 1)]
    pub fn compare_to(self, value: crate::system::object::Object) -> i32;

    #[method(name = "CompareTo", args = 1)]
    pub fn compare_to_2(self, value: f32) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: f32) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "ToString", args = 1)]
    pub fn to_string_2(self, format: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "Parse", args = 1)]
    pub fn parse(s: ::unity2::Il2CppString) -> f32;

    #[method(name = "TryParse", args = 2)]
    pub fn try_parse(s: ::unity2::Il2CppString, result: f32) -> bool;
}
