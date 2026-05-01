
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/xr/meshid/MeshId.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct MeshId {
    pub m_sub_id1: u64,
    pub m_sub_id2: u64,
}

impl ::unity2::ClassIdentity for MeshId {
    const NAMESPACE: &'static str = "UnityEngine.XR";

    const NAME: &'static str = "MeshId";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MeshId {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-xr-meshid")]
#[::unity2::methods(value)]
impl MeshId {
    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::xr::meshid::MeshId) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
