
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/equalitycomparer_1/EqualityComparer_1.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "EqualityComparer`1")]
pub struct EqualityComparer_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "defaultComparer")]
    pub default_comparer:
        crate::system::collections::generic::equalitycomparer_1::EqualityComparer_1<T0>,
}

#[cfg(feature = "system-collections-generic-equalitycomparer_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> EqualityComparer_1<T0> {
    #[method(name = "get_Default", args = 0)]
    pub fn get_default(
    ) -> crate::system::collections::generic::equalitycomparer_1::EqualityComparer_1<T0>;

    #[method(name = "CreateComparer", args = 0)]
    pub fn create_comparer(
    ) -> crate::system::collections::generic::equalitycomparer_1::EqualityComparer_1<T0>;

    #[method(name = "Equals", args = 2)]
    pub fn equals(self, x: T0, y: T0) -> bool;

    #[method(name = "GetHashCode", args = 1)]
    pub fn get_hash_code(self, obj: T0) -> i32;

    #[method(name = "IndexOf", args = 4)]
    pub fn index_of(
        self,
        array: ::unity2::Array<T0>,
        value: T0,
        start_index: i32,
        count: i32,
    ) -> i32;

    #[method(name = "LastIndexOf", args = 4)]
    pub fn last_index_of(
        self,
        array: ::unity2::Array<T0>,
        value: T0,
        start_index: i32,
        count: i32,
    ) -> i32;

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

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-collections-generic-equalitycomparer_1")]
impl<T0: ::unity2::ClassIdentity> EqualityComparer_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EqualityComparer_1),
                ::core::stringify!(new),
            )
        });
        <Self as IEqualityComparer_1Methods<T0>>::ctor(this);
        this
    }
}
