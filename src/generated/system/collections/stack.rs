
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/stack/Stack_StackDebugView.md")))]
#[::unity2::class(namespace = "System.Collections", name = "Stack.StackDebugView")]
#[parent(crate::system::object::Object)]
pub struct Stack_StackDebugView {}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/stack/Stack.md")))]
#[::unity2::class(namespace = "System.Collections", name = "Stack")]
#[parent(crate::system::object::Object)]
pub struct Stack {
    #[rename(name = "_array")]
    pub array: ::unity2::Array<crate::system::object::Object>,
    #[rename(name = "_size")]
    pub size: i32,
    #[rename(name = "_version")]
    pub version: i32,
    #[rename(name = "_syncRoot")]
    pub sync_root: ::unity2::IlInstance,
}

#[cfg(feature = "system-collections-stack")]
#[::unity2::methods]
impl Stack {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, initial_capacity: i32) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_IsSynchronized", args = 0)]
    pub fn get_is_synchronized(self) -> bool;

    #[method(name = "get_SyncRoot", args = 0)]
    pub fn get_sync_root(self) -> crate::system::object::Object;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

    #[method(name = "CopyTo", args = 2)]
    pub fn copy_to(self, array: ::unity2::IlInstance, index: i32) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Peek", args = 0)]
    pub fn peek(self) -> crate::system::object::Object;

    #[method(name = "Pop", args = 0)]
    pub fn pop(self) -> crate::system::object::Object;

    #[method(name = "Push", args = 1)]
    pub fn push(self, obj: crate::system::object::Object) -> ();
}

#[cfg(feature = "system-collections-stack")]
impl Stack {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Stack),
                ::core::stringify!(new),
            )
        });
        <Self as IStackMethods>::ctor(this);
        this
    }

    pub fn new_2(initial_capacity: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Stack),
                ::core::stringify!(new_2),
            )
        });
        <Self as IStackMethods>::ctor_2(this, initial_capacity);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/collections/stack/Stack_StackEnumerator.md")))]
#[::unity2::class(namespace = "System.Collections", name = "Stack.StackEnumerator")]
#[parent(crate::system::object::Object)]
pub struct Stack_StackEnumerator {
    #[rename(name = "_stack")]
    pub stack: crate::system::collections::stack::Stack,
    #[rename(name = "_index")]
    pub index: i32,
    #[rename(name = "_version")]
    pub version: i32,
    #[rename(name = "currentElement")]
    pub current_element: ::unity2::IlInstance,
}

#[cfg(feature = "system-collections-stack")]
#[::unity2::methods]
impl Stack_StackEnumerator {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, stack: crate::system::collections::stack::Stack) -> ();

    #[method(name = "Clone", args = 0)]
    pub fn clone(self) -> crate::system::object::Object;

    #[method(name = "MoveNext", args = 0)]
    pub fn move_next(self) -> bool;

    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::system::object::Object;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();
}

#[cfg(feature = "system-collections-stack")]
impl Stack_StackEnumerator {
    pub fn new(stack: crate::system::collections::stack::Stack) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Stack_StackEnumerator),
                ::core::stringify!(new),
            )
        });
        <Self as IStack_StackEnumeratorMethods>::ctor(this, stack);
        this
    }
}
