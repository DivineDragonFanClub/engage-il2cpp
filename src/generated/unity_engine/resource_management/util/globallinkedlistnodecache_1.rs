
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/resource_management/util/globallinkedlistnodecache_1/GlobalLinkedListNodeCache_1.md")))]
#[::unity2::class(
    namespace = "UnityEngine.ResourceManagement.Util",
    name = "GlobalLinkedListNodeCache`1"
)]
pub struct GlobalLinkedListNodeCache_1 < T0 : :: unity2 :: ClassIdentity > {
# [static_field] # [rename (name = "m_globalCache")] pub m_global_cache : crate :: unity_engine :: resource_management :: util :: linkedlistnodecache_1 :: LinkedListNodeCache_1 < T0 > ,
}

#[cfg(feature = "unity_engine-resource_management-util-globallinkedlistnodecache_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> GlobalLinkedListNodeCache_1<T0> {
    #[method(name = "Acquire", args = 1)]
    pub fn acquire(
        val: T0,
    ) -> crate::system::collections::generic::linkedlistnode_1::LinkedListNode_1<T0>;

    #[method(name = "Release", args = 1)]
    pub fn release(
        node: crate::system::collections::generic::linkedlistnode_1::LinkedListNode_1<T0>,
    ) -> ();
}
