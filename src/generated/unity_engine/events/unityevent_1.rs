
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::events::unityeventbase::IUnityEventBase;
use crate::unity_engine::events::unityeventbase::UnityEventBase;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/events/unityevent_1/UnityEvent_1.md")))]
#[::unity2::class(namespace = "UnityEngine.Events", name = "UnityEvent`1")]
pub struct UnityEvent_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_InvokeArray")]
    pub m_invoke_array: ::unity2::Array<crate::system::object::Object>,
}

#[cfg(feature = "unity_engine-events-unityevent_1")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> UnityEvent_1<T0> {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "AddListener", args = 1)]
    pub fn add_listener(
        self,
        call: crate::unity_engine::events::unityaction_1::UnityAction_1<T0>,
    ) -> ();

    #[method(name = "RemoveListener", args = 1)]
    pub fn remove_listener(
        self,
        call: crate::unity_engine::events::unityaction_1::UnityAction_1<T0>,
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
        action: crate::unity_engine::events::unityaction_1::UnityAction_1<T0>,
    ) -> crate::unity_engine::events::baseinvokablecall::BaseInvokableCall;

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, arg0: T0) -> ();
}

#[cfg(feature = "unity_engine-events-unityevent_1")]
impl<T0: ::unity2::ClassIdentity> UnityEvent_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnityEvent_1),
                ::core::stringify!(new),
            )
        });
        <Self as IUnityEvent_1Methods<T0>>::ctor(this);
        this
    }
}
