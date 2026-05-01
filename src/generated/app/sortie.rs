
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortie/Sortie.md")))]
#[::unity2::class(namespace = "App", name = "Sortie")]
#[parent(crate::system::object::Object)]
pub struct Sortie {}

#[cfg(feature = "app-sortie")]
#[::unity2::methods]
impl Sortie {
    #[method(name = "Setup", args = 0)]
    pub fn setup() -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortie")]
impl Sortie {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Sortie),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieMethods>::ctor(this);
        this
    }
}
