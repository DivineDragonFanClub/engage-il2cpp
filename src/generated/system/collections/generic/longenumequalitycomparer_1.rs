
use crate::system::collections::generic::equalitycomparer_1::EqualityComparer_1;
use crate::system::collections::generic::equalitycomparer_1::IEqualityComparer_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/longenumequalitycomparer_1/LongEnumEqualityComparer_1.md")))]
#[::unity2::class(
    namespace = "System.Collections.Generic",
    name = "LongEnumEqualityComparer`1"
)]
pub struct LongEnumEqualityComparer_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-collections-generic-longenumequalitycomparer_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> LongEnumEqualityComparer_1<T0> {
    #[method(name = "Equals", args = 2)]
    pub fn equals(self, x: T0, y: T0) -> bool;

    #[method(name = "GetHashCode", args = 1)]
    pub fn get_hash_code(self, obj: T0) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code_2(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-collections-generic-longenumequalitycomparer_1")]
impl<T0: ::unity2::ClassIdentity> LongEnumEqualityComparer_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LongEnumEqualityComparer_1),
                ::core::stringify!(new),
            )
        });
        <Self as ILongEnumEqualityComparer_1Methods<T0>>::ctor(this);
        this
    }
}
