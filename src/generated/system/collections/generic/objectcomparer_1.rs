
use crate::system::collections::generic::comparer_1::Comparer_1;
use crate::system::collections::generic::comparer_1::IComparer_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/objectcomparer_1/ObjectComparer_1.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "ObjectComparer`1")]
pub struct ObjectComparer_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-collections-generic-objectcomparer_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> ObjectComparer_1<T0> {
    #[method(name = "Compare", args = 2)]
    pub fn compare(self, x: T0, y: T0) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-collections-generic-objectcomparer_1")]
impl<T0: ::unity2::ClassIdentity> ObjectComparer_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ObjectComparer_1),
                ::core::stringify!(new),
            )
        });
        <Self as IObjectComparer_1Methods<T0>>::ctor(this);
        this
    }
}
