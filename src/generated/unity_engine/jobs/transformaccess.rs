
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/jobs/transformaccess/TransformAccess.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct TransformAccess {
    pub hierarchy: ::unity2::IntPtr,
    pub index: i32,
}

impl ::unity2::ClassIdentity for TransformAccess {
    const NAMESPACE: &'static str = "UnityEngine.Jobs";

    const NAME: &'static str = "TransformAccess";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TransformAccess {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-jobs-transformaccess")]
#[::unity2::methods(value)]
impl TransformAccess {
    #[method(name = "get_position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_rotation", args = 0)]
    pub fn get_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "set_localRotation", args = 1)]
    pub fn set_local_rotation(self, value: crate::unity_engine::quaternion::Quaternion) -> ();

    #[method(name = "GetPosition", args = 2)]
    pub fn get_position_2(
        access: crate::unity_engine::jobs::transformaccess::TransformAccess,
        p: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "GetRotation", args = 2)]
    pub fn get_rotation_2(
        access: crate::unity_engine::jobs::transformaccess::TransformAccess,
        r: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = "SetLocalRotation", args = 2)]
    pub fn set_local_rotation_2(
        access: crate::unity_engine::jobs::transformaccess::TransformAccess,
        r: crate::unity_engine::quaternion::Quaternion,
    ) -> ();
}
