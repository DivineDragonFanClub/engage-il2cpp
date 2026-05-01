
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/boundsint/BoundsInt.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct BoundsInt {
    pub m_position: crate::unity_engine::vector3int::Vector3Int,
    pub m_size: crate::unity_engine::vector3int::Vector3Int,
}

impl ::unity2::ClassIdentity for BoundsInt {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "BoundsInt";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BoundsInt {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-boundsint")]
#[::unity2::methods(value)]
impl BoundsInt {
    #[method(name = "get_position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector3int::Vector3Int;

    #[method(name = "get_size", args = 0)]
    pub fn get_size(self) -> crate::unity_engine::vector3int::Vector3Int;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::boundsint::BoundsInt) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}
