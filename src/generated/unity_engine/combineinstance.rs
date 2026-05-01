
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/combineinstance/CombineInstance.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct CombineInstance {
    pub m_mesh_instance_id: i32,
    pub m_sub_mesh_index: i32,
    pub m_transform: crate::unity_engine::matrix4x4::Matrix4x4,
    pub m_lightmap_scale_offset: crate::unity_engine::vector4::Vector4,
    pub m_realtime_lightmap_scale_offset: crate::unity_engine::vector4::Vector4,
}

impl ::unity2::ClassIdentity for CombineInstance {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "CombineInstance";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CombineInstance {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
