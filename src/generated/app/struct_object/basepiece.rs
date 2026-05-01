
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/struct_object/basepiece/BasePiece.md")))]
#[::unity2::class(namespace = "App.StructObject", name = "BasePiece")]
#[parent(crate::system::object::Object)]
pub struct BasePiece {}

#[cfg(feature = "app-struct_object-basepiece")]
#[::unity2::methods]
impl BasePiece {
    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-struct_object-basepiece")]
impl BasePiece {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BasePiece),
                ::core::stringify!(new),
            )
        });
        <Self as IBasePieceMethods>::ctor(this);
        this
    }
}
