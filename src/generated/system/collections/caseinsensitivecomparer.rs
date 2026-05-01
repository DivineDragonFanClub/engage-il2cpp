
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/caseinsensitivecomparer/CaseInsensitiveComparer.md")))]
#[::unity2::class(namespace = "System.Collections", name = "CaseInsensitiveComparer")]
#[parent(crate::system::object::Object)]
pub struct CaseInsensitiveComparer {}

#[cfg(feature = "system-collections-caseinsensitivecomparer")]
#[::unity2::methods]
impl CaseInsensitiveComparer {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Default", args = 0)]
    pub fn get_default(
    ) -> crate::system::collections::caseinsensitivecomparer::CaseInsensitiveComparer;

    #[method(name = "Compare", args = 2)]
    pub fn compare(self, a: crate::system::object::Object, b: crate::system::object::Object)
        -> i32;
}

#[cfg(feature = "system-collections-caseinsensitivecomparer")]
impl CaseInsensitiveComparer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CaseInsensitiveComparer),
                ::core::stringify!(new),
            )
        });
        <Self as ICaseInsensitiveComparerMethods>::ctor(this);
        this
    }
}
