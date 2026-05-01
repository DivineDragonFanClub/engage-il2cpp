
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/comparer_1/Comparer_1.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "Comparer`1")]
pub struct Comparer_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "defaultComparer")]
    pub default_comparer: crate::system::collections::generic::comparer_1::Comparer_1<T0>,
}

#[cfg(feature = "system-collections-generic-comparer_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> Comparer_1<T0> {
    #[method(name = "get_Default", args = 0)]
    pub fn get_default() -> crate::system::collections::generic::comparer_1::Comparer_1<T0>;

    #[method(name = "CreateComparer", args = 0)]
    pub fn create_comparer() -> crate::system::collections::generic::comparer_1::Comparer_1<T0>;

    #[method(name = "Compare", args = 2)]
    pub fn compare(self, x: T0, y: T0) -> i32;

    #[method(name = "System.Collections.IComparer.Compare", args = 2)]
    pub fn system_collections_i_comparer_compare(
        self,
        x: crate::system::object::Object,
        y: crate::system::object::Object,
    ) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-collections-generic-comparer_1")]
impl<T0: ::unity2::ClassIdentity> Comparer_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Comparer_1),
                ::core::stringify!(new),
            )
        });
        <Self as IComparer_1Methods<T0>>::ctor(this);
        this
    }
}
