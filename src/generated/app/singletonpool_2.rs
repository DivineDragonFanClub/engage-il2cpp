
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/singletonpool_2/SingletonPool_2.md")))]
#[::unity2::class(namespace = "App", name = "SingletonPool`2")]
pub struct SingletonPool_2<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {
    #[static_field]
    #[rename(name = "OnlineInstanceID")]
    pub online_instance_id: i32,
    #[rename(name = "m_List")]
    pub m_list: crate::system::collections::generic::list_1::List_1<T1>,
    #[rename(name = "m_Sort")]
    pub m_sort: crate::system::collections::generic::list_1::List_1<T1>,
    #[rename(name = "m_Pool")]
    pub m_pool: crate::system::collections::generic::linkedlist_1::LinkedList_1<T1>,
    #[rename(name = "m_Used")]
    pub m_used: crate::system::collections::generic::linkedlist_1::LinkedList_1<T1>,
    #[rename(name = "m_Compare")]
    pub m_compare:
        crate::system::collections::generic::icomparer_1_interface::IComparer_1_Interface<T1>,
    #[rename(name = "m_Dictionary")]
    pub m_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<i32, T1>,
    #[rename(name = "m_InstanceIDs")]
    pub m_instance_i_ds: crate::system::collections::generic::queue_1::Queue_1<i32>,
    #[rename(name = "m_IsOnline")]
    pub m_is_online: bool,
}

#[cfg(feature = "app-singletonpool_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> SingletonPool_2<T0, T1> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, capacity: i32) -> ();

    #[method(name = "CreateImpl", args = 1)]
    pub fn create_impl(self, instance_id: i32) -> T1;

    #[method(name = "DeleteImpl", args = 1)]
    pub fn delete_impl(self, value: T1) -> ();

    #[method(name = "ClearImpl", args = 0)]
    pub fn clear_impl(self) -> ();

    #[method(name = "ReassignInstanceIDForOnline", args = 0)]
    pub fn reassign_instance_id_for_online(self) -> ();

    #[method(name = "ResetInstanceID", args = 0)]
    pub fn reset_instance_id(self) -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> T1;

    #[method(name = "Delete", args = 1)]
    pub fn delete(value: T1) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear() -> ();

    #[method(name = "SerializeForOnline", args = 1)]
    pub fn serialize_for_online(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "DeserializeForOnline", args = 1)]
    pub fn deserialize_for_online(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "SerializeSingle", args = 2)]
    pub fn serialize_single(stream: crate::app::stream_2::Stream_2, value: T1) -> ();

    #[method(name = "DeserializeSingle", args = 1)]
    pub fn deserialize_single(stream: crate::app::stream_2::Stream_2) -> T1;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "SortImpl", args = 0)]
    pub fn sort_impl(self) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::generic::list_1::List_1_Enumerator<T1>;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count() -> i32;

    #[method(name = "get_List", args = 0)]
    pub fn get_list() -> crate::system::collections::generic::list_1::List_1<T1>;

    #[method(name = "Find", args = 1)]
    pub fn find(instance_id: i32) -> T1;

    #[method(name = "IsFromOnline", args = 1)]
    pub fn is_from_online(value: T1) -> bool;

    #[method(name = "Sort", args = 0)]
    pub fn sort() -> ();
}

#[cfg(feature = "app-singletonpool_2")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> SingletonPool_2<T0, T1> {
    pub fn new(capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SingletonPool_2),
                ::core::stringify!(new),
            )
        });
        <Self as ISingletonPool_2Methods<T0, T1>>::ctor(this, capacity);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/singletonpool_2/SingletonPool_2_Comparer.md")))]
#[::unity2::class(namespace = "App", name = "SingletonPool`2.Comparer")]
pub struct SingletonPool_2_Comparer<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {}

#[cfg(feature = "app-singletonpool_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> SingletonPool_2_Comparer<T0, T1> {
    #[method(name = "Compare", args = 2)]
    pub fn compare(self, x: T1, y: T1) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-singletonpool_2")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> SingletonPool_2_Comparer<T0, T1> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SingletonPool_2_Comparer),
                ::core::stringify!(new),
            )
        });
        <Self as ISingletonPool_2_ComparerMethods<T0, T1>>::ctor(this);
        this
    }
}
