
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/list_1/List_1.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "List`1")]
pub struct List_1<T0: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "_defaultCapacity")]
    pub default_capacity: i32,
    #[rename(name = "_items")]
    pub items: ::unity2::Array<T0>,
    #[rename(name = "_size")]
    pub size: i32,
    #[rename(name = "_version")]
    pub version: i32,
    #[rename(name = "_syncRoot")]
    pub sync_root: ::unity2::IlInstance,
    #[static_field]
    #[rename(name = "_emptyArray")]
    pub empty_array: ::unity2::Array<T0>,
}

#[cfg(feature = "system-collections-generic-list_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> List_1<T0> {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, capacity: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(
        self,
        collection: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
    ) -> ();

    #[method(name = "get_Capacity", args = 0)]
    pub fn get_capacity(self) -> i32;

    #[method(name = "set_Capacity", args = 1)]
    pub fn set_capacity(self, value: i32) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "System.Collections.IList.get_IsReadOnly", args = 0)]
    pub fn system_collections_i_list_get_is_read_only(self) -> bool;

    #[method(name = "System.Collections.ICollection.get_IsSynchronized", args = 0)]
    pub fn system_collections_i_collection_get_is_synchronized(self) -> bool;

    #[method(name = "System.Collections.ICollection.get_SyncRoot", args = 0)]
    pub fn system_collections_i_collection_get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> T0;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: T0) -> ();

    #[method(name = "IsCompatibleObject", args = 1)]
    pub fn is_compatible_object(value: crate::system::object::Object) -> bool;

    #[method(name = "System.Collections.IList.get_Item", args = 1)]
    pub fn system_collections_i_list_get_item(self, index: i32) -> crate::system::object::Object;

    #[method(name = "System.Collections.IList.set_Item", args = 2)]
    pub fn system_collections_i_list_set_item(
        self,
        index: i32,
        value: crate::system::object::Object,
    ) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, item: T0) -> ();

    #[method(name = "System.Collections.IList.Add", args = 1)]
    pub fn system_collections_i_list_add(self, item: crate::system::object::Object) -> i32;

    #[method(name = "AddRange", args = 1)]
    pub fn add_range(
        self,
        collection: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
    ) -> ();

    #[method(name = "AsReadOnly", args = 0)]
    pub fn as_read_only(
        self,
    ) -> crate::system::collections::object_model::readonlycollection_1::ReadOnlyCollection_1<T0>;

    #[method(name = "BinarySearch", args = 4)]
    pub fn binary_search(
        self,
        index: i32,
        count: i32,
        item: T0,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> i32;

    #[method(name = "BinarySearch", args = 2)]
    pub fn binary_search_2(
        self,
        item: T0,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> i32;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, item: T0) -> bool;

    #[method(name = "System.Collections.IList.Contains", args = 1)]
    pub fn system_collections_i_list_contains(self, item: crate::system::object::Object) -> bool;

    #[method(name = "CopyTo", args = 1)]
    pub fn copy_to(self, array: ::unity2::Array<T0>) -> ();

    #[method(name = "System.Collections.ICollection.CopyTo", args = 2)]
    pub fn system_collections_i_collection_copy_to(
        self,
        array: ::unity2::IlInstance,
        array_index: i32,
    ) -> ();

    #[method(name = "CopyTo", args = 4)]
    pub fn copy_to_2(
        self,
        index: i32,
        array: ::unity2::Array<T0>,
        array_index: i32,
        count: i32,
    ) -> ();

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to_3(self, array: ::unity2::Array<T0>, array_index: i32) -> ();

    #[method(name = "EnsureCapacity", args = 1)]
    pub fn ensure_capacity(self, min: i32) -> ();

    #[method(name = "Exists", args = 1)]
    pub fn exists(self, r#match: crate::system::predicate_1::Predicate_1<T0>) -> bool;

    #[method(name = "Find", args = 1)]
    pub fn find(self, r#match: crate::system::predicate_1::Predicate_1<T0>) -> T0;

    #[method(name = "FindAll", args = 1)]
    pub fn find_all(
        self,
        r#match: crate::system::predicate_1::Predicate_1<T0>,
    ) -> crate::system::collections::generic::list_1::List_1<T0>;

    #[method(name = "FindIndex", args = 1)]
    pub fn find_index(self, r#match: crate::system::predicate_1::Predicate_1<T0>) -> i32;

    #[method(name = "FindIndex", args = 3)]
    pub fn find_index_2(
        self,
        start_index: i32,
        count: i32,
        r#match: crate::system::predicate_1::Predicate_1<T0>,
    ) -> i32;

    #[method(name = "ForEach", args = 1)]
    pub fn for_each(self, action: crate::system::action_1::Action_1<T0>) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::generic::list_1::List_1_Enumerator<T0>;

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of(self, item: T0) -> i32;

    #[method(name = "System.Collections.IList.IndexOf", args = 1)]
    pub fn system_collections_i_list_index_of(self, item: crate::system::object::Object) -> i32;

    #[method(name = "Insert", args = 2)]
    pub fn insert(self, index: i32, item: T0) -> ();

    #[method(name = "System.Collections.IList.Insert", args = 2)]
    pub fn system_collections_i_list_insert(
        self,
        index: i32,
        item: crate::system::object::Object,
    ) -> ();

    #[method(name = "InsertRange", args = 2)]
    pub fn insert_range(
        self,
        index: i32,
        collection: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
    ) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, item: T0) -> bool;

    #[method(name = "System.Collections.IList.Remove", args = 1)]
    pub fn system_collections_i_list_remove(self, item: crate::system::object::Object) -> ();

    #[method(name = "RemoveAll", args = 1)]
    pub fn remove_all(self, r#match: crate::system::predicate_1::Predicate_1<T0>) -> i32;

    #[method(name = "RemoveAt", args = 1)]
    pub fn remove_at(self, index: i32) -> ();

    #[method(name = "RemoveRange", args = 2)]
    pub fn remove_range(self, index: i32, count: i32) -> ();

    #[method(name = "Reverse", args = 0)]
    pub fn reverse(self) -> ();

    #[method(name = "Reverse", args = 2)]
    pub fn reverse_2(self, index: i32, count: i32) -> ();

    #[method(name = "Sort", args = 0)]
    pub fn sort(self) -> ();

    #[method(name = "Sort", args = 1)]
    pub fn sort_2(
        self,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> ();

    #[method(name = "Sort", args = 3)]
    pub fn sort_3(
        self,
        index: i32,
        count: i32,
        comparer: crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<
            T0,
        >,
    ) -> ();

    #[method(name = "Sort", args = 1)]
    pub fn sort_4(self, comparison: crate::system::comparison_1::Comparison_1<T0>) -> ();

    #[method(name = "ToArray", args = 0)]
    pub fn to_array(self) -> ::unity2::Array<T0>;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "system-collections-generic-list_1")]
impl<T0: ::unity2::ClassIdentity> List_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(List_1),
                ::core::stringify!(new),
            )
        });
        <Self as IList_1Methods<T0>>::ctor(this);
        this
    }

    pub fn new_2(capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(List_1),
                ::core::stringify!(new_2),
            )
        });
        <Self as IList_1Methods<T0>>::ctor_2(this, capacity);
        this
    }

    pub fn new_3(
        collection: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(List_1),
                ::core::stringify!(new_3),
            )
        });
        <Self as IList_1Methods<T0>>::ctor_3(this, collection);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/list_1/List_1_Enumerator.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct List_1_Enumerator<T0> {
    pub _phantom: ::core::marker::PhantomData<(T0,)>,
}

impl<T0: ::unity2::ClassIdentity> ::unity2::ClassIdentity for List_1_Enumerator<T0> {
    const NAMESPACE: &'static str = "System.Collections.Generic";

    const NAME: &'static str = "List`1.Enumerator";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| {
            ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME)
                .make_generic(&[<T0 as ::unity2::ClassIdentity>::class()])
                .expect("generic instantiation")
        })
    }
}

impl<T0: ::unity2::ClassIdentity> ::unity2::IlType for List_1_Enumerator<T0> {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "system-collections-generic-list_1")]
#[::unity2::methods(value)]
impl<T0: ::unity2::ClassIdentity> List_1_Enumerator<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, list: crate::system::collections::generic::list_1::List_1<T0>) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "MoveNextRare", args = 0)]
    pub fn move_next_rare(self) -> bool;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> T0;

    #[method(name = "System.Collections.IEnumerator.get_Current", args = 0)]
    pub fn system_collections_i_enumerator_get_current(self) -> crate::system::object::Object;

    #[method(name = "System.Collections.IEnumerator.Reset", args = 0)]
    pub fn system_collections_i_enumerator_reset(self) -> ();
}
