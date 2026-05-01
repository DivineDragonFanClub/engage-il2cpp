
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/icollection_1/ICollection_1.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "ICollection`1")]
pub struct ICollection_1<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "system-collections-generic-icollection_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> ICollection_1<T0> {
    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "Add", args = 1)]
    pub fn add(self, item: T0) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, item: T0) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::Array<T0>, array_index: i32) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, item: T0) -> bool;
}
