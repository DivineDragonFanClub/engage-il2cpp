
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/shadertagid/ShaderTagId.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ShaderTagId {
    pub m_id: i32,
}

impl ::unity2::ClassIdentity for ShaderTagId {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "ShaderTagId";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ShaderTagId {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-shadertagid")]
#[::unity2::methods(value)]
impl ShaderTagId {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "get_id", args = 0)]
    pub fn get_id(self) -> i32;

    #[method(name = "set_id", args = 1)]
    pub fn set_id(self, value: i32) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::rendering::shadertagid::ShaderTagId) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        tag1: crate::unity_engine::rendering::shadertagid::ShaderTagId,
        tag2: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    ) -> bool;

    #[method(name = "op_Inequality", args = 2)]
    pub fn op_inequality(
        tag1: crate::unity_engine::rendering::shadertagid::ShaderTagId,
        tag2: crate::unity_engine::rendering::shadertagid::ShaderTagId,
    ) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
