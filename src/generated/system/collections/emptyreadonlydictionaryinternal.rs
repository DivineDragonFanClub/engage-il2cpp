
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/emptyreadonlydictionaryinternal/EmptyReadOnlyDictionaryInternal_NodeEnumerator.md")))]
#[::unity2::class(
    namespace = "System.Collections",
    name = "EmptyReadOnlyDictionaryInternal.NodeEnumerator"
)]
#[parent(crate::system::object::Object)]
pub struct EmptyReadOnlyDictionaryInternal_NodeEnumerator {}

#[cfg(feature = "system-collections-emptyreadonlydictionaryinternal")]
#[::unity2::methods]
impl EmptyReadOnlyDictionaryInternal_NodeEnumerator {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::system::object::Object;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "get_Key", args = 0)]
    pub fn get_key(self) -> crate::system::object::Object;

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> crate::system::object::Object;

    #[method(name = "get_Entry", args = 0)]
    pub fn get_entry(self) -> crate::system::collections::dictionaryentry::DictionaryEntry;
}

#[cfg(feature = "system-collections-emptyreadonlydictionaryinternal")]
impl EmptyReadOnlyDictionaryInternal_NodeEnumerator {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EmptyReadOnlyDictionaryInternal_NodeEnumerator),
                ::core::stringify!(new),
            )
        });
        <Self as IEmptyReadOnlyDictionaryInternal_NodeEnumeratorMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/emptyreadonlydictionaryinternal/EmptyReadOnlyDictionaryInternal.md")))]
#[::unity2::class(
    namespace = "System.Collections",
    name = "EmptyReadOnlyDictionaryInternal"
)]
#[parent(crate::system::object::Object)]
pub struct EmptyReadOnlyDictionaryInternal {}

#[cfg(feature = "system-collections-emptyreadonlydictionaryinternal")]
#[::unity2::methods]
impl EmptyReadOnlyDictionaryInternal {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, index: i32) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

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

    #[method(name = "get_Keys", args = 0)]
    pub fn get_keys(self) -> crate::system::collections::icollection::ICollection;

    #[method(name = "get_Values", args = 0)]
    pub fn get_values(self) -> crate::system::collections::icollection::ICollection;

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, key: crate::system::object::Object) -> bool;

    #[method(name = "Add", args = 2)]
    pub fn add(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "get_IsFixedSize", args = 0)]
    pub fn get_is_fixed_size(self) -> bool;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::idictionaryenumerator::IDictionaryEnumerator;

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, key: crate::system::object::Object) -> ();
}

#[cfg(feature = "system-collections-emptyreadonlydictionaryinternal")]
impl EmptyReadOnlyDictionaryInternal {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EmptyReadOnlyDictionaryInternal),
                ::core::stringify!(new),
            )
        });
        <Self as IEmptyReadOnlyDictionaryInternalMethods>::ctor(this);
        this
    }
}
