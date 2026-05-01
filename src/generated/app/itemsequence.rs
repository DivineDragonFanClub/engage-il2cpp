
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/itemsequence/ItemSequence.md")))]
#[::unity2::class(namespace = "App", name = "ItemSequence")]
#[parent(crate::system::object::Object)]
pub struct ItemSequence {}

#[cfg(feature = "app-itemsequence")]
#[::unity2::methods]
impl ItemSequence {
    #[method(name = "SetOffset", args = 1)]
    pub fn set_offset(msg: crate::app::gamemessage::GameMessage) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-itemsequence")]
impl ItemSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ItemSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IItemSequenceMethods>::ctor(this);
        this
    }
}
