
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/boneweight1/BoneWeight1.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct BoneWeight1 {
    pub m_weight: f32,
    pub m_bone_index: i32,
}

impl ::unity2::ClassIdentity for BoneWeight1 {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "BoneWeight1";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BoneWeight1 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-boneweight1")]
#[::unity2::methods(value)]
impl BoneWeight1 {
    #[method(name = "get_weight", args = 0)]
    pub fn get_weight(self) -> f32;

    #[method(name = "get_boneIndex", args = 0)]
    pub fn get_bone_index(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::boneweight1::BoneWeight1) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}
