
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/anonwrapper/AnonWrapper.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Interop", name = "AnonWrapper")]
#[parent(crate::system::object::Object)]
pub struct AnonWrapper {}

#[cfg(feature = "moon_sharp-interpreter-interop-anonwrapper")]
#[::unity2::methods]
impl AnonWrapper {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-interop-anonwrapper")]
impl AnonWrapper {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnonWrapper),
                ::core::stringify!(new),
            )
        });
        <Self as IAnonWrapperMethods>::ctor(this);
        this
    }
}
