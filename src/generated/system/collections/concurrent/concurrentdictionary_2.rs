
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/concurrent/concurrentdictionary_2/ConcurrentDictionary_2_Tables.md")))]
#[::unity2::class(
    namespace = "System.Collections.Concurrent",
    name = "ConcurrentDictionary`2.Tables"
)]
pub struct ConcurrentDictionary_2_Tables<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {
    #[rename(name = "_buckets")]
    pub buckets: ::unity2::Array<
        crate::system::collections::concurrent::concurrentdictionary_2::ConcurrentDictionary_2_Node<
            T0,
            T1,
        >,
    >,
    #[rename(name = "_locks")]
    pub locks: ::unity2::Array<crate::system::object::Object>,
    #[rename(name = "_countPerLock")]
    pub count_per_lock: ::unity2::Array<i32>,
}

#[cfg(feature = "system-collections-concurrent-concurrentdictionary_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity>
    ConcurrentDictionary_2_Tables<T0, T1>
{
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        buckets : :: unity2 :: Array < crate :: system :: collections :: concurrent :: concurrentdictionary_2 :: ConcurrentDictionary_2_Node < T0 , T1 > >,
        locks: ::unity2::Array<crate::system::object::Object>,
        count_per_lock: ::unity2::Array<i32>,
    ) -> ();
}

#[cfg(feature = "system-collections-concurrent-concurrentdictionary_2")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity>
    ConcurrentDictionary_2_Tables<T0, T1>
{
    pub fn new(
        buckets : :: unity2 :: Array < crate :: system :: collections :: concurrent :: concurrentdictionary_2 :: ConcurrentDictionary_2_Node < T0 , T1 > >,
        locks: ::unity2::Array<crate::system::object::Object>,
        count_per_lock: ::unity2::Array<i32>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConcurrentDictionary_2_Tables),
                ::core::stringify!(new),
            )
        });
        <Self as IConcurrentDictionary_2_TablesMethods<T0, T1>>::ctor(
            this,
            buckets,
            locks,
            count_per_lock,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/concurrent/concurrentdictionary_2/ConcurrentDictionary_2_DictionaryEnumerator.md")))]
#[::unity2::class(
    namespace = "System.Collections.Concurrent",
    name = "ConcurrentDictionary`2.DictionaryEnumerator"
)]
pub struct ConcurrentDictionary_2_DictionaryEnumerator<
    T0: ::unity2::ClassIdentity,
    T1: ::unity2::ClassIdentity,
> {
    #[rename(name = "_enumerator")]
    pub enumerator: crate::system::collections::generic::ienumerator_1::IEnumerator_1<
        crate::system::collections::generic::keyvaluepair_2::KeyValuePair_2<T0, T1>,
    >,
}

#[cfg(feature = "system-collections-concurrent-concurrentdictionary_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity>
    ConcurrentDictionary_2_DictionaryEnumerator<T0, T1>
{
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        dictionary : crate :: system :: collections :: concurrent :: concurrentdictionary_2 :: ConcurrentDictionary_2 < T0 , T1 >,
    ) -> ();

    #[method(name = "get_Entry", args = 0)]
    pub fn get_entry(self) -> crate::system::collections::dictionaryentry::DictionaryEntry;

    #[method(name = "get_Key", args = 0)]
    pub fn get_key(self) -> crate::system::object::Object;

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> crate::system::object::Object;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();
}

#[cfg(feature = "system-collections-concurrent-concurrentdictionary_2")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity>
    ConcurrentDictionary_2_DictionaryEnumerator<T0, T1>
{
    pub fn new(
        dictionary : crate :: system :: collections :: concurrent :: concurrentdictionary_2 :: ConcurrentDictionary_2 < T0 , T1 >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConcurrentDictionary_2_DictionaryEnumerator),
                ::core::stringify!(new),
            )
        });
        <Self as IConcurrentDictionary_2_DictionaryEnumeratorMethods<T0, T1>>::ctor(
            this, dictionary,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/concurrent/concurrentdictionary_2/ConcurrentDictionary_2_Node.md")))]
#[::unity2::class(
    namespace = "System.Collections.Concurrent",
    name = "ConcurrentDictionary`2.Node"
)]
pub struct ConcurrentDictionary_2_Node<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {
    #[rename(name = "_key")]
    pub key: T0,
    #[rename(name = "_value")]
    pub value: T1,
    #[rename(name = "_next")]
    pub next:
        crate::system::collections::concurrent::concurrentdictionary_2::ConcurrentDictionary_2_Node<
            T0,
            T1,
        >,
    #[rename(name = "_hashcode")]
    pub hashcode: i32,
}

#[cfg(feature = "system-collections-concurrent-concurrentdictionary_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> ConcurrentDictionary_2_Node<T0, T1> {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        key: T0,
        value: T1,
        hashcode: i32,
        next : crate :: system :: collections :: concurrent :: concurrentdictionary_2 :: ConcurrentDictionary_2_Node < T0 , T1 >,
    ) -> ();
}

#[cfg(feature = "system-collections-concurrent-concurrentdictionary_2")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> ConcurrentDictionary_2_Node<T0, T1> {
    pub fn new(
        key: T0,
        value: T1,
        hashcode: i32,
        next : crate :: system :: collections :: concurrent :: concurrentdictionary_2 :: ConcurrentDictionary_2_Node < T0 , T1 >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConcurrentDictionary_2_Node),
                ::core::stringify!(new),
            )
        });
        <Self as IConcurrentDictionary_2_NodeMethods<T0, T1>>::ctor(
            this, key, value, hashcode, next,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/concurrent/concurrentdictionary_2/ConcurrentDictionary_2.md")))]
#[::unity2::class(
    namespace = "System.Collections.Concurrent",
    name = "ConcurrentDictionary`2"
)]
pub struct ConcurrentDictionary_2 < T0 : :: unity2 :: ClassIdentity , T1 : :: unity2 :: ClassIdentity > {
# [rename (name = "_tables")] pub tables : crate :: system :: collections :: concurrent :: concurrentdictionary_2 :: ConcurrentDictionary_2_Tables < T0 , T1 > ,
# [rename (name = "_comparer")] pub comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 > ,
# [rename (name = "_growLockArray")] pub grow_lock_array : bool ,
# [rename (name = "_budget")] pub budget : i32 ,
# [static_field] # [rename (name = "s_isValueWriteAtomic")] pub s_is_value_write_atomic : bool ,
}

#[cfg(feature = "system-collections-concurrent-concurrentdictionary_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> ConcurrentDictionary_2<T0, T1> {
    #[method(name = "IsValueWriteAtomic", args = 0)]
    pub fn is_value_write_atomic() -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 4)]
    pub fn ctor_2(
        self,
        concurrency_level: i32,
        capacity: i32,
        grow_lock_array: bool,
        comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 >,
    ) -> ();

    #[method(name = "TryAdd", args = 2)]
    pub fn try_add(self, key: T0, value: T1) -> bool;

    #[method(name = "ContainsKey", args = 1)]
    pub fn contains_key(self, key: T0) -> bool;

    #[method(name = "TryRemove", args = 2)]
    pub fn try_remove(self, key: T0, value: T1) -> bool;

    #[method(name = "TryRemoveInternal", args = 4)]
    pub fn try_remove_internal(self, key: T0, value: T1, match_value: bool, old_value: T1) -> bool;

    #[method(name = "TryGetValue", args = 2)]
    pub fn try_get_value(self, key: T0, value: T1) -> bool;

    #[method(name = "TryGetValueInternal", args = 3)]
    pub fn try_get_value_internal(self, key: T0, hashcode: i32, value: T1) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "CopyToPairs", args = 2)]
    pub fn copy_to_pairs(
        self,
        array: ::unity2::Array<
            crate::system::collections::generic::keyvaluepair_2::KeyValuePair_2<T0, T1>,
        >,
        index: i32,
    ) -> ();

    #[method(name = "CopyToEntries", args = 2)]
    pub fn copy_to_entries(
        self,
        array: ::unity2::Array<crate::system::collections::dictionaryentry::DictionaryEntry>,
        index: i32,
    ) -> ();

    #[method(name = "CopyToObjects", args = 2)]
    pub fn copy_to_objects(
        self,
        array: ::unity2::Array<crate::system::object::Object>,
        index: i32,
    ) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::generic::ienumerator_1::IEnumerator_1<
        crate::system::collections::generic::keyvaluepair_2::KeyValuePair_2<T0, T1>,
    >;

    #[method(name = "TryAddInternal", args = 6)]
    pub fn try_add_internal(
        self,
        key: T0,
        hashcode: i32,
        value: T1,
        update_if_exists: bool,
        acquire_lock: bool,
        resulting_value: T1,
    ) -> bool;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, key: T0, value: T1) -> ();

    #[method(name = "ThrowKeyNullException", args = 0)]
    pub fn throw_key_null_exception() -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "GetCountInternal", args = 0)]
    pub fn get_count_internal(self) -> i32;

    #[method(name = "GetOrAdd", args = 2)]
    pub fn get_or_add(self, key: T0, value_factory: crate::system::func_2::Func_2<T0, T1>) -> T1;

    #[method(name = "get_Keys", args = 0)]
    pub fn get_keys(self) -> crate::system::collections::generic::icollection_1::ICollection_1<T0>;

    #[method(name = "get_Values", args = 0)]
    pub fn get_values(
        self,
    ) -> crate::system::collections::generic::icollection_1::ICollection_1<T1>;

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "System.Collections.IDictionary.Add", args = 2)]
    pub fn system_collections_i_dictionary_add(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "System.Collections.IDictionary.Contains", args = 1)]
    pub fn system_collections_i_dictionary_contains(
        self,
        key: crate::system::object::Object,
    ) -> bool;

    #[method(name = "System.Collections.IDictionary.GetEnumerator", args = 0)]
    pub fn system_collections_i_dictionary_get_enumerator(
        self,
    ) -> crate::system::collections::idictionaryenumerator::IDictionaryEnumerator;

    #[method(name = "System.Collections.IDictionary.get_IsFixedSize", args = 0)]
    pub fn system_collections_i_dictionary_get_is_fixed_size(self) -> bool;

    #[method(name = "System.Collections.IDictionary.get_IsReadOnly", args = 0)]
    pub fn system_collections_i_dictionary_get_is_read_only(self) -> bool;

    #[method(name = "System.Collections.IDictionary.get_Keys", args = 0)]
    pub fn system_collections_i_dictionary_get_keys(
        self,
    ) -> crate::system::collections::icollection::ICollection;

    #[method(name = "System.Collections.IDictionary.Remove", args = 1)]
    pub fn system_collections_i_dictionary_remove(self, key: crate::system::object::Object) -> ();

    #[method(name = "System.Collections.IDictionary.get_Values", args = 0)]
    pub fn system_collections_i_dictionary_get_values(
        self,
    ) -> crate::system::collections::icollection::ICollection;

    #[method(name = "System.Collections.IDictionary.get_Item", args = 1)]
    pub fn system_collections_i_dictionary_get_item(
        self,
        key: crate::system::object::Object,
    ) -> crate::system::object::Object;

    #[method(name = "System.Collections.IDictionary.set_Item", args = 2)]
    pub fn system_collections_i_dictionary_set_item(
        self,
        key: crate::system::object::Object,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "System.Collections.ICollection.CopyTo", args = 2)]
    pub fn system_collections_i_collection_copy_to(
        self,
        array: ::unity2::IlInstance,
        index: i32,
    ) -> ();

    #[method(name = "System.Collections.ICollection.get_IsSynchronized", args = 0)]
    pub fn system_collections_i_collection_get_is_synchronized(self) -> bool;

    #[method(name = "System.Collections.ICollection.get_SyncRoot", args = 0)]
    pub fn system_collections_i_collection_get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "GrowTable", args = 1)]
    pub fn grow_table(
        self,
        tables : crate :: system :: collections :: concurrent :: concurrentdictionary_2 :: ConcurrentDictionary_2_Tables < T0 , T1 >,
    ) -> ();

    #[method(name = "GetBucket", args = 2)]
    pub fn get_bucket(hashcode: i32, bucket_count: i32) -> i32;

    #[method(name = "GetBucketAndLockNo", args = 5)]
    pub fn get_bucket_and_lock_no(
        hashcode: i32,
        bucket_no: i32,
        lock_no: i32,
        bucket_count: i32,
        lock_count: i32,
    ) -> ();

    #[method(name = "get_DefaultConcurrencyLevel", args = 0)]
    pub fn get_default_concurrency_level() -> i32;

    #[method(name = "AcquireAllLocks", args = 1)]
    pub fn acquire_all_locks(self, locks_acquired: i32) -> ();

    #[method(name = "AcquireLocks", args = 3)]
    pub fn acquire_locks(self, from_inclusive: i32, to_exclusive: i32, locks_acquired: i32) -> ();

    #[method(name = "ReleaseLocks", args = 2)]
    pub fn release_locks(self, from_inclusive: i32, to_exclusive: i32) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "system-collections-concurrent-concurrentdictionary_2")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> ConcurrentDictionary_2<T0, T1> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConcurrentDictionary_2),
                ::core::stringify!(new),
            )
        });
        <Self as IConcurrentDictionary_2Methods<T0, T1>>::ctor(this);
        this
    }

    pub fn new_2(
        concurrency_level: i32,
        capacity: i32,
        grow_lock_array: bool,
        comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ConcurrentDictionary_2),
                ::core::stringify!(new_2),
            )
        });
        <Self as IConcurrentDictionary_2Methods<T0, T1>>::ctor_2(
            this,
            concurrency_level,
            capacity,
            grow_lock_array,
            comparer,
        );
        this
    }
}
