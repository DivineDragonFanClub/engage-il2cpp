
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/sortedlist/SortedList_SortedListDebugView.md")))]
#[::unity2::class(
    namespace = "System.Collections",
    name = "SortedList.SortedListDebugView"
)]
#[parent(crate::system::object::Object)]
pub struct SortedList_SortedListDebugView {}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/sortedlist/SortedList_SyncSortedList.md")))]
#[::unity2::class(namespace = "System.Collections", name = "SortedList.SyncSortedList")]
#[parent(crate::system::collections::sortedlist::SortedList)]
pub struct SortedList_SyncSortedList {
    #[rename(name = "_list")]
    pub list: crate::system::collections::sortedlist::SortedList,
    #[rename(name = "_root")]
    pub root: ::unity2::IlInstance,
}

#[cfg(feature = "system-collections-sortedlist")]
#[::unity2::methods]
impl SortedList_SyncSortedList {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, list: crate::system::collections::sortedlist::SortedList) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "get_IsFixedSize", args = 0)]
    pub fn get_is_fixed_size(self) -> bool;

    #[method(name = "get_IsSynchronized", args = 0)]
    pub fn get_is_synchronized(self) -> bool;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, key: crate::system::object::Object) -> crate::system::object::Object;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "get_Capacity", args = 0)]
    pub fn get_capacity(self) -> i32;

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, key: crate::system::object::Object) -> bool;

    #[method(name = "ContainsKey", args = 1)]
    pub fn contains_key(self, key: crate::system::object::Object) -> bool;

    #[method(name = "ContainsValue", args = 1)]
    pub fn contains_value(self, key: crate::system::object::Object) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, index: i32) -> ();

    #[method(name = "GetByIndex", args = 1)]
    pub fn get_by_index(self, index: i32) -> crate::system::object::Object;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::idictionaryenumerator::IDictionaryEnumerator;

    #[method(name = "GetKey", args = 1)]
    pub fn get_key(self, index: i32) -> crate::system::object::Object;

    #[method(name = "GetKeyList", args = 0)]
    pub fn get_key_list(self) -> crate::system::collections::ilist::IList;

    #[method(name = "GetValueList", args = 0)]
    pub fn get_value_list(self) -> crate::system::collections::ilist::IList;

    #[method(name = "IndexOfKey", args = 1)]
    pub fn index_of_key(self, key: crate::system::object::Object) -> i32;

    #[method(name = "IndexOfValue", args = 1)]
    pub fn index_of_value(self, value: crate::system::object::Object) -> i32;

    #[method(name = "RemoveAt", args = 1)]
    pub fn remove_at(self, index: i32) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, key: crate::system::object::Object) -> ();
}

#[cfg(feature = "system-collections-sortedlist")]
impl SortedList_SyncSortedList {
    pub fn new(list: crate::system::collections::sortedlist::SortedList) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortedList_SyncSortedList),
                ::core::stringify!(new),
            )
        });
        <Self as ISortedList_SyncSortedListMethods>::ctor(this, list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/sortedlist/SortedList_KeyList.md")))]
#[::unity2::class(namespace = "System.Collections", name = "SortedList.KeyList")]
#[parent(crate::system::object::Object)]
pub struct SortedList_KeyList {
    #[rename(name = "sortedList")]
    pub sorted_list: crate::system::collections::sortedlist::SortedList,
}

#[cfg(feature = "system-collections-sortedlist")]
#[::unity2::methods]
impl SortedList_KeyList {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, sorted_list: crate::system::collections::sortedlist::SortedList) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "get_IsSynchronized", args = 0)]
    pub fn get_is_synchronized(self) -> bool;

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "Add", args = 1)]
    pub fn add(self, key: crate::system::object::Object) -> i32;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, key: crate::system::object::Object) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, array_index: i32) -> ();

    #[method(name = "Insert", args = 2)]
    pub fn insert(self, index: i32, value: crate::system::object::Object) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> crate::system::object::Object;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: crate::system::object::Object) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of(self, key: crate::system::object::Object) -> i32;

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, key: crate::system::object::Object) -> ();

    #[method(name = "RemoveAt", args = 1)]
    pub fn remove_at(self, index: i32) -> ();
}

#[cfg(feature = "system-collections-sortedlist")]
impl SortedList_KeyList {
    pub fn new(sorted_list: crate::system::collections::sortedlist::SortedList) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortedList_KeyList),
                ::core::stringify!(new),
            )
        });
        <Self as ISortedList_KeyListMethods>::ctor(this, sorted_list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/sortedlist/SortedList_SortedListEnumerator.md")))]
#[::unity2::class(
    namespace = "System.Collections",
    name = "SortedList.SortedListEnumerator"
)]
#[parent(crate::system::object::Object)]
pub struct SortedList_SortedListEnumerator {
    #[rename(name = "sortedList")]
    pub sorted_list: crate::system::collections::sortedlist::SortedList,
    #[rename(name = "key")]
    pub key: ::unity2::IlInstance,
    #[rename(name = "value")]
    pub value: ::unity2::IlInstance,
    #[rename(name = "index")]
    pub index: i32,
    #[rename(name = "startIndex")]
    pub start_index: i32,
    #[rename(name = "endIndex")]
    pub end_index: i32,
    #[rename(name = "version")]
    pub version: i32,
    #[rename(name = "current")]
    pub current: bool,
    #[rename(name = "getObjectRetType")]
    pub get_object_ret_type: i32,
}

#[cfg(feature = "system-collections-sortedlist")]
#[::unity2::methods]
impl SortedList_SortedListEnumerator {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        sorted_list: crate::system::collections::sortedlist::SortedList,
        index: i32,
        count: i32,
        get_obj_ret_type: i32,
    ) -> ();

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

    #[method(name = "get_Key", args = 0)]
    pub fn get_key(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "get_Entry", args = 0)]
    pub fn get_entry(self) -> crate::system::collections::dictionaryentry::DictionaryEntry;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::system::object::Object;

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> crate::system::object::Object;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();
}

#[cfg(feature = "system-collections-sortedlist")]
impl SortedList_SortedListEnumerator {
    pub fn new(
        sorted_list: crate::system::collections::sortedlist::SortedList,
        index: i32,
        count: i32,
        get_obj_ret_type: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortedList_SortedListEnumerator),
                ::core::stringify!(new),
            )
        });
        <Self as ISortedList_SortedListEnumeratorMethods>::ctor(
            this,
            sorted_list,
            index,
            count,
            get_obj_ret_type,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/sortedlist/SortedList.md")))]
#[::unity2::class(namespace = "System.Collections", name = "SortedList")]
#[parent(crate::system::object::Object)]
pub struct SortedList {
    #[rename(name = "keys")]
    pub keys: ::unity2::Array<crate::system::object::Object>,
    #[rename(name = "values")]
    pub values: ::unity2::Array<crate::system::object::Object>,
    #[rename(name = "_size")]
    pub size: i32,
    #[rename(name = "version")]
    pub version: i32,
    #[rename(name = "comparer")]
    pub comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    #[rename(name = "keyList")]
    pub key_list: crate::system::collections::sortedlist::SortedList_KeyList,
    #[rename(name = "valueList")]
    pub value_list: crate::system::collections::sortedlist::SortedList_ValueList,
    #[rename(name = "_syncRoot")]
    pub sync_root: ::unity2::IlInstance,
    #[static_field]
    #[rename(name = "_defaultCapacity")]
    pub default_capacity: i32,
    #[static_field]
    #[rename(name = "emptyArray")]
    pub empty_array: ::unity2::Array<crate::system::object::Object>,
}

#[cfg(feature = "system-collections-sortedlist")]
#[::unity2::methods]
impl SortedList {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, initial_capacity: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(
        self,
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_4(
        self,
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
        capacity: i32,
    ) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "get_Capacity", args = 0)]
    pub fn get_capacity(self) -> i32;

    #[method(name = "set_Capacity", args = 1)]
    pub fn set_capacity(self, value: i32) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Keys", args = 0)]
    pub fn get_keys(self) -> crate::system::collections::icollection::ICollection;

    #[method(name = "get_Values", args = 0)]
    pub fn get_values(self) -> crate::system::collections::icollection::ICollection;

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "get_IsFixedSize", args = 0)]
    pub fn get_is_fixed_size(self) -> bool;

    #[method(name = "get_IsSynchronized", args = 0)]
    pub fn get_is_synchronized(self) -> bool;

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, key: crate::system::object::Object) -> bool;

    #[method(name = "ContainsKey", args = 1)]
    pub fn contains_key(self, key: crate::system::object::Object) -> bool;

    #[method(name = "ContainsValue", args = 1)]
    pub fn contains_value(self, value: crate::system::object::Object) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, array_index: i32) -> ();

    #[method(name = "EnsureCapacity", args = 1)]
    pub fn ensure_capacity(self, min: i32) -> ();

    #[method(name = "GetByIndex", args = 1)]
    pub fn get_by_index(self, index: i32) -> crate::system::object::Object;

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::idictionaryenumerator::IDictionaryEnumerator;

    #[method(name = "GetKey", args = 1)]
    pub fn get_key(self, index: i32) -> crate::system::object::Object;

    #[method(name = "GetKeyList", args = 0)]
    pub fn get_key_list(self) -> crate::system::collections::ilist::IList;

    #[method(name = "GetValueList", args = 0)]
    pub fn get_value_list(self) -> crate::system::collections::ilist::IList;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, key: crate::system::object::Object) -> crate::system::object::Object;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "IndexOfKey", args = 1)]
    pub fn index_of_key(self, key: crate::system::object::Object) -> i32;

    #[method(name = "IndexOfValue", args = 1)]
    pub fn index_of_value(self, value: crate::system::object::Object) -> i32;

    #[method(name = "Insert", args = 3)]
    pub fn insert(
        self,
        index: i32,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "RemoveAt", args = 1)]
    pub fn remove_at(self, index: i32) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, key: crate::system::object::Object) -> ();

    #[method(name = "Synchronized", args = 1)]
    pub fn synchronized(
        list: crate::system::collections::sortedlist::SortedList,
    ) -> crate::system::collections::sortedlist::SortedList;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "system-collections-sortedlist")]
impl SortedList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortedList),
                ::core::stringify!(new),
            )
        });
        <Self as ISortedListMethods>::ctor(this);
        this
    }

    pub fn new_2(initial_capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortedList),
                ::core::stringify!(new_2),
            )
        });
        <Self as ISortedListMethods>::ctor_2(this, initial_capacity);
        this
    }

    pub fn new_3(
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortedList),
                ::core::stringify!(new_3),
            )
        });
        <Self as ISortedListMethods>::ctor_3(this, comparer);
        this
    }

    pub fn new_4(
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
        capacity: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortedList),
                ::core::stringify!(new_4),
            )
        });
        <Self as ISortedListMethods>::ctor_4(this, comparer, capacity);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/sortedlist/SortedList_ValueList.md")))]
#[::unity2::class(namespace = "System.Collections", name = "SortedList.ValueList")]
#[parent(crate::system::object::Object)]
pub struct SortedList_ValueList {
    #[rename(name = "sortedList")]
    pub sorted_list: crate::system::collections::sortedlist::SortedList,
}

#[cfg(feature = "system-collections-sortedlist")]
#[::unity2::methods]
impl SortedList_ValueList {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, sorted_list: crate::system::collections::sortedlist::SortedList) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "get_IsSynchronized", args = 0)]
    pub fn get_is_synchronized(self) -> bool;

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "Add", args = 1)]
    pub fn add(self, key: crate::system::object::Object) -> i32;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, value: crate::system::object::Object) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, array_index: i32) -> ();

    #[method(name = "Insert", args = 2)]
    pub fn insert(self, index: i32, value: crate::system::object::Object) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> crate::system::object::Object;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: crate::system::object::Object) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of(self, value: crate::system::object::Object) -> i32;

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, value: crate::system::object::Object) -> ();

    #[method(name = "RemoveAt", args = 1)]
    pub fn remove_at(self, index: i32) -> ();
}

#[cfg(feature = "system-collections-sortedlist")]
impl SortedList_ValueList {
    pub fn new(sorted_list: crate::system::collections::sortedlist::SortedList) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortedList_ValueList),
                ::core::stringify!(new),
            )
        });
        <Self as ISortedList_ValueListMethods>::ctor(this, sorted_list);
        this
    }
}
