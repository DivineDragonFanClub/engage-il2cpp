
use crate::system::collections::generic::equalitycomparer_1::EqualityComparer_1;
use crate::system::collections::generic::equalitycomparer_1::IEqualityComparer_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/internalstringcomparer/InternalStringComparer.md")))]
#[::unity2::class(
    namespace = "System.Collections.Generic",
    name = "InternalStringComparer"
)]
# [parent (crate :: system :: collections :: generic :: equalitycomparer_1 :: EqualityComparer_1 < :: unity2 :: Il2CppString >)]
pub struct InternalStringComparer {}

#[cfg(feature = "system-collections-generic-internalstringcomparer")]
#[::unity2::methods]
impl InternalStringComparer {
    #[method(name = "GetHashCode", args = 1)]
    pub fn get_hash_code(self, obj: ::unity2::Il2CppString) -> i32;

    #[method(name = "Equals", args = 2)]
    pub fn equals(self, x: ::unity2::Il2CppString, y: ::unity2::Il2CppString) -> bool;

    #[method(name = "IndexOf", args = 4)]
    pub fn index_of(
        self,
        array: ::unity2::Array<::unity2::Il2CppString>,
        value: ::unity2::Il2CppString,
        start_index: i32,
        count: i32,
    ) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-collections-generic-internalstringcomparer")]
impl InternalStringComparer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InternalStringComparer),
                ::core::stringify!(new),
            )
        });
        <Self as IInternalStringComparerMethods>::ctor(this);
        this
    }
}
