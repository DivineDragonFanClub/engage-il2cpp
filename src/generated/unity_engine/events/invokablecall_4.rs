
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::events::baseinvokablecall::BaseInvokableCall;
use crate::unity_engine::events::baseinvokablecall::IBaseInvokableCall;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/events/invokablecall_4/InvokableCall_4.md")))]
#[::unity2::class(namespace = "UnityEngine.Events", name = "InvokableCall`4")]
pub struct InvokableCall_4<
    T0: ::unity2::ClassIdentity,
    T1: ::unity2::ClassIdentity,
    T2: ::unity2::ClassIdentity,
    T3: ::unity2::ClassIdentity,
> {
    #[rename(name = "Delegate")]
    pub delegate: crate::unity_engine::events::unityaction_4::UnityAction_4<T0, T1, T2, T3>,
}

#[cfg(feature = "unity_engine-events-invokablecall_4")]
#[::unity2::methods]
impl<
        T0: ::unity2::ClassIdentity,
        T1: ::unity2::ClassIdentity,
        T2: ::unity2::ClassIdentity,
        T3: ::unity2::ClassIdentity,
    > InvokableCall_4<T0, T1, T2, T3>
{
    #[method(name = "add_Delegate", args = 1)]
    pub fn add_delegate(
        self,
        value: crate::unity_engine::events::unityaction_4::UnityAction_4<T0, T1, T2, T3>,
    ) -> ();

    #[method(name = "remove_Delegate", args = 1)]
    pub fn remove_delegate(
        self,
        value: crate::unity_engine::events::unityaction_4::UnityAction_4<T0, T1, T2, T3>,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        target: crate::system::object::Object,
        the_function: crate::system::reflection::methodinfo::MethodInfo,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        action: crate::unity_engine::events::unityaction_4::UnityAction_4<T0, T1, T2, T3>,
    ) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, args: ::unity2::Array<crate::system::object::Object>) -> ();

    #[method(name = "Find", args = 2)]
    pub fn find(
        self,
        target_obj: crate::system::object::Object,
        method: crate::system::reflection::methodinfo::MethodInfo,
    ) -> bool;
}

#[cfg(feature = "unity_engine-events-invokablecall_4")]
impl<
        T0: ::unity2::ClassIdentity,
        T1: ::unity2::ClassIdentity,
        T2: ::unity2::ClassIdentity,
        T3: ::unity2::ClassIdentity,
    > InvokableCall_4<T0, T1, T2, T3>
{
    pub fn new(
        target: crate::system::object::Object,
        the_function: crate::system::reflection::methodinfo::MethodInfo,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvokableCall_4),
                ::core::stringify!(new),
            )
        });
        <Self as IInvokableCall_4Methods<T0, T1, T2, T3>>::ctor(this, target, the_function);
        this
    }

    pub fn new_2(
        action: crate::unity_engine::events::unityaction_4::UnityAction_4<T0, T1, T2, T3>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvokableCall_4),
                ::core::stringify!(new_2),
            )
        });
        <Self as IInvokableCall_4Methods<T0, T1, T2, T3>>::ctor_2(this, action);
        this
    }
}
