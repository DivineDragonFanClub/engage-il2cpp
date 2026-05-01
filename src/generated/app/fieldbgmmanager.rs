
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fieldbgmmanager/FieldBgmManager_BgmHandle.md")))]
#[::unity2::class(namespace = "App", name = "FieldBgmManager.BgmHandle")]
#[parent(crate::system::object::Object)]
pub struct FieldBgmManager_BgmHandle {
    #[rename(name = "m_soundHandle")]
    pub m_sound_handle: crate::app::soundsystem::SoundSystem_SoundHandle,
    #[rename(name = "m_isFirstPlayed")]
    pub m_is_first_played: bool,
    #[rename(name = "m_isPaused")]
    pub m_is_paused: bool,
}

#[cfg(feature = "app-fieldbgmmanager")]
#[::unity2::methods]
impl FieldBgmManager_BgmHandle {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, sound_handle: crate::app::soundsystem::SoundSystem_SoundHandle) -> ();

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "IsFirstPlayed", args = 0)]
    pub fn is_first_played(self) -> bool;

    #[method(name = "IsPlaying", args = 0)]
    pub fn is_playing(self) -> bool;

    #[method(name = "IsPaused", args = 0)]
    pub fn is_paused(self) -> bool;

    #[method(name = "GetEventName", args = 0)]
    pub fn get_event_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Stop", args = 1)]
    pub fn stop(self, fade_msec: i32) -> ();

    #[method(name = "Pause", args = 1)]
    pub fn pause(self, fade_msec: i32) -> ();

    #[method(name = "Resume", args = 1)]
    pub fn resume(self, fade_msec: i32) -> ();

    #[method(name = "SetFirstPlayed", args = 0)]
    pub fn set_first_played(self) -> ();
}

#[cfg(feature = "app-fieldbgmmanager")]
impl FieldBgmManager_BgmHandle {
    pub fn new(sound_handle: crate::app::soundsystem::SoundSystem_SoundHandle) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FieldBgmManager_BgmHandle),
                ::core::stringify!(new),
            )
        });
        <Self as IFieldBgmManager_BgmHandleMethods>::ctor(this, sound_handle);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fieldbgmmanager/FieldBgmManager_ProcChangeBgm.md")))]
#[::unity2::class(namespace = "App", name = "FieldBgmManager.ProcChangeBgm")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: fieldbgmmanager :: FieldBgmManager_ProcChangeBgm >)]
pub struct FieldBgmManager_ProcChangeBgm {
    #[rename(name = "m_bgmManager")]
    pub m_bgm_manager: crate::app::fieldbgmmanager::FieldBgmManager,
    #[rename(name = "m_isReturnToNormalBgm")]
    pub m_is_return_to_normal_bgm: bool,
}

#[cfg(feature = "app-fieldbgmmanager")]
#[::unity2::methods]
impl FieldBgmManager_ProcChangeBgm {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        bgm_manager: crate::app::fieldbgmmanager::FieldBgmManager,
        is_return_to_normal_bgm: bool,
    ) -> ();

    #[method(name = "ProcCall_ResumeBgm", args = 0)]
    pub fn proc_call_resume_bgm(self) -> ();

    #[method(name = "ProcCall_PauseBgm", args = 0)]
    pub fn proc_call_pause_bgm(self) -> ();

    #[method(name = "Create", args = 4)]
    pub fn create(
        bgm_manager: crate::app::fieldbgmmanager::FieldBgmManager,
        force_type: crate::app::force::Force_Type,
        is_return_to_normal_bgm: bool,
        super_: crate::app::procinst::ProcInst,
    ) -> ();
}

#[cfg(feature = "app-fieldbgmmanager")]
impl FieldBgmManager_ProcChangeBgm {
    pub fn new(
        bgm_manager: crate::app::fieldbgmmanager::FieldBgmManager,
        is_return_to_normal_bgm: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FieldBgmManager_ProcChangeBgm),
                ::core::stringify!(new),
            )
        });
        <Self as IFieldBgmManager_ProcChangeBgmMethods>::ctor(
            this,
            bgm_manager,
            is_return_to_normal_bgm,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fieldbgmmanager/FieldBgmManager.md")))]
#[::unity2::class(namespace = "App", name = "FieldBgmManager")]
#[parent(crate::system::object::Object)]
pub struct FieldBgmManager {
    #[static_field]
    #[rename(name = "GameObjectName")]
    pub game_object_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FadeInMsec_ChangeBgm")]
    pub fade_in_msec_change_bgm: i32,
    #[static_field]
    #[rename(name = "FadeOutMsec_ChangeBgm")]
    pub fade_out_msec_change_bgm: i32,
    #[static_field]
    #[rename(name = "FadeInMsec_ReturnToNormalBgm")]
    pub fade_in_msec_return_to_normal_bgm: i32,
    #[static_field]
    #[rename(name = "FadeInMsec_SpecialBattleBgm")]
    pub fade_in_msec_special_battle_bgm: i32,
    #[static_field]
    #[rename(name = "FadeOutMsec_SpecialBattleBgm")]
    pub fade_out_msec_special_battle_bgm: i32,
    #[static_field]
    #[rename(name = "SpecialBattleBgmContinueTurnMax")]
    pub special_battle_bgm_continue_turn_max: i32,
    #[rename(name = "m_isSetPhaseBgm")]
    pub m_is_set_phase_bgm: bool,
    #[rename(name = "m_isFirstPhaseChanged")]
    pub m_is_first_phase_changed: ::unity2::Array<bool>,
    #[rename(name = "m_sndMgr")]
    pub m_snd_mgr: crate::app::soundmanager::SoundManager,
    #[rename(name = "m_gameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_handleNameArray")]
    pub m_handle_name_array: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_prevBgmHandleName")]
    pub m_prev_bgm_handle_name: ::unity2::Il2CppString,
    #[rename(name = "m_curBgmHandleName")]
    pub m_cur_bgm_handle_name: ::unity2::Il2CppString,
    #[rename(name = "m_bgmHandleList")]
    pub m_bgm_handle_list: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::fieldbgmmanager::FieldBgmManager_BgmHandle,
    >,
    #[rename(name = "m_specialCombatBgmHandleList")]
    pub m_special_combat_bgm_handle_list:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            crate::app::fieldbgmmanager::FieldBgmManager_BgmHandle,
        >,
    #[rename(name = "m_warSituationStateName")]
    pub m_war_situation_state_name: ::unity2::Il2CppString,
    #[rename(name = "m_volume")]
    pub m_volume: crate::app::fieldbgmmanager::FieldBgmManager_VolumeFader,
    #[rename(name = "m_restoreEventNameArray")]
    pub m_restore_event_name_array: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_restoreWarSituationStateName")]
    pub m_restore_war_situation_state_name: ::unity2::Il2CppString,
    #[rename(name = "m_BattleBgmContinueTurn")]
    pub m_battle_bgm_continue_turn: i32,
    #[static_field]
    #[rename(name = "NowStreamVersion")]
    pub now_stream_version: i32,
}

#[cfg(feature = "app-fieldbgmmanager")]
#[::unity2::methods]
impl FieldBgmManager {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, snd_mgr: crate::app::soundmanager::SoundManager) -> ();

    #[method(name = "IsSetPhaseBgm", args = 0)]
    pub fn is_set_phase_bgm(self) -> bool;

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "Final", args = 1)]
    pub fn r#final(self, fade_msec: i32) -> ();

    #[method(name = "Reset", args = 2)]
    pub fn reset(self, fade_msec: i32, is_reset_restore_data: bool) -> ();

    #[method(name = "StartSpecialBattleBgmContinueTurn", args = 0)]
    pub fn start_special_battle_bgm_continue_turn(self) -> ();

    #[method(name = "SetSpecialBattleBgmContinueTurnForRewind", args = 1)]
    pub fn set_special_battle_bgm_continue_turn_for_rewind(self, turn: i32) -> ();

    #[method(name = "SetPhaseBgm", args = 2)]
    pub fn set_phase_bgm(
        self,
        chapter: crate::app::chapterdata::ChapterData,
        is_encount: bool,
    ) -> ();

    #[method(name = "RestorePhaseBgm", args = 0)]
    pub fn restore_phase_bgm(self) -> ();

    #[method(name = "SetPhaseBgm", args = 3)]
    pub fn set_phase_bgm_2(
        self,
        player_bgm_event_name: ::unity2::Il2CppString,
        enemy_bgm_event_name: ::unity2::Il2CppString,
        ally_bgm_event_name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "PlayBgm", args = 1)]
    pub fn play_bgm(self, force_type: crate::app::force::Force_Type) -> ();

    #[method(name = "StopBgm", args = 1)]
    pub fn stop_bgm(self, fade_msec: i32) -> ();

    #[method(name = "PauseBgm", args = 1)]
    pub fn pause_bgm(self, fade_msec: i32) -> ();

    #[method(name = "ResumeBgm", args = 1)]
    pub fn resume_bgm(self, fade_msec: i32) -> ();

    #[method(name = "SetBgmVolume", args = 2)]
    pub fn set_bgm_volume(self, vol: f32, fade_msec: i32) -> ();

    #[method(name = "ChangeForceType", args = 3)]
    pub fn change_force_type(
        self,
        force_type: crate::app::force::Force_Type,
        super_: crate::app::procinst::ProcInst,
        is_turn_elapsed: bool,
    ) -> ();

    #[method(name = "ChangeForceTypeImm", args = 1)]
    pub fn change_force_type_imm(self, force_type: crate::app::force::Force_Type) -> ();

    #[method(name = "PlaySpecialBattleBgm", args = 1)]
    pub fn play_special_battle_bgm(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "PauseSpecialBattleBgm", args = 1)]
    pub fn pause_special_battle_bgm(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "StopSpecialBattleBgm", args = 1)]
    pub fn stop_special_battle_bgm(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "IsSpecialBattleBgmExist", args = 1)]
    pub fn is_special_battle_bgm_exist(self, event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "SetWarSituationParam", args = 1)]
    pub fn set_war_situation_param(self, value_name: ::unity2::Il2CppString) -> ();

    #[method(name = "RestoreWarSituationParam", args = 0)]
    pub fn restore_war_situation_param(self) -> ();

    #[method(name = "SetFirstPlayedFlag", args = 0)]
    pub fn set_first_played_flag(self) -> ();

    #[method(name = "ClearBgm", args = 1)]
    pub fn clear_bgm(self, fade_msec: i32) -> ();

    #[method(name = "GetStateGroupName", args = 0)]
    pub fn get_state_group_name(self) -> ::unity2::Il2CppString;

    #[method(name = "PlayBgm", args = 1)]
    pub fn play_bgm_2(self, bgm_handle_name: ::unity2::Il2CppString) -> ();

    #[method(name = "StopBgm", args = 2)]
    pub fn stop_bgm_2(self, bgm_handle_name: ::unity2::Il2CppString, fade_msec: i32) -> ();

    #[method(name = "PauseBgm", args = 2)]
    pub fn pause_bgm_2(self, bgm_handle_name: ::unity2::Il2CppString, fade_msec: i32) -> ();

    #[method(name = "ResumeBgm", args = 3)]
    pub fn resume_bgm_2(
        self,
        bgm_handle_name: ::unity2::Il2CppString,
        fade_msec: i32,
        isfade_msec_by_manual: bool,
    ) -> ();
}

#[cfg(feature = "app-fieldbgmmanager")]
impl FieldBgmManager {
    pub fn new(snd_mgr: crate::app::soundmanager::SoundManager) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FieldBgmManager),
                ::core::stringify!(new),
            )
        });
        <Self as IFieldBgmManagerMethods>::ctor(this, snd_mgr);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fieldbgmmanager/FieldBgmManager_VolumeFader.md")))]
#[::unity2::class(namespace = "App", name = "FieldBgmManager.VolumeFader")]
#[parent(crate::system::object::Object)]
pub struct FieldBgmManager_VolumeFader {
    #[rename(name = "m_vol")]
    pub m_vol: f32,
    #[rename(name = "m_volFrom")]
    pub m_vol_from: f32,
    #[rename(name = "m_volTo")]
    pub m_vol_to: f32,
    #[rename(name = "m_time")]
    pub m_time: f32,
    #[rename(name = "m_duration")]
    pub m_duration: f32,
}

#[cfg(feature = "app-fieldbgmmanager")]
#[::unity2::methods]
impl FieldBgmManager_VolumeFader {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = "Set", args = 2)]
    pub fn set(self, vol: f32, msec: i32) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();
}

#[cfg(feature = "app-fieldbgmmanager")]
impl FieldBgmManager_VolumeFader {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FieldBgmManager_VolumeFader),
                ::core::stringify!(new),
            )
        });
        <Self as IFieldBgmManager_VolumeFaderMethods>::ctor(this);
        this
    }
}
