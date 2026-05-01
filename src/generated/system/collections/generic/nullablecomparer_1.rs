
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/nullablecomparer_1/NullableComparer_1.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "NullableComparer`1")]
pub struct NullableComparer_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-collections-generic-nullablecomparer_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> NullableComparer_1<T0> {
    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-collections-generic-nullablecomparer_1")]
impl<T0: ::unity2::ClassIdentity> NullableComparer_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NullableComparer_1),
                ::core::stringify!(new),
            )
        });
        <Self as INullableComparer_1Methods<T0>>::ctor(this);
        this
    }
}
