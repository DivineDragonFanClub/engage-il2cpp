
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/arraylist/ArrayList_ReadOnlyArrayList.md")))]
#[::unity2::class(namespace = "System.Collections", name = "ArrayList.ReadOnlyArrayList")]
#[parent(crate::system::collections::arraylist::ArrayList)]
pub struct ArrayList_ReadOnlyArrayList {
    #[rename(name = "_list")]
    pub list: crate::system::collections::arraylist::ArrayList,
}

#[cfg(feature = "system-collections-arraylist")]
#[::unity2::methods]
impl ArrayList_ReadOnlyArrayList {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, l: crate::system::collections::arraylist::ArrayList) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "get_IsSynchronized", args = 0)]
    pub fn get_is_synchronized(self) -> bool;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> crate::system::object::Object;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: crate::system::object::Object) -> ();

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "Add", args = 1)]
    pub fn add(self, obj: crate::system::object::Object) -> i32;

    #[method(name = "AddRange", args = 1)]
    pub fn add_range(self, c: crate::system::collections::icollection::ICollection) -> ();

    #[method(name = "set_Capacity", args = 1)]
    pub fn set_capacity(self, value: i32) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, obj: crate::system::object::Object) -> bool;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, index: i32) -> ();

    #[method(name = "CopyTo", args = 4)]
    pub fn copy_to_2(
        self,
        index: i32,
        array: ::unity2::IlInstance,
        array_index: i32,
        count: i32,
    ) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of(self, value: crate::system::object::Object) -> i32;

    #[method(name = "Insert", args = 2)]
    pub fn insert(self, index: i32, obj: crate::system::object::Object) -> ();

    #[method(name = "InsertRange", args = 2)]
    pub fn insert_range(
        self,
        index: i32,
        c: crate::system::collections::icollection::ICollection,
    ) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, value: crate::system::object::Object) -> ();

    #[method(name = "RemoveAt", args = 1)]
    pub fn remove_at(self, index: i32) -> ();

    #[method(name = "RemoveRange", args = 2)]
    pub fn remove_range(self, index: i32, count: i32) -> ();

    #[method(name = "ToArray", args = 0)]
    pub fn to_array(self) -> ::unity2::Array<crate::system::object::Object>;

    #[method(name = "ToArray", args = 1)]
    pub fn to_array_2(self, r#type: ::unity2::SystemType) -> ::unity2::IlInstance;
}

#[cfg(feature = "system-collections-arraylist")]
impl ArrayList_ReadOnlyArrayList {
    pub fn new(l: crate::system::collections::arraylist::ArrayList) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArrayList_ReadOnlyArrayList),
                ::core::stringify!(new),
            )
        });
        <Self as IArrayList_ReadOnlyArrayListMethods>::ctor(this, l);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/arraylist/ArrayList.md")))]
#[::unity2::class(namespace = "System.Collections", name = "ArrayList")]
#[parent(crate::system::object::Object)]
pub struct ArrayList {
    #[rename(name = "_items")]
    pub items: ::unity2::Array<crate::system::object::Object>,
    #[rename(name = "_size")]
    pub size: i32,
    #[rename(name = "_version")]
    pub version: i32,
    #[rename(name = "_syncRoot")]
    pub sync_root: ::unity2::IlInstance,
    #[static_field]
    #[rename(name = "emptyArray")]
    pub empty_array: ::unity2::Array<crate::system::object::Object>,
}

#[cfg(feature = "system-collections-arraylist")]
#[::unity2::methods]
impl ArrayList {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, capacity: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, c: crate::system::collections::icollection::ICollection) -> ();

    #[method(name = "set_Capacity", args = 1)]
    pub fn set_capacity(self, value: i32) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = "get_IsSynchronized", args = 0)]
    pub fn get_is_synchronized(self) -> bool;

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> crate::system::object::Object;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: crate::system::object::Object) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, value: crate::system::object::Object) -> i32;

    #[method(name = "AddRange", args = 1)]
    pub fn add_range(self, c: crate::system::collections::icollection::ICollection) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, item: crate::system::object::Object) -> bool;

    #[method(name = "CopyTo", args = 1)]
    pub fn copy_to(self, array: ::unity2::IlInstance) -> ();

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to_2(self, array: ::unity2::IlInstance, array_index: i32) -> ();

    #[method(name = "CopyTo", args = 4)]
    pub fn copy_to_3(
        self,
        index: i32,
        array: ::unity2::IlInstance,
        array_index: i32,
        count: i32,
    ) -> ();

    #[method(name = "EnsureCapacity", args = 1)]
    pub fn ensure_capacity(self, min: i32) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of(self, value: crate::system::object::Object) -> i32;

    #[method(name = "Insert", args = 2)]
    pub fn insert(self, index: i32, value: crate::system::object::Object) -> ();

    #[method(name = "InsertRange", args = 2)]
    pub fn insert_range(
        self,
        index: i32,
        c: crate::system::collections::icollection::ICollection,
    ) -> ();

    #[method(name = "ReadOnly", args = 1)]
    pub fn read_only(
        list: crate::system::collections::arraylist::ArrayList,
    ) -> crate::system::collections::arraylist::ArrayList;

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, obj: crate::system::object::Object) -> ();

    #[method(name = "RemoveAt", args = 1)]
    pub fn remove_at(self, index: i32) -> ();

    #[method(name = "RemoveRange", args = 2)]
    pub fn remove_range(self, index: i32, count: i32) -> ();

    #[method(name = "ToArray", args = 0)]
    pub fn to_array(self) -> ::unity2::Array<crate::system::object::Object>;

    #[method(name = "ToArray", args = 1)]
    pub fn to_array_2(self, r#type: ::unity2::SystemType) -> ::unity2::IlInstance;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "system-collections-arraylist")]
impl ArrayList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArrayList),
                ::core::stringify!(new),
            )
        });
        <Self as IArrayListMethods>::ctor(this);
        this
    }

    pub fn new_2(capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArrayList),
                ::core::stringify!(new_2),
            )
        });
        <Self as IArrayListMethods>::ctor_2(this, capacity);
        this
    }

    pub fn new_3(c: crate::system::collections::icollection::ICollection) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArrayList),
                ::core::stringify!(new_3),
            )
        });
        <Self as IArrayListMethods>::ctor_3(this, c);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/arraylist/ArrayList_ArrayListDebugView.md")))]
#[::unity2::class(
    namespace = "System.Collections",
    name = "ArrayList.ArrayListDebugView"
)]
#[parent(crate::system::object::Object)]
pub struct ArrayList_ArrayListDebugView {}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/arraylist/ArrayList_ArrayListEnumeratorSimple.md")))]
#[::unity2::class(
    namespace = "System.Collections",
    name = "ArrayList.ArrayListEnumeratorSimple"
)]
#[parent(crate::system::object::Object)]
pub struct ArrayList_ArrayListEnumeratorSimple {
    #[rename(name = "list")]
    pub list: crate::system::collections::arraylist::ArrayList,
    #[rename(name = "index")]
    pub index: i32,
    #[rename(name = "version")]
    pub version: i32,
    #[rename(name = "currentElement")]
    pub current_element: ::unity2::IlInstance,
    #[rename(name = "isArrayList")]
    pub is_array_list: bool,
    #[static_field]
    #[rename(name = "dummyObject")]
    pub dummy_object: ::unity2::IlInstance,
}

#[cfg(feature = "system-collections-arraylist")]
#[::unity2::methods]
impl ArrayList_ArrayListEnumeratorSimple {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, list: crate::system::collections::arraylist::ArrayList) -> ();

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::system::object::Object;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "system-collections-arraylist")]
impl ArrayList_ArrayListEnumeratorSimple {
    pub fn new(list: crate::system::collections::arraylist::ArrayList) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArrayList_ArrayListEnumeratorSimple),
                ::core::stringify!(new),
            )
        });
        <Self as IArrayList_ArrayListEnumeratorSimpleMethods>::ctor(this, list);
        this
    }
}
