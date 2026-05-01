
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/reflectiontypeloadexception/ReflectionTypeLoadException.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "ReflectionTypeLoadException")]
pub struct ReflectionTypeLoadException {
    #[rename(name = "_classes")]
    pub classes: ::unity2::Array<::unity2::SystemType>,
}

#[cfg(feature = "system-reflection-reflectiontypeloadexception")]
#[::unity2::methods]
impl ReflectionTypeLoadException {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-reflectiontypeloadexception")]
impl ReflectionTypeLoadException {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReflectionTypeLoadException),
                ::core::stringify!(new),
            )
        });
        <Self as IReflectionTypeLoadExceptionMethods>::ctor(this);
        this
    }
}
