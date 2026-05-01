
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lib_tess_dot_net/priorityheap_1/PriorityHeap_1.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal.LibTessDotNet",
    name = "PriorityHeap`1"
)]
pub struct PriorityHeap_1 < T0 : :: unity2 :: ClassIdentity > {
# [rename (name = "_leq")] pub leq : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: priorityheap_1 :: PriorityHeap_1_LessOrEqual < T0 > ,
# [rename (name = "_nodes")] pub nodes : :: unity2 :: Array < i32 > ,
# [rename (name = "_handles")] pub handles : :: unity2 :: Array < crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: priorityheap_1 :: PriorityHeap_1_HandleElem < T0 > > ,
# [rename (name = "_size")] pub size : i32 ,
# [rename (name = "_max")] pub max : i32 ,
# [rename (name = "_freeList")] pub free_list : i32 ,
# [rename (name = "_initialized")] pub initialized : bool ,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-priorityheap_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> PriorityHeap_1<T0> {
    #[method(name = "get_Empty", args = 0)]
    pub fn get_empty(self) -> bool;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        initial_size: i32,
        leq : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: priorityheap_1 :: PriorityHeap_1_LessOrEqual < T0 >,
    ) -> ();

    #[method(name = "FloatDown", args = 1)]
    pub fn float_down(self, curr: i32) -> ();

    #[method(name = "FloatUp", args = 1)]
    pub fn float_up(self, curr: i32) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "Insert", args = 1)]
    pub fn insert(
        self,
        value: T0,
    ) -> crate::unity_engine::experimental::rendering::universal::lib_tess_dot_net::pqhandle::PQHandle;

    #[method(name = "ExtractMin", args = 0)]
    pub fn extract_min(self) -> T0;

    #[method(name = "Minimum", args = 0)]
    pub fn minimum(self) -> T0;

    #[method(name = "Remove", args = 1)]
    pub fn remove(
        self,
        handle : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: pqhandle :: PQHandle,
    ) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-priorityheap_1")]
impl<T0: ::unity2::ClassIdentity> PriorityHeap_1<T0> {
    pub fn new(
        initial_size: i32,
        leq : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: priorityheap_1 :: PriorityHeap_1_LessOrEqual < T0 >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PriorityHeap_1),
                ::core::stringify!(new),
            )
        });
        <Self as IPriorityHeap_1Methods<T0>>::ctor(this, initial_size, leq);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lib_tess_dot_net/priorityheap_1/PriorityHeap_1_HandleElem.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal.LibTessDotNet",
    name = "PriorityHeap`1.HandleElem"
)]
pub struct PriorityHeap_1_HandleElem<T0: ::unity2::ClassIdentity> {
    #[rename(name = "_key")]
    pub key: T0,
    #[rename(name = "_node")]
    pub node: i32,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-priorityheap_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> PriorityHeap_1_HandleElem<T0> {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-priorityheap_1")]
impl<T0: ::unity2::ClassIdentity> PriorityHeap_1_HandleElem<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PriorityHeap_1_HandleElem),
                ::core::stringify!(new),
            )
        });
        <Self as IPriorityHeap_1_HandleElemMethods<T0>>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lib_tess_dot_net/priorityheap_1/PriorityHeap_1_LessOrEqual.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal.LibTessDotNet",
    name = "PriorityHeap`1.LessOrEqual"
)]
pub struct PriorityHeap_1_LessOrEqual<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-priorityheap_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> PriorityHeap_1_LessOrEqual<T0> {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, lhs: T0, rhs: T0) -> bool;
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-priorityheap_1")]
impl<T0: ::unity2::ClassIdentity> PriorityHeap_1_LessOrEqual<T0> {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PriorityHeap_1_LessOrEqual),
                ::core::stringify!(new),
            )
        });
        <Self as IPriorityHeap_1_LessOrEqualMethods<T0>>::ctor(this, object, method);
        this
    }
}
