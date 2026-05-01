
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/icollection/ICollection.md")))]
#[::unity2::class(namespace = "System.Collections", name = "ICollection")]
pub struct ICollection {}

#[cfg(feature = "system-collections-icollection")]
#[::unity2::methods]
impl ICollection {
    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, index: i32) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "get_IsSynchronized", args = 0)]
    pub fn get_is_synchronized(self) -> bool;
}
