
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/events/persistentcall/PersistentCall.md")))]
#[::unity2::class(namespace = "UnityEngine.Events", name = "PersistentCall")]
#[parent(crate::system::object::Object)]
pub struct PersistentCall {
    #[rename(name = "m_Target")]
    pub m_target: crate::unity_engine::object_2::Object_2,
    #[rename(name = "m_TargetAssemblyTypeName")]
    pub m_target_assembly_type_name: ::unity2::Il2CppString,
    #[rename(name = "m_MethodName")]
    pub m_method_name: ::unity2::Il2CppString,
    #[rename(name = "m_Mode")]
    pub m_mode: crate::unity_engine::events::persistentlistenermode::PersistentListenerMode,
    #[rename(name = "m_Arguments")]
    pub m_arguments: crate::unity_engine::events::argumentcache::ArgumentCache,
    #[rename(name = "m_CallState")]
    pub m_call_state: crate::unity_engine::events::unityeventcallstate::UnityEventCallState,
}

#[cfg(feature = "unity_engine-events-persistentcall")]
#[::unity2::methods]
impl PersistentCall {
    #[method(name = "get_target", args = 0)]
    pub fn get_target(self) -> crate::unity_engine::object_2::Object_2;

    #[method(name = "get_targetAssemblyTypeName", args = 0)]
    pub fn get_target_assembly_type_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_methodName", args = 0)]
    pub fn get_method_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_mode", args = 0)]
    pub fn get_mode(
        self,
    ) -> crate::unity_engine::events::persistentlistenermode::PersistentListenerMode;

    #[method(name = "get_arguments", args = 0)]
    pub fn get_arguments(self) -> crate::unity_engine::events::argumentcache::ArgumentCache;

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "GetRuntimeCall", args = 1)]
    pub fn get_runtime_call(
        self,
        the_event: crate::unity_engine::events::unityeventbase::UnityEventBase,
    ) -> crate::unity_engine::events::baseinvokablecall::BaseInvokableCall;

    #[method(name = "GetObjectCall", args = 3)]
    pub fn get_object_call(
        target: crate::unity_engine::object_2::Object_2,
        method: crate::system::reflection::methodinfo::MethodInfo,
        arguments: crate::unity_engine::events::argumentcache::ArgumentCache,
    ) -> crate::unity_engine::events::baseinvokablecall::BaseInvokableCall;

    #[method(name = "OnBeforeSerialize", args = 0)]
    pub fn on_before_serialize(self) -> ();

    #[method(name = "OnAfterDeserialize", args = 0)]
    pub fn on_after_deserialize(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-events-persistentcall")]
impl PersistentCall {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PersistentCall),
                ::core::stringify!(new),
            )
        });
        <Self as IPersistentCallMethods>::ctor(this);
        this
    }
}
