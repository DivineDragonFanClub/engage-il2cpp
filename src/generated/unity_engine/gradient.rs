
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/gradient/Gradient.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Gradient")]
#[parent(crate::system::object::Object)]
pub struct Gradient {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
}

#[cfg(feature = "unity_engine-gradient")]
#[::unity2::methods]
impl Gradient {
    #[method(name = "Init", args = 0)]
    pub fn init() -> ::unity2::IntPtr;

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = "Internal_Equals", args = 1)]
    pub fn internal_equals(self, other: ::unity2::IntPtr) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, o: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, other: crate::unity_engine::gradient::Gradient) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;
}

#[cfg(feature = "unity_engine-gradient")]
impl Gradient {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Gradient),
                ::core::stringify!(new),
            )
        });
        <Self as IGradientMethods>::ctor(this);
        this
    }
}
