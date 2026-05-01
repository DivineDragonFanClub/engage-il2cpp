
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/iequalitycomparer_1_interface/IEqualityComparer_1_Interface.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "IEqualityComparer`1")]
pub struct IEqualityComparer_1_Interface<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-collections-generic-iequalitycomparer_1_interface")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> IEqualityComparer_1_Interface<T0> {
    #[method(name = "Equals", args = 2)]
    pub fn equals(self, x: T0, y: T0) -> bool;

    #[method(name = "GetHashCode", args = 1)]
    pub fn get_hash_code(self, obj: T0) -> i32;
}
