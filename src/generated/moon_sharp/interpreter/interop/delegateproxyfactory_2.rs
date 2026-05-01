
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/delegateproxyfactory_2/DelegateProxyFactory_2.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "DelegateProxyFactory`2"
)]
pub struct DelegateProxyFactory_2<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {
    #[rename(name = "wrapDelegate")]
    pub wrap_delegate: crate::system::func_2::Func_2<T1, T0>,
}

#[cfg(feature = "moon_sharp-interpreter-interop-delegateproxyfactory_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> DelegateProxyFactory_2<T0, T1> {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, wrap_delegate: crate::system::func_2::Func_2<T1, T0>) -> ();

    #[method(name = "CreateProxyObject", args = 1)]
    pub fn create_proxy_object(self, target: T1) -> T0;

    #[method(name = "CreateProxyObject", args = 1)]
    pub fn create_proxy_object_2(
        self,
        o: crate::system::object::Object,
    ) -> crate::system::object::Object;

    #[method(name = "get_TargetType", args = 0)]
    pub fn get_target_type(self) -> ::unity2::SystemType;

    #[method(name = "get_ProxyType", args = 0)]
    pub fn get_proxy_type(self) -> ::unity2::SystemType;
}

#[cfg(feature = "moon_sharp-interpreter-interop-delegateproxyfactory_2")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> DelegateProxyFactory_2<T0, T1> {
    pub fn new(wrap_delegate: crate::system::func_2::Func_2<T1, T0>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DelegateProxyFactory_2),
                ::core::stringify!(new),
            )
        });
        <Self as IDelegateProxyFactory_2Methods<T0, T1>>::ctor(this, wrap_delegate);
        this
    }
}
