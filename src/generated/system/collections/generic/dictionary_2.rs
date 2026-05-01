
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/dictionary_2/Dictionary_2_Entry.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Dictionary_2_Entry<T0, T1> {
    pub _phantom: ::core::marker::PhantomData<(T0, T1)>,
}

impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> ::unity2::ClassIdentity
    for Dictionary_2_Entry<T0, T1>
{
    const NAMESPACE: &'static str = "System.Collections.Generic";

    const NAME: &'static str = "Dictionary`2.Entry";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| {
            ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME)
                .make_generic(&[
                    <T0 as ::unity2::ClassIdentity>::class(),
                    <T1 as ::unity2::ClassIdentity>::class(),
                ])
                .expect("generic instantiation")
        })
    }
}

impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> ::unity2::IlType
    for Dictionary_2_Entry<T0, T1>
{
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/dictionary_2/Dictionary_2_ValueCollection.md")))]
#[::unity2::class(
    namespace = "System.Collections.Generic",
    name = "Dictionary`2.ValueCollection"
)]
pub struct Dictionary_2_ValueCollection<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {
    #[rename(name = "dictionary")]
    pub dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<T0, T1>,
}

#[cfg(feature = "system-collections-generic-dictionary_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity>
    Dictionary_2_ValueCollection<T0, T1>
{
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<T0, T1>,
    ) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator (self ,) -> crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2_ValueCollection_Enumerator < T0 , T1 > ;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::Array<T1>, index: i32) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

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
}

#[cfg(feature = "system-collections-generic-dictionary_2")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity>
    Dictionary_2_ValueCollection<T0, T1>
{
    pub fn new(
        dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<T0, T1>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Dictionary_2_ValueCollection),
                ::core::stringify!(new),
            )
        });
        <Self as IDictionary_2_ValueCollectionMethods<T0, T1>>::ctor(this, dictionary);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/dictionary_2/Dictionary_2_ValueCollection_Enumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Dictionary_2_ValueCollection_Enumerator<T0, T1> {
    pub _phantom: ::core::marker::PhantomData<(T0, T1)>,
}

impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> ::unity2::ClassIdentity
    for Dictionary_2_ValueCollection_Enumerator<T0, T1>
{
    const NAMESPACE: &'static str = "System.Collections.Generic";

    const NAME: &'static str = "Dictionary`2.ValueCollection.Enumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| {
            ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME)
                .make_generic(&[
                    <T0 as ::unity2::ClassIdentity>::class(),
                    <T1 as ::unity2::ClassIdentity>::class(),
                ])
                .expect("generic instantiation")
        })
    }
}

impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> ::unity2::IlType
    for Dictionary_2_ValueCollection_Enumerator<T0, T1>
{
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-collections-generic-dictionary_2")]
#[::unity2::methods(value)]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity>
    Dictionary_2_ValueCollection_Enumerator<T0, T1>
{
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<T0, T1>,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> T1;

    #[method(name = "System.Collections.IEnumerator.get_Current", args = 0)]
    pub fn system_collections_i_enumerator_get_current(self) -> crate::system::object::Object;

    #[method(name = "System.Collections.IEnumerator.Reset", args = 0)]
    pub fn system_collections_i_enumerator_reset(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/dictionary_2/Dictionary_2_Enumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Dictionary_2_Enumerator<T0, T1> {
    pub _phantom: ::core::marker::PhantomData<(T0, T1)>,
}

impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> ::unity2::ClassIdentity
    for Dictionary_2_Enumerator<T0, T1>
{
    const NAMESPACE: &'static str = "System.Collections.Generic";

    const NAME: &'static str = "Dictionary`2.Enumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| {
            ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME)
                .make_generic(&[
                    <T0 as ::unity2::ClassIdentity>::class(),
                    <T1 as ::unity2::ClassIdentity>::class(),
                ])
                .expect("generic instantiation")
        })
    }
}

impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> ::unity2::IlType
    for Dictionary_2_Enumerator<T0, T1>
{
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-collections-generic-dictionary_2")]
#[::unity2::methods(value)]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> Dictionary_2_Enumerator<T0, T1> {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<T0, T1>,
        get_enumerator_ret_type: i32,
    ) -> ();

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(
        self,
    ) -> crate::system::collections::generic::keyvaluepair_2::KeyValuePair_2<T0, T1>;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "System.Collections.IEnumerator.get_Current", args = 0)]
    pub fn system_collections_i_enumerator_get_current(self) -> crate::system::object::Object;

    #[method(name = "System.Collections.IEnumerator.Reset", args = 0)]
    pub fn system_collections_i_enumerator_reset(self) -> ();

    #[method(name = "System.Collections.IDictionaryEnumerator.get_Entry", args = 0)]
    pub fn system_collections_i_dictionary_enumerator_get_entry(
        self,
    ) -> crate::system::collections::dictionaryentry::DictionaryEntry;

    #[method(name = "System.Collections.IDictionaryEnumerator.get_Key", args = 0)]
    pub fn system_collections_i_dictionary_enumerator_get_key(
        self,
    ) -> crate::system::object::Object;

    #[method(name = "System.Collections.IDictionaryEnumerator.get_Value", args = 0)]
    pub fn system_collections_i_dictionary_enumerator_get_value(
        self,
    ) -> crate::system::object::Object;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/dictionary_2/Dictionary_2_KeyCollection_Enumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Dictionary_2_KeyCollection_Enumerator<T0, T1> {
    pub _phantom: ::core::marker::PhantomData<(T0, T1)>,
}

impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> ::unity2::ClassIdentity
    for Dictionary_2_KeyCollection_Enumerator<T0, T1>
{
    const NAMESPACE: &'static str = "System.Collections.Generic";

    const NAME: &'static str = "Dictionary`2.KeyCollection.Enumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| {
            ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME)
                .make_generic(&[
                    <T0 as ::unity2::ClassIdentity>::class(),
                    <T1 as ::unity2::ClassIdentity>::class(),
                ])
                .expect("generic instantiation")
        })
    }
}

impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> ::unity2::IlType
    for Dictionary_2_KeyCollection_Enumerator<T0, T1>
{
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-collections-generic-dictionary_2")]
#[::unity2::methods(value)]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity>
    Dictionary_2_KeyCollection_Enumerator<T0, T1>
{
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<T0, T1>,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> T0;

    #[method(name = "System.Collections.IEnumerator.get_Current", args = 0)]
    pub fn system_collections_i_enumerator_get_current(self) -> crate::system::object::Object;

    #[method(name = "System.Collections.IEnumerator.Reset", args = 0)]
    pub fn system_collections_i_enumerator_reset(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/dictionary_2/Dictionary_2.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "Dictionary`2")]
pub struct Dictionary_2 < T0 : :: unity2 :: ClassIdentity , T1 : :: unity2 :: ClassIdentity > {
# [rename (name = "buckets")] pub buckets : :: unity2 :: Array < i32 > ,
# [rename (name = "entries")] pub entries : :: unity2 :: Array < crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2_Entry < T0 , T1 > > ,
# [rename (name = "count")] pub count : i32 ,
# [rename (name = "version")] pub version : i32 ,
# [rename (name = "freeList")] pub free_list : i32 ,
# [rename (name = "freeCount")] pub free_count : i32 ,
# [rename (name = "comparer")] pub comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 > ,
# [rename (name = "keys")] pub keys : crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2_KeyCollection < T0 , T1 > ,
# [rename (name = "values")] pub values : crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2_ValueCollection < T0 , T1 > ,
# [rename (name = "_syncRoot")] pub sync_root : :: unity2 :: IlInstance ,
# [static_field] # [rename (name = "VersionName")] pub version_name : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "HashSizeName")] pub hash_size_name : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "KeyValuePairsName")] pub key_value_pairs_name : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "ComparerName")] pub comparer_name : :: unity2 :: Il2CppString ,
}

#[cfg(feature = "system-collections-generic-dictionary_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> Dictionary_2<T0, T1> {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, capacity: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(
        self,
        comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 >,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_4(
        self,
        capacity: i32,
        comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 >,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_5(
        self,
        dictionary : crate :: system :: collections :: generic :: idictionary_2_interface :: IDictionary_2_Interface < T0 , T1 >,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_6(
        self,
        dictionary : crate :: system :: collections :: generic :: idictionary_2_interface :: IDictionary_2_Interface < T0 , T1 >,
        comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 >,
    ) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Keys", args = 0)]
    pub fn get_keys(
        self,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2_KeyCollection<T0, T1>;

    #[method(name = "get_Values", args = 0)]
    pub fn get_values(
        self,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2_ValueCollection<T0, T1>;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, key: T0) -> T1;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, key: T0, value: T1) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(self, key: T0, value: T1) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "ContainsKey", args = 1)]
    pub fn contains_key(self, key: T0) -> bool;

    #[method(name = "ContainsValue", args = 1)]
    pub fn contains_value(self, value: T1) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(
        self,
        array: ::unity2::Array<
            crate::system::collections::generic::keyvaluepair_2::KeyValuePair_2<T0, T1>,
        >,
        index: i32,
    ) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::generic::dictionary_2::Dictionary_2_Enumerator<T0, T1>;

    #[method(name = "FindEntry", args = 1)]
    pub fn find_entry(self, key: T0) -> i32;

    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, capacity: i32) -> ();

    #[method(name = "TryInsert", args = 3)]
    pub fn try_insert(
        self,
        key: T0,
        value: T1,
        behavior: crate::system::collections::generic::insertionbehavior::InsertionBehavior,
    ) -> bool;

    #[method(name = "OnDeserialization", args = 1)]
    pub fn on_deserialization(self, sender: crate::system::object::Object) -> ();

    #[method(name = "Resize", args = 0)]
    pub fn resize(self) -> ();

    #[method(name = "Resize", args = 2)]
    pub fn resize_2(self, new_size: i32, force_new_hash_codes: bool) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, key: T0) -> bool;

    #[method(name = "TryGetValue", args = 2)]
    pub fn try_get_value(self, key: T0, value: T1) -> bool;

    #[method(name = "TryAdd", args = 2)]
    pub fn try_add(self, key: T0, value: T1) -> bool;

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

    #[method(name = "System.Collections.ICollection.get_IsSynchronized", args = 0)]
    pub fn system_collections_i_collection_get_is_synchronized(self) -> bool;

    #[method(name = "System.Collections.ICollection.get_SyncRoot", args = 0)]
    pub fn system_collections_i_collection_get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "System.Collections.IDictionary.get_IsFixedSize", args = 0)]
    pub fn system_collections_i_dictionary_get_is_fixed_size(self) -> bool;

    #[method(name = "System.Collections.IDictionary.get_IsReadOnly", args = 0)]
    pub fn system_collections_i_dictionary_get_is_read_only(self) -> bool;

    #[method(name = "System.Collections.IDictionary.get_Keys", args = 0)]
    pub fn system_collections_i_dictionary_get_keys(
        self,
    ) -> crate::system::collections::icollection::ICollection;

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

    #[method(name = "IsCompatibleKey", args = 1)]
    pub fn is_compatible_key(key: crate::system::object::Object) -> bool;

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

    #[method(name = "System.Collections.IDictionary.Remove", args = 1)]
    pub fn system_collections_i_dictionary_remove(self, key: crate::system::object::Object) -> ();
}

#[cfg(feature = "system-collections-generic-dictionary_2")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> Dictionary_2<T0, T1> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Dictionary_2),
                ::core::stringify!(new),
            )
        });
        <Self as IDictionary_2Methods<T0, T1>>::ctor(this);
        this
    }

    pub fn new_2(capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Dictionary_2),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDictionary_2Methods<T0, T1>>::ctor_2(this, capacity);
        this
    }

    pub fn new_3(
        comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Dictionary_2),
                ::core::stringify!(new_3),
            )
        });
        <Self as IDictionary_2Methods<T0, T1>>::ctor_3(this, comparer);
        this
    }

    pub fn new_4(
        capacity: i32,
        comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Dictionary_2),
                ::core::stringify!(new_4),
            )
        });
        <Self as IDictionary_2Methods<T0, T1>>::ctor_4(this, capacity, comparer);
        this
    }

    pub fn new_5(
        dictionary : crate :: system :: collections :: generic :: idictionary_2_interface :: IDictionary_2_Interface < T0 , T1 >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Dictionary_2),
                ::core::stringify!(new_5),
            )
        });
        <Self as IDictionary_2Methods<T0, T1>>::ctor_5(this, dictionary);
        this
    }

    pub fn new_6(
        dictionary : crate :: system :: collections :: generic :: idictionary_2_interface :: IDictionary_2_Interface < T0 , T1 >,
        comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Dictionary_2),
                ::core::stringify!(new_6),
            )
        });
        <Self as IDictionary_2Methods<T0, T1>>::ctor_6(this, dictionary, comparer);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/dictionary_2/Dictionary_2_KeyCollection.md")))]
#[::unity2::class(
    namespace = "System.Collections.Generic",
    name = "Dictionary`2.KeyCollection"
)]
pub struct Dictionary_2_KeyCollection<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {
    #[rename(name = "dictionary")]
    pub dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<T0, T1>,
}

#[cfg(feature = "system-collections-generic-dictionary_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> Dictionary_2_KeyCollection<T0, T1> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<T0, T1>,
    ) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator (self ,) -> crate :: system :: collections :: generic :: dictionary_2 :: Dictionary_2_KeyCollection_Enumerator < T0 , T1 > ;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::Array<T0>, index: i32) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

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
}

#[cfg(feature = "system-collections-generic-dictionary_2")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> Dictionary_2_KeyCollection<T0, T1> {
    pub fn new(
        dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<T0, T1>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Dictionary_2_KeyCollection),
                ::core::stringify!(new),
            )
        });
        <Self as IDictionary_2_KeyCollectionMethods<T0, T1>>::ctor(this, dictionary);
        this
    }
}
