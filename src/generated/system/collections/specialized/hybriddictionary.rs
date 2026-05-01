
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/specialized/hybriddictionary/HybridDictionary.md")))]
#[::unity2::class(
    namespace = "System.Collections.Specialized",
    name = "HybridDictionary"
)]
#[parent(crate::system::object::Object)]
pub struct HybridDictionary {
    #[rename(name = "list")]
    pub list: crate::system::collections::specialized::listdictionary::ListDictionary,
    #[rename(name = "hashtable")]
    pub hashtable: crate::system::collections::hashtable::Hashtable,
    #[rename(name = "caseInsensitive")]
    pub case_insensitive: bool,
}

#[cfg(feature = "system-collections-specialized-hybriddictionary")]
#[::unity2::methods]
impl HybridDictionary {
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

    #[method(name = "get_List", args = 0)]
    pub fn get_list(
        self,
    ) -> crate::system::collections::specialized::listdictionary::ListDictionary;

    #[method(name = "ChangeOver", args = 0)]
    pub fn change_over(self) -> ();

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

#[cfg(feature = "system-collections-specialized-hybriddictionary")]
impl HybridDictionary {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HybridDictionary),
                ::core::stringify!(new),
            )
        });
        <Self as IHybridDictionaryMethods>::ctor(this);
        this
    }
}
