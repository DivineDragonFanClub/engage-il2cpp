
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/selectionbaseattribute/SelectionBaseAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "SelectionBaseAttribute")]
pub struct SelectionBaseAttribute {}

#[cfg(feature = "unity_engine-selectionbaseattribute")]
#[::unity2::methods]
impl SelectionBaseAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-selectionbaseattribute")]
impl SelectionBaseAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SelectionBaseAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as ISelectionBaseAttributeMethods>::ctor(this);
        this
    }
}
