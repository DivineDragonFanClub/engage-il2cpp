
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::events::baseinvokablecall::BaseInvokableCall;
use crate::unity_engine::events::baseinvokablecall::IBaseInvokableCall;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/events/invokablecall_3/InvokableCall_3.md")))]
#[::unity2::class(namespace = "UnityEngine.Events", name = "InvokableCall`3")]
pub struct InvokableCall_3<
    T0: ::unity2::ClassIdentity,
    T1: ::unity2::ClassIdentity,
    T2: ::unity2::ClassIdentity,
> {
    #[rename(name = "Delegate")]
    pub delegate: crate::unity_engine::events::unityaction_3::UnityAction_3<T0, T1, T2>,
}

#[cfg(feature = "unity_engine-events-invokablecall_3")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity, T2: ::unity2::ClassIdentity>
    InvokableCall_3<T0, T1, T2>
{
    #[method(name = "add_Delegate", args = 1)]
    pub fn add_delegate(
        self,
        value: crate::unity_engine::events::unityaction_3::UnityAction_3<T0, T1, T2>,
    ) -> ();

    #[method(name = "remove_Delegate", args = 1)]
    pub fn remove_delegate(
        self,
        value: crate::unity_engine::events::unityaction_3::UnityAction_3<T0, T1, T2>,
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
        action: crate::unity_engine::events::unityaction_3::UnityAction_3<T0, T1, T2>,
    ) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, args: ::unity2::Array<crate::system::object::Object>) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke_2(self, args0: T0, args1: T1, args2: T2) -> ();

    #[method(name = "Find", args = 2)]
    pub fn find(
        self,
        target_obj: crate::system::object::Object,
        method: crate::system::reflection::methodinfo::MethodInfo,
    ) -> bool;
}

#[cfg(feature = "unity_engine-events-invokablecall_3")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity, T2: ::unity2::ClassIdentity>
    InvokableCall_3<T0, T1, T2>
{
    pub fn new(
        target: crate::system::object::Object,
        the_function: crate::system::reflection::methodinfo::MethodInfo,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvokableCall_3),
                ::core::stringify!(new),
            )
        });
        <Self as IInvokableCall_3Methods<T0, T1, T2>>::ctor(this, target, the_function);
        this
    }

    pub fn new_2(
        action: crate::unity_engine::events::unityaction_3::UnityAction_3<T0, T1, T2>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvokableCall_3),
                ::core::stringify!(new_2),
            )
        });
        <Self as IInvokableCall_3Methods<T0, T1, T2>>::ctor_2(this, action);
        this
    }
}
