
use crate::app::procdesc::IProcDesc;
use crate::app::procdesc::ProcDesc;
use crate::app::procdescuser::IProcDescUser;
use crate::app::procdescuser::ProcDescUser;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesound/GameSound.md")))]
#[::unity2::class(namespace = "App", name = "GameSound")]
#[parent(crate::system::object::Object)]
pub struct GameSound {
    #[static_field]
    #[rename(name = "DefaultBankNameArray")]
    pub default_bank_name_array: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "DefaultDLCBankNameArray")]
    pub default_dlc_bank_name_array: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "EventName_SystemVoice")]
    pub event_name_system_voice: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "EventName_UnitPickVoice")]
    pub event_name_unit_pick_voice: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SwitchGroupName_Person")]
    pub switch_group_name_person: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SwitchGroupName_Engage")]
    pub switch_group_name_engage: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SwitchGroupName_SystemVoice")]
    pub switch_group_name_system_voice: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SwitchNameArray_SystemVoice")]
    pub switch_name_array_system_voice: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "SwitchGroupName_UnitHp")]
    pub switch_group_name_unit_hp: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SwitchNameArray_UnitHp")]
    pub switch_name_array_unit_hp: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "s_initialized")]
    pub s_initialized: bool,
    #[static_field]
    #[rename(name = "s_LoadedPackageFileNameList")]
    pub s_loaded_package_file_name_list:
        crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "s_chapterBankName")]
    pub s_chapter_bank_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "strLevelTable")]
    pub str_level_table: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "strPatternTable")]
    pub str_pattern_table: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "strSituationTable")]
    pub str_situation_table: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "strRingCleaningPatternTable")]
    pub str_ring_cleaning_pattern_table: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-gamesound")]
#[::unity2::methods]
impl GameSound {
    #[method(name = "GetFadeMsecByFadeSpeedType", args = 1)]
    pub fn get_fade_msec_by_fade_speed_type(
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> i32;

    #[method(name = "GetStringByFadeSpeedType", args = 1)]
    pub fn get_string_by_fade_speed_type(
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ::unity2::Il2CppString;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable() -> bool;

    #[method(name = "IsGameSkip", args = 0)]
    pub fn is_game_skip() -> bool;

    #[method(name = "IsUseGameSoundMode", args = 0)]
    pub fn is_use_game_sound_mode() -> bool;

    #[method(name = "GetAudioListenerObject", args = 0)]
    pub fn get_audio_listener_object() -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "AddBasePaths", args = 0)]
    pub fn add_base_paths() -> ();

    #[method(name = "GetPatchVerNum", args = 0)]
    pub fn get_patch_ver_num() -> i32;

    #[method(name = "GetPatchPackageFileName", args = 1)]
    pub fn get_patch_package_file_name(patch_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetPatchPackageFilePath", args = 1)]
    pub fn get_patch_package_file_path(patch_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "LoadPackageFiles", args = 0)]
    pub fn load_package_files() -> ();

    #[method(name = "LoadPackageFile", args = 1)]
    pub fn load_package_file(patch_index: i32) -> bool;

    #[method(name = "LoadPackageFileImpl", args = 1)]
    pub fn load_package_file_impl(package_file_path: ::unity2::Il2CppString) -> bool;

    #[method(name = "ConvertToDLCSoundBankName", args = 2)]
    pub fn convert_to_dlc_sound_bank_name(
        original_bank_name: ::unity2::Il2CppString,
        package_file_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "Initialize", args = 0)]
    pub fn initialize() -> ();

    #[method(name = "LoadDefaultSoundBanks", args = 0)]
    pub fn load_default_sound_banks() -> ();

    #[method(name = "PostInitialize", args = 0)]
    pub fn post_initialize() -> ();

    #[method(name = "IsInitialized", args = 0)]
    pub fn is_initialized() -> bool;

    #[method(name = "ResetMasterVolume", args = 0)]
    pub fn reset_master_volume() -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "SetLanguage", args = 1)]
    pub fn set_language(language: crate::app::language::Language_Voices) -> ();

    #[method(name = "SetState_Language", args = 1)]
    pub fn set_state_language(language: crate::app::language::Language_Voices) -> ();

    #[method(name = "LoadBank", args = 1)]
    pub fn load_bank(bank_name: ::unity2::Il2CppString) -> ();

    #[method(name = "LoadBankAsync", args = 1)]
    pub fn load_bank_async(bank_name: ::unity2::Il2CppString) -> ();

    #[method(name = "LoadBanks", args = 1)]
    pub fn load_banks(bank_names: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "LoadBanksAsync", args = 1)]
    pub fn load_banks_async(bank_names: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "UnloadBank", args = 1)]
    pub fn unload_bank(bank_name: ::unity2::Il2CppString) -> ();

    #[method(name = "UnloadBanks", args = 1)]
    pub fn unload_banks(bank_names: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "IsLoadingBank", args = 0)]
    pub fn is_loading_bank() -> bool;

    #[method(name = "IsLoadingBank", args = 1)]
    pub fn is_loading_bank_2(bank_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "LoadChapterBank", args = 1)]
    pub fn load_chapter_bank(chapter: crate::app::chapterdata::ChapterData) -> ();

    #[method(name = "UnloadChapterBank", args = 0)]
    pub fn unload_chapter_bank() -> ();

    #[method(name = "ClearPrepare", args = 0)]
    pub fn clear_prepare() -> ();

    #[method(name = "IsEventLoaded", args = 1)]
    pub fn is_event_loaded(event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsEventPlaying", args = 1)]
    pub fn is_event_playing(event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "FindSoundHandleByEventName", args = 1)]
    pub fn find_sound_handle_by_event_name(
        event_name: ::unity2::Il2CppString,
    ) -> crate::app::gamesound::GameSound_Handle;

    #[method(name = "FindSoundHandlesByEventName", args = 1)]
    pub fn find_sound_handles_by_event_name(
        event_name: ::unity2::Il2CppString,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::gamesound::GameSound_Handle>;

    #[method(name = "GetSoundHandleList", args = 1)]
    pub fn get_sound_handle_list(
        prefix: ::unity2::Il2CppString,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::gamesound::GameSound_Handle>;

    #[method(name = "PostEvent", args = 2)]
    pub fn post_event(
        event_name: ::unity2::Il2CppString,
        character: crate::combat::character::Character,
    ) -> crate::app::gamesound::GameSound_Handle;

    #[method(name = "PostEvent", args = 3)]
    pub fn post_event_2(
        event_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        character: crate::combat::character::Character,
    ) -> crate::app::gamesound::GameSound_Handle;

    #[method(name = "PostEvent", args = 3)]
    pub fn post_event_3(
        event_name: ::unity2::Il2CppString,
        position: crate::unity_engine::vector3::Vector3,
        character: crate::combat::character::Character,
    ) -> crate::app::gamesound::GameSound_Handle;

    #[method(name = "PostEventWithTemporaryGameObject", args = 3)]
    pub fn post_event_with_temporary_game_object(
        event_name: ::unity2::Il2CppString,
        temporary_game_object: crate::unity_engine::gameobject::GameObject,
        character: crate::combat::character::Character,
    ) -> crate::app::gamesound::GameSound_Handle;

    #[method(name = "StopSoundOnEvent", args = 2)]
    pub fn stop_sound_on_event(
        event_name: ::unity2::Il2CppString,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "StopSoundOnEvent", args = 2)]
    pub fn stop_sound_on_event_2(event_name: ::unity2::Il2CppString, fade_msec: i32) -> ();

    #[method(name = "StopSoundOnEvent", args = 3)]
    pub fn stop_sound_on_event_3(
        event_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "StopSoundOnEvent", args = 3)]
    pub fn stop_sound_on_event_4(
        event_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        fade_msec: i32,
    ) -> ();

    #[method(name = "PauseSoundOnEvent", args = 2)]
    pub fn pause_sound_on_event(
        event_name: ::unity2::Il2CppString,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "PauseSoundOnEvent", args = 2)]
    pub fn pause_sound_on_event_2(event_name: ::unity2::Il2CppString, fade_msec: i32) -> ();

    #[method(name = "PauseSoundOnEvent", args = 3)]
    pub fn pause_sound_on_event_3(
        event_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "PauseSoundOnEvent", args = 3)]
    pub fn pause_sound_on_event_4(
        event_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        fade_msec: i32,
    ) -> ();

    #[method(name = "ResumeSoundOnEvent", args = 2)]
    pub fn resume_sound_on_event(
        event_name: ::unity2::Il2CppString,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "ResumeSoundOnEvent", args = 2)]
    pub fn resume_sound_on_event_2(event_name: ::unity2::Il2CppString, fade_msec: i32) -> ();

    #[method(name = "ResumeSoundOnEvent", args = 3)]
    pub fn resume_sound_on_event_3(
        event_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "ResumeSoundOnEvent", args = 3)]
    pub fn resume_sound_on_event_4(
        event_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        fade_msec: i32,
    ) -> ();

    #[method(name = "StopAllBgm", args = 1)]
    pub fn stop_all_bgm(fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType) -> ();

    #[method(name = "StopAllSe", args = 1)]
    pub fn stop_all_se(fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType) -> ();

    #[method(name = "StopAllVoice", args = 1)]
    pub fn stop_all_voice(fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType) -> ();

    #[method(name = "StopAllEnv", args = 1)]
    pub fn stop_all_env(fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType) -> ();

    #[method(name = "StopAll", args = 1)]
    pub fn stop_all(fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType) -> ();

    #[method(name = "StopAllCommon", args = 2)]
    pub fn stop_all_common(
        event_name_base: ::unity2::Il2CppString,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "GetPlayPosition", args = 1)]
    pub fn get_play_position(playing_id: u32) -> i32;

    #[method(name = "GetMasterBgmVolume", args = 0)]
    pub fn get_master_bgm_volume() -> f32;

    #[method(name = "GetMasterSeVolume", args = 0)]
    pub fn get_master_se_volume() -> f32;

    #[method(name = "GetMasterEnvVolume", args = 0)]
    pub fn get_master_env_volume() -> f32;

    #[method(name = "GetMasterVoiceVolume", args = 0)]
    pub fn get_master_voice_volume() -> f32;

    #[method(name = "SetMasterBgmVolume", args = 2)]
    pub fn set_master_bgm_volume(
        vol: f32,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "SetMasterSeVolume", args = 2)]
    pub fn set_master_se_volume(
        vol: f32,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "SetMasterEnvVolume", args = 2)]
    pub fn set_master_env_volume(
        vol: f32,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "SetMasterVoiceVolume", args = 2)]
    pub fn set_master_voice_volume(
        vol: f32,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "SetMasterAllVolume", args = 2)]
    pub fn set_master_all_volume(
        vol: f32,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "GetConfigBgmVolume", args = 0)]
    pub fn get_config_bgm_volume() -> f32;

    #[method(name = "GetConfigSeVolume", args = 0)]
    pub fn get_config_se_volume() -> f32;

    #[method(name = "GetConfigEnvVolume", args = 0)]
    pub fn get_config_env_volume() -> f32;

    #[method(name = "GetConfigVoiceVolume", args = 0)]
    pub fn get_config_voice_volume() -> f32;

    #[method(name = "SetConfigBgmVolume", args = 1)]
    pub fn set_config_bgm_volume(vol: f32) -> ();

    #[method(name = "SetConfigSeVolume", args = 1)]
    pub fn set_config_se_volume(vol: f32) -> ();

    #[method(name = "SetConfigEnvVolume", args = 1)]
    pub fn set_config_env_volume(vol: f32) -> ();

    #[method(name = "SetConfigVoiceVolume", args = 1)]
    pub fn set_config_voice_volume(vol: f32) -> ();

    #[method(name = "FieldBgm_Init", args = 0)]
    pub fn field_bgm_init() -> ();

    #[method(name = "FieldBgm_Final", args = 0)]
    pub fn field_bgm_final() -> ();

    #[method(name = "FieldBgm_IsSetPhaseBgm", args = 0)]
    pub fn field_bgm_is_set_phase_bgm() -> bool;

    #[method(name = "FieldBgm_SetPhaseBgm", args = 2)]
    pub fn field_bgm_set_phase_bgm(
        chapter: crate::app::chapterdata::ChapterData,
        is_encount: bool,
    ) -> ();

    #[method(name = "FieldBgm_RestorePhaseBgm", args = 0)]
    pub fn field_bgm_restore_phase_bgm() -> ();

    #[method(name = "FieldBgm_SetPhaseBgm", args = 3)]
    pub fn field_bgm_set_phase_bgm_2(
        player_phase_bgm: ::unity2::Il2CppString,
        enemy_phase_bgm: ::unity2::Il2CppString,
        ally_phase_bgm: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "FieldBgm_Play", args = 1)]
    pub fn field_bgm_play(force_type: crate::app::force::Force_Type) -> ();

    #[method(name = "FieldBgm_Stop", args = 1)]
    pub fn field_bgm_stop(fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType) -> ();

    #[method(name = "FieldBgm_Stop", args = 1)]
    pub fn field_bgm_stop_2(fade_msec: i32) -> ();

    #[method(name = "FieldBgm_Pause", args = 1)]
    pub fn field_bgm_pause(fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType) -> ();

    #[method(name = "FieldBgm_Pause", args = 1)]
    pub fn field_bgm_pause_2(fade_msec: i32) -> ();

    #[method(name = "FieldBgm_Resume", args = 1)]
    pub fn field_bgm_resume(fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType) -> ();

    #[method(name = "FieldBgm_Resume", args = 1)]
    pub fn field_bgm_resume_2(fade_msec: i32) -> ();

    #[method(name = "FieldBgm_SetVolume", args = 2)]
    pub fn field_bgm_set_volume(
        vol: f32,
        fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType,
    ) -> ();

    #[method(name = "FieldBgm_SetVolume", args = 2)]
    pub fn field_bgm_set_volume_2(vol: f32, fade_msec: i32) -> ();

    #[method(name = "FieldBgm_SetWarSituationParam", args = 1)]
    pub fn field_bgm_set_war_situation_param(
        war_situation_state_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "FieldBgm_RestoreWarSituationParam", args = 0)]
    pub fn field_bgm_restore_war_situation_param() -> ();

    #[method(name = "FieldBgm_StartSpecialBattleBgmContinueTurn", args = 0)]
    pub fn field_bgm_start_special_battle_bgm_continue_turn() -> ();

    #[method(name = "FieldBgm_SetSpecialBattleBgmContinueTurnForRewind", args = 1)]
    pub fn field_bgm_set_special_battle_bgm_continue_turn_for_rewind(turn: i32) -> ();

    #[method(name = "FieldBgm_SetFirstPlayedFlag", args = 0)]
    pub fn field_bgm_set_first_played_flag() -> ();

    #[method(name = "FieldBgm_ChangeForceType", args = 2)]
    pub fn field_bgm_change_force_type(
        force_type: crate::app::force::Force_Type,
        super_: crate::app::procinst::ProcInst,
    ) -> ();

    #[method(name = "FieldBgm_ChangeForceTypeImm", args = 1)]
    pub fn field_bgm_change_force_type_imm(force_type: crate::app::force::Force_Type) -> ();

    #[method(name = "FieldBgm_PlaySpecialBattleBgm", args = 1)]
    pub fn field_bgm_play_special_battle_bgm(event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "FieldBgm_PauseSpecialBattleBgm", args = 1)]
    pub fn field_bgm_pause_special_battle_bgm(event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "FieldBgm_IsSpecialBattleBgmExist", args = 1)]
    pub fn field_bgm_is_special_battle_bgm_exist(event_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "FieldBgm_ToPreBattleBgm", args = 2)]
    pub fn field_bgm_to_pre_battle_bgm(
        calculator: crate::app::battlecalculator::BattleCalculator,
        is_simple_battle: bool,
    ) -> ();

    #[method(name = "FieldBgm_ToMainBattleBgm", args = 2)]
    pub fn field_bgm_to_main_battle_bgm(
        calculator: crate::app::battlecalculator::BattleCalculator,
        is_simple_battle: bool,
    ) -> ();

    #[method(name = "FieldBgm_ReturnFromBattleBgm", args = 2)]
    pub fn field_bgm_return_from_battle_bgm(
        calculator: crate::app::battlecalculator::BattleCalculator,
        is_simple_battle: bool,
    ) -> ();

    #[method(name = "FieldBgm_IsSpecialBattleBGM", args = 1)]
    pub fn field_bgm_is_special_battle_bgm(
        calculator: crate::app::battlecalculator::BattleCalculator,
    ) -> bool;

    #[method(name = "FieldBgm_GetSpecialBattleBGMEventName", args = 1)]
    pub fn field_bgm_get_special_battle_bgm_event_name(
        calculator: crate::app::battlecalculator::BattleCalculator,
    ) -> ::unity2::Il2CppString;

    #[method(name = "IsLeaderEnemyUnit", args = 1)]
    pub fn is_leader_enemy_unit(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsCombatBgmExist", args = 1)]
    pub fn is_combat_bgm_exist(person: crate::app::persondata::PersonData) -> bool;

    #[method(name = "LoadSystemVoice", args = 1)]
    pub fn load_system_voice(
        person_switch_name: ::unity2::Il2CppString,
    ) -> crate::app::gamesound::GameSound_ResultLoad;

    #[method(name = "UnloadSystemVoice", args = 1)]
    pub fn unload_system_voice(person_switch_name: ::unity2::Il2CppString) -> ();

    #[method(name = "LoadSystemVoiceForEngageInCombat", args = 1)]
    pub fn load_system_voice_for_engage_in_combat(
        engage_switch_name: ::unity2::Il2CppString,
    ) -> crate::app::gamesound::GameSound_ResultLoad;

    #[method(name = "UnloadSystemVoiceForEngageInCombat", args = 1)]
    pub fn unload_system_voice_for_engage_in_combat(
        engage_switch_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SetState_Phase", args = 1)]
    pub fn set_state_phase(turn: crate::app::force::Force_Type) -> ();

    #[method(name = "SetState_FieldSituation", args = 1)]
    pub fn set_state_field_situation(name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetState_SpecialBattle", args = 1)]
    pub fn set_state_special_battle(name: ::unity2::Il2CppString) -> ();

    #[method(name = "RestoreSpecialBattleParam", args = 0)]
    pub fn restore_special_battle_param() -> ();

    #[method(name = "PushState_MapOrCombat", args = 1)]
    pub fn push_state_map_or_combat(name: ::unity2::Il2CppString) -> ();

    #[method(name = "PopState_MapOrCombat", args = 0)]
    pub fn pop_state_map_or_combat() -> ();

    #[method(name = "SetSwitch_Person", args = 1)]
    pub fn set_switch_person(name: ::unity2::Il2CppString) -> bool;

    #[method(name = "SetSwitch_Person", args = 2)]
    pub fn set_switch_person_2(
        game_object: crate::unity_engine::gameobject::GameObject,
        name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "SetSwitch_Engage", args = 1)]
    pub fn set_switch_engage(name: ::unity2::Il2CppString) -> bool;

    #[method(name = "SetSwitch_Engage", args = 2)]
    pub fn set_switch_engage_2(
        game_object: crate::unity_engine::gameobject::GameObject,
        name: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "ConvertHeroEngageSwitchName", args = 1)]
    pub fn convert_hero_engage_switch_name(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "SetSwitch_Weapon", args = 3)]
    pub fn set_switch_weapon(
        weapon_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        item_kind: crate::app::itemdata::ItemData_Kinds,
    ) -> ();

    #[method(name = "ItemGet", args = 2)]
    pub fn item_get(item: crate::app::itemdata::ItemData, is_force_important: bool) -> ();

    #[method(name = "MoneyGet", args = 0)]
    pub fn money_get() -> ();

    #[method(name = "JoinUnit", args = 0)]
    pub fn join_unit() -> ();

    #[method(name = "SelectUnit", args = 0)]
    pub fn select_unit() -> ();

    #[method(name = "ExchangeUnit", args = 0)]
    pub fn exchange_unit() -> ();

    #[method(name = "Cancel", args = 0)]
    pub fn cancel() -> ();

    #[method(name = "Failure", args = 0)]
    pub fn failure() -> ();

    #[method(name = "UnitSound", args = 7)]
    pub fn unit_sound(
        event_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        asset_sound: crate::app::assettable::AssetTable_Sound,
        ground_material: ::unity2::Il2CppString,
        asset_weapon_name: ::unity2::Il2CppString,
        item_kind: crate::app::itemdata::ItemData_Kinds,
        unit_speed: f32,
    ) -> ();

    #[method(name = "UnitSound", args = 7)]
    pub fn unit_sound_2(
        event_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        asset_sound: crate::app::assettable::AssetTable_Sound,
        position: crate::unity_engine::vector3::Vector3,
        asset_weapon_name: ::unity2::Il2CppString,
        item_kind: crate::app::itemdata::ItemData_Kinds,
        unit_speed: f32,
    ) -> ();

    #[method(name = "Footstep", args = 4)]
    pub fn footstep(
        game_object: crate::unity_engine::gameobject::GameObject,
        ground_material: ::unity2::Il2CppString,
        asset_sound: crate::app::assettable::AssetTable_Sound,
        walking_speed: f32,
    ) -> ();

    #[method(name = "Footstep", args = 4)]
    pub fn footstep_2(
        game_object: crate::unity_engine::gameobject::GameObject,
        position: crate::unity_engine::vector3::Vector3,
        asset_sound: crate::app::assettable::AssetTable_Sound,
        walking_speed: f32,
    ) -> ();

    #[method(name = "Landing", args = 3)]
    pub fn landing(
        game_object: crate::unity_engine::gameobject::GameObject,
        ground_material: ::unity2::Il2CppString,
        asset_sound: crate::app::assettable::AssetTable_Sound,
    ) -> ();

    #[method(name = "Landing", args = 3)]
    pub fn landing_2(
        game_object: crate::unity_engine::gameobject::GameObject,
        position: crate::unity_engine::vector3::Vector3,
        asset_sound: crate::app::assettable::AssetTable_Sound,
    ) -> ();

    #[method(name = "DefeatLanding", args = 3)]
    pub fn defeat_landing(
        game_object: crate::unity_engine::gameobject::GameObject,
        ground_material: ::unity2::Il2CppString,
        asset_sound: crate::app::assettable::AssetTable_Sound,
    ) -> ();

    #[method(name = "DefeatLanding", args = 3)]
    pub fn defeat_landing_2(
        game_object: crate::unity_engine::gameobject::GameObject,
        position: crate::unity_engine::vector3::Vector3,
        asset_sound: crate::app::assettable::AssetTable_Sound,
    ) -> ();

    #[method(name = "TerrainSound", args = 6)]
    pub fn terrain_sound(
        event_name: ::unity2::Il2CppString,
        game_object: crate::unity_engine::gameobject::GameObject,
        ground_material: ::unity2::Il2CppString,
        unit_footstep: ::unity2::Il2CppString,
        unit_material: ::unity2::Il2CppString,
        walking_speed: f32,
    ) -> ();

    #[method(name = "Flap", args = 2)]
    pub fn flap(
        game_object: crate::unity_engine::gameobject::GameObject,
        unit_footstep: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Flap", args = 2)]
    pub fn flap_2(
        game_object: crate::unity_engine::gameobject::GameObject,
        asset_sound: crate::app::assettable::AssetTable_Sound,
    ) -> ();

    #[method(name = "Swing", args = 4)]
    pub fn swing(
        game_object: crate::unity_engine::gameobject::GameObject,
        asset_weapon_name: ::unity2::Il2CppString,
        item_kind: crate::app::itemdata::ItemData_Kinds,
        unit_speed: f32,
    ) -> ();

    #[method(name = "Shoot", args = 4)]
    pub fn shoot(
        game_object: crate::unity_engine::gameobject::GameObject,
        asset_weapon_name: ::unity2::Il2CppString,
        item_kind: crate::app::itemdata::ItemData_Kinds,
        unit_speed: f32,
    ) -> ();

    #[method(name = "WeaponLanding", args = 4)]
    pub fn weapon_landing(
        game_object: crate::unity_engine::gameobject::GameObject,
        asset_weapon_name: ::unity2::Il2CppString,
        item_kind: crate::app::itemdata::ItemData_Kinds,
        ground_material: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "WeaponLanding", args = 4)]
    pub fn weapon_landing_2(
        game_object: crate::unity_engine::gameobject::GameObject,
        asset_weapon_name: ::unity2::Il2CppString,
        item_kind: crate::app::itemdata::ItemData_Kinds,
        position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "WeaponOpen", args = 3)]
    pub fn weapon_open(
        game_object: crate::unity_engine::gameobject::GameObject,
        asset_weapon_name: ::unity2::Il2CppString,
        item_kind: crate::app::itemdata::ItemData_Kinds,
    ) -> ();

    #[method(name = "WeaponClose", args = 3)]
    pub fn weapon_close(
        game_object: crate::unity_engine::gameobject::GameObject,
        asset_weapon_name: ::unity2::Il2CppString,
        item_kind: crate::app::itemdata::ItemData_Kinds,
    ) -> ();

    #[method(name = "Hit", args = 4)]
    pub fn hit(
        position: crate::unity_engine::vector3::Vector3,
        attack_type: crate::app::attacktype::AttackType,
        damage_level: crate::app::damagelevel::DamageLevel,
        asset_sound: crate::app::assettable::AssetTable_Sound,
    ) -> ();

    #[method(name = "Hit", args = 4)]
    pub fn hit_2(
        position: crate::unity_engine::vector3::Vector3,
        attack_type: crate::app::attacktype::AttackType,
        damage_level: crate::app::damagelevel::DamageLevel,
        asset_result: crate::app::assettable::AssetTable_Result,
    ) -> ();

    #[method(name = "Hit", args = 4)]
    pub fn hit_3(
        position: crate::unity_engine::vector3::Vector3,
        attack_type: crate::app::attacktype::AttackType,
        damage_level: crate::app::damagelevel::DamageLevel,
        unit_material: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "IsSeriousChapter", args = 0)]
    pub fn is_serious_chapter() -> bool;

    #[method(name = "TalkVoice", args = 1)]
    pub fn talk_voice(event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "TalkVoice", args = 2)]
    pub fn talk_voice_2(
        event_name: ::unity2::Il2CppString,
        character: crate::combat::character::Character,
    ) -> ();

    #[method(name = "PersonVoice", args = 3)]
    pub fn person_voice(
        person_switch_name: ::unity2::Il2CppString,
        engage_switch_name: ::unity2::Il2CppString,
        event_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "PersonVoice", args = 4)]
    pub fn person_voice_2(
        person_switch_name: ::unity2::Il2CppString,
        engage_switch_name: ::unity2::Il2CppString,
        event_name: ::unity2::Il2CppString,
        character: crate::combat::character::Character,
    ) -> ();

    #[method(name = "PersonVoice", args = 4)]
    pub fn person_voice_3(
        game_object: crate::unity_engine::gameobject::GameObject,
        person_switch_name: ::unity2::Il2CppString,
        engage_switch_name: ::unity2::Il2CppString,
        event_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "PersonVoice", args = 5)]
    pub fn person_voice_4(
        game_object: crate::unity_engine::gameobject::GameObject,
        person_switch_name: ::unity2::Il2CppString,
        engage_switch_name: ::unity2::Il2CppString,
        event_name: ::unity2::Il2CppString,
        character: crate::combat::character::Character,
    ) -> ();

    #[method(name = "UnitPickVoice", args = 1)]
    pub fn unit_pick_voice(unit: crate::app::unit::Unit) -> ();

    #[method(name = "GetWakeupSoundBankName", args = 1)]
    pub fn get_wakeup_sound_bank_name(patch_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "LoadWakeupVoice", args = 0)]
    pub fn load_wakeup_voice() -> ();

    #[method(name = "UnloadWakeupVoice", args = 0)]
    pub fn unload_wakeup_voice() -> ();

    #[method(name = "WakeupVoice", args = 5)]
    pub fn wakeup_voice(
        person_switch_name: ::unity2::Il2CppString,
        level: crate::app::reliancedata::RelianceData_Level,
        situation: crate::app::gamesound::GameSound_WakeupVoiceSituation,
        pattern: crate::app::gamesound::GameSound_WakeupVoicePattern,
        character: crate::combat::character::Character,
    ) -> ::unity2::Il2CppString;

    #[method(name = "RingCleaningVoice", args = 2)]
    pub fn ring_cleaning_voice(
        person_switch_name: ::unity2::Il2CppString,
        pattern: crate::app::gamesound::GameSound_RingCleaningVoicePattern,
    ) -> ();

    #[method(name = "RingCleaningVoice", args = 3)]
    pub fn ring_cleaning_voice_2(
        person_switch_name: ::unity2::Il2CppString,
        pattern: crate::app::gamesound::GameSound_RingCleaningVoicePattern,
        number: i32,
    ) -> ();

    #[method(name = "RingCleaningVoice", args = 3)]
    pub fn ring_cleaning_voice_3(
        person_switch_name: ::unity2::Il2CppString,
        event_name: ::unity2::Il2CppString,
        character: crate::combat::character::Character,
    ) -> ();

    #[method(name = "IsPlayingRingFinishCleaningVoice", args = 0)]
    pub fn is_playing_ring_finish_cleaning_voice() -> bool;

    #[method(name = "GetEventSeVolume", args = 0)]
    pub fn get_event_se_volume() -> f32;

    #[method(name = "IsEventSePlaying", args = 0)]
    pub fn is_event_se_playing() -> bool;

    #[method(name = "UnloadLipSyncDataAll", args = 0)]
    pub fn unload_lip_sync_data_all() -> ();

    #[method(name = "GetLipSyncData", args = 1)]
    pub fn get_lip_sync_data(
        event_name: ::unity2::Il2CppString,
    ) -> crate::app::gamesound::GameSound_LipSyncData;

    #[method(name = "PEvent", args = 1)]
    pub fn p_event(event_name: ::unity2::Il2CppString) -> crate::app::procdesc::ProcDesc;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-gamesound")]
impl GameSound {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSound),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSoundMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesound/GameSound_WakeupVoicePattern.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameSound_WakeupVoicePattern {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameSound_WakeupVoicePattern {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameSound.WakeupVoicePattern";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameSound_WakeupVoicePattern {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameSound_WakeupVoicePattern {
    pub fn pattern1() -> Self {
        Self { value: 0 }
    }

    pub fn pattern2() -> Self {
        Self { value: 1 }
    }

    pub fn max() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesound/GameSound_UnitHP.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameSound_UnitHP {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameSound_UnitHP {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameSound.UnitHP";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameSound_UnitHP {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameSound_UnitHP {
    pub fn high() -> Self {
        Self { value: 0 }
    }

    pub fn high_serious() -> Self {
        Self { value: 1 }
    }

    pub fn middle() -> Self {
        Self { value: 2 }
    }

    pub fn low() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesound/GameSound_LipSyncData.md")))]
#[::unity2::class(namespace = "App", name = "GameSound.LipSyncData")]
#[parent(crate::system::object::Object)]
pub struct GameSound_LipSyncData {
    #[rename(name = "m_data")]
    pub m_data: crate::app::soundsystem::SoundSystem_LipSyncData,
}

#[cfg(feature = "app-gamesound")]
#[::unity2::methods]
impl GameSound_LipSyncData {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, data: crate::app::soundsystem::SoundSystem_LipSyncData) -> ();

    #[method(name = "get_Msec", args = 0)]
    pub fn get_msec(self) -> f32;

    #[method(name = "get_A", args = 0)]
    pub fn get_a(self) -> f32;

    #[method(name = "get_I", args = 0)]
    pub fn get_i(self) -> f32;

    #[method(name = "get_U", args = 0)]
    pub fn get_u(self) -> f32;

    #[method(name = "get_E", args = 0)]
    pub fn get_e(self) -> f32;

    #[method(name = "get_O", args = 0)]
    pub fn get_o(self) -> f32;

    #[method(name = "get_Vol", args = 0)]
    pub fn get_vol(self) -> f32;

    #[method(name = "get_Width", args = 0)]
    pub fn get_width(self) -> f32;

    #[method(name = "get_Height", args = 0)]
    pub fn get_height(self) -> f32;
}

#[cfg(feature = "app-gamesound")]
impl GameSound_LipSyncData {
    pub fn new(data: crate::app::soundsystem::SoundSystem_LipSyncData) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSound_LipSyncData),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSound_LipSyncDataMethods>::ctor(this, data);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesound/GameSound_Handle.md")))]
#[::unity2::class(namespace = "App", name = "GameSound.Handle")]
#[parent(crate::system::object::Object)]
pub struct GameSound_Handle {
    #[rename(name = "m_soundHandle")]
    pub m_sound_handle: crate::app::soundsystem::SoundSystem_SoundHandle,
}

#[cfg(feature = "app-gamesound")]
#[::unity2::methods]
impl GameSound_Handle {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, sound_handle: crate::app::soundsystem::SoundSystem_SoundHandle) -> ();

    #[method(name = "GetEventName", args = 0)]
    pub fn get_event_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetGameObject", args = 0)]
    pub fn get_game_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetPlayingId", args = 0)]
    pub fn get_playing_id(self) -> u32;

    #[method(name = "GetLipSyncDataFileName", args = 0)]
    pub fn get_lip_sync_data_file_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "IsPlaying", args = 0)]
    pub fn is_playing(self) -> bool;

    #[method(name = "Stop", args = 1)]
    pub fn stop(self, fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType) -> ();

    #[method(name = "Stop", args = 1)]
    pub fn stop_2(self, fade_msec: i32) -> ();

    #[method(name = "Pause", args = 1)]
    pub fn pause(self, fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType) -> ();

    #[method(name = "Pause", args = 1)]
    pub fn pause_2(self, fade_msec: i32) -> ();

    #[method(name = "Resume", args = 1)]
    pub fn resume(self, fade_speed_type: crate::app::gamesound::GameSound_FadeSpeedType) -> ();

    #[method(name = "Resume", args = 1)]
    pub fn resume_2(self, fade_msec: i32) -> ();
}

#[cfg(feature = "app-gamesound")]
impl GameSound_Handle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSound_Handle),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSound_HandleMethods>::ctor(this);
        this
    }

    pub fn new_2(sound_handle: crate::app::soundsystem::SoundSystem_SoundHandle) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSound_Handle),
                ::core::stringify!(new_2),
            )
        });
        <Self as IGameSound_HandleMethods>::ctor_2(this, sound_handle);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesound/GameSound_WakeupVoiceSituation.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameSound_WakeupVoiceSituation {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameSound_WakeupVoiceSituation {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameSound.WakeupVoiceSituation";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameSound_WakeupVoiceSituation {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameSound_WakeupVoiceSituation {
    pub fn enter() -> Self {
        Self { value: 0 }
    }

    pub fn greet_before() -> Self {
        Self { value: 1 }
    }

    pub fn wakeup() -> Self {
        Self { value: 2 }
    }

    pub fn greet_after() -> Self {
        Self { value: 3 }
    }

    pub fn max() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesound/GameSound_FadeSpeedType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameSound_FadeSpeedType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameSound_FadeSpeedType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameSound.FadeSpeedType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameSound_FadeSpeedType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameSound_FadeSpeedType {
    pub fn immediate() -> Self {
        Self { value: 0 }
    }

    pub fn very_fast() -> Self {
        Self { value: 1 }
    }

    pub fn fast() -> Self {
        Self { value: 2 }
    }

    pub fn normal() -> Self {
        Self { value: 3 }
    }

    pub fn slow() -> Self {
        Self { value: 4 }
    }

    pub fn very_slow() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesound/GameSound_ProcDescPostEvent.md")))]
#[::unity2::class(namespace = "App", name = "GameSound.ProcDescPostEvent")]
#[parent(crate::app::procdescuser::ProcDescUser)]
pub struct GameSound_ProcDescPostEvent {
    #[rename(name = "m_eventName")]
    pub m_event_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-gamesound")]
#[::unity2::methods]
impl GameSound_ProcDescPostEvent {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "Execute", args = 1)]
    pub fn execute(
        self,
        inst: crate::app::procinst::ProcInst,
    ) -> crate::app::procdesc::ProcDesc_Result;
}

#[cfg(feature = "app-gamesound")]
impl GameSound_ProcDescPostEvent {
    pub fn new(event_name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSound_ProcDescPostEvent),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSound_ProcDescPostEventMethods>::ctor(this, event_name);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesound/GameSound_ResultLoad.md")))]
#[::unity2::class(namespace = "App", name = "GameSound.ResultLoad")]
#[parent(crate::system::object::Object)]
pub struct GameSound_ResultLoad {
    #[rename(name = "m_resultList")]
    pub m_result_list: crate::app::soundsystem::SoundSystem_ResultSoundLoadList,
}

#[cfg(feature = "app-gamesound")]
#[::unity2::methods]
impl GameSound_ResultLoad {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add(self, result: crate::app::soundsystem::SoundSystem_ResultSoundLoad) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "IsLoadSuccess", args = 0)]
    pub fn is_load_success(self) -> bool;
}

#[cfg(feature = "app-gamesound")]
impl GameSound_ResultLoad {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSound_ResultLoad),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSound_ResultLoadMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesound/GameSound_RingCleaningVoicePattern.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameSound_RingCleaningVoicePattern {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameSound_RingCleaningVoicePattern {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameSound.RingCleaningVoicePattern";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameSound_RingCleaningVoicePattern {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameSound_RingCleaningVoicePattern {
    pub fn touch() -> Self {
        Self { value: 0 }
    }

    pub fn dirt() -> Self {
        Self { value: 1 }
    }

    pub fn thanks() -> Self {
        Self { value: 2 }
    }

    pub fn num() -> Self {
        Self { value: 3 }
    }
}
