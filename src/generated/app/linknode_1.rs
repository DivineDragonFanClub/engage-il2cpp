
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/linknode_1/LinkNode_1.md")))]
#[::unity2::class(namespace = "App", name = "LinkNode`1")]
pub struct LinkNode_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_InstanceID")]
    pub m_instance_id: i32,
    #[rename(name = "m_Node")]
    pub m_node: crate::system::collections::generic::linkedlistnode_1::LinkedListNode_1<T0>,
}

#[cfg(feature = "app-linknode_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> LinkNode_1<T0> {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "GetInstanceID", args = 0)]
    pub fn get_instance_id(self) -> i32;

    #[method(name = "SetInstanceID", args = 1)]
    pub fn set_instance_id(self, instance_id: i32) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "get_SortKey", args = 0)]
    pub fn get_sort_key(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(
        node: crate::app::linknode_1::LinkNode_1<T0>,
    ) -> crate::system::collections::generic::linkedlistnode_1::LinkedListNode_1<T0>;
}

#[cfg(feature = "app-linknode_1")]
impl<T0: ::unity2::ClassIdentity> LinkNode_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LinkNode_1),
                ::core::stringify!(new),
            )
        });
        <Self as ILinkNode_1Methods<T0>>::ctor(this);
        this
    }
}
