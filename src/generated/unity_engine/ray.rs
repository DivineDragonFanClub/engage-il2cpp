
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ray/Ray.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Ray {
    pub m_origin: crate::unity_engine::vector3::Vector3,
    pub m_direction: crate::unity_engine::vector3::Vector3,
}

impl ::unity2::ClassIdentity for Ray {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "Ray";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Ray {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-ray")]
#[::unity2::methods(value)]
impl Ray {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        origin: crate::unity_engine::vector3::Vector3,
        direction: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "get_origin", args = 0)]
    pub fn get_origin(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_origin", args = 1)]
    pub fn set_origin(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_direction", args = 0)]
    pub fn get_direction(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_direction", args = 1)]
    pub fn set_direction(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "GetPoint", args = 1)]
    pub fn get_point(self, distance: f32) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}
