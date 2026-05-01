
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::app::soundsystem::ISoundSystem_SoundHandle;
use crate::app::soundsystem::SoundSystem_SoundHandle;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise.md")))]
#[::unity2::class(namespace = "App", name = "SoundWwise")]
#[parent(crate::system::object::Object)]
pub struct SoundWwise {
    #[static_field]
    #[rename(name = "WwiseGlobalObjectName")]
    pub wwise_global_object_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "AudioListenerObjectName")]
    pub audio_listener_object_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_WwiseGlobalObject")]
    pub s_wwise_global_object: crate::unity_engine::gameobject::GameObject,
    #[static_field]
    #[rename(name = "s_audioListenerObject")]
    pub s_audio_listener_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise {
    #[method(name = "Init", args = 1)]
    pub fn init(wwise_global_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "GetWwiseGlobalObject", args = 0)]
    pub fn get_wwise_global_object() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetAudioListenerObject", args = 0)]
    pub fn get_audio_listener_object() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "SetLanguage", args = 1)]
    pub fn set_language(language: crate::app::language::Language_Voices) -> ();

    #[method(name = "CalcPrepareEventNameArray", args = 1)]
    pub fn calc_prepare_event_name_array(
        event_name_array: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "UnprepareEvent", args = 1)]
    pub fn unprepare_event(event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "UnprepareEvent", args = 1)]
    pub fn unprepare_event_2(event_name_array: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "ClearPrepareEvent", args = 1)]
    pub fn clear_prepare_event(event_name_array: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "CalcUnprepareEventNameArray", args = 1)]
    pub fn calc_unprepare_event_name_array(
        event_name_array: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "UnprepareSwitch", args = 2)]
    pub fn unprepare_switch(
        switch_group_name: ::unity2::Il2CppString,
        switch_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "UnprepareSwitch", args = 2)]
    pub fn unprepare_switch_2(
        switch_group_name: ::unity2::Il2CppString,
        switch_name_array: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "ClearPrepareSwitch", args = 2)]
    pub fn clear_prepare_switch(
        switch_group_name: ::unity2::Il2CppString,
        switch_name_array: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "ClearPrepare", args = 0)]
    pub fn clear_prepare() -> ();

    #[method(name = "IsEventLoaded", args = 1)]
    pub fn is_event_loaded(event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "PostEvent", args = 2)]
    pub fn post_event(
        event_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> u32;

    #[method(name = "StopSoundOnEvent", args = 4)]
    pub fn stop_sound_on_event(
        event_name: ::unity2::Il2CppString,
        msec: i32,
        game_object: crate::unity_engine::gameobject::GameObject,
        playing_id: u32,
    ) -> ();

    #[method(name = "PauseSoundOnEvent", args = 4)]
    pub fn pause_sound_on_event(
        event_name: ::unity2::Il2CppString,
        msec: i32,
        game_object: crate::unity_engine::gameobject::GameObject,
        playing_id: u32,
    ) -> ();

    #[method(name = "ResumeSoundOnEvent", args = 4)]
    pub fn resume_sound_on_event(
        event_name: ::unity2::Il2CppString,
        msec: i32,
        game_object: crate::unity_engine::gameobject::GameObject,
        playing_id: u32,
    ) -> ();

    #[method(name = "SetVolume", args = 2)]
    pub fn set_volume(vol: f32, game_object: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "StopByPlayingID", args = 2)]
    pub fn stop_by_playing_id(playing_id: u32, msec: i32) -> ();

    #[method(name = "GetState", args = 2)]
    pub fn get_state(state_group_name: ::unity2::Il2CppString, value: u32) -> bool;

    #[method(name = "GetSwitch", args = 3)]
    pub fn get_switch(
        switch_group_name: ::unity2::Il2CppString,
        game_object_id: crate::unity_engine::gameobject::GameObject,
        value: u32,
    ) -> bool;

    #[method(name = "GetGameParameter", args = 2)]
    pub fn get_game_parameter(param_name: ::unity2::Il2CppString, value: f32) -> bool;

    #[method(name = "GetGameParameter", args = 3)]
    pub fn get_game_parameter_2(
        param_name: ::unity2::Il2CppString,
        game_object_id: crate::unity_engine::gameobject::GameObject,
        value: f32,
    ) -> bool;

    #[method(name = "SetState", args = 2)]
    pub fn set_state(
        state_group_name: ::unity2::Il2CppString,
        state_name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "SetState", args = 2)]
    pub fn set_state_2(state_group_name: ::unity2::Il2CppString, state_value: u32) -> bool;

    #[method(name = "SetSwitch", args = 3)]
    pub fn set_switch(
        switch_group_name: ::unity2::Il2CppString,
        switch_name: ::unity2::Il2CppString,
        game_object_id: crate::unity_engine::gameobject::GameObject,
    ) -> bool;

    #[method(name = "SetSwitch", args = 3)]
    pub fn set_switch_2(
        switch_group_name: ::unity2::Il2CppString,
        switch_value: u32,
        game_object_id: crate::unity_engine::gameobject::GameObject,
    ) -> bool;

    #[method(name = "SetGameParameter", args = 3)]
    pub fn set_game_parameter(param_name: ::unity2::Il2CppString, value: f32, msec: i32) -> bool;

    #[method(name = "SetGameParameter", args = 4)]
    pub fn set_game_parameter_2(
        param_name: ::unity2::Il2CppString,
        value: f32,
        game_object_id: crate::unity_engine::gameobject::GameObject,
        msec: i32,
    ) -> bool;

    #[method(name = "ResetGameParameter", args = 2)]
    pub fn reset_game_parameter(param_name: ::unity2::Il2CppString, msec: i32) -> bool;

    #[method(name = "ResetGameParameter", args = 3)]
    pub fn reset_game_parameter_2(
        param_name: ::unity2::Il2CppString,
        game_object_id: crate::unity_engine::gameobject::GameObject,
        msec: i32,
    ) -> bool;

    #[method(name = "GetPlayPosition", args = 2)]
    pub fn get_play_position(playing_id: u32, position_offset: i32) -> i32;

    #[method(name = "SetPosition", args = 4)]
    pub fn set_position(
        game_object: crate::unity_engine::gameobject::GameObject,
        pos: crate::unity_engine::vector3::Vector3,
        forward: crate::unity_engine::vector3::Vector3,
        up: crate::unity_engine::vector3::Vector3,
    ) -> bool;

    #[method(name = "GetAudioListenerPosistion", args = 0)]
    pub fn get_audio_listener_posistion() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetAudioListenerRotation", args = 0)]
    pub fn get_audio_listener_rotation() -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "SetAudioListenerPosistion", args = 1)]
    pub fn set_audio_listener_posistion(listener_pos: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "SetAudioListenerRotation", args = 1)]
    pub fn set_audio_listener_rotation(
        listener_rot: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwiseMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundHandle.md")))]
#[::unity2::class(namespace = "App", name = "SoundWwise.SoundHandle")]
#[parent(crate::app::soundsystem::SoundSystem_SoundHandle)]
pub struct SoundWwise_SoundHandle {
    #[rename(name = "m_eventName")]
    pub m_event_name: ::unity2::Il2CppString,
    #[rename(name = "m_eventId")]
    pub m_event_id: u32,
    #[rename(name = "m_lipSyncDataFileName")]
    pub m_lip_sync_data_file_name: ::unity2::Il2CppString,
    #[rename(name = "m_gameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_playingId")]
    pub m_playing_id: u32,
    #[rename(name = "m_isPlaying")]
    pub m_is_playing: bool,
    #[rename(name = "m_isTemporaryGameObject")]
    pub m_is_temporary_game_object: bool,
    #[rename(name = "m_character")]
    pub m_character: crate::combat::character::Character,
    #[rename(name = "m_eventCharacterMouthController")]
    pub m_event_character_mouth_controller:
        crate::app::eventcharactermouthcontroller::EventCharacterMouthController,
}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundHandle {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Init", args = 4)]
    pub fn init(
        self,
        event_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        playing_id: u32,
        is_temporary_game_object: bool,
    ) -> ();

    #[method(name = "GetEventName", args = 0)]
    pub fn get_event_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetEventId", args = 0)]
    pub fn get_event_id(self) -> u32;

    #[method(name = "IsPlaying", args = 0)]
    pub fn is_playing(self) -> bool;

    #[method(name = "GetGameObject", args = 0)]
    pub fn get_game_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetPlayingId", args = 0)]
    pub fn get_playing_id(self) -> u32;

    #[method(name = "GetCharacter", args = 0)]
    pub fn get_character(self) -> crate::combat::character::Character;

    #[method(name = "GetEventCharacterMouthController", args = 0)]
    pub fn get_event_character_mouth_controller(
        self,
    ) -> crate::app::eventcharactermouthcontroller::EventCharacterMouthController;

    #[method(name = "SetCharacter", args = 1)]
    pub fn set_character(self, character: crate::combat::character::Character) -> ();

    #[method(name = "SetEventCharacterMouthController", args = 1)]
    pub fn set_event_character_mouth_controller(
        self,
        event_character_mouth_controller : crate :: app :: eventcharactermouthcontroller :: EventCharacterMouthController,
    ) -> ();

    #[method(name = "GetLipSyncDataFileName", args = 0)]
    pub fn get_lip_sync_data_file_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsTemporaryObject", args = 0)]
    pub fn is_temporary_object(self) -> bool;

    #[method(name = "SetIsPlaying", args = 1)]
    pub fn set_is_playing(self, is_playing: bool) -> ();

    #[method(name = "SetLipSyncDataFileName", args = 1)]
    pub fn set_lip_sync_data_file_name(self, lip_sync_data_file_name: ::unity2::Il2CppString)
        -> ();

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "Stop", args = 1)]
    pub fn stop(self, fade_msec: i32) -> ();

    #[method(name = "Pause", args = 1)]
    pub fn pause(self, fade_msec: i32) -> ();

    #[method(name = "Resume", args = 1)]
    pub fn resume(self, fade_msec: i32) -> ();
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundHandle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundHandle),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundHandleMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundBankManager_States.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SoundWwise_SoundBankManager_States {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SoundWwise_SoundBankManager_States {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SoundWwise.SoundBankManager.States";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SoundWwise_SoundBankManager_States {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SoundWwise_SoundBankManager_States {
    pub fn loading() -> Self {
        Self { value: 0 }
    }

    pub fn loaded() -> Self {
        Self { value: 1 }
    }

    pub fn unexist() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundLoad.md")))]
#[::unity2::class(namespace = "App", name = "SoundWwise.SoundLoad")]
#[parent(crate::system::object::Object)]
pub struct SoundWwise_SoundLoad {}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundLoad {
    #[method(name = "Load", args = 1)]
    pub fn load(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "LoadAsync", args = 1)]
    pub fn load_async(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "IsLoading", args = 1)]
    pub fn is_loading_2(self, name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsLoaded", args = 1)]
    pub fn is_loaded(self, name: ::unity2::Il2CppString) -> bool;

    #[method(name = "Unload", args = 1)]
    pub fn unload(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "UnloadAll", args = 0)]
    pub fn unload_all(self) -> ();

    #[method(name = "ReloadBySetLanguage", args = 1)]
    pub fn reload_by_set_language(self, language: crate::app::language::Language_Voices) -> ();

    #[method(name = "PrepareEvent", args = 1)]
    pub fn prepare_event(self, event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "PrepareEvent", args = 1)]
    pub fn prepare_event_2(self, event_name_array: ::unity2::Array<::unity2::Il2CppString>)
        -> bool;

    #[method(name = "PrepareEventAsync", args = 2)]
    pub fn prepare_event_async(
        self,
        event_name: ::unity2::Il2CppString,
        result_sound_load: crate::app::soundsystem::SoundSystem_ResultSoundLoad,
    ) -> bool;

    #[method(name = "PrepareEventAsync", args = 2)]
    pub fn prepare_event_async_2(
        self,
        event_name_array: ::unity2::Array<::unity2::Il2CppString>,
        result_sound_load: crate::app::soundsystem::SoundSystem_ResultSoundLoad,
    ) -> bool;

    #[method(name = "UnprepareEvent", args = 1)]
    pub fn unprepare_event(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "UnprepareEvent", args = 1)]
    pub fn unprepare_event_2(self, event_name_array: ::unity2::Array<::unity2::Il2CppString>)
        -> ();

    #[method(name = "PrepareSwitch", args = 2)]
    pub fn prepare_switch(
        self,
        switch_group_name: ::unity2::Il2CppString,
        switch_name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "PrepareSwitch", args = 2)]
    pub fn prepare_switch_2(
        self,
        switch_group_name: ::unity2::Il2CppString,
        switch_name_array: ::unity2::Array<::unity2::Il2CppString>,
    ) -> bool;

    #[method(name = "PrepareSwitchAsync", args = 3)]
    pub fn prepare_switch_async(
        self,
        switch_group_name: ::unity2::Il2CppString,
        switch_name: ::unity2::Il2CppString,
        result_sound_load: crate::app::soundsystem::SoundSystem_ResultSoundLoad,
    ) -> bool;

    #[method(name = "PrepareSwitchAsync", args = 3)]
    pub fn prepare_switch_async_2(
        self,
        switch_group_name: ::unity2::Il2CppString,
        switch_name_array: ::unity2::Array<::unity2::Il2CppString>,
        result_sound_load: crate::app::soundsystem::SoundSystem_ResultSoundLoad,
    ) -> bool;

    #[method(name = "UnprepareSwitch", args = 2)]
    pub fn unprepare_switch(
        self,
        switch_group_name: ::unity2::Il2CppString,
        switch_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "UnprepareSwitch", args = 2)]
    pub fn unprepare_switch_2(
        self,
        switch_group_name: ::unity2::Il2CppString,
        switch_name_array: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "ClearPrepare", args = 0)]
    pub fn clear_prepare(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundLoad {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundLoad),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundLoadMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundPrepareManager_SwitchParam.md")))]
#[::unity2::class(namespace = "App", name = "SoundWwise.SoundPrepareManager.SwitchParam")]
#[parent(crate::app::soundwwise::SoundWwise_SoundPrepareManager_Param)]
pub struct SoundWwise_SoundPrepareManager_SwitchParam {
    #[rename(name = "m_switchGroupName")]
    pub m_switch_group_name: ::unity2::Il2CppString,
    #[rename(name = "m_switchName")]
    pub m_switch_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundPrepareManager_SwitchParam {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        switch_group_name: ::unity2::Il2CppString,
        switch_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "GetSwitchGroupName", args = 0)]
    pub fn get_switch_group_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetSwitchName", args = 0)]
    pub fn get_switch_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundPrepareManager_SwitchParam {
    pub fn new(
        switch_group_name: ::unity2::Il2CppString,
        switch_name: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundPrepareManager_SwitchParam),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundPrepareManager_SwitchParamMethods>::ctor(
            this,
            switch_group_name,
            switch_name,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundBankManager_InternalStates.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SoundWwise_SoundBankManager_InternalStates {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SoundWwise_SoundBankManager_InternalStates {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SoundWwise.SoundBankManager.InternalStates";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SoundWwise_SoundBankManager_InternalStates {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SoundWwise_SoundBankManager_InternalStates {
    pub fn loading() -> Self {
        Self { value: 0 }
    }

    pub fn load_succeeded() -> Self {
        Self { value: 1 }
    }

    pub fn load_failed() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundBankManager.md")))]
#[::unity2::class(namespace = "App", name = "SoundWwise.SoundBankManager")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: soundwwise :: SoundWwise_SoundBankManager >)]
pub struct SoundWwise_SoundBankManager {
    #[rename(name = "m_bankHandles")]
    pub m_bank_handles: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::soundwwise::SoundWwise_SoundBankManager_BankHandle,
    >,
    #[rename(name = "m_workRemovingHandles")]
    pub m_work_removing_handles: crate::system::collections::generic::list_1::List_1<
        crate::app::soundwwise::SoundWwise_SoundBankManager_BankHandle,
    >,
}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundBankManager {
    #[method(name = "Create", args = 0)]
    pub fn create() -> ();

    #[method(name = "LoadBank", args = 1)]
    pub fn load_bank(name: ::unity2::Il2CppString) -> bool;

    #[method(name = "LoadBankAsync", args = 1)]
    pub fn load_bank_async(name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading() -> bool;

    #[method(name = "UnloadBank", args = 1)]
    pub fn unload_bank(name: ::unity2::Il2CppString) -> ();

    #[method(name = "UnloadAllBanks", args = 0)]
    pub fn unload_all_banks() -> ();

    #[method(name = "Cleanup", args = 0)]
    pub fn cleanup() -> ();

    #[method(name = "ReloadBankBySetLanguage", args = 1)]
    pub fn reload_bank_by_set_language(language: crate::app::language::Language_Voices) -> ();

    #[method(name = "GetState", args = 1)]
    pub fn get_state(
        name: ::unity2::Il2CppString,
    ) -> crate::app::soundwwise::SoundWwise_SoundBankManager_States;

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "LoadBankImpl", args = 1)]
    pub fn load_bank_impl(self, bank_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "LoadBankAsyncImpl", args = 1)]
    pub fn load_bank_async_impl(self, bank_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsLoadingImpl", args = 0)]
    pub fn is_loading_impl(self) -> bool;

    #[method(name = "UnloadBankImpl", args = 1)]
    pub fn unload_bank_impl(self, bank_name: ::unity2::Il2CppString) -> ();

    #[method(name = "UnloadBankImpl", args = 1)]
    pub fn unload_bank_impl_2(
        self,
        handle: crate::app::soundwwise::SoundWwise_SoundBankManager_BankHandle,
    ) -> ();

    #[method(name = "UnloadAllBanksImpl", args = 0)]
    pub fn unload_all_banks_impl(self) -> ();

    #[method(name = "CleanupImpl", args = 0)]
    pub fn cleanup_impl(self) -> ();

    #[method(name = "ReloadImpl", args = 1)]
    pub fn reload_impl(self, language: crate::app::language::Language_Voices) -> ();

    #[method(name = "GetStateImpl", args = 1)]
    pub fn get_state_impl(
        self,
        bank_name: ::unity2::Il2CppString,
    ) -> crate::app::soundwwise::SoundWwise_SoundBankManager_States;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundBankManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundBankManager),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundBankManagerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundBankManager_AsyncBankHandle.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SoundWwise.SoundBankManager.AsyncBankHandle"
)]
#[parent(crate::app::soundwwise::SoundWwise_SoundBankManager_BankHandle)]
pub struct SoundWwise_SoundBankManager_AsyncBankHandle {}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundBankManager_AsyncBankHandle {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, name: ::unity2::Il2CppString, is_prepare_load: bool) -> ();
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundBankManager_AsyncBankHandle {
    pub fn new(name: ::unity2::Il2CppString, is_prepare_load: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundBankManager_AsyncBankHandle),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundBankManager_AsyncBankHandleMethods>::ctor(
            this,
            name,
            is_prepare_load,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundPrepareManager.md")))]
#[::unity2::class(namespace = "App", name = "SoundWwise.SoundPrepareManager")]
#[parent(crate::system::object::Object)]
pub struct SoundWwise_SoundPrepareManager {
    #[static_field]
    #[rename(name = "m_eventParamList")]
    pub m_event_param_list: crate::app::soundwwise::SoundWwise_SoundPrepareManager_EventParamList,
    #[static_field]
    #[rename(name = "m_switchGroupParamList")]
    pub m_switch_group_param_list:
        crate::app::soundwwise::SoundWwise_SoundPrepareManager_SwitchGroupParamList,
}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundPrepareManager {
    #[method(name = "IncRef_Event", args = 1)]
    pub fn inc_ref_event(event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "DecRef_Event", args = 1)]
    pub fn dec_ref_event(event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IncRef_Switch", args = 2)]
    pub fn inc_ref_switch(
        switch_group_name: ::unity2::Il2CppString,
        event_name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "DecRef_Switch", args = 2)]
    pub fn dec_ref_switch(
        switch_group_name: ::unity2::Il2CppString,
        event_name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundPrepareManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundPrepareManager),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundPrepareManagerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundPrepareManager_SwitchParamList.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SoundWwise.SoundPrepareManager.SwitchParamList"
)]
#[parent(crate::system::object::Object)]
pub struct SoundWwise_SoundPrepareManager_SwitchParamList {
    #[rename(name = "m_switchGroupName")]
    pub m_switch_group_name: ::unity2::Il2CppString,
    #[rename(name = "m_paramList")]
    pub m_param_list: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::soundwwise::SoundWwise_SoundPrepareManager_SwitchParam,
    >,
}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundPrepareManager_SwitchParamList {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, switch_group_name: ::unity2::Il2CppString) -> ();

    #[method(name = "IncRef", args = 1)]
    pub fn inc_ref(self, switch_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "DecRef", args = 1)]
    pub fn dec_ref(self, switch_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsEmpty", args = 0)]
    pub fn is_empty(self) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundPrepareManager_SwitchParamList {
    pub fn new(switch_group_name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundPrepareManager_SwitchParamList),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundPrepareManager_SwitchParamListMethods>::ctor(
            this,
            switch_group_name,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundBankManager_BankHandle.md")))]
#[::unity2::class(namespace = "App", name = "SoundWwise.SoundBankManager.BankHandle")]
#[parent(crate::system::object::Object)]
pub struct SoundWwise_SoundBankManager_BankHandle {
    #[rename(name = "m_isPrepareLoad")]
    pub m_is_prepare_load: bool,
    #[rename(name = "m_bankName")]
    pub m_bank_name: ::unity2::Il2CppString,
    #[rename(name = "m_bankID")]
    pub m_bank_id: u32,
    #[rename(name = "m_refCount")]
    pub m_ref_count: i32,
    #[rename(name = "m_state")]
    pub m_state: crate::app::soundwwise::SoundWwise_SoundBankManager_States,
    #[rename(name = "m_internalState")]
    pub m_internal_state: crate::app::soundwwise::SoundWwise_SoundBankManager_InternalStates,
}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundBankManager_BankHandle {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, bank_name: ::unity2::Il2CppString, is_prepare_load: bool) -> ();

    #[method(name = "LoadBank", args = 0)]
    pub fn load_bank(self) -> bool;

    #[method(name = "UnloadBank", args = 0)]
    pub fn unload_bank(self) -> ();

    #[method(name = "IncRef", args = 0)]
    pub fn inc_ref(self) -> ();

    #[method(name = "DecRef", args = 0)]
    pub fn dec_ref(self) -> ();

    #[method(name = "get_BankName", args = 0)]
    pub fn get_bank_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_RefCount", args = 0)]
    pub fn get_ref_count(self) -> i32;

    #[method(name = "get_State", args = 0)]
    pub fn get_state(self) -> crate::app::soundwwise::SoundWwise_SoundBankManager_States;

    #[method(name = "set_State", args = 1)]
    pub fn set_state(self, value: crate::app::soundwwise::SoundWwise_SoundBankManager_States)
        -> ();

    #[method(name = "get_InternalState", args = 0)]
    pub fn get_internal_state(
        self,
    ) -> crate::app::soundwwise::SoundWwise_SoundBankManager_InternalStates;

    #[method(name = "DoUnloadBank", args = 0)]
    pub fn do_unload_bank(self) -> ();
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundBankManager_BankHandle {
    pub fn new(bank_name: ::unity2::Il2CppString, is_prepare_load: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundBankManager_BankHandle),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundBankManager_BankHandleMethods>::ctor(
            this,
            bank_name,
            is_prepare_load,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundPrepareManager_EventParamList.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SoundWwise.SoundPrepareManager.EventParamList"
)]
#[parent(crate::system::object::Object)]
pub struct SoundWwise_SoundPrepareManager_EventParamList {
    #[rename(name = "m_paramList")]
    pub m_param_list: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::soundwwise::SoundWwise_SoundPrepareManager_EventParam,
    >,
}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundPrepareManager_EventParamList {
    #[method(name = "IncRef", args = 1)]
    pub fn inc_ref(self, event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "DecRef", args = 1)]
    pub fn dec_ref(self, event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundPrepareManager_EventParamList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundPrepareManager_EventParamList),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundPrepareManager_EventParamListMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundPrepareManager_EventParam.md")))]
#[::unity2::class(namespace = "App", name = "SoundWwise.SoundPrepareManager.EventParam")]
#[parent(crate::app::soundwwise::SoundWwise_SoundPrepareManager_Param)]
pub struct SoundWwise_SoundPrepareManager_EventParam {
    #[rename(name = "m_eventName")]
    pub m_event_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundPrepareManager_EventParam {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "GetEventName", args = 0)]
    pub fn get_event_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundPrepareManager_EventParam {
    pub fn new(event_name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundPrepareManager_EventParam),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundPrepareManager_EventParamMethods>::ctor(this, event_name);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundParam.md")))]
#[::unity2::class(namespace = "App", name = "SoundWwise.SoundParam")]
#[parent(crate::system::object::Object)]
pub struct SoundWwise_SoundParam {
    #[static_field]
    #[rename(name = "MasterVolumeMin")]
    pub master_volume_min: i32,
    #[static_field]
    #[rename(name = "MasterVolumeMax")]
    pub master_volume_max: i32,
    #[static_field]
    #[rename(name = "ConfigVolumeMin")]
    pub config_volume_min: i32,
    #[static_field]
    #[rename(name = "ConfigVolumeMax")]
    pub config_volume_max: i32,
    #[static_field]
    #[rename(name = "MasterBgmVolumeName")]
    pub master_bgm_volume_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "MasterEnvVolumeName")]
    pub master_env_volume_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "MasterSeVolumeName")]
    pub master_se_volume_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "MasterVoiceVolumeName")]
    pub master_voice_volume_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ConfigBgmVolumeName")]
    pub config_bgm_volume_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ConfigEnvVolumeName")]
    pub config_env_volume_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ConfigSeVolumeName")]
    pub config_se_volume_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ConfigVoiceVolumeName")]
    pub config_voice_volume_name: ::unity2::Il2CppString,
    #[rename(name = "m_rootGameObject")]
    pub m_root_game_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundParam {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetVolumeCommon", args = 3)]
    pub fn get_volume_common(self, param_name: ::unity2::Il2CppString, min: f32, max: f32) -> f32;

    #[method(name = "SetVolumeCommon", args = 5)]
    pub fn set_volume_common(
        self,
        param_name: ::unity2::Il2CppString,
        vol: f32,
        min: f32,
        max: f32,
        fade_msec: i32,
    ) -> ();

    #[method(name = "ResetVolumeCommon", args = 2)]
    pub fn reset_volume_common(self, param_name: ::unity2::Il2CppString, fade_msec: i32) -> ();

    #[method(name = "GetMasterVolumeCommon", args = 1)]
    pub fn get_master_volume_common(self, param_name: ::unity2::Il2CppString) -> f32;

    #[method(name = "SetMasterVolumeCommon", args = 3)]
    pub fn set_master_volume_common(
        self,
        param_name: ::unity2::Il2CppString,
        vol: f32,
        fade_msec: i32,
    ) -> ();

    #[method(name = "ResetMasterVolumeCommon", args = 2)]
    pub fn reset_master_volume_common(
        self,
        param_name: ::unity2::Il2CppString,
        fade_msec: i32,
    ) -> ();

    #[method(name = "GetConfigVolumeCommon", args = 1)]
    pub fn get_config_volume_common(self, param_name: ::unity2::Il2CppString) -> f32;

    #[method(name = "SetConfigVolumeCommon", args = 3)]
    pub fn set_config_volume_common(
        self,
        param_name: ::unity2::Il2CppString,
        vol: f32,
        fade_msec: i32,
    ) -> ();

    #[method(name = "GetMasterBgmVolume", args = 0)]
    pub fn get_master_bgm_volume(self) -> f32;

    #[method(name = "GetMasterSeVolume", args = 0)]
    pub fn get_master_se_volume(self) -> f32;

    #[method(name = "GetMasterEnvVolume", args = 0)]
    pub fn get_master_env_volume(self) -> f32;

    #[method(name = "GetMasterVoiceVolume", args = 0)]
    pub fn get_master_voice_volume(self) -> f32;

    #[method(name = "SetMasterBgmVolume", args = 2)]
    pub fn set_master_bgm_volume(self, vol: f32, fade_msec: i32) -> ();

    #[method(name = "SetMasterSeVolume", args = 2)]
    pub fn set_master_se_volume(self, vol: f32, fade_msec: i32) -> ();

    #[method(name = "SetMasterEnvVolume", args = 2)]
    pub fn set_master_env_volume(self, vol: f32, fade_msec: i32) -> ();

    #[method(name = "SetMasterVoiceVolume", args = 2)]
    pub fn set_master_voice_volume(self, vol: f32, fade_msec: i32) -> ();

    #[method(name = "ResetMasterBgmVolume", args = 1)]
    pub fn reset_master_bgm_volume(self, fade_msec: i32) -> ();

    #[method(name = "ResetMasterSeVolume", args = 1)]
    pub fn reset_master_se_volume(self, fade_msec: i32) -> ();

    #[method(name = "ResetMasterEnvVolume", args = 1)]
    pub fn reset_master_env_volume(self, fade_msec: i32) -> ();

    #[method(name = "ResetMasterVoiceVolume", args = 1)]
    pub fn reset_master_voice_volume(self, fade_msec: i32) -> ();

    #[method(name = "GetConfigBgmVolume", args = 0)]
    pub fn get_config_bgm_volume(self) -> f32;

    #[method(name = "GetConfigSeVolume", args = 0)]
    pub fn get_config_se_volume(self) -> f32;

    #[method(name = "GetConfigEnvVolume", args = 0)]
    pub fn get_config_env_volume(self) -> f32;

    #[method(name = "GetConfigVoiceVolume", args = 0)]
    pub fn get_config_voice_volume(self) -> f32;

    #[method(name = "SetConfigBgmVolume", args = 2)]
    pub fn set_config_bgm_volume(self, vol: f32, fade_msec: i32) -> ();

    #[method(name = "SetConfigSeVolume", args = 2)]
    pub fn set_config_se_volume(self, vol: f32, fade_msec: i32) -> ();

    #[method(name = "SetConfigEnvVolume", args = 2)]
    pub fn set_config_env_volume(self, vol: f32, fade_msec: i32) -> ();

    #[method(name = "SetConfigVoiceVolume", args = 2)]
    pub fn set_config_voice_volume(self, vol: f32, fade_msec: i32) -> ();

    #[method(name = "SetVolume", args = 2)]
    pub fn set_volume(
        self,
        vol: f32,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "GetValue", args = 3)]
    pub fn get_value(self, value_name: ::unity2::Il2CppString, is_global: bool, value: u32)
        -> bool;

    #[method(name = "GetValue", args = 4)]
    pub fn get_value_2(
        self,
        value_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        is_global: bool,
        value: u32,
    ) -> bool;

    #[method(name = "GetValue", args = 2)]
    pub fn get_value_3(self, value_name: ::unity2::Il2CppString, value: f32) -> bool;

    #[method(name = "GetValue", args = 3)]
    pub fn get_value_4(
        self,
        value_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        value: f32,
    ) -> bool;

    #[method(name = "SetValue", args = 3)]
    pub fn set_value(
        self,
        value_name: ::unity2::Il2CppString,
        is_global: bool,
        value: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "SetValue", args = 3)]
    pub fn set_value_2(
        self,
        value_name: ::unity2::Il2CppString,
        is_global: bool,
        value: u32,
    ) -> bool;

    #[method(name = "SetValue", args = 4)]
    pub fn set_value_3(
        self,
        value_name: ::unity2::Il2CppString,
        is_global: bool,
        value: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> bool;

    #[method(name = "SetValue", args = 4)]
    pub fn set_value_4(
        self,
        value_name: ::unity2::Il2CppString,
        is_global: bool,
        value: u32,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> bool;

    #[method(name = "SetValue", args = 2)]
    pub fn set_value_5(self, value_name: ::unity2::Il2CppString, value: f32) -> bool;

    #[method(name = "SetValue", args = 3)]
    pub fn set_value_6(
        self,
        value_name: ::unity2::Il2CppString,
        value: f32,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> bool;

    #[method(name = "SetPosition", args = 4)]
    pub fn set_position(
        self,
        pos: crate::unity_engine::vector3::Vector3,
        foward: crate::unity_engine::vector3::Vector3,
        up: crate::unity_engine::vector3::Vector3,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> bool;

    #[method(name = "GetAudioListenerObject", args = 0)]
    pub fn get_audio_listener_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetAudioListenerPosistion", args = 0)]
    pub fn get_audio_listener_posistion(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetAudioListenerRotation", args = 0)]
    pub fn get_audio_listener_rotation(self) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "SetAudioListenerPosition", args = 1)]
    pub fn set_audio_listener_position(
        self,
        listener_pos: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "SetAudioListenerRotation", args = 1)]
    pub fn set_audio_listener_rotation(
        self,
        listener_rot: crate::unity_engine::quaternion::Quaternion,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundParam {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundParam),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundParamMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundPrepareManager_Param.md")))]
#[::unity2::class(namespace = "App", name = "SoundWwise.SoundPrepareManager.Param")]
#[parent(crate::system::object::Object)]
pub struct SoundWwise_SoundPrepareManager_Param {
    #[rename(name = "m_reference")]
    pub m_reference: i32,
}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundPrepareManager_Param {
    #[method(name = "IncRef", args = 0)]
    pub fn inc_ref(self) -> bool;

    #[method(name = "DecRef", args = 0)]
    pub fn dec_ref(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundPrepareManager_Param {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundPrepareManager_Param),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundPrepareManager_ParamMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundPlay_GameObjectPool.md")))]
#[::unity2::class(namespace = "App", name = "SoundWwise.SoundPlay.GameObjectPool")]
#[parent(crate::system::object::Object)]
pub struct SoundWwise_SoundPlay_GameObjectPool {
    #[static_field]
    #[rename(name = "GameObjectDefaultNum")]
    pub game_object_default_num: i32,
    #[rename(name = "m_objList")]
    pub m_obj_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundPlay_GameObjectPool {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, parent: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "Pop", args = 0)]
    pub fn pop(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "Push", args = 1)]
    pub fn push(self, obj: crate::unity_engine::gameobject::GameObject) -> ();
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundPlay_GameObjectPool {
    pub fn new(parent: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundPlay_GameObjectPool),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundPlay_GameObjectPoolMethods>::ctor(this, parent);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundPlay.md")))]
#[::unity2::class(namespace = "App", name = "SoundWwise.SoundPlay")]
#[parent(crate::system::object::Object)]
pub struct SoundWwise_SoundPlay {
    #[static_field]
    #[rename(name = "DefaultCallbackFlag")]
    pub default_callback_flag: u32,
    #[static_field]
    #[rename(name = "GetPositionFlag")]
    pub get_position_flag: u32,
    #[rename(name = "m_rootGameObject")]
    pub m_root_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_soundObjects")]
    pub m_sound_objects: crate::app::soundwwise::SoundWwise_SoundPlay_GameObjectPool,
}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundPlay {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "PopTemporaryGameObject", args = 0)]
    pub fn pop_temporary_game_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "PushTemporaryGameObject", args = 1)]
    pub fn push_temporary_game_object(
        self,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "GetMarkerCmdArgs", args = 2)]
    pub fn get_marker_cmd_args(
        self,
        marker_name: ::unity2::Il2CppString,
        cmd_name: ::unity2::Il2CppString,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "IsEventLoaded", args = 1)]
    pub fn is_event_loaded(self, event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsEventLoaded_Common", args = 1)]
    pub fn is_event_loaded_common(self, event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "PostEvent", args = 2)]
    pub fn post_event(
        self,
        event_name: ::unity2::Il2CppString,
        is_get_position: bool,
    ) -> crate::app::soundsystem::SoundSystem_SoundHandle;

    #[method(name = "PostEvent", args = 3)]
    pub fn post_event_2(
        self,
        event_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        is_get_position: bool,
    ) -> crate::app::soundsystem::SoundSystem_SoundHandle;

    #[method(name = "PostEventWithTemporaryGameObject", args = 3)]
    pub fn post_event_with_temporary_game_object(
        self,
        event_name: ::unity2::Il2CppString,
        temporary_game_object: crate::unity_engine::gameobject::GameObject,
        is_get_position: bool,
    ) -> crate::app::soundsystem::SoundSystem_SoundHandle;

    #[method(name = "StopSoundOnEvent", args = 2)]
    pub fn stop_sound_on_event(self, event_name: ::unity2::Il2CppString, fade_msec: i32) -> ();

    #[method(name = "StopSoundOnEvent", args = 3)]
    pub fn stop_sound_on_event_2(
        self,
        event_name: ::unity2::Il2CppString,
        fade_msec: i32,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "PauseSoundOnEvent", args = 2)]
    pub fn pause_sound_on_event(self, event_name: ::unity2::Il2CppString, fade_msec: i32) -> ();

    #[method(name = "PauseSoundOnEvent", args = 3)]
    pub fn pause_sound_on_event_2(
        self,
        event_name: ::unity2::Il2CppString,
        fade_msec: i32,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "ResumeSoundOnEvent", args = 2)]
    pub fn resume_sound_on_event(self, event_name: ::unity2::Il2CppString, fade_msec: i32) -> ();

    #[method(name = "ResumeSoundOnEvent", args = 3)]
    pub fn resume_sound_on_event_2(
        self,
        event_name: ::unity2::Il2CppString,
        fade_msec: i32,
        game_object: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "StopByPlayingId", args = 2)]
    pub fn stop_by_playing_id(self, playing_id: u32, fade_msec: i32) -> ();

    #[method(name = "GetPlayPosition", args = 2)]
    pub fn get_play_position(self, playing_id: u32, position_offset: i32) -> i32;
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundPlay {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundPlay),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundPlayMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundwwise/SoundWwise_SoundPrepareManager_SwitchGroupParamList.md")))]
#[::unity2::class(
    namespace = "App",
    name = "SoundWwise.SoundPrepareManager.SwitchGroupParamList"
)]
#[parent(crate::system::object::Object)]
pub struct SoundWwise_SoundPrepareManager_SwitchGroupParamList {
    #[rename(name = "m_paramListList")]
    pub m_param_list_list: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::soundwwise::SoundWwise_SoundPrepareManager_SwitchParamList,
    >,
}

#[cfg(feature = "app-soundwwise")]
#[::unity2::methods]
impl SoundWwise_SoundPrepareManager_SwitchGroupParamList {
    #[method(name = "IncRef", args = 2)]
    pub fn inc_ref(
        self,
        switch_group_name: ::unity2::Il2CppString,
        switch_name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "DecRef", args = 2)]
    pub fn dec_ref(
        self,
        switch_group_name: ::unity2::Il2CppString,
        switch_name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-soundwwise")]
impl SoundWwise_SoundPrepareManager_SwitchGroupParamList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundWwise_SoundPrepareManager_SwitchGroupParamList),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundWwise_SoundPrepareManager_SwitchGroupParamListMethods>::ctor(this);
        this
    }
}
