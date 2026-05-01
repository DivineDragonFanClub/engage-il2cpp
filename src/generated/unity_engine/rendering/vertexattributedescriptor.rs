
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/vertexattributedescriptor/VertexAttributeDescriptor.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct VertexAttributeDescriptor {}

impl ::unity2::ClassIdentity for VertexAttributeDescriptor {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "VertexAttributeDescriptor";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for VertexAttributeDescriptor {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-vertexattributedescriptor")]
#[::unity2::methods(value)]
impl VertexAttributeDescriptor {
    #[method(name = "get_attribute", args = 0)]
    pub fn get_attribute(self) -> crate::unity_engine::rendering::vertexattribute::VertexAttribute;

    #[method(name = "set_attribute", args = 1)]
    pub fn set_attribute(
        self,
        value: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
    ) -> ();

    #[method(name = "get_format", args = 0)]
    pub fn get_format(
        self,
    ) -> crate::unity_engine::rendering::vertexattributeformat::VertexAttributeFormat;

    #[method(name = "set_format", args = 1)]
    pub fn set_format(
        self,
        value: crate::unity_engine::rendering::vertexattributeformat::VertexAttributeFormat,
    ) -> ();

    #[method(name = "get_dimension", args = 0)]
    pub fn get_dimension(self) -> i32;

    #[method(name = "set_dimension", args = 1)]
    pub fn set_dimension(self, value: i32) -> ();

    #[method(name = "get_stream", args = 0)]
    pub fn get_stream(self) -> i32;

    #[method(name = "set_stream", args = 1)]
    pub fn set_stream(self, value: i32) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        attribute: crate::unity_engine::rendering::vertexattribute::VertexAttribute,
        format: crate::unity_engine::rendering::vertexattributeformat::VertexAttributeFormat,
        dimension: i32,
        stream: i32,
    ) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, other: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(
        self,
        other: crate::unity_engine::rendering::vertexattributedescriptor::VertexAttributeDescriptor,
    ) -> bool;
}
