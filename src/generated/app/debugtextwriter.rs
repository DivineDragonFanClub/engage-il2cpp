
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/debugtextwriter/DebugTextWriter.md")))]
#[::unity2::class(namespace = "App", name = "DebugTextWriter")]
#[parent(crate::system::object::Object)]
pub struct DebugTextWriter {}

#[cfg(feature = "app-debugtextwriter")]
#[::unity2::methods]
impl DebugTextWriter {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "Write", args = 1)]
    pub fn write(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "WriteLine", args = 1)]
    pub fn write_line(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "WriteFormat", args = 2)]
    pub fn write_format(
        self,
        text: ::unity2::Il2CppString,
        objects: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "Save", args = 1)]
    pub fn save(self, name: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-debugtextwriter")]
impl DebugTextWriter {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugTextWriter),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugTextWriterMethods>::ctor(this);
        this
    }

    pub fn new_2(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugTextWriter),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDebugTextWriterMethods>::ctor_2(this, name);
        this
    }
}
