
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/playableoutputhandle/PlayableOutputHandle.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct PlayableOutputHandle {
    pub m_handle: ::unity2::IntPtr,
    pub m_version: u32,
}

impl ::unity2::ClassIdentity for PlayableOutputHandle {
    const NAMESPACE: &'static str = "UnityEngine.Playables";

    const NAME: &'static str = "PlayableOutputHandle";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for PlayableOutputHandle {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-playables-playableoutputhandle")]
#[::unity2::methods(value)]
impl PlayableOutputHandle {
    #[method(name = "get_Null", args = 0)]
    pub fn get_null() -> crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "op_Equality", args = 2)]
    pub fn op_equality(
        lhs: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
        rhs: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
    ) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, p: crate::system::object::Object) -> bool;

    #[method(name = "Equals", args = 1)]
    pub fn equals_2(
        self,
        other: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
    ) -> bool;

    #[method(name = "CompareVersion", args = 2)]
    pub fn compare_version(
        lhs: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
        rhs: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
    ) -> bool;

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "GetPlayableOutputType", args = 0)]
    pub fn get_playable_output_type(self) -> ::unity2::SystemType;

    #[method(name = "SetReferenceObject", args = 1)]
    pub fn set_reference_object(self, target: crate::unity_engine::object_2::Object_2) -> ();

    #[method(name = "SetUserData", args = 1)]
    pub fn set_user_data(self, target: crate::unity_engine::object_2::Object_2) -> ();

    #[method(name = "GetSourcePlayable", args = 0)]
    pub fn get_source_playable(
        self,
    ) -> crate::unity_engine::playables::playablehandle::PlayableHandle;

    #[method(name = "SetSourcePlayable", args = 2)]
    pub fn set_source_playable(
        self,
        target: crate::unity_engine::playables::playablehandle::PlayableHandle,
        port: i32,
    ) -> ();

    #[method(name = "GetSourceOutputPort", args = 0)]
    pub fn get_source_output_port(self) -> i32;

    #[method(name = "SetWeight", args = 1)]
    pub fn set_weight(self, weight: f32) -> ();

    #[method(name = "PushNotification", args = 3)]
    pub fn push_notification(
        self,
        origin: crate::unity_engine::playables::playablehandle::PlayableHandle,
        notification: crate::unity_engine::playables::inotification::INotification,
        context: crate::system::object::Object,
    ) -> ();

    #[method(name = "AddNotificationReceiver", args = 1)]
    pub fn add_notification_receiver(
        self,
        receiver: crate::unity_engine::playables::inotificationreceiver::INotificationReceiver,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();

    #[method(name = "IsValid_Injected", args = 1)]
    pub fn is_valid_injected(
        unity_self: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
    ) -> bool;

    #[method(name = "GetPlayableOutputType_Injected", args = 1)]
    pub fn get_playable_output_type_injected(
        unity_self: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
    ) -> ::unity2::SystemType;

    #[method(name = "SetReferenceObject_Injected", args = 2)]
    pub fn set_reference_object_injected(
        unity_self: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
        target: crate::unity_engine::object_2::Object_2,
    ) -> ();

    #[method(name = "SetUserData_Injected", args = 2)]
    pub fn set_user_data_injected(
        unity_self: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
        target: crate::unity_engine::object_2::Object_2,
    ) -> ();

    #[method(name = "GetSourcePlayable_Injected", args = 2)]
    pub fn get_source_playable_injected(
        unity_self: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
        ret: crate::unity_engine::playables::playablehandle::PlayableHandle,
    ) -> ();

    #[method(name = "SetSourcePlayable_Injected", args = 3)]
    pub fn set_source_playable_injected(
        unity_self: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
        target: crate::unity_engine::playables::playablehandle::PlayableHandle,
        port: i32,
    ) -> ();

    #[method(name = "GetSourceOutputPort_Injected", args = 1)]
    pub fn get_source_output_port_injected(
        unity_self: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
    ) -> i32;

    #[method(name = "SetWeight_Injected", args = 2)]
    pub fn set_weight_injected(
        unity_self: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
        weight: f32,
    ) -> ();

    #[method(name = "PushNotification_Injected", args = 4)]
    pub fn push_notification_injected(
        unity_self: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
        origin: crate::unity_engine::playables::playablehandle::PlayableHandle,
        notification: crate::unity_engine::playables::inotification::INotification,
        context: crate::system::object::Object,
    ) -> ();

    #[method(name = "AddNotificationReceiver_Injected", args = 2)]
    pub fn add_notification_receiver_injected(
        unity_self: crate::unity_engine::playables::playableoutputhandle::PlayableOutputHandle,
        receiver: crate::unity_engine::playables::inotificationreceiver::INotificationReceiver,
    ) -> ();
}
