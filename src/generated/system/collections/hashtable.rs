
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/hashtable/Hashtable_HashtableEnumerator.md")))]
#[::unity2::class(
    namespace = "System.Collections",
    name = "Hashtable.HashtableEnumerator"
)]
#[parent(crate::system::object::Object)]
pub struct Hashtable_HashtableEnumerator {
    #[rename(name = "hashtable")]
    pub hashtable: crate::system::collections::hashtable::Hashtable,
    #[rename(name = "bucket")]
    pub bucket: i32,
    #[rename(name = "version")]
    pub version: i32,
    #[rename(name = "current")]
    pub current: bool,
    #[rename(name = "getObjectRetType")]
    pub get_object_ret_type: i32,
    #[rename(name = "currentKey")]
    pub current_key: ::unity2::IlInstance,
    #[rename(name = "currentValue")]
    pub current_value: ::unity2::IlInstance,
}

#[cfg(feature = "system-collections-hashtable")]
#[::unity2::methods]
impl Hashtable_HashtableEnumerator {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        hashtable: crate::system::collections::hashtable::Hashtable,
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

#[cfg(feature = "system-collections-hashtable")]
impl Hashtable_HashtableEnumerator {
    pub fn new(
        hashtable: crate::system::collections::hashtable::Hashtable,
        get_obj_ret_type: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Hashtable_HashtableEnumerator),
                ::core::stringify!(new),
            )
        });
        <Self as IHashtable_HashtableEnumeratorMethods>::ctor(this, hashtable, get_obj_ret_type);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/hashtable/Hashtable_ValueCollection.md")))]
#[::unity2::class(namespace = "System.Collections", name = "Hashtable.ValueCollection")]
#[parent(crate::system::object::Object)]
pub struct Hashtable_ValueCollection {
    #[rename(name = "_hashtable")]
    pub hashtable: crate::system::collections::hashtable::Hashtable,
}

#[cfg(feature = "system-collections-hashtable")]
#[::unity2::methods]
impl Hashtable_ValueCollection {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, hashtable: crate::system::collections::hashtable::Hashtable) -> ();

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, array_index: i32) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "get_IsSynchronized", args = 0)]
    pub fn get_is_synchronized(self) -> bool;

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;
}

#[cfg(feature = "system-collections-hashtable")]
impl Hashtable_ValueCollection {
    pub fn new(hashtable: crate::system::collections::hashtable::Hashtable) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Hashtable_ValueCollection),
                ::core::stringify!(new),
            )
        });
        <Self as IHashtable_ValueCollectionMethods>::ctor(this, hashtable);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/hashtable/Hashtable_SyncHashtable.md")))]
#[::unity2::class(namespace = "System.Collections", name = "Hashtable.SyncHashtable")]
#[parent(crate::system::collections::hashtable::Hashtable)]
pub struct Hashtable_SyncHashtable {
    #[rename(name = "_table")]
    pub table: crate::system::collections::hashtable::Hashtable,
}

#[cfg(feature = "system-collections-hashtable")]
#[::unity2::methods]
impl Hashtable_SyncHashtable {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, table: crate::system::collections::hashtable::Hashtable) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

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

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

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

    #[method(name = "ContainsKey", args = 1)]
    pub fn contains_key(self, key: crate::system::object::Object) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, array_index: i32) -> ();

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::idictionaryenumerator::IDictionaryEnumerator;

    #[method(name = "get_Keys", args = 0)]
    pub fn get_keys(self) -> crate::system::collections::icollection::ICollection;

    #[method(name = "get_Values", args = 0)]
    pub fn get_values(self) -> crate::system::collections::icollection::ICollection;

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, key: crate::system::object::Object) -> ();

    #[method(name = "OnDeserialization", args = 1)]
    pub fn on_deserialization(self, sender: crate::system::object::Object) -> ();
}

#[cfg(feature = "system-collections-hashtable")]
impl Hashtable_SyncHashtable {
    pub fn new(table: crate::system::collections::hashtable::Hashtable) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Hashtable_SyncHashtable),
                ::core::stringify!(new),
            )
        });
        <Self as IHashtable_SyncHashtableMethods>::ctor(this, table);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/hashtable/Hashtable.md")))]
#[::unity2::class(namespace = "System.Collections", name = "Hashtable")]
#[parent(crate::system::object::Object)]
pub struct Hashtable {
    #[static_field]
    #[rename(name = "HashPrime")]
    pub hash_prime: i32,
    #[static_field]
    #[rename(name = "InitialSize")]
    pub initial_size: i32,
    #[static_field]
    #[rename(name = "LoadFactorName")]
    pub load_factor_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "VersionName")]
    pub version_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ComparerName")]
    pub comparer_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "HashCodeProviderName")]
    pub hash_code_provider_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "HashSizeName")]
    pub hash_size_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "KeysName")]
    pub keys_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ValuesName")]
    pub values_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "KeyComparerName")]
    pub key_comparer_name: ::unity2::Il2CppString,
    #[rename(name = "buckets")]
    pub buckets: ::unity2::Array<crate::system::collections::hashtable::Hashtable_bucket>,
    #[rename(name = "count")]
    pub count: i32,
    #[rename(name = "occupancy")]
    pub occupancy: i32,
    #[rename(name = "loadsize")]
    pub loadsize: i32,
    #[rename(name = "loadFactor")]
    pub load_factor: f32,
    #[rename(name = "version")]
    pub version: i32,
    #[rename(name = "isWriterInProgress")]
    pub is_writer_in_progress: bool,
    #[rename(name = "keys")]
    pub keys: crate::system::collections::icollection::ICollection,
    #[rename(name = "values")]
    pub values: crate::system::collections::icollection::ICollection,
    #[rename(name = "_keycomparer")]
    pub keycomparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    #[rename(name = "_syncRoot")]
    pub sync_root: ::unity2::IlInstance,
}

#[cfg(feature = "system-collections-hashtable")]
#[::unity2::methods]
impl Hashtable {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, trash: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor_2(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, capacity: i32) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_4(self, capacity: i32, load_factor: f32) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_5(
        self,
        capacity: i32,
        load_factor: f32,
        hcp: crate::system::collections::ihashcodeprovider::IHashCodeProvider,
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_6(
        self,
        capacity: i32,
        load_factor: f32,
        equality_comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_7(
        self,
        hcp: crate::system::collections::ihashcodeprovider::IHashCodeProvider,
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_8(
        self,
        equality_comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_9(
        self,
        capacity: i32,
        equality_comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> ();

    #[method(name = "InitHash", args = 4)]
    pub fn init_hash(
        self,
        key: crate::system::object::Object,
        hashsize: i32,
        seed: u32,
        incr: u32,
    ) -> u32;

    #[method(name = "Add", args = 2)]
    pub fn add(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, key: crate::system::object::Object) -> bool;

    #[method(name = "ContainsKey", args = 1)]
    pub fn contains_key(self, key: crate::system::object::Object) -> bool;

    #[method(name = "CopyKeys", args = 2)]
    pub fn copy_keys(self, array: ::unity2::IlInstance, array_index: i32) -> ();

    #[method(name = "CopyEntries", args = 2)]
    pub fn copy_entries(self, array: ::unity2::IlInstance, array_index: i32) -> ();

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, array_index: i32) -> ();

    #[method(name = "CopyValues", args = 2)]
    pub fn copy_values(self, array: ::unity2::IlInstance, array_index: i32) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, key: crate::system::object::Object) -> crate::system::object::Object;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "expand", args = 0)]
    pub fn expand(self) -> ();

    #[method(name = "rehash", args = 0)]
    pub fn rehash(self) -> ();

    #[method(name = "UpdateVersion", args = 0)]
    pub fn update_version(self) -> ();

    #[method(name = "rehash", args = 2)]
    pub fn rehash_2(self, newsize: i32, force_new_hash_code: bool) -> ();

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::idictionaryenumerator::IDictionaryEnumerator;

    #[method(name = "GetHash", args = 1)]
    pub fn get_hash(self, key: crate::system::object::Object) -> i32;

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "get_IsFixedSize", args = 0)]
    pub fn get_is_fixed_size(self) -> bool;

    #[method(name = "get_IsSynchronized", args = 0)]
    pub fn get_is_synchronized(self) -> bool;

    #[method(name = "KeyEquals", args = 2)]
    pub fn key_equals(
        self,
        item: crate::system::object::Object,
        key: crate::system::object::Object,
    ) -> bool;

    #[method(name = "get_Keys", args = 0)]
    pub fn get_keys(self) -> crate::system::collections::icollection::ICollection;

    #[method(name = "get_Values", args = 0)]
    pub fn get_values(self) -> crate::system::collections::icollection::ICollection;

    #[method(name = "Insert", args = 3)]
    pub fn insert(
        self,
        key: crate::system::object::Object,
        nvalue: crate::system::object::Object,
        add: bool,
    ) -> ();

    #[method(name = "putEntry", args = 4)]
    pub fn put_entry(
        self,
        new_buckets: ::unity2::Array<crate::system::collections::hashtable::Hashtable_bucket>,
        key: crate::system::object::Object,
        nvalue: crate::system::object::Object,
        hashcode: i32,
    ) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, key: crate::system::object::Object) -> ();

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "Synchronized", args = 1)]
    pub fn synchronized(
        table: crate::system::collections::hashtable::Hashtable,
    ) -> crate::system::collections::hashtable::Hashtable;

    #[method(name = "OnDeserialization", args = 1)]
    pub fn on_deserialization(self, sender: crate::system::object::Object) -> ();
}

#[cfg(feature = "system-collections-hashtable")]
impl Hashtable {
    pub fn new(trash: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Hashtable),
                ::core::stringify!(new),
            )
        });
        <Self as IHashtableMethods>::ctor(this, trash);
        this
    }

    pub fn new_2() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Hashtable),
                ::core::stringify!(new_2),
            )
        });
        <Self as IHashtableMethods>::ctor_2(this);
        this
    }

    pub fn new_3(capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Hashtable),
                ::core::stringify!(new_3),
            )
        });
        <Self as IHashtableMethods>::ctor_3(this, capacity);
        this
    }

    pub fn new_4(capacity: i32, load_factor: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Hashtable),
                ::core::stringify!(new_4),
            )
        });
        <Self as IHashtableMethods>::ctor_4(this, capacity, load_factor);
        this
    }

    pub fn new_5(
        capacity: i32,
        load_factor: f32,
        hcp: crate::system::collections::ihashcodeprovider::IHashCodeProvider,
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Hashtable),
                ::core::stringify!(new_5),
            )
        });
        <Self as IHashtableMethods>::ctor_5(this, capacity, load_factor, hcp, comparer);
        this
    }

    pub fn new_6(
        capacity: i32,
        load_factor: f32,
        equality_comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Hashtable),
                ::core::stringify!(new_6),
            )
        });
        <Self as IHashtableMethods>::ctor_6(this, capacity, load_factor, equality_comparer);
        this
    }

    pub fn new_7(
        hcp: crate::system::collections::ihashcodeprovider::IHashCodeProvider,
        comparer: crate::system::collections::icomparer_interface::IComparer_Interface,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Hashtable),
                ::core::stringify!(new_7),
            )
        });
        <Self as IHashtableMethods>::ctor_7(this, hcp, comparer);
        this
    }

    pub fn new_8(
        equality_comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Hashtable),
                ::core::stringify!(new_8),
            )
        });
        <Self as IHashtableMethods>::ctor_8(this, equality_comparer);
        this
    }

    pub fn new_9(
        capacity: i32,
        equality_comparer: crate::system::collections::iequalitycomparer::IEqualityComparer,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Hashtable),
                ::core::stringify!(new_9),
            )
        });
        <Self as IHashtableMethods>::ctor_9(this, capacity, equality_comparer);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/hashtable/Hashtable_HashtableDebugView.md")))]
#[::unity2::class(
    namespace = "System.Collections",
    name = "Hashtable.HashtableDebugView"
)]
#[parent(crate::system::object::Object)]
pub struct Hashtable_HashtableDebugView {}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/hashtable/Hashtable_KeyCollection.md")))]
#[::unity2::class(namespace = "System.Collections", name = "Hashtable.KeyCollection")]
#[parent(crate::system::object::Object)]
pub struct Hashtable_KeyCollection {
    #[rename(name = "_hashtable")]
    pub hashtable: crate::system::collections::hashtable::Hashtable,
}

#[cfg(feature = "system-collections-hashtable")]
#[::unity2::methods]
impl Hashtable_KeyCollection {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, hashtable: crate::system::collections::hashtable::Hashtable) -> ();

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, array_index: i32) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "get_IsSynchronized", args = 0)]
    pub fn get_is_synchronized(self) -> bool;

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;
}

#[cfg(feature = "system-collections-hashtable")]
impl Hashtable_KeyCollection {
    pub fn new(hashtable: crate::system::collections::hashtable::Hashtable) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Hashtable_KeyCollection),
                ::core::stringify!(new),
            )
        });
        <Self as IHashtable_KeyCollectionMethods>::ctor(this, hashtable);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/hashtable/Hashtable_bucket.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Hashtable_bucket {
    pub key: ::unity2::IlInstance,
    pub val: ::unity2::IlInstance,
    pub hash_coll: i32,
}

impl ::unity2::ClassIdentity for Hashtable_bucket {
    const NAMESPACE: &'static str = "System.Collections";

    const NAME: &'static str = "Hashtable.bucket";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Hashtable_bucket {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
