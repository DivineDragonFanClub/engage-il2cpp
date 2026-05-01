
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rangeint/RangeInt.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct RangeInt {
    pub start: i32,
    pub length: i32,
}

impl ::unity2::ClassIdentity for RangeInt {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "RangeInt";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RangeInt {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rangeint")]
#[::unity2::methods(value)]
impl RangeInt {
    #[method(name = "get_end", args = 0)]
    pub fn get_end(self) -> i32;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, start: i32, length: i32) -> ();
}
