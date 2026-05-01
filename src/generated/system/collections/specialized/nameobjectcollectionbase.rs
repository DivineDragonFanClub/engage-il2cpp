
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/specialized/nameobjectcollectionbase/NameObjectCollectionBase.md")))]
#[::unity2::class(
    namespace = "System.Collections.Specialized",
    name = "NameObjectCollectionBase"
)]
#[parent(crate::system::object::Object)]
pub struct NameObjectCollectionBase {
# [rename (name = "_readOnly")] pub read_only : bool ,
# [rename (name = "_entriesArray")] pub entries_array : crate :: system :: collections :: arraylist :: ArrayList ,
# [rename (name = "_keyComparer")] pub key_comparer : crate :: system :: collections :: iequalitycomparer :: IEqualityComparer ,
# [rename (name = "_entriesTable")] pub entries_table : crate :: system :: collections :: hashtable :: Hashtable ,
# [rename (name = "_nullKeyEntry")] pub null_key_entry : crate :: system :: collections :: specialized :: nameobjectcollectionbase :: NameObjectCollectionBase_NameObjectEntry ,
# [rename (name = "_version")] pub version : i32 ,
# [rename (name = "_syncRoot")] pub sync_root : :: unity2 :: IlInstance ,
}

#[cfg(feature = "system-collections-specialized-nameobjectcollectionbase")]
#[::unity2::methods]
impl NameObjectCollectionBase {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        equality_comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_3(
        self,
        capacity: i32,
        equality_comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> ();

    #[method(name = "OnDeserialization", args = 1)]
    pub fn on_deserialization(self, sender: crate::system::object::Object) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Reset", args = 1)]
    pub fn reset_2(self, capacity: i32) -> ();

    #[method(name = "FindEntry", args = 1)]
    pub fn find_entry (self , key : :: unity2 :: Il2CppString) -> crate :: system :: collections :: specialized :: nameobjectcollectionbase :: NameObjectCollectionBase_NameObjectEntry ;

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "BaseAdd", args = 2)]
    pub fn base_add(self, name: ::unity2::Il2CppString, value: crate::system::object::Object)
        -> ();

    #[method(name = "BaseRemove", args = 1)]
    pub fn base_remove(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "BaseGet", args = 1)]
    pub fn base_get(self, name: ::unity2::Il2CppString) -> crate::system::object::Object;

    #[method(name = "BaseSet", args = 2)]
    pub fn base_set(self, name: ::unity2::Il2CppString, value: crate::system::object::Object)
        -> ();

    #[method(name = "BaseGet", args = 1)]
    pub fn base_get_2(self, index: i32) -> crate::system::object::Object;

    #[method(name = "BaseGetKey", args = 1)]
    pub fn base_get_key(self, index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "System.Collections.ICollection.CopyTo", args = 2)]
    pub fn system_collections_i_collection_copy_to(
        self,
        array: ::unity2::IlInstance,
        index: i32,
    ) -> ();

    #[method(name = "System.Collections.ICollection.get_SyncRoot", args = 0)]
    pub fn system_collections_i_collection_get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "System.Collections.ICollection.get_IsSynchronized", args = 0)]
    pub fn system_collections_i_collection_get_is_synchronized(self) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "system-collections-specialized-nameobjectcollectionbase")]
impl NameObjectCollectionBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NameObjectCollectionBase),
                ::core::stringify!(new),
            )
        });
        <Self as INameObjectCollectionBaseMethods>::ctor(this);
        this
    }

    pub fn new_2(
        equality_comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NameObjectCollectionBase),
                ::core::stringify!(new_2),
            )
        });
        <Self as INameObjectCollectionBaseMethods>::ctor_2(this, equality_comparer);
        this
    }

    pub fn new_3(
        capacity: i32,
        equality_comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NameObjectCollectionBase),
                ::core::stringify!(new_3),
            )
        });
        <Self as INameObjectCollectionBaseMethods>::ctor_3(this, capacity, equality_comparer);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/specialized/nameobjectcollectionbase/NameObjectCollectionBase_NameObjectEntry.md")))]
#[::unity2::class(
    namespace = "System.Collections.Specialized",
    name = "NameObjectCollectionBase.NameObjectEntry"
)]
#[parent(crate::system::object::Object)]
pub struct NameObjectCollectionBase_NameObjectEntry {
    #[rename(name = "Key")]
    pub key: ::unity2::Il2CppString,
    #[rename(name = "Value")]
    pub value: ::unity2::IlInstance,
}

#[cfg(feature = "system-collections-specialized-nameobjectcollectionbase")]
#[::unity2::methods]
impl NameObjectCollectionBase_NameObjectEntry {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, name: ::unity2::Il2CppString, value: crate::system::object::Object) -> ();
}

#[cfg(feature = "system-collections-specialized-nameobjectcollectionbase")]
impl NameObjectCollectionBase_NameObjectEntry {
    pub fn new(name: ::unity2::Il2CppString, value: crate::system::object::Object) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NameObjectCollectionBase_NameObjectEntry),
                ::core::stringify!(new),
            )
        });
        <Self as INameObjectCollectionBase_NameObjectEntryMethods>::ctor(this, name, value);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/specialized/nameobjectcollectionbase/NameObjectCollectionBase_NameObjectKeysEnumerator.md")))]
#[::unity2::class(
    namespace = "System.Collections.Specialized",
    name = "NameObjectCollectionBase.NameObjectKeysEnumerator"
)]
#[parent(crate::system::object::Object)]
pub struct NameObjectCollectionBase_NameObjectKeysEnumerator {
    #[rename(name = "_pos")]
    pub pos: i32,
    #[rename(name = "_coll")]
    pub coll:
        crate::system::collections::specialized::nameobjectcollectionbase::NameObjectCollectionBase,
    #[rename(name = "_version")]
    pub version: i32,
}

#[cfg(feature = "system-collections-specialized-nameobjectcollectionbase")]
#[::unity2::methods]
impl NameObjectCollectionBase_NameObjectKeysEnumerator {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        coll : crate :: system :: collections :: specialized :: nameobjectcollectionbase :: NameObjectCollectionBase,
    ) -> ();

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::system::object::Object;
}

#[cfg(feature = "system-collections-specialized-nameobjectcollectionbase")]
impl NameObjectCollectionBase_NameObjectKeysEnumerator {
    pub fn new(
        coll : crate :: system :: collections :: specialized :: nameobjectcollectionbase :: NameObjectCollectionBase,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NameObjectCollectionBase_NameObjectKeysEnumerator),
                ::core::stringify!(new),
            )
        });
        <Self as INameObjectCollectionBase_NameObjectKeysEnumeratorMethods>::ctor(this, coll);
        this
    }
}
