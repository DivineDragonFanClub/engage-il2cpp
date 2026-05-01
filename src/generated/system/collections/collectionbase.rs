
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/collectionbase/CollectionBase.md")))]
#[::unity2::class(namespace = "System.Collections", name = "CollectionBase")]
#[parent(crate::system::object::Object)]
pub struct CollectionBase {
    #[rename(name = "list")]
    pub list: crate::system::collections::arraylist::ArrayList,
}

#[cfg(feature = "system-collections-collectionbase")]
#[::unity2::methods]
impl CollectionBase {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_InnerList", args = 0)]
    pub fn get_inner_list(self) -> crate::system::collections::arraylist::ArrayList;

    #[method(name = "get_List", args = 0)]
    pub fn get_list(self) -> crate::system::collections::ilist::IList;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "RemoveAt", args = 1)]
    pub fn remove_at(self, index: i32) -> ();

    #[method(name = "System.Collections.IList.get_IsReadOnly", args = 0)]
    pub fn system_collections_i_list_get_is_read_only(self) -> bool;

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

    #[method(name = "System.Collections.IList.get_Item", args = 1)]
    pub fn system_collections_i_list_get_item(self, index: i32) -> crate::system::object::Object;

    #[method(name = "System.Collections.IList.set_Item", args = 2)]
    pub fn system_collections_i_list_set_item(
        self,
        index: i32,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "System.Collections.IList.Contains", args = 1)]
    pub fn system_collections_i_list_contains(self, value: crate::system::object::Object) -> bool;

    #[method(name = "System.Collections.IList.Add", args = 1)]
    pub fn system_collections_i_list_add(self, value: crate::system::object::Object) -> i32;

    #[method(name = "System.Collections.IList.Remove", args = 1)]
    pub fn system_collections_i_list_remove(self, value: crate::system::object::Object) -> ();

    #[method(name = "System.Collections.IList.IndexOf", args = 1)]
    pub fn system_collections_i_list_index_of(self, value: crate::system::object::Object) -> i32;

    #[method(name = "System.Collections.IList.Insert", args = 2)]
    pub fn system_collections_i_list_insert(
        self,
        index: i32,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "OnSet", args = 3)]
    pub fn on_set(
        self,
        index: i32,
        old_value: crate::system::object::Object,
        new_value: crate::system::object::Object,
    ) -> ();

    #[method(name = "OnInsert", args = 2)]
    pub fn on_insert(self, index: i32, value: crate::system::object::Object) -> ();

    #[method(name = "OnClear", args = 0)]
    pub fn on_clear(self) -> ();

    #[method(name = "OnRemove", args = 2)]
    pub fn on_remove(self, index: i32, value: crate::system::object::Object) -> ();

    #[method(name = "OnValidate", args = 1)]
    pub fn on_validate(self, value: crate::system::object::Object) -> ();

    #[method(name = "OnSetComplete", args = 3)]
    pub fn on_set_complete(
        self,
        index: i32,
        old_value: crate::system::object::Object,
        new_value: crate::system::object::Object,
    ) -> ();

    #[method(name = "OnInsertComplete", args = 2)]
    pub fn on_insert_complete(self, index: i32, value: crate::system::object::Object) -> ();

    #[method(name = "OnClearComplete", args = 0)]
    pub fn on_clear_complete(self) -> ();

    #[method(name = "OnRemoveComplete", args = 2)]
    pub fn on_remove_complete(self, index: i32, value: crate::system::object::Object) -> ();
}

#[cfg(feature = "system-collections-collectionbase")]
impl CollectionBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CollectionBase),
                ::core::stringify!(new),
            )
        });
        <Self as ICollectionBaseMethods>::ctor(this);
        this
    }
}
