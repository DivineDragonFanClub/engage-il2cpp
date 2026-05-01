
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/listdictionaryinternal/ListDictionaryInternal.md")))]
#[::unity2::class(namespace = "System.Collections", name = "ListDictionaryInternal")]
#[parent(crate::system::object::Object)]
pub struct ListDictionaryInternal {
    #[rename(name = "head")]
    pub head:
        crate::system::collections::listdictionaryinternal::ListDictionaryInternal_DictionaryNode,
    #[rename(name = "version")]
    pub version: i32,
    #[rename(name = "count")]
    pub count: i32,
    #[rename(name = "_syncRoot")]
    pub sync_root: ::unity2::IlInstance,
}

#[cfg(feature = "system-collections-listdictionaryinternal")]
#[::unity2::methods]
impl ListDictionaryInternal {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, key: crate::system::object::Object) -> crate::system::object::Object;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Keys", args = 0)]
    pub fn get_keys(self) -> crate::system::collections::icollection::ICollection;

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "get_IsFixedSize", args = 0)]
    pub fn get_is_fixed_size(self) -> bool;

    #[method(name = "get_IsSynchronized", args = 0)]
    pub fn get_is_synchronized(self) -> bool;

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "get_Values", args = 0)]
    pub fn get_values(self) -> crate::system::collections::icollection::ICollection;

    #[method(name = "Add", args = 2)]
    pub fn add(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, key: crate::system::object::Object) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, index: i32) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::idictionaryenumerator::IDictionaryEnumerator;

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, key: crate::system::object::Object) -> ();
}

#[cfg(feature = "system-collections-listdictionaryinternal")]
impl ListDictionaryInternal {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ListDictionaryInternal),
                ::core::stringify!(new),
            )
        });
        <Self as IListDictionaryInternalMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/listdictionaryinternal/ListDictionaryInternal_NodeEnumerator.md")))]
#[::unity2::class(
    namespace = "System.Collections",
    name = "ListDictionaryInternal.NodeEnumerator"
)]
#[parent(crate::system::object::Object)]
pub struct ListDictionaryInternal_NodeEnumerator {
    #[rename(name = "list")]
    pub list: crate::system::collections::listdictionaryinternal::ListDictionaryInternal,
    #[rename(name = "current")]
    pub current:
        crate::system::collections::listdictionaryinternal::ListDictionaryInternal_DictionaryNode,
    #[rename(name = "version")]
    pub version: i32,
    #[rename(name = "start")]
    pub start: bool,
}

#[cfg(feature = "system-collections-listdictionaryinternal")]
#[::unity2::methods]
impl ListDictionaryInternal_NodeEnumerator {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        list: crate::system::collections::listdictionaryinternal::ListDictionaryInternal,
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

#[cfg(feature = "system-collections-listdictionaryinternal")]
impl ListDictionaryInternal_NodeEnumerator {
    pub fn new(
        list: crate::system::collections::listdictionaryinternal::ListDictionaryInternal,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ListDictionaryInternal_NodeEnumerator),
                ::core::stringify!(new),
            )
        });
        <Self as IListDictionaryInternal_NodeEnumeratorMethods>::ctor(this, list);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/listdictionaryinternal/ListDictionaryInternal_NodeKeyValueCollection.md")))]
#[::unity2::class(
    namespace = "System.Collections",
    name = "ListDictionaryInternal.NodeKeyValueCollection"
)]
#[parent(crate::system::object::Object)]
pub struct ListDictionaryInternal_NodeKeyValueCollection {
    #[rename(name = "list")]
    pub list: crate::system::collections::listdictionaryinternal::ListDictionaryInternal,
    #[rename(name = "isKeys")]
    pub is_keys: bool,
}

#[cfg(feature = "system-collections-listdictionaryinternal")]
#[::unity2::methods]
impl ListDictionaryInternal_NodeKeyValueCollection {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        list: crate::system::collections::listdictionaryinternal::ListDictionaryInternal,
        is_keys: bool,
    ) -> ();

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

#[cfg(feature = "system-collections-listdictionaryinternal")]
impl ListDictionaryInternal_NodeKeyValueCollection {
    pub fn new(
        list: crate::system::collections::listdictionaryinternal::ListDictionaryInternal,
        is_keys: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ListDictionaryInternal_NodeKeyValueCollection),
                ::core::stringify!(new),
            )
        });
        <Self as IListDictionaryInternal_NodeKeyValueCollectionMethods>::ctor(this, list, is_keys);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/listdictionaryinternal/ListDictionaryInternal_DictionaryNode.md")))]
#[::unity2::class(
    namespace = "System.Collections",
    name = "ListDictionaryInternal.DictionaryNode"
)]
#[parent(crate::system::object::Object)]
pub struct ListDictionaryInternal_DictionaryNode {
    #[rename(name = "key")]
    pub key: ::unity2::IlInstance,
    #[rename(name = "value")]
    pub value: ::unity2::IlInstance,
    #[rename(name = "next")]
    pub next:
        crate::system::collections::listdictionaryinternal::ListDictionaryInternal_DictionaryNode,
}

#[cfg(feature = "system-collections-listdictionaryinternal")]
#[::unity2::methods]
impl ListDictionaryInternal_DictionaryNode {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-collections-listdictionaryinternal")]
impl ListDictionaryInternal_DictionaryNode {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ListDictionaryInternal_DictionaryNode),
                ::core::stringify!(new),
            )
        });
        <Self as IListDictionaryInternal_DictionaryNodeMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/listdictionaryinternal/ListDictionaryInternal_NodeKeyValueCollection_NodeKeyValueEnumerator.md")))]
#[::unity2::class(
    namespace = "System.Collections",
    name = "ListDictionaryInternal.NodeKeyValueCollection.NodeKeyValueEnumerator"
)]
#[parent(crate::system::object::Object)]
pub struct ListDictionaryInternal_NodeKeyValueCollection_NodeKeyValueEnumerator {
    #[rename(name = "list")]
    pub list: crate::system::collections::listdictionaryinternal::ListDictionaryInternal,
    #[rename(name = "current")]
    pub current:
        crate::system::collections::listdictionaryinternal::ListDictionaryInternal_DictionaryNode,
    #[rename(name = "version")]
    pub version: i32,
    #[rename(name = "isKeys")]
    pub is_keys: bool,
    #[rename(name = "start")]
    pub start: bool,
}

#[cfg(feature = "system-collections-listdictionaryinternal")]
#[::unity2::methods]
impl ListDictionaryInternal_NodeKeyValueCollection_NodeKeyValueEnumerator {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        list: crate::system::collections::listdictionaryinternal::ListDictionaryInternal,
        is_keys: bool,
    ) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();
}

#[cfg(feature = "system-collections-listdictionaryinternal")]
impl ListDictionaryInternal_NodeKeyValueCollection_NodeKeyValueEnumerator {
    pub fn new(
        list: crate::system::collections::listdictionaryinternal::ListDictionaryInternal,
        is_keys: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    ListDictionaryInternal_NodeKeyValueCollection_NodeKeyValueEnumerator
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IListDictionaryInternal_NodeKeyValueCollection_NodeKeyValueEnumeratorMethods > :: ctor (this , list , is_keys) ;
        this
    }
}
