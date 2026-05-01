
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/prefetchedsignalexmethods/PrefetchedSignalExMethods.md")))]
#[::unity2::class(namespace = "Combat", name = "PrefetchedSignalExMethods")]
#[parent(crate::system::object::Object)]
pub struct PrefetchedSignalExMethods {}

#[cfg(feature = "combat-prefetchedsignalexmethods")]
#[::unity2::methods]
impl PrefetchedSignalExMethods {
    #[method(name = "IsNull", args = 1)]
    pub fn is_null(a: crate::combat::prefetchedsignal::PrefetchedSignal) -> bool;

    #[method(name = "IsNotNull", args = 1)]
    pub fn is_not_null(a: crate::combat::prefetchedsignal::PrefetchedSignal) -> bool;
}
