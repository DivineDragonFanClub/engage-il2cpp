
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/humanlimit/HumanLimit.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct HumanLimit {
    pub m_min: crate::unity_engine::vector3::Vector3,
    pub m_max: crate::unity_engine::vector3::Vector3,
    pub m_center: crate::unity_engine::vector3::Vector3,
    pub m_axis_length: f32,
    pub m_use_default_values: i32,
}

impl ::unity2::ClassIdentity for HumanLimit {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "HumanLimit";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HumanLimit {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
