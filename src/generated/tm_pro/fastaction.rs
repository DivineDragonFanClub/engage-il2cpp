
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/tm_pro/fastaction/FastAction.md")))]
#[::unity2::class(namespace = "TMPro", name = "FastAction")]
#[parent(crate::system::object::Object)]
pub struct FastAction {
    #[rename(name = "delegates")]
    pub delegates: crate::system::collections::generic::linkedlist_1::LinkedList_1<
        crate::system::action::Action,
    >,
    #[rename(name = "lookup")]
    pub lookup: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::system::action::Action,
        crate::system::collections::generic::linkedlistnode_1::LinkedListNode_1<
            crate::system::action::Action,
        >,
    >,
}

#[cfg(feature = "tm_pro-fastaction")]
#[::unity2::methods]
impl FastAction {
    #[method(name = "Add", args = 1)]
    pub fn add(self, rhs: crate::system::action::Action) -> ();

    #[method(name = "Remove", args = 1)]
    pub fn remove(self, rhs: crate::system::action::Action) -> ();

    #[method(name = "Call", args = 0)]
    pub fn call(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "tm_pro-fastaction")]
impl FastAction {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FastAction),
                ::core::stringify!(new),
            )
        });
        <Self as IFastActionMethods>::ctor(this);
        this
    }
}
