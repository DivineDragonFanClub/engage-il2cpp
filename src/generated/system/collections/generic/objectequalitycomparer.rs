
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/objectequalitycomparer/ObjectEqualityComparer.md")))]
#[::unity2::class(
    namespace = "System.Collections.Generic",
    name = "ObjectEqualityComparer"
)]
#[parent(crate::system::object::Object)]
pub struct ObjectEqualityComparer {
    #[static_field]
    #[rename(name = "Default")]
    pub default:
        crate::system::collections::generic::objectequalitycomparer::ObjectEqualityComparer,
}

#[cfg(feature = "system-collections-generic-objectequalitycomparer")]
#[::unity2::methods]
impl ObjectEqualityComparer {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "System.Collections.IEqualityComparer.GetHashCode", args = 1)]
    pub fn system_collections_i_equality_comparer_get_hash_code(
        self,
        obj: crate::system::object::Object,
    ) -> i32;

    #[method(name = "System.Collections.IEqualityComparer.Equals", args = 2)]
    pub fn system_collections_i_equality_comparer_equals(
        self,
        x: crate::system::object::Object,
        y: crate::system::object::Object,
    ) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "system-collections-generic-objectequalitycomparer")]
impl ObjectEqualityComparer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ObjectEqualityComparer),
                ::core::stringify!(new),
            )
        });
        <Self as IObjectEqualityComparerMethods>::ctor(this);
        this
    }
}
