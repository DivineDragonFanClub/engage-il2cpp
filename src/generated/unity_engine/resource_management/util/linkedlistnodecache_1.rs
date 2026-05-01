
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/util/linkedlistnodecache_1/LinkedListNodeCache_1.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.Util",
    name = "LinkedListNodeCache`1"
)]
pub struct LinkedListNodeCache_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_NodesCreated")]
    pub m_nodes_created: i32,
    #[rename(name = "m_NodeCache")]
    pub m_node_cache: crate::system::collections::generic::linkedlist_1::LinkedList_1<T0>,
}

#[cfg(feature = "unity_engine-resource_management-util-linkedlistnodecache_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> LinkedListNodeCache_1<T0> {
    #[method(name = "Acquire", args = 1)]
    pub fn acquire(
        self,
        val: T0,
    ) -> crate::system::collections::generic::linkedlistnode_1::LinkedListNode_1<T0>;

    #[method(name = "Release", args = 1)]
    pub fn release(
        self,
        node: crate::system::collections::generic::linkedlistnode_1::LinkedListNode_1<T0>,
    ) -> ();

    #[method(name = "get_CreatedNodeCount", args = 0)]
    pub fn get_created_node_count(self) -> i32;

    #[method(name = "get_CachedNodeCount", args = 0)]
    pub fn get_cached_node_count(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-resource_management-util-linkedlistnodecache_1")]
impl<T0: ::unity2::ClassIdentity> LinkedListNodeCache_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LinkedListNodeCache_1),
                ::core::stringify!(new),
            )
        });
        <Self as ILinkedListNodeCache_1Methods<T0>>::ctor(this);
        this
    }
}
