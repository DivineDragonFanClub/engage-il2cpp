
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/queue/Queue.md")))]
#[::unity2::class(namespace = "System.Collections", name = "Queue")]
#[parent(crate::system::object::Object)]
pub struct Queue {
    #[rename(name = "_array")]
    pub array: ::unity2::Array<crate::system::object::Object>,
    #[rename(name = "_head")]
    pub head: i32,
    #[rename(name = "_tail")]
    pub tail: i32,
    #[rename(name = "_size")]
    pub size: i32,
    #[rename(name = "_growFactor")]
    pub grow_factor: i32,
    #[rename(name = "_version")]
    pub version: i32,
    #[rename(name = "_syncRoot")]
    pub sync_root: ::unity2::IlInstance,
}

#[cfg(feature = "system-collections-queue")]
#[::unity2::methods]
impl Queue {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, capacity: i32) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_3(self, capacity: i32, grow_factor: f32) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_4(self, col: crate::system::collections::icollection::ICollection) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

    #[method(name = "get_IsSynchronized", args = 0)]
    pub fn get_is_synchronized(self) -> bool;

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, index: i32) -> ();

    #[method(name = "Enqueue", args = 1)]
    pub fn enqueue(self, obj: crate::system::object::Object) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Dequeue", args = 0)]
    pub fn dequeue(self) -> crate::system::object::Object;

    #[method(name = "Peek", args = 0)]
    pub fn peek(self) -> crate::system::object::Object;

    #[method(name = "GetElement", args = 1)]
    pub fn get_element(self, i: i32) -> crate::system::object::Object;

    #[method(name = "SetCapacity", args = 1)]
    pub fn set_capacity(self, capacity: i32) -> ();
}

#[cfg(feature = "system-collections-queue")]
impl Queue {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Queue),
                ::core::stringify!(new),
            )
        });
        <Self as IQueueMethods>::ctor(this);
        this
    }

    pub fn new_2(capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Queue),
                ::core::stringify!(new_2),
            )
        });
        <Self as IQueueMethods>::ctor_2(this, capacity);
        this
    }

    pub fn new_3(capacity: i32, grow_factor: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Queue),
                ::core::stringify!(new_3),
            )
        });
        <Self as IQueueMethods>::ctor_3(this, capacity, grow_factor);
        this
    }

    pub fn new_4(col: crate::system::collections::icollection::ICollection) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Queue),
                ::core::stringify!(new_4),
            )
        });
        <Self as IQueueMethods>::ctor_4(this, col);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/queue/Queue_QueueDebugView.md")))]
#[::unity2::class(namespace = "System.Collections", name = "Queue.QueueDebugView")]
#[parent(crate::system::object::Object)]
pub struct Queue_QueueDebugView {}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/queue/Queue_QueueEnumerator.md")))]
#[::unity2::class(namespace = "System.Collections", name = "Queue.QueueEnumerator")]
#[parent(crate::system::object::Object)]
pub struct Queue_QueueEnumerator {
    #[rename(name = "_q")]
    pub q: crate::system::collections::queue::Queue,
    #[rename(name = "_index")]
    pub index: i32,
    #[rename(name = "_version")]
    pub version: i32,
    #[rename(name = "currentElement")]
    pub current_element: ::unity2::IlInstance,
}

#[cfg(feature = "system-collections-queue")]
#[::unity2::methods]
impl Queue_QueueEnumerator {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, q: crate::system::collections::queue::Queue) -> ();

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::system::object::Object;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();
}

#[cfg(feature = "system-collections-queue")]
impl Queue_QueueEnumerator {
    pub fn new(q: crate::system::collections::queue::Queue) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Queue_QueueEnumerator),
                ::core::stringify!(new),
            )
        });
        <Self as IQueue_QueueEnumeratorMethods>::ctor(this, q);
        this
    }
}
