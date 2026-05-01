
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/lowlevelcomparer/LowLevelComparer.md")))]
#[::unity2::class(namespace = "System.Collections", name = "LowLevelComparer")]
#[parent(crate::system::object::Object)]
pub struct LowLevelComparer {
    #[static_field]
    #[rename(name = "Default")]
    pub default: crate::system::collections::lowlevelcomparer::LowLevelComparer,
}

#[cfg(feature = "system-collections-lowlevelcomparer")]
#[::unity2::methods]
impl LowLevelComparer {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Compare", args = 2)]
    pub fn compare(self, a: crate::system::object::Object, b: crate::system::object::Object)
        -> i32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "system-collections-lowlevelcomparer")]
impl LowLevelComparer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LowLevelComparer),
                ::core::stringify!(new),
            )
        });
        <Self as ILowLevelComparerMethods>::ctor(this);
        this
    }
}
