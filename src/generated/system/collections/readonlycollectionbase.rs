
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/readonlycollectionbase/ReadOnlyCollectionBase.md")))]
#[::unity2::class(namespace = "System.Collections", name = "ReadOnlyCollectionBase")]
#[parent(crate::system::object::Object)]
pub struct ReadOnlyCollectionBase {
    #[rename(name = "list")]
    pub list: crate::system::collections::arraylist::ArrayList,
}

#[cfg(feature = "system-collections-readonlycollectionbase")]
#[::unity2::methods]
impl ReadOnlyCollectionBase {
    #[method(name = "get_InnerList", args = 0)]
    pub fn get_inner_list(self) -> crate::system::collections::arraylist::ArrayList;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "System.Collections.ICollection.get_IsSynchronized", args = 0)]
    pub fn system_collections_i_collection_get_is_synchronized(self) -> bool;

    #[method(name = "System.Collections.ICollection.get_SyncRoot", args = 0)]
    pub fn system_collections_i_collection_get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "System.Collections.ICollection.CopyTo", args = 2)]
    pub fn system_collections_i_collection_copy_to(
        self,
        array: ::unity2::IlInstance,
        index: i32,
    ) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-collections-readonlycollectionbase")]
impl ReadOnlyCollectionBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReadOnlyCollectionBase),
                ::core::stringify!(new),
            )
        });
        <Self as IReadOnlyCollectionBaseMethods>::ctor(this);
        this
    }
}
