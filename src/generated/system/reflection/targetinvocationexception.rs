
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/targetinvocationexception/TargetInvocationException.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "TargetInvocationException")]
pub struct TargetInvocationException {}

#[cfg(feature = "system-reflection-targetinvocationexception")]
#[::unity2::methods]
impl TargetInvocationException {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-targetinvocationexception")]
impl TargetInvocationException {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TargetInvocationException),
                ::core::stringify!(new),
            )
        });
        <Self as ITargetInvocationExceptionMethods>::ctor(this);
        this
    }
}
