
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/boolean/Boolean.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Boolean {
    pub m_value: bool,
}

impl ::unity2::ClassIdentity for Boolean {
    const NAMESPACE: &'static str = "System";

    const NAME: &'static str = "Boolean";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Boolean {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-boolean")]
#[::unity2::methods(value)]
impl Boolean {
    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: bool) -> bool;

    #[method(name = "CompareTo", args = 1)]
    pub fn compare_to(self, obj: crate::system::object::Object) -> i32;

    #[method(name = "CompareTo", args = 1)]
    pub fn compare_to_2(self, value: bool) -> i32;

    #[method(name = "Parse", args = 1)]
    pub fn parse(value: ::unity2::Il2CppString) -> bool;

    #[method(name = "TryParse", args = 2)]
    pub fn try_parse(value: ::unity2::Il2CppString, result: bool) -> bool;

    #[method(name = "TrimWhiteSpaceAndNull", args = 1)]
    pub fn trim_white_space_and_null(value: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
