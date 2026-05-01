
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/hashset_1/HashSet_1_ElementCount.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct HashSet_1_ElementCount<T0> {
    pub _phantom: ::core::marker::PhantomData<(T0,)>,
}

impl<T0: ::unity2::ClassIdentity> ::unity2::ClassIdentity for HashSet_1_ElementCount<T0> {
    const NAMESPACE: &'static str = "System.Collections.Generic";

    const NAME: &'static str = "HashSet`1.ElementCount";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| {
            ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME)
                .make_generic(&[<T0 as ::unity2::ClassIdentity>::class()])
                .expect("generic instantiation")
        })
    }
}

impl<T0: ::unity2::ClassIdentity> ::unity2::IlType for HashSet_1_ElementCount<T0> {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/hashset_1/HashSet_1.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "HashSet`1")]
pub struct HashSet_1 < T0 : :: unity2 :: ClassIdentity > {
# [static_field] # [rename (name = "Lower31BitMask")] pub lower31_bit_mask : i32 ,
# [static_field] # [rename (name = "StackAllocThreshold")] pub stack_alloc_threshold : i32 ,
# [static_field] # [rename (name = "ShrinkThreshold")] pub shrink_threshold : i32 ,
# [static_field] # [rename (name = "CapacityName")] pub capacity_name : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "ElementsName")] pub elements_name : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "ComparerName")] pub comparer_name : :: unity2 :: Il2CppString ,
# [static_field] # [rename (name = "VersionName")] pub version_name : :: unity2 :: Il2CppString ,
# [rename (name = "_buckets")] pub buckets : :: unity2 :: Array < i32 > ,
# [rename (name = "_slots")] pub slots : :: unity2 :: Array < crate :: system :: collections :: generic :: hashset_1 :: HashSet_1_Slot < T0 > > ,
# [rename (name = "_count")] pub count : i32 ,
# [rename (name = "_lastIndex")] pub last_index : i32 ,
# [rename (name = "_freeList")] pub free_list : i32 ,
# [rename (name = "_comparer")] pub comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 > ,
# [rename (name = "_version")] pub version : i32 ,
}

#[cfg(feature = "system-collections-generic-hashset_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> HashSet_1<T0> {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 >,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(
        self,
        collection: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_4(
        self,
        collection: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
        comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 >,
    ) -> ();

    #[method(name = "CopyFrom", args = 1)]
    pub fn copy_from(
        self,
        source: crate::system::collections::generic::hashset_1::HashSet_1<T0>,
    ) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, item: T0) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::Array<T0>, array_index: i32) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, item: T0) -> bool;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::generic::hashset_1::HashSet_1_Enumerator<T0>;

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "OnDeserialization", args = 1)]
    pub fn on_deserialization(self, sender: crate::system::object::Object) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, item: T0) -> bool;

    #[method(name = "UnionWith", args = 1)]
    pub fn union_with(
        self,
        other: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
    ) -> ();

    #[method(name = "IntersectWith", args = 1)]
    pub fn intersect_with(
        self,
        other: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
    ) -> ();

    #[method(name = "SetEquals", args = 1)]
    pub fn set_equals(
        self,
        other: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
    ) -> bool;

    #[method(name = "CopyTo", args = 1)]
    pub fn copy_to_2(self, array: ::unity2::Array<T0>) -> ();

    #[method(name = "CopyTo", args = 3)]
    pub fn copy_to_3(self, array: ::unity2::Array<T0>, array_index: i32, count: i32) -> ();

    #[method(name = "get_Comparer", args = 0)]
    pub fn get_comparer (self ,) -> crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 > ;

    #[method(name = "TrimExcess", args = 0)]
    pub fn trim_excess(self) -> ();

    #[method(name = "Initialize", args = 1)]
    pub fn initialize(self, capacity: i32) -> ();

    #[method(name = "IncreaseCapacity", args = 0)]
    pub fn increase_capacity(self) -> ();

    #[method(name = "SetCapacity", args = 1)]
    pub fn set_capacity(self, new_size: i32) -> ();

    #[method(name = "AddIfNotPresent", args = 1)]
    pub fn add_if_not_present(self, value: T0) -> bool;

    #[method(name = "AddValue", args = 3)]
    pub fn add_value(self, index: i32, hash_code: i32, value: T0) -> ();

    #[method(name = "ContainsAllElements", args = 1)]
    pub fn contains_all_elements(
        self,
        other: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
    ) -> bool;

    #[method(name = "IntersectWithHashSetWithSameEC", args = 1)]
    pub fn intersect_with_hash_set_with_same_ec(
        self,
        other: crate::system::collections::generic::hashset_1::HashSet_1<T0>,
    ) -> ();

    #[method(name = "IntersectWithEnumerable", args = 1)]
    pub fn intersect_with_enumerable(
        self,
        other: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
    ) -> ();

    #[method(name = "InternalIndexOf", args = 1)]
    pub fn internal_index_of(self, item: T0) -> i32;

    #[method(name = "CheckUniqueAndUnfoundElements", args = 2)]
    pub fn check_unique_and_unfound_elements(
        self,
        other: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
        return_if_unfound: bool,
    ) -> crate::system::collections::generic::hashset_1::HashSet_1_ElementCount<T0>;

    #[method(name = "AreEqualityComparersEqual", args = 2)]
    pub fn are_equality_comparers_equal(
        set1: crate::system::collections::generic::hashset_1::HashSet_1<T0>,
        set2: crate::system::collections::generic::hashset_1::HashSet_1<T0>,
    ) -> bool;

    #[method(name = "InternalGetHashCode", args = 1)]
    pub fn internal_get_hash_code(self, item: T0) -> i32;
}

#[cfg(feature = "system-collections-generic-hashset_1")]
impl<T0: ::unity2::ClassIdentity> HashSet_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HashSet_1),
                ::core::stringify!(new),
            )
        });
        <Self as IHashSet_1Methods<T0>>::ctor(this);
        this
    }

    pub fn new_2(
        comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HashSet_1),
                ::core::stringify!(new_2),
            )
        });
        <Self as IHashSet_1Methods<T0>>::ctor_2(this, comparer);
        this
    }

    pub fn new_3(
        collection: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HashSet_1),
                ::core::stringify!(new_3),
            )
        });
        <Self as IHashSet_1Methods<T0>>::ctor_3(this, collection);
        this
    }

    pub fn new_4(
        collection: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
        comparer : crate :: system :: collections :: generic :: iequalitycomparer_1_interface :: IEqualityComparer_1_Interface < T0 >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HashSet_1),
                ::core::stringify!(new_4),
            )
        });
        <Self as IHashSet_1Methods<T0>>::ctor_4(this, collection, comparer);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/hashset_1/HashSet_1_Slot.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct HashSet_1_Slot<T0> {
    pub _phantom: ::core::marker::PhantomData<(T0,)>,
}

impl<T0: ::unity2::ClassIdentity> ::unity2::ClassIdentity for HashSet_1_Slot<T0> {
    const NAMESPACE: &'static str = "System.Collections.Generic";

    const NAME: &'static str = "HashSet`1.Slot";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| {
            ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME)
                .make_generic(&[<T0 as ::unity2::ClassIdentity>::class()])
                .expect("generic instantiation")
        })
    }
}

impl<T0: ::unity2::ClassIdentity> ::unity2::IlType for HashSet_1_Slot<T0> {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/hashset_1/HashSet_1_Enumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct HashSet_1_Enumerator<T0> {
    pub _phantom: ::core::marker::PhantomData<(T0,)>,
}

impl<T0: ::unity2::ClassIdentity> ::unity2::ClassIdentity for HashSet_1_Enumerator<T0> {
    const NAMESPACE: &'static str = "System.Collections.Generic";

    const NAME: &'static str = "HashSet`1.Enumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| {
            ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME)
                .make_generic(&[<T0 as ::unity2::ClassIdentity>::class()])
                .expect("generic instantiation")
        })
    }
}

impl<T0: ::unity2::ClassIdentity> ::unity2::IlType for HashSet_1_Enumerator<T0> {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-collections-generic-hashset_1")]
#[::unity2::methods(value)]
impl<T0: ::unity2::ClassIdentity> HashSet_1_Enumerator<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, set: crate::system::collections::generic::hashset_1::HashSet_1<T0>) -> ();

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
