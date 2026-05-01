
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/cachedassetbundle/CachedAssetBundle.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct CachedAssetBundle {
    pub m_name: ::unity2::Il2CppString,
    pub m_hash: crate::unity_engine::hash128::Hash128,
}

impl ::unity2::ClassIdentity for CachedAssetBundle {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "CachedAssetBundle";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for CachedAssetBundle {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-cachedassetbundle")]
#[::unity2::methods(value)]
impl CachedAssetBundle {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        name: ::unity2::Il2CppString,
        hash: crate::unity_engine::hash128::Hash128,
    ) -> ();

    #[method(name = "get_name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_hash", args = 0)]
    pub fn get_hash(self) -> crate::unity_engine::hash128::Hash128;
}
