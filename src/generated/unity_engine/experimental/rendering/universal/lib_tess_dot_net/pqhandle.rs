
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lib_tess_dot_net/pqhandle/PQHandle.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct PQHandle {
    pub handle: i32,
}

impl ::unity2::ClassIdentity for PQHandle {
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering.Universal.LibTessDotNet";

    const NAME: &'static str = "PQHandle";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for PQHandle {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-pqhandle")]
#[::unity2::methods(value)]
impl PQHandle {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
