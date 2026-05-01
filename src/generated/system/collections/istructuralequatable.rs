
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/istructuralequatable/IStructuralEquatable.md")))]
#[::unity2::class(namespace = "System.Collections", name = "IStructuralEquatable")]
pub struct IStructuralEquatable {}

#[cfg(feature = "system-collections-istructuralequatable")]
#[::unity2::methods]
impl IStructuralEquatable {
    #[method(name = "Equals", args = 2)]
    pub fn equals(
        self,
        other: crate::system::object::Object,
        comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> bool;

    #[method(name = "GetHashCode", args = 1)]
    pub fn get_hash_code(
        self,
        comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> i32;
}
