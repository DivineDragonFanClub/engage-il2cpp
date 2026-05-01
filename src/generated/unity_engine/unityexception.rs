
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/unityexception/UnityException.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "UnityException")]
pub struct UnityException {}

#[cfg(feature = "unity_engine-unityexception")]
#[::unity2::methods]
impl UnityException {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, message: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "unity_engine-unityexception")]
impl UnityException {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnityException),
                ::core::stringify!(new),
            )
        });
        <Self as IUnityExceptionMethods>::ctor(this);
        this
    }

    pub fn new_2(message: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnityException),
                ::core::stringify!(new_2),
            )
        });
        <Self as IUnityExceptionMethods>::ctor_2(this, message);
        this
    }
}
