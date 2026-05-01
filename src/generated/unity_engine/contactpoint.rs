
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/contactpoint/ContactPoint.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ContactPoint {
    pub m_point: crate::unity_engine::vector3::Vector3,
    pub m_normal: crate::unity_engine::vector3::Vector3,
    pub m_this_collider_instance_id: i32,
    pub m_other_collider_instance_id: i32,
    pub m_separation: f32,
}

impl ::unity2::ClassIdentity for ContactPoint {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ContactPoint";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ContactPoint {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
