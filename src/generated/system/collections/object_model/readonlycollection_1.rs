
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/object_model/readonlycollection_1/ReadOnlyCollection_1.md")))]
#[::unity2::class(
    namespace = "System.Collections.ObjectModel",
    name = "ReadOnlyCollection`1"
)]
pub struct ReadOnlyCollection_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "list")]
    pub list: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<T0>,
    #[rename(name = "_syncRoot")]
    pub sync_root: ::unity2::IlInstance,
}

#[cfg(feature = "system-collections-object_model-readonlycollection_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> ReadOnlyCollection_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        list: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<T0>,
    ) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> T0;

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, value: T0) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::Array<T0>, index: i32) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::generic::ienumerator_1::IEnumerator_1<T0>;

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of(self, value: T0) -> i32;

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

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

    #[method(name = "System.Collections.IList.get_IsReadOnly", args = 0)]
    pub fn system_collections_i_list_get_is_read_only(self) -> bool;

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

    #[method(name = "System.Collections.IList.Clear", args = 0)]
    pub fn system_collections_i_list_clear(self) -> ();

    #[method(name = "IsCompatibleObject", args = 1)]
    pub fn is_compatible_object(value: crate::system::object::Object) -> bool;

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

    #[method(name = "System.Collections.IList.RemoveAt", args = 1)]
    pub fn system_collections_i_list_remove_at(self, index: i32) -> ();
}

#[cfg(feature = "system-collections-object_model-readonlycollection_1")]
impl<T0: ::unity2::ClassIdentity> ReadOnlyCollection_1<T0> {
    pub fn new(
        list: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<T0>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReadOnlyCollection_1),
                ::core::stringify!(new),
            )
        });
        <Self as IReadOnlyCollection_1Methods<T0>>::ctor(this, list);
        this
    }
}
