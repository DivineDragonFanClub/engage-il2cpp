
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/generic/linkedlistnode_1/LinkedListNode_1.md")))]
#[::unity2::class(namespace = "System.Collections.Generic", name = "LinkedListNode`1")]
pub struct LinkedListNode_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "list")]
    pub list: crate::system::collections::generic::linkedlist_1::LinkedList_1<T0>,
    #[rename(name = "next")]
    pub next: crate::system::collections::generic::linkedlistnode_1::LinkedListNode_1<T0>,
    #[rename(name = "prev")]
    pub prev: crate::system::collections::generic::linkedlistnode_1::LinkedListNode_1<T0>,
    #[rename(name = "item")]
    pub item: T0,
}

#[cfg(feature = "system-collections-generic-linkedlistnode_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> LinkedListNode_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, value: T0) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        list: crate::system::collections::generic::linkedlist_1::LinkedList_1<T0>,
        value: T0,
    ) -> ();

    #[method(name = "get_List", args = 0)]
    pub fn get_list(self) -> crate::system::collections::generic::linkedlist_1::LinkedList_1<T0>;

    #[method(name = "get_Next", args = 0)]
    pub fn get_next(
        self,
    ) -> crate::system::collections::generic::linkedlistnode_1::LinkedListNode_1<T0>;

    #[method(name = "get_Previous", args = 0)]
    pub fn get_previous(
        self,
    ) -> crate::system::collections::generic::linkedlistnode_1::LinkedListNode_1<T0>;

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> T0;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: T0) -> ();

    #[method(name = "Invalidate", args = 0)]
    pub fn invalidate(self) -> ();
}

#[cfg(feature = "system-collections-generic-linkedlistnode_1")]
impl<T0: ::unity2::ClassIdentity> LinkedListNode_1<T0> {
    pub fn new(value: T0) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LinkedListNode_1),
                ::core::stringify!(new),
            )
        });
        <Self as ILinkedListNode_1Methods<T0>>::ctor(this, value);
        this
    }

    pub fn new_2(
        list: crate::system::collections::generic::linkedlist_1::LinkedList_1<T0>,
        value: T0,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LinkedListNode_1),
                ::core::stringify!(new_2),
            )
        });
        <Self as ILinkedListNode_1Methods<T0>>::ctor_2(this, list, value);
        this
    }
}
