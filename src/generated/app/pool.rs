
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/pool/Pool_Node.md")))]
#[::unity2::class(namespace = "App", name = "Pool.Node")]
#[parent(crate::system::object::Object)]
pub struct Pool_Node {}

#[cfg(feature = "app-pool")]
#[::unity2::methods]
impl Pool_Node {
    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnExit", args = 0)]
    pub fn on_exit(self) -> ();

    #[method(name = "get_SortKey", args = 0)]
    pub fn get_sort_key(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-pool")]
impl Pool_Node {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Pool_Node),
                ::core::stringify!(new),
            )
        });
        <Self as IPool_NodeMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/pool/Pool_LockStack_1.md")))]
#[::unity2::class(namespace = "App", name = "Pool.LockStack`1")]
pub struct Pool_LockStack_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "lockObject")]
    pub lock_object: ::unity2::IlInstance,
}

#[cfg(feature = "app-pool")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> Pool_LockStack_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, max: i32) -> ();

    #[method(name = "Pop", args = 0)]
    pub fn pop(self) -> T0;

    #[method(name = "Push", args = 1)]
    pub fn push(self, p: T0) -> ();
}

#[cfg(feature = "app-pool")]
impl<T0: ::unity2::ClassIdentity> Pool_LockStack_1<T0> {
    pub fn new(max: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Pool_LockStack_1),
                ::core::stringify!(new),
            )
        });
        <Self as IPool_LockStack_1Methods<T0>>::ctor(this, max);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/pool/Pool_Stack_1.md")))]
#[::unity2::class(namespace = "App", name = "Pool.Stack`1")]
pub struct Pool_Stack_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_Pool")]
    pub m_pool: crate::system::collections::generic::stack_1::Stack_1<T0>,
}

#[cfg(feature = "app-pool")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> Pool_Stack_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, max: i32) -> ();

    #[method(name = "Pop", args = 0)]
    pub fn pop(self) -> T0;

    #[method(name = "Push", args = 1)]
    pub fn push(self, p: T0) -> ();
}

#[cfg(feature = "app-pool")]
impl<T0: ::unity2::ClassIdentity> Pool_Stack_1<T0> {
    pub fn new(max: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Pool_Stack_1),
                ::core::stringify!(new),
            )
        });
        <Self as IPool_Stack_1Methods<T0>>::ctor(this, max);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/pool/Pool_List_1.md")))]
#[::unity2::class(namespace = "App", name = "Pool.List`1")]
pub struct Pool_List_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_List")]
    pub m_list: crate::system::collections::generic::list_1::List_1<T0>,
    #[rename(name = "m_Pool")]
    pub m_pool: crate::system::collections::generic::stack_1::Stack_1<T0>,
}

#[cfg(feature = "app-pool")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> Pool_List_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, max: i32) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, i: i32) -> T0;

    #[method(name = "Get", args = 1)]
    pub fn get(self, i: i32) -> T0;

    #[method(name = "Swap", args = 2)]
    pub fn swap(self, index_a: i32, index_b: i32) -> ();

    #[method(name = "Create", args = 0)]
    pub fn create(self) -> T0;

    #[method(name = "Delete", args = 1)]
    pub fn delete(self, p: T0) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Sort", args = 0)]
    pub fn sort(self) -> ();

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::generic::ienumerator_1::IEnumerator_1<T0>;
}

#[cfg(feature = "app-pool")]
impl<T0: ::unity2::ClassIdentity> Pool_List_1<T0> {
    pub fn new(max: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Pool_List_1),
                ::core::stringify!(new),
            )
        });
        <Self as IPool_List_1Methods<T0>>::ctor(this, max);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/pool/Pool.md")))]
#[::unity2::class(namespace = "App", name = "Pool")]
#[parent(crate::system::object::Object)]
pub struct Pool {}

#[cfg(feature = "app-pool")]
#[::unity2::methods]
impl Pool {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-pool")]
impl Pool {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Pool),
                ::core::stringify!(new),
            )
        });
        <Self as IPoolMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/pool/Pool_Hierarchy_1.md")))]
#[::unity2::class(namespace = "App", name = "Pool.Hierarchy`1")]
pub struct Pool_Hierarchy_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_Pool")]
    pub m_pool: crate::system::collections::generic::stack_1::Stack_1<T0>,
    #[rename(name = "m_Used")]
    pub m_used: crate::system::collections::generic::stack_1::Stack_1<T0>,
}

#[cfg(feature = "app-pool")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> Pool_Hierarchy_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, max: i32) -> ();

    #[method(name = "Push", args = 0)]
    pub fn push(self) -> T0;

    #[method(name = "Pop", args = 0)]
    pub fn pop(self) -> T0;
}

#[cfg(feature = "app-pool")]
impl<T0: ::unity2::ClassIdentity> Pool_Hierarchy_1<T0> {
    pub fn new(max: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Pool_Hierarchy_1),
                ::core::stringify!(new),
            )
        });
        <Self as IPool_Hierarchy_1Methods<T0>>::ctor(this, max);
        this
    }
}
