
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/bounds/Bounds.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Bounds {
    pub m_center: crate::unity_engine::vector3::Vector3,
    pub m_extents: crate::unity_engine::vector3::Vector3,
}

impl ::unity2::ClassIdentity for Bounds {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Bounds";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Bounds {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-bounds")]
#[::unity2::methods(value)]
impl Bounds {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        center: crate::unity_engine::vector3::Vector3,
        size: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::bounds::Bounds) -> bool;

    #[method(name = "get_center", args = 0)]
    pub fn get_center(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_center", args = 1)]
    pub fn set_center(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_size", args = 0)]
    pub fn get_size(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_size", args = 1)]
    pub fn set_size(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_extents", args = 0)]
    pub fn get_extents(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_extents", args = 1)]
    pub fn set_extents(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_min", args = 0)]
    pub fn get_min(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_min", args = 1)]
    pub fn set_min(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_max", args = 0)]
    pub fn get_max(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_max", args = 1)]
    pub fn set_max(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::unity_engine::bounds::Bounds,
        rhs: crate::unity_engine::bounds::Bounds,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        lhs: crate::unity_engine::bounds::Bounds,
        rhs: crate::unity_engine::bounds::Bounds,
    ) -> bool;

    #[method(name = "SetMinMax", args = 2)]
    pub fn set_min_max(
        self,
        min: crate::unity_engine::vector3::Vector3,
        max: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "Encapsulate", args = 1)]
    pub fn encapsulate(self, point: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "Encapsulate", args = 1)]
    pub fn encapsulate_2(self, bounds: crate::unity_engine::bounds::Bounds) -> ();

    #[method(name = "Intersects", args = 1)]
    pub fn intersects(self, bounds: crate::unity_engine::bounds::Bounds) -> bool;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}
