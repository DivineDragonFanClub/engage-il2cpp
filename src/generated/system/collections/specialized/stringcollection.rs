
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/specialized/stringcollection/StringCollection.md")))]
#[::unity2::class(
    namespace = "System.Collections.Specialized",
    name = "StringCollection"
)]
#[parent(crate::system::object::Object)]
pub struct StringCollection {
    #[rename(name = "data")]
    pub data: crate::system::collections::arraylist::ArrayList,
}

#[cfg(feature = "system-collections-specialized-stringcollection")]
#[::unity2::methods]
impl StringCollection {
    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "System.Collections.IList.get_IsReadOnly", args = 0)]
    pub fn system_collections_i_list_get_is_read_only(self) -> bool;

    #[method(name = "Add", args = 1)]
    pub fn add(self, value: ::unity2::Il2CppString) -> i32;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, value: ::unity2::Il2CppString) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::Array<::unity2::Il2CppString>, index: i32) -> ();

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of(self, value: ::unity2::Il2CppString) -> i32;

    #[method(name = "Insert", args = 2)]
    pub fn insert(self, index: i32, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsSynchronized", args = 0)]
    pub fn get_is_synchronized(self) -> bool;

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "RemoveAt", args = 1)]
    pub fn remove_at(self, index: i32) -> ();

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "System.Collections.IList.get_Item", args = 1)]
    pub fn system_collections_i_list_get_item(self, index: i32) -> crate::system::object::Object;

    #[method(name = "System.Collections.IList.set_Item", args = 2)]
    pub fn system_collections_i_list_set_item(
        self,
        index: i32,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "System.Collections.IList.Add", args = 1)]
    pub fn system_collections_i_list_add(self, value: crate::system::object::Object) -> i32;

    #[method(name = "System.Collections.IList.Contains", args = 1)]
    pub fn system_collections_i_list_contains(self, value: crate::system::object::Object) -> bool;

    #[method(name = "System.Collections.IList.IndexOf", args = 1)]
    pub fn system_collections_i_list_index_of(self, value: crate::system::object::Object) -> i32;

    #[method(name = "System.Collections.IList.Insert", args = 2)]
    pub fn system_collections_i_list_insert(
        self,
        index: i32,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "System.Collections.IList.Remove", args = 1)]
    pub fn system_collections_i_list_remove(self, value: crate::system::object::Object) -> ();

    #[method(name = "System.Collections.ICollection.CopyTo", args = 2)]
    pub fn system_collections_i_collection_copy_to(
        self,
        array: ::unity2::IlInstance,
        index: i32,
    ) -> ();

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-collections-specialized-stringcollection")]
impl StringCollection {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StringCollection),
                ::core::stringify!(new),
            )
        });
        <Self as IStringCollectionMethods>::ctor(this);
        this
    }
}
