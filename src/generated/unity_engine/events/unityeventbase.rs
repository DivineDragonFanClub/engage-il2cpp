
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/events/unityeventbase/UnityEventBase.md")))]
#[::unity2::class(namespace = "UnityEngine.Events", name = "UnityEventBase")]
#[parent(crate::system::object::Object)]
pub struct UnityEventBase {
    #[rename(name = "m_Calls")]
    pub m_calls: crate::unity_engine::events::invokablecalllist::InvokableCallList,
    #[rename(name = "m_PersistentCalls")]
    pub m_persistent_calls: crate::unity_engine::events::persistentcallgroup::PersistentCallGroup,
    #[rename(name = "m_CallsDirty")]
    pub m_calls_dirty: bool,
}

#[cfg(feature = "unity_engine-events-unityeventbase")]
#[::unity2::methods]
impl UnityEventBase {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(
        name = "UnityEngine.ISerializationCallbackReceiver.OnBeforeSerialize",
        args = 0
    )]
    pub fn unity_engine_i_serialization_callback_receiver_on_before_serialize(self) -> ();

    #[method(
        name = "UnityEngine.ISerializationCallbackReceiver.OnAfterDeserialize",
        args = 0
    )]
    pub fn unity_engine_i_serialization_callback_receiver_on_after_deserialize(self) -> ();

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

    #[method(name = "FindMethod", args = 1)]
    pub fn find_method(
        self,
        call: crate::unity_engine::events::persistentcall::PersistentCall,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "FindMethod", args = 4)]
    pub fn find_method_2(
        self,
        name: ::unity2::Il2CppString,
        listener_type: ::unity2::SystemType,
        mode: crate::unity_engine::events::persistentlistenermode::PersistentListenerMode,
        argument_type: ::unity2::SystemType,
    ) -> crate::system::reflection::methodinfo::MethodInfo;

    #[method(name = "DirtyPersistentCalls", args = 0)]
    pub fn dirty_persistent_calls(self) -> ();

    #[method(name = "RebuildPersistentCallsIfNeeded", args = 0)]
    pub fn rebuild_persistent_calls_if_needed(self) -> ();

    #[method(name = "AddCall", args = 1)]
    pub fn add_call(
        self,
        call: crate::unity_engine::events::baseinvokablecall::BaseInvokableCall,
    ) -> ();

    #[method(name = "RemoveListener", args = 2)]
    pub fn remove_listener(
        self,
        target_obj: crate::system::object::Object,
        method: crate::system::reflection::methodinfo::MethodInfo,
    ) -> ();

    #[method(name = "PrepareInvoke", args = 0)]
    pub fn prepare_invoke(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::events::baseinvokablecall::BaseInvokableCall,
    >;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetValidMethodInfo", args = 3)]
    pub fn get_valid_method_info(
        object_type: ::unity2::SystemType,
        function_name: ::unity2::Il2CppString,
        argument_types: ::unity2::Array<::unity2::SystemType>,
    ) -> crate::system::reflection::methodinfo::MethodInfo;
}

#[cfg(feature = "unity_engine-events-unityeventbase")]
impl UnityEventBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnityEventBase),
                ::core::stringify!(new),
            )
        });
        <Self as IUnityEventBaseMethods>::ctor(this);
        this
    }
}
