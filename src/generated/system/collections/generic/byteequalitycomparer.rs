
use crate::system::collections::generic::equalitycomparer_1::EqualityComparer_1;
use crate::system::collections::generic::equalitycomparer_1::IEqualityComparer_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/byteequalitycomparer/ByteEqualityComparer.md")))]
#[::unity2::class(
    namespace = "System.Collections.Generic",
    name = "ByteEqualityComparer"
)]
# [parent (crate :: system :: collections :: generic :: equalitycomparer_1 :: EqualityComparer_1 < u8 >)]
pub struct ByteEqualityComparer {}

#[cfg(feature = "system-collections-generic-byteequalitycomparer")]
#[::unity2::methods]
impl ByteEqualityComparer {
    #[method(name = "Equals", args = 2)]
    pub fn equals(self, x: u8, y: u8) -> bool;

    #[method(name = "GetHashCode", args = 1)]
    pub fn get_hash_code(self, b: u8) -> i32;

    #[method(name = "IndexOf", args = 4)]
    pub fn index_of(
        self,
        array: ::unity2::Array<u8>,
        value: u8,
        start_index: i32,
        count: i32,
    ) -> i32;

    #[method(name = "LastIndexOf", args = 4)]
    pub fn last_index_of(
        self,
        array: ::unity2::Array<u8>,
        value: u8,
        start_index: i32,
        count: i32,
    ) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code_2(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-collections-generic-byteequalitycomparer")]
impl ByteEqualityComparer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ByteEqualityComparer),
                ::core::stringify!(new),
            )
        });
        <Self as IByteEqualityComparerMethods>::ctor(this);
        this
    }
}
