
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/perf/Perf.md")))]
#[::unity2::class(namespace = "App", name = "Perf")]
#[parent(crate::system::object::Object)]
pub struct Perf {}

#[cfg(feature = "app-perf")]
#[::unity2::methods]
impl Perf {
    #[method(name = "Begin", args = 1)]
    pub fn begin(name: ::unity2::Il2CppString) -> ();

    #[method(name = "End", args = 1)]
    pub fn end(name: ::unity2::Il2CppString) -> ();

    #[method(name = "Update", args = 1)]
    pub fn update(is_remove: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-perf")]
impl Perf {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Perf),
                ::core::stringify!(new),
            )
        });
        <Self as IPerfMethods>::ctor(this);
        this
    }
}
