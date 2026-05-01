
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/specialized/ordereddictionary/OrderedDictionary_OrderedDictionaryKeyValueCollection.md")))]
#[::unity2::class(
    namespace = "System.Collections.Specialized",
    name = "OrderedDictionary.OrderedDictionaryKeyValueCollection"
)]
#[parent(crate::system::object::Object)]
pub struct OrderedDictionary_OrderedDictionaryKeyValueCollection {
    #[rename(name = "_objects")]
    pub objects: crate::system::collections::arraylist::ArrayList,
    #[rename(name = "isKeys")]
    pub is_keys: bool,
}

#[cfg(feature = "system-collections-specialized-ordereddictionary")]
#[::unity2::methods]
impl OrderedDictionary_OrderedDictionaryKeyValueCollection {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, array: crate::system::collections::arraylist::ArrayList, is_keys: bool)
        -> ();

    #[method(name = "System.Collections.ICollection.CopyTo", args = 2)]
    pub fn system_collections_i_collection_copy_to(
        self,
        array: ::unity2::IlInstance,
        index: i32,
    ) -> ();

    #[method(name = "System.Collections.ICollection.get_Count", args = 0)]
    pub fn system_collections_i_collection_get_count(self) -> i32;

    #[method(name = "System.Collections.ICollection.get_IsSynchronized", args = 0)]
    pub fn system_collections_i_collection_get_is_synchronized(self) -> bool;

    #[method(name = "System.Collections.ICollection.get_SyncRoot", args = 0)]
    pub fn system_collections_i_collection_get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;
}

#[cfg(feature = "system-collections-specialized-ordereddictionary")]
impl OrderedDictionary_OrderedDictionaryKeyValueCollection {
    pub fn new(array: crate::system::collections::arraylist::ArrayList, is_keys: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(OrderedDictionary_OrderedDictionaryKeyValueCollection),
                ::core::stringify!(new),
            )
        });
        <Self as IOrderedDictionary_OrderedDictionaryKeyValueCollectionMethods>::ctor(
            this, array, is_keys,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/specialized/ordereddictionary/OrderedDictionary.md")))]
#[::unity2::class(
    namespace = "System.Collections.Specialized",
    name = "OrderedDictionary"
)]
#[parent(crate::system::object::Object)]
pub struct OrderedDictionary {
    #[rename(name = "_objectsArray")]
    pub objects_array: crate::system::collections::arraylist::ArrayList,
    #[rename(name = "_objectsTable")]
    pub objects_table: crate::system::collections::hashtable::Hashtable,
    #[rename(name = "_initialCapacity")]
    pub initial_capacity: i32,
    #[rename(name = "_comparer")]
    pub comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    #[rename(name = "_readOnly")]
    pub read_only: bool,
    #[rename(name = "_syncRoot")]
    pub sync_root: ::unity2::IlInstance,
}

#[cfg(feature = "system-collections-specialized-ordereddictionary")]
#[::unity2::methods]
impl OrderedDictionary {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, capacity: i32) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_3(
        self,
        capacity: i32,
        comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "System.Collections.IDictionary.get_IsFixedSize", args = 0)]
    pub fn system_collections_i_dictionary_get_is_fixed_size(self) -> bool;

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "System.Collections.ICollection.get_IsSynchronized", args = 0)]
    pub fn system_collections_i_collection_get_is_synchronized(self) -> bool;

    #[method(name = "get_Keys", args = 0)]
    pub fn get_keys(self) -> crate::system::collections::icollection::ICollection;

    #[method(name = "get_objectsArray", args = 0)]
    pub fn get_objects_array(self) -> crate::system::collections::arraylist::ArrayList;

    #[method(name = "get_objectsTable", args = 0)]
    pub fn get_objects_table(self) -> crate::system::collections::hashtable::Hashtable;

    #[method(name = "System.Collections.ICollection.get_SyncRoot", args = 0)]
    pub fn system_collections_i_collection_get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, key: crate::system::object::Object) -> crate::system::object::Object;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "get_Values", args = 0)]
    pub fn get_values(self) -> crate::system::collections::icollection::ICollection;

    #[method(name = "Add", args = 2)]
    pub fn add(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, key: crate::system::object::Object) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, index: i32) -> ();

    #[method(name = "IndexOfKey", args = 1)]
    pub fn index_of_key(self, key: crate::system::object::Object) -> i32;

    #[method(name = "OnDeserialization", args = 1)]
    pub fn on_deserialization(self, sender: crate::system::object::Object) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, key: crate::system::object::Object) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::idictionaryenumerator::IDictionaryEnumerator;

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(
        name = "System.Runtime.Serialization.IDeserializationCallback.OnDeserialization",
        args = 1
    )]
    pub fn system_runtime_serialization_i_deserialization_callback_on_deserialization(
        self,
        sender: crate::system::object::Object,
    ) -> ();
}

#[cfg(feature = "system-collections-specialized-ordereddictionary")]
impl OrderedDictionary {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(OrderedDictionary),
                ::core::stringify!(new),
            )
        });
        <Self as IOrderedDictionaryMethods>::ctor(this);
        this
    }

    pub fn new_2(capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(OrderedDictionary),
                ::core::stringify!(new_2),
            )
        });
        <Self as IOrderedDictionaryMethods>::ctor_2(this, capacity);
        this
    }

    pub fn new_3(
        capacity: i32,
        comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(OrderedDictionary),
                ::core::stringify!(new_3),
            )
        });
        <Self as IOrderedDictionaryMethods>::ctor_3(this, capacity, comparer);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/specialized/ordereddictionary/OrderedDictionary_OrderedDictionaryEnumerator.md")))]
#[::unity2::class(
    namespace = "System.Collections.Specialized",
    name = "OrderedDictionary.OrderedDictionaryEnumerator"
)]
#[parent(crate::system::object::Object)]
pub struct OrderedDictionary_OrderedDictionaryEnumerator {
    #[rename(name = "_objectReturnType")]
    pub object_return_type: i32,
    #[rename(name = "arrayEnumerator")]
    pub array_enumerator: crate::system::collections::ienumerator::IEnumerator,
}

#[cfg(feature = "system-collections-specialized-ordereddictionary")]
#[::unity2::methods]
impl OrderedDictionary_OrderedDictionaryEnumerator {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        array: crate::system::collections::arraylist::ArrayList,
        object_return_type: i32,
    ) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::system::object::Object;

    #[method(name = "get_Entry", args = 0)]
    pub fn get_entry(self) -> crate::system::collections::dictionaryentry::DictionaryEntry;

    #[method(name = "get_Key", args = 0)]
    pub fn get_key(self) -> crate::system::object::Object;

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();
}

#[cfg(feature = "system-collections-specialized-ordereddictionary")]
impl OrderedDictionary_OrderedDictionaryEnumerator {
    pub fn new(
        array: crate::system::collections::arraylist::ArrayList,
        object_return_type: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(OrderedDictionary_OrderedDictionaryEnumerator),
                ::core::stringify!(new),
            )
        });
        <Self as IOrderedDictionary_OrderedDictionaryEnumeratorMethods>::ctor(
            this,
            array,
            object_return_type,
        );
        this
    }
}
