
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::events::unityeventbase::IUnityEventBase;
use crate::unity_engine::events::unityeventbase::UnityEventBase;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/events/unityevent_2/UnityEvent_2.md")))]
#[::unity2::class(namespace = "UnityEngine.Events", name = "UnityEvent`2")]
pub struct UnityEvent_2<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> {
    #[rename(name = "m_InvokeArray")]
    pub m_invoke_array: ::unity2::Array<crate::system::object::Object>,
}

#[cfg(feature = "unity_engine-events-unityevent_2")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> UnityEvent_2<T0, T1> {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "AddListener", args = 1)]
    pub fn add_listener(
        self,
        call: crate::unity_engine::events::unityaction_2::UnityAction_2<T0, T1>,
    ) -> ();

    #[method(name = "RemoveListener", args = 1)]
    pub fn remove_listener(
        self,
        call: crate::unity_engine::events::unityaction_2::UnityAction_2<T0, T1>,
    ) -> ();

    #[method(name = "FindMethod_Impl", args = 2)]
    pub fn find_method_impl(
        self,
        name: ::unity2::Il2CppString,
        target_obj_type: ::unity2::SystemType,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "GetDelegate", args = 2)]
    pub fn get_delegate(
        self,
        target: crate::system::object::Object,
        the_function: crate::system::reflection::methodinfo::MethodInfo,
    ) -> crate::unity_engine::events::baseinvokablecall::BaseInvokableCall;

    #[method(name = "GetDelegate", args = 1)]
    pub fn get_delegate_2(
        action: crate::unity_engine::events::unityaction_2::UnityAction_2<T0, T1>,
    ) -> crate::unity_engine::events::baseinvokablecall::BaseInvokableCall;

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(self, arg0: T0, arg1: T1) -> ();
}

#[cfg(feature = "unity_engine-events-unityevent_2")]
impl<T0: ::unity2::ClassIdentity, T1: ::unity2::ClassIdentity> UnityEvent_2<T0, T1> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnityEvent_2),
                ::core::stringify!(new),
            )
        });
        <Self as IUnityEvent_2Methods<T0, T1>>::ctor(this);
        this
    }
}
