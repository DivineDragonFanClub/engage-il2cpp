
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/flare/Flare.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "Flare")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct Flare {}

#[cfg(feature = "unity_engine-flare")]
#[::unity2::methods]
impl Flare {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Internal_Create", args = 1)]
    pub fn internal_create(self_: crate::unity_engine::flare::Flare) -> ();
}

#[cfg(feature = "unity_engine-flare")]
impl Flare {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Flare),
                ::core::stringify!(new),
            )
        });
        <Self as IFlareMethods>::ctor(this);
        this
    }
}
