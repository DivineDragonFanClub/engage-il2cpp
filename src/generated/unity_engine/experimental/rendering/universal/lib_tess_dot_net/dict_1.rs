
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lib_tess_dot_net/dict_1/Dict_1.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal.LibTessDotNet",
    name = "Dict`1"
)]
pub struct Dict_1 < T0 : :: unity2 :: ClassIdentity > {
# [rename (name = "_leq")] pub leq : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_LessOrEqual < T0 > ,
# [rename (name = "_head")] pub head : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_Node < T0 > ,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-dict_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> Dict_1<T0> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        leq : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_LessOrEqual < T0 >,
    ) -> ();

    #[method(name = "Insert", args = 1)]
    pub fn insert (self , key : T0) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_Node < T0 > ;

    #[method(name = "InsertBefore", args = 2)]
    pub fn insert_before (self , node : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_Node < T0 > , key : T0) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_Node < T0 > ;

    #[method(name = "Find", args = 1)]
    pub fn find (self , key : T0) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_Node < T0 > ;

    #[method(name = "Min", args = 0)]
    pub fn min (self ,) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_Node < T0 > ;

    #[method(name = "Remove", args = 1)]
    pub fn remove(
        self,
        node : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_Node < T0 >,
    ) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-dict_1")]
impl<T0: ::unity2::ClassIdentity> Dict_1<T0> {
    pub fn new(
        leq : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_LessOrEqual < T0 >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Dict_1),
                ::core::stringify!(new),
            )
        });
        <Self as IDict_1Methods<T0>>::ctor(this, leq);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lib_tess_dot_net/dict_1/Dict_1_LessOrEqual.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal.LibTessDotNet",
    name = "Dict`1.LessOrEqual"
)]
pub struct Dict_1_LessOrEqual<T0: ::unity2::ClassIdentity> {}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-dict_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> Dict_1_LessOrEqual<T0> {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, lhs: T0, rhs: T0) -> bool;
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-dict_1")]
impl<T0: ::unity2::ClassIdentity> Dict_1_LessOrEqual<T0> {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Dict_1_LessOrEqual),
                ::core::stringify!(new),
            )
        });
        <Self as IDict_1_LessOrEqualMethods<T0>>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/lib_tess_dot_net/dict_1/Dict_1_Node.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal.LibTessDotNet",
    name = "Dict`1.Node"
)]
pub struct Dict_1_Node < T0 : :: unity2 :: ClassIdentity > {
# [rename (name = "_key")] pub key : T0 ,
# [rename (name = "_prev")] pub prev : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_Node < T0 > ,
# [rename (name = "_next")] pub next : crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_Node < T0 > ,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-dict_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> Dict_1_Node<T0> {
    #[method(name = "get_Key", args = 0)]
    pub fn get_key(self) -> T0;

    #[method(name = "get_Prev", args = 0)]
    pub fn get_prev (self ,) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_Node < T0 > ;

    #[method(name = "get_Next", args = 0)]
    pub fn get_next (self ,) -> crate :: unity_engine :: experimental :: rendering :: universal :: lib_tess_dot_net :: dict_1 :: Dict_1_Node < T0 > ;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-lib_tess_dot_net-dict_1")]
impl<T0: ::unity2::ClassIdentity> Dict_1_Node<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Dict_1_Node),
                ::core::stringify!(new),
            )
        });
        <Self as IDict_1_NodeMethods<T0>>::ctor(this);
        this
    }
}
