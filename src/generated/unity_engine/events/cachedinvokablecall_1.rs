
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::events::baseinvokablecall::BaseInvokableCall;
use crate::unity_engine::events::baseinvokablecall::IBaseInvokableCall;
use crate::unity_engine::events::invokablecall_1::IInvokableCall_1;
use crate::unity_engine::events::invokablecall_1::InvokableCall_1;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/events/cachedinvokablecall_1/CachedInvokableCall_1.md")))]
#[::unity2::class(namespace = "UnityEngine.Events", name = "CachedInvokableCall`1")]
pub struct CachedInvokableCall_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_Arg1")]
    pub m_arg1: T0,
}

#[cfg(feature = "unity_engine-events-cachedinvokablecall_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> CachedInvokableCall_1<T0> {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        target: crate::unity_engine::object_2::Object_2,
        the_function: crate::system::reflection::methodinfo::MethodInfo,
        argument: T0,
    ) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, args: ::unity2::Array<crate::system::object::Object>) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke_2(self, arg0: T0) -> ();
}

#[cfg(feature = "unity_engine-events-cachedinvokablecall_1")]
impl<T0: ::unity2::ClassIdentity> CachedInvokableCall_1<T0> {
    pub fn new(
        target: crate::unity_engine::object_2::Object_2,
        the_function: crate::system::reflection::methodinfo::MethodInfo,
        argument: T0,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CachedInvokableCall_1),
                ::core::stringify!(new),
            )
        });
        <Self as ICachedInvokableCall_1Methods<T0>>::ctor(this, target, the_function, argument);
        this
    }
}
