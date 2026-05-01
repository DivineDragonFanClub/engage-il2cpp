
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/structreader/StructReader.md")))]
#[::unity2::class(namespace = "App", name = "StructReader")]
#[parent(crate::system::object::Object)]
pub struct StructReader {}

#[cfg(feature = "app-structreader")]
#[::unity2::methods]
impl StructReader {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-structreader")]
impl StructReader {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StructReader),
                ::core::stringify!(new),
            )
        });
        <Self as IStructReaderMethods>::ctor(this);
        this
    }
}
