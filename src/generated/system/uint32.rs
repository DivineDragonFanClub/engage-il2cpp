
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/uint32/UInt32.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct UInt32 {
    pub m_value: u32,
}

impl ::unity2::ClassIdentity for UInt32 {
    const NAMESPACE: &'static str = "System";

    const NAME: &'static str = "UInt32";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for UInt32 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-uint32")]
#[::unity2::methods(value)]
impl UInt32 {
    #[method(name = "CompareTo", args = 1)]
    pub fn compare_to(self, value: crate::system::object::Object) -> i32;

    #[method(name = "CompareTo", args = 1)]
    pub fn compare_to_2(self, value: u32) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: u32) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "ToString", args = 1)]
    pub fn to_string_2(self, format: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "Parse", args = 1)]
    pub fn parse(s: ::unity2::Il2CppString) -> u32;
}
