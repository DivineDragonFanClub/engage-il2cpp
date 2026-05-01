
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/globaljavaobjectref/GlobalJavaObjectRef.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GlobalJavaObjectRef")]
#[parent(crate::system::object::Object)]
pub struct GlobalJavaObjectRef {
    #[rename(name = "m_disposed")]
    pub m_disposed: bool,
    #[rename(name = "m_jobject")]
    pub m_jobject: ::unity2::IntPtr,
}

#[cfg(feature = "unity_engine-globaljavaobjectref")]
#[::unity2::methods]
impl GlobalJavaObjectRef {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, jobject: ::unity2::IntPtr) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        obj: crate::unity_engine::globaljavaobjectref::GlobalJavaObjectRef,
    ) -> ::unity2::IntPtr;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg(feature = "unity_engine-globaljavaobjectref")]
impl GlobalJavaObjectRef {
    pub fn new(jobject: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GlobalJavaObjectRef),
                ::core::stringify!(new),
            )
        });
        <Self as IGlobalJavaObjectRefMethods>::ctor(this, jobject);
        this
    }
}
