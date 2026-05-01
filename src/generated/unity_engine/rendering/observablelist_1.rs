
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/rendering/observablelist_1/ObservableList_1.md")))]
#[::unity2::class(namespace = "UnityEngine.Rendering", name = "ObservableList`1")]
pub struct ObservableList_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_List")]
    pub m_list: crate::system::collections::generic::ilist_1_interface::IList_1_Interface<T0>,
    #[rename(name = "ItemAdded")]
    pub item_added:
        crate::unity_engine::rendering::listchangedeventhandler_1::ListChangedEventHandler_1<T0>,
    #[rename(name = "ItemRemoved")]
    pub item_removed:
        crate::unity_engine::rendering::listchangedeventhandler_1::ListChangedEventHandler_1<T0>,
}

#[cfg(feature = "unity_engine-rendering-observablelist_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> ObservableList_1<T0> {
    #[method(name = "add_ItemAdded", args = 1)]
    pub fn add_item_added(
        self,
        value: crate::unity_engine::rendering::listchangedeventhandler_1::ListChangedEventHandler_1<
            T0,
        >,
    ) -> ();

    #[method(name = "remove_ItemAdded", args = 1)]
    pub fn remove_item_added(
        self,
        value: crate::unity_engine::rendering::listchangedeventhandler_1::ListChangedEventHandler_1<
            T0,
        >,
    ) -> ();

    #[method(name = "add_ItemRemoved", args = 1)]
    pub fn add_item_removed(
        self,
        value: crate::unity_engine::rendering::listchangedeventhandler_1::ListChangedEventHandler_1<
            T0,
        >,
    ) -> ();

    #[method(name = "remove_ItemRemoved", args = 1)]
    pub fn remove_item_removed(
        self,
        value: crate::unity_engine::rendering::listchangedeventhandler_1::ListChangedEventHandler_1<
            T0,
        >,
    ) -> ();

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, index: i32) -> T0;

    #[method(name = "set_Item", args = 2)]
    pub fn set_item(self, index: i32, value: T0) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_IsReadOnly", args = 0)]
    pub fn get_is_read_only(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, capacity: i32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(
        self,
        collection: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
    ) -> ();

    #[method(name = "OnEvent", args = 3)]
    pub fn on_event(
        self,
        e: crate::unity_engine::rendering::listchangedeventhandler_1::ListChangedEventHandler_1<T0>,
        index: i32,
        item: T0,
    ) -> ();

    #[method(name = "Contains", args = 1)]
    pub fn contains(self, item: T0) -> bool;

    #[method(name = "IndexOf", args = 1)]
    pub fn index_of(self, item: T0) -> i32;

    #[method(name = "Add", args = 1)]
    pub fn add(self, item: T0) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add_2(self, items: ::unity2::Array<T0>) -> ();

    #[method(name = "Insert", args = 2)]
    pub fn insert(self, index: i32, item: T0) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, item: T0) -> bool;

    #[method(name = "Remove", args = 1)]
    pub fn remove_2(self, items: ::unity2::Array<T0>) -> i32;

    #[method(name = "RemoveAt", args = 1)]
    pub fn remove_at(self, index: i32) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::Array<T0>, array_index: i32) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::generic::ienumerator_1::IEnumerator_1<T0>;

    #[method(name = "System.Collections.IEnumerable.GetEnumerator", args = 0)]
    pub fn system_collections_i_enumerable_get_enumerator(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;
}

#[cfg(feature = "unity_engine-rendering-observablelist_1")]
impl<T0: ::unity2::ClassIdentity> ObservableList_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ObservableList_1),
                ::core::stringify!(new),
            )
        });
        <Self as IObservableList_1Methods<T0>>::ctor(this);
        this
    }

    pub fn new_2(capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ObservableList_1),
                ::core::stringify!(new_2),
            )
        });
        <Self as IObservableList_1Methods<T0>>::ctor_2(this, capacity);
        this
    }

    pub fn new_3(
        collection: crate::system::collections::generic::ienumerable_1::IEnumerable_1<T0>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ObservableList_1),
                ::core::stringify!(new_3),
            )
        });
        <Self as IObservableList_1Methods<T0>>::ctor_3(this, collection);
        this
    }
}
