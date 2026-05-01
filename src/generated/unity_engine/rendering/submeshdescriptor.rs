
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/submeshdescriptor/SubMeshDescriptor.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct SubMeshDescriptor {}

impl ::unity2::ClassIdentity for SubMeshDescriptor {
    const NAMESPACE: &'static str = "UnityEngine.Rendering";

    const NAME: &'static str = "SubMeshDescriptor";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SubMeshDescriptor {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-rendering-submeshdescriptor")]
#[::unity2::methods(value)]
impl SubMeshDescriptor {
    #[method(name = "get_bounds", args = 0)]
    pub fn get_bounds(self) -> crate::unity_engine::bounds::Bounds;

    #[method(name = "get_topology", args = 0)]
    pub fn get_topology(self) -> crate::unity_engine::meshtopology::MeshTopology;

    #[method(name = "get_indexStart", args = 0)]
    pub fn get_index_start(self) -> i32;

    #[method(name = "get_indexCount", args = 0)]
    pub fn get_index_count(self) -> i32;

    #[method(name = "get_baseVertex", args = 0)]
    pub fn get_base_vertex(self) -> i32;

    #[method(name = "get_firstVertex", args = 0)]
    pub fn get_first_vertex(self) -> i32;

    #[method(name = "get_vertexCount", args = 0)]
    pub fn get_vertex_count(self) -> i32;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}
