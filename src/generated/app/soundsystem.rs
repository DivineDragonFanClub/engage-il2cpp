
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundsystem/SoundSystem_SoundHandle.md")))]
#[::unity2::class(namespace = "App", name = "SoundSystem.SoundHandle")]
#[parent(crate::system::object::Object)]
pub struct SoundSystem_SoundHandle {}

#[cfg(feature = "app-soundsystem")]
#[::unity2::methods]
impl SoundSystem_SoundHandle {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Finalize", args = 0)]
    pub fn finalize(self) -> ();

    #[method(name = "GetGameObject", args = 0)]
    pub fn get_game_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetEventName", args = 0)]
    pub fn get_event_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetEventId", args = 0)]
    pub fn get_event_id(self) -> u32;

    #[method(name = "GetPlayingId", args = 0)]
    pub fn get_playing_id(self) -> u32;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "IsPlaying", args = 0)]
    pub fn is_playing(self) -> bool;

    #[method(name = "GetCharacter", args = 0)]
    pub fn get_character(self) -> crate::combat::character::Character;

    #[method(name = "GetEventCharacterMouthController", args = 0)]
    pub fn get_event_character_mouth_controller(
        self,
    ) -> crate::app::eventcharactermouthcontroller::EventCharacterMouthController;

    #[method(name = "GetLipSyncDataFileName", args = 0)]
    pub fn get_lip_sync_data_file_name(self) -> ::unity2::Il2CppString;

    #[method(name = "SetCharacter", args = 1)]
    pub fn set_character(self, character: crate::combat::character::Character) -> ();

    #[method(name = "SetEventCharacterMouthController", args = 1)]
    pub fn set_event_character_mouth_controller(
        self,
        event_character_mouth_controller : crate :: app :: eventcharactermouthcontroller :: EventCharacterMouthController,
    ) -> ();

    #[method(name = "Stop", args = 1)]
    pub fn stop(self, fade_msec: i32) -> ();

    #[method(name = "Pause", args = 1)]
    pub fn pause(self, fade_msec: i32) -> ();

    #[method(name = "Resume", args = 1)]
    pub fn resume(self, fade_msec: i32) -> ();
}

#[cfg(feature = "app-soundsystem")]
impl SoundSystem_SoundHandle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundSystem_SoundHandle),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundSystem_SoundHandleMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundsystem/SoundSystem_LipSyncDataFile.md")))]
#[::unity2::class(namespace = "App", name = "SoundSystem.LipSyncDataFile")]
#[parent(crate::system::object::Object)]
pub struct SoundSystem_LipSyncDataFile {
    #[static_field]
    #[rename(name = "LoadDirectoryMax")]
    pub load_directory_max: i32,
    #[rename(name = "m_isExpandFileData")]
    pub m_is_expand_file_data: bool,
    #[rename(name = "m_lipSyncDataList")]
    pub m_lip_sync_data_list: crate::system::collections::generic::list_1::List_1<
        crate::app::soundsystem::SoundSystem_LipSyncData,
    >,
    #[rename(name = "m_rawFileHandle")]
    pub m_raw_file_handle: ::unity2::Array<crate::app::rawfilehandle::RawFileHandle>,
}

#[cfg(feature = "app-soundsystem")]
#[::unity2::methods]
impl SoundSystem_LipSyncDataFile {
    #[method(name = "GetLipSyncFilePath", args = 2)]
    pub fn get_lip_sync_file_path(
        file_name: ::unity2::Il2CppString,
        patch_index: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetLipSyncFileRelativePath", args = 1)]
    pub fn get_lip_sync_file_relative_path(
        file_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetLipSyncDataCount", args = 0)]
    pub fn get_lip_sync_data_count(self) -> i32;

    #[method(name = "GetLipSyncData", args = 1)]
    pub fn get_lip_sync_data(self, index: i32) -> crate::app::soundsystem::SoundSystem_LipSyncData;

    #[method(name = "LoadAsync", args = 1)]
    pub fn load_async(self, file_name: ::unity2::Il2CppString) -> ();

    #[method(name = "ExpandFileData", args = 0)]
    pub fn expand_file_data(self) -> bool;

    #[method(name = "Unload", args = 0)]
    pub fn unload(self) -> ();
}

#[cfg(feature = "app-soundsystem")]
impl SoundSystem_LipSyncDataFile {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundSystem_LipSyncDataFile),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundSystem_LipSyncDataFileMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundsystem/SoundSystem_ResultSoundLoad_Status.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SoundSystem_ResultSoundLoad_Status {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SoundSystem_ResultSoundLoad_Status {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SoundSystem.ResultSoundLoad.Status";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SoundSystem_ResultSoundLoad_Status {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SoundSystem_ResultSoundLoad_Status {
    pub fn load() -> Self {
        Self { value: 0 }
    }

    pub fn success() -> Self {
        Self { value: 1 }
    }

    pub fn fault() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundsystem/SoundSystem_ISoundPlay.md")))]
#[::unity2::class(namespace = "App", name = "SoundSystem.ISoundPlay")]
pub struct SoundSystem_ISoundPlay {}

#[cfg(feature = "app-soundsystem")]
#[::unity2::methods]
impl SoundSystem_ISoundPlay {
    #[method(name = "PopTemporaryGameObject", args = 0)]
    pub fn pop_temporary_game_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "IsEventLoaded", args = 1)]
    pub fn is_event_loaded(self, event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "PostEvent", args = 2)]
    pub fn post_event(
        self,
        name: ::unity2::Il2CppString,
        is_get_position: bool,
    ) -> crate::app::soundsystem::SoundSystem_SoundHandle;

    #[method(name = "PostEvent", args = 3)]
    pub fn post_event_2(
        self,
        name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        is_get_position: bool,
    ) -> crate::app::soundsystem::SoundSystem_SoundHandle;

    #[method(name = "PostEventWithTemporaryGameObject", args = 3)]
    pub fn post_event_with_temporary_game_object(
        self,
        name: ::unity2::Il2CppString,
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundsystem/SoundSystem.md")))]
#[::unity2::class(namespace = "App", name = "SoundSystem")]
#[parent(crate::system::object::Object)]
pub struct SoundSystem {
    #[static_field]
    #[rename(name = "SoundHandleCount")]
    pub sound_handle_count: i32,
}

#[cfg(feature = "app-soundsystem")]
#[::unity2::methods]
impl SoundSystem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-soundsystem")]
impl SoundSystem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundSystem),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundSystemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundsystem/SoundSystem_ResultSoundLoadList.md")))]
#[::unity2::class(namespace = "App", name = "SoundSystem.ResultSoundLoadList")]
#[parent(crate::system::object::Object)]
pub struct SoundSystem_ResultSoundLoadList {
    #[rename(name = "m_resultList")]
    pub m_result_list: crate::system::collections::generic::list_1::List_1<
        crate::app::soundsystem::SoundSystem_ResultSoundLoad,
    >,
}

#[cfg(feature = "app-soundsystem")]
#[::unity2::methods]
impl SoundSystem_ResultSoundLoadList {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, result: crate::app::soundsystem::SoundSystem_ResultSoundLoad) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "IsLoadSuccess", args = 0)]
    pub fn is_load_success(self) -> bool;
}

#[cfg(feature = "app-soundsystem")]
impl SoundSystem_ResultSoundLoadList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundSystem_ResultSoundLoadList),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundSystem_ResultSoundLoadListMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundsystem/SoundSystem_LipSyncData.md")))]
#[::unity2::class(namespace = "App", name = "SoundSystem.LipSyncData")]
#[parent(crate::system::object::Object)]
pub struct SoundSystem_LipSyncData {
    #[rename(name = "m_data")]
    pub m_data: crate::app::soundsystem::SoundSystem_LipSyncStructData,
}

#[cfg(feature = "app-soundsystem")]
#[::unity2::methods]
impl SoundSystem_LipSyncData {
    #[method(name = "get_Msec", args = 0)]
    pub fn get_msec(self) -> i32;

    #[method(name = "set_Msec", args = 1)]
    pub fn set_msec(self, value: i32) -> ();

    #[method(name = "get_Width", args = 0)]
    pub fn get_width(self) -> f32;

    #[method(name = "set_Width", args = 1)]
    pub fn set_width(self, value: f32) -> ();

    #[method(name = "get_Height", args = 0)]
    pub fn get_height(self) -> f32;

    #[method(name = "set_Height", args = 1)]
    pub fn set_height(self, value: f32) -> ();

    #[method(name = "get_Tongue", args = 0)]
    pub fn get_tongue(self) -> f32;

    #[method(name = "set_Tongue", args = 1)]
    pub fn set_tongue(self, value: f32) -> ();

    #[method(name = "get_A", args = 0)]
    pub fn get_a(self) -> f32;

    #[method(name = "set_A", args = 1)]
    pub fn set_a(self, value: f32) -> ();

    #[method(name = "get_I", args = 0)]
    pub fn get_i(self) -> f32;

    #[method(name = "set_I", args = 1)]
    pub fn set_i(self, value: f32) -> ();

    #[method(name = "get_U", args = 0)]
    pub fn get_u(self) -> f32;

    #[method(name = "set_U", args = 1)]
    pub fn set_u(self, value: f32) -> ();

    #[method(name = "get_E", args = 0)]
    pub fn get_e(self) -> f32;

    #[method(name = "set_E", args = 1)]
    pub fn set_e(self, value: f32) -> ();

    #[method(name = "get_O", args = 0)]
    pub fn get_o(self) -> f32;

    #[method(name = "set_O", args = 1)]
    pub fn set_o(self, value: f32) -> ();

    #[method(name = "get_Vol", args = 0)]
    pub fn get_vol(self) -> f32;

    #[method(name = "set_Vol", args = 1)]
    pub fn set_vol(self, value: f32) -> ();

    #[method(name = "SetData", args = 1)]
    pub fn set_data(self, data: crate::app::soundsystem::SoundSystem_LipSyncStructData) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-soundsystem")]
impl SoundSystem_LipSyncData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundSystem_LipSyncData),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundSystem_LipSyncDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundsystem/SoundSystem_LipSyncStructData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct SoundSystem_LipSyncStructData {
    pub m_msec: i32,
    pub m_width: f32,
    pub m_height: f32,
    pub m_tongue: f32,
    pub m_a: f32,
    pub m_i: f32,
    pub m_u: f32,
    pub m_e: f32,
    pub m_o: f32,
    pub m_vol: f32,
}

impl ::unity2::ClassIdentity for SoundSystem_LipSyncStructData {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SoundSystem.LipSyncStructData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SoundSystem_LipSyncStructData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundsystem/SoundSystem_ResultSoundLoad.md")))]
#[::unity2::class(namespace = "App", name = "SoundSystem.ResultSoundLoad")]
#[parent(crate::system::object::Object)]
pub struct SoundSystem_ResultSoundLoad {
    #[rename(name = "m_status")]
    pub m_status: crate::app::soundsystem::SoundSystem_ResultSoundLoad_Status,
}

#[cfg(feature = "app-soundsystem")]
#[::unity2::methods]
impl SoundSystem_ResultSoundLoad {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "SetStatus", args = 1)]
    pub fn set_status(
        self,
        status: crate::app::soundsystem::SoundSystem_ResultSoundLoad_Status,
    ) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "IsLoadSuccess", args = 0)]
    pub fn is_load_success(self) -> bool;

    #[method(name = "IsLoadFault", args = 0)]
    pub fn is_load_fault(self) -> bool;
}

#[cfg(feature = "app-soundsystem")]
impl SoundSystem_ResultSoundLoad {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SoundSystem_ResultSoundLoad),
                ::core::stringify!(new),
            )
        });
        <Self as ISoundSystem_ResultSoundLoadMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundsystem/SoundSystem_ISoundLoad.md")))]
#[::unity2::class(namespace = "App", name = "SoundSystem.ISoundLoad")]
pub struct SoundSystem_ISoundLoad {}

#[cfg(feature = "app-soundsystem")]
#[::unity2::methods]
impl SoundSystem_ISoundLoad {
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
    pub fn unprepare_event(self, event_name_array: ::unity2::Il2CppString) -> ();

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
        switch_name_array: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "UnprepareSwitch", args = 2)]
    pub fn unprepare_switch_2(
        self,
        switch_group_name: ::unity2::Il2CppString,
        switch_name_array: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "ClearPrepare", args = 0)]
    pub fn clear_prepare(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/soundsystem/SoundSystem_ISoundParam.md")))]
#[::unity2::class(namespace = "App", name = "SoundSystem.ISoundParam")]
pub struct SoundSystem_ISoundParam {}

#[cfg(feature = "app-soundsystem")]
#[::unity2::methods]
impl SoundSystem_ISoundParam {
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
}
