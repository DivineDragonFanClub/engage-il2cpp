
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::procscenesequence_1::IProcSceneSequence_1;
use crate::app::procscenesequence_1::ProcSceneSequence_1;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequence/MapSequence.md")))]
#[::unity2::class(namespace = "App", name = "MapSequence")]
# [parent (crate :: app :: procscenesequence_1 :: ProcSceneSequence_1 < crate :: app :: mapsequence :: MapSequence >)]
pub struct MapSequence {
    #[rename(name = "m_IsCompleted")]
    pub m_is_completed: bool,
    #[rename(name = "m_IsSortieCancel")]
    pub m_is_sortie_cancel: bool,
    #[rename(name = "m_IsCallangeFailer")]
    pub m_is_callange_failer: bool,
    #[rename(name = "m_PreloadHandles")]
    pub m_preload_handles: crate::system::collections::generic::list_1::List_1<
        crate::app::resourcehandle_2::ResourceHandle_2,
    >,
    #[rename(name = "m_Time")]
    pub m_time: f64,
}

#[cfg(feature = "app-mapsequence")]
#[::unity2::methods]
impl MapSequence {
    #[method(name = "get_LoadingMode", args = 0)]
    pub fn get_loading_mode(self) -> crate::app::loadingmanager::LoadingManager_Modes;

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnShutdown", args = 0)]
    pub fn on_shutdown(self) -> ();

    #[method(name = "CanAutoSave", args = 0)]
    pub fn can_auto_save(self) -> bool;

    #[method(name = "TryRestartChapterSave", args = 0)]
    pub fn try_restart_chapter_save(self) -> ();

    #[method(name = "TryRestartSortieSave", args = 0)]
    pub fn try_restart_sortie_save(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "GetGrowMode", args = 1)]
    pub fn get_grow_mode(grow: crate::app::growmode::GrowMode) -> crate::app::growmode::GrowMode;

    #[method(name = "SetupChapter", args = 0)]
    pub fn setup_chapter(self) -> ();

    #[method(name = "LoadScript", args = 0)]
    pub fn load_script(self) -> ();

    #[method(name = "UnloadScript", args = 0)]
    pub fn unload_script(self) -> ();

    #[method(name = "OpeningEvent", args = 0)]
    pub fn opening_event(self) -> ();

    #[method(name = "MapOpeningEventForReplay", args = 0)]
    pub fn map_opening_event_for_replay(self) -> ();

    #[method(name = "SetupFieldA", args = 0)]
    pub fn setup_field_a(self) -> ();

    #[method(name = "SetupFieldB", args = 0)]
    pub fn setup_field_b(self) -> ();

    #[method(name = "PostSetupField", args = 0)]
    pub fn post_setup_field(self) -> ();

    #[method(name = "LoadDispos", args = 0)]
    pub fn load_dispos(self) -> ();

    #[method(name = "UnloadDispos", args = 0)]
    pub fn unload_dispos(self) -> ();

    #[method(name = "DisposEvent", args = 0)]
    pub fn dispos_event(self) -> ();

    #[method(name = "DisposUnit", args = 0)]
    pub fn dispos_unit(self) -> ();

    #[method(name = "LoadAsyncActor", args = 0)]
    pub fn load_async_actor(self) -> ();

    #[method(name = "AddPreloadCombatAssets", args = 1)]
    pub fn add_preload_combat_assets(self, result: crate::app::assettable::AssetTable_Result)
        -> ();

    #[method(name = "PreloadCombatAssets", args = 0)]
    pub fn preload_combat_assets(self) -> ();

    #[method(name = "LoadMenu", args = 0)]
    pub fn load_menu(self) -> ();

    #[method(name = "IsLoadingMenu", args = 0)]
    pub fn is_loading_menu(self) -> bool;

    #[method(name = "UnloadMenu", args = 0)]
    pub fn unload_menu(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "SetupViewMode", args = 0)]
    pub fn setup_view_mode(self) -> ();

    #[method(name = "SetupMap", args = 0)]
    pub fn setup_map(self) -> ();

    #[method(name = "CommitTemporary", args = 0)]
    pub fn commit_temporary(self) -> ();

    #[method(name = "ResumeBranch", args = 0)]
    pub fn resume_branch(self) -> ();

    #[method(name = "UnloadActor", args = 0)]
    pub fn unload_actor(self) -> ();

    #[method(name = "UnloadCombatAssets", args = 0)]
    pub fn unload_combat_assets(self) -> ();

    #[method(name = "CleanupField", args = 0)]
    pub fn cleanup_field(self) -> ();

    #[method(name = "CleanupUnits", args = 0)]
    pub fn cleanup_units(self) -> ();

    #[method(name = "Sortie", args = 0)]
    pub fn sortie(self) -> ();

    #[method(name = "SortieBranch", args = 0)]
    pub fn sortie_branch(self) -> ();

    #[method(name = "PreSortie", args = 0)]
    pub fn pre_sortie(self) -> ();

    #[method(name = "PostSortie", args = 0)]
    pub fn post_sortie(self) -> ();

    #[method(name = "MapBegin", args = 0)]
    pub fn map_begin(self) -> ();

    #[method(name = "MapHistoryBegin", args = 0)]
    pub fn map_history_begin(self) -> ();

    #[method(name = "PostMapBegin", args = 0)]
    pub fn post_map_begin(self) -> ();

    #[method(name = "MapEnd", args = 0)]
    pub fn map_end(self) -> ();

    #[method(name = "TurnBegin", args = 0)]
    pub fn turn_begin(self) -> ();

    #[method(name = "TurnEvent", args = 0)]
    pub fn turn_event(self) -> ();

    #[method(name = "TurnBeginAfter", args = 0)]
    pub fn turn_begin_after(self) -> ();

    #[method(name = "TurnBeginMultiChange", args = 0)]
    pub fn turn_begin_multi_change(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "TryWaitTime", args = 1)]
    pub fn try_wait_time(self, time: f32) -> ();

    #[method(name = "TurnAfterEvent", args = 0)]
    pub fn turn_after_event(self) -> ();

    #[method(name = "TurnEndEvent", args = 0)]
    pub fn turn_end_event(self) -> ();

    #[method(name = "GetUnderRoofUnitCount", args = 1)]
    pub fn get_under_roof_unit_count(self, force: crate::app::force::Force_Type) -> i32;

    #[method(name = "TurnSkip", args = 0)]
    pub fn turn_skip(self) -> ();

    #[method(name = "GetFirstUnit", args = 0)]
    pub fn get_first_unit(self) -> crate::app::unit::Unit;

    #[method(name = "TurnAction", args = 0)]
    pub fn turn_action(self) -> ();

    #[method(name = "TurnEffect", args = 0)]
    pub fn turn_effect(self) -> ();

    #[method(name = "TurnEntrust", args = 0)]
    pub fn turn_entrust(self) -> ();

    #[method(name = "AutoSave", args = 0)]
    pub fn auto_save(self) -> ();

    #[method(name = "TurnBranch", args = 0)]
    pub fn turn_branch(self) -> ();

    #[method(name = "HumanStart", args = 0)]
    pub fn human_start(self) -> ();

    #[method(name = "PostHumanAIBranch", args = 0)]
    pub fn post_human_ai_branch(self) -> ();

    #[method(name = "ReplayStart", args = 0)]
    pub fn replay_start(self) -> ();

    #[method(name = "TurnEnd", args = 0)]
    pub fn turn_end(self) -> ();

    #[method(name = "TurnNext", args = 0)]
    pub fn turn_next(self) -> ();

    #[method(name = "GameEndBranch", args = 0)]
    pub fn game_end_branch(self) -> ();

    #[method(name = "UpdateReliance", args = 0)]
    pub fn update_reliance(self) -> ();

    #[method(name = "CreateCompleteTelop", args = 0)]
    pub fn create_complete_telop(self) -> ();

    #[method(name = "Complete", args = 0)]
    pub fn complete(self) -> ();

    #[method(name = "GetEncountReward", args = 0)]
    pub fn get_encount_reward(self) -> ();

    #[method(name = "TryEnding", args = 0)]
    pub fn try_ending(self) -> ();

    #[method(name = "TryChallengeResult", args = 0)]
    pub fn try_challenge_result(self) -> ();

    #[method(name = "TryRestartMapResult", args = 0)]
    pub fn try_restart_map_result(self) -> ();

    #[method(name = "GameOver", args = 0)]
    pub fn game_over(self) -> ();

    #[method(name = "TryRestart", args = 0)]
    pub fn try_restart(self) -> ();

    #[method(name = "SaveDataLoad", args = 0)]
    pub fn save_data_load(self) -> ();

    #[method(name = "SaveDataLoadResult", args = 0)]
    pub fn save_data_load_result(self) -> ();

    #[method(name = "SaveDataRelease", args = 0)]
    pub fn save_data_release(self) -> ();

    #[method(name = "SaveDataNormalize", args = 0)]
    pub fn save_data_normalize(self) -> ();

    #[method(name = "TryRestart", args = 1)]
    pub fn try_restart_2(
        self,
        target: crate::app::gameuserrestartdata::GameUserRestartData_Targtes,
    ) -> bool;

    #[method(name = "SaveDataAfter", args = 0)]
    pub fn save_data_after(self) -> ();

    #[method(name = "RestartLoad", args = 0)]
    pub fn restart_load(self) -> ();

    #[method(name = "UnitContienud", args = 0)]
    pub fn unit_contienud(self) -> ();

    #[method(name = "UnitResurrect", args = 0)]
    pub fn unit_resurrect(self) -> ();

    #[method(name = "BeginSilentEnv", args = 0)]
    pub fn begin_silent_env(self) -> ();

    #[method(name = "EndSilentEnv", args = 0)]
    pub fn end_silent_env(self) -> ();

    #[method(name = "Download", args = 0)]
    pub fn download(self) -> ();

    #[method(name = "PutBonus", args = 0)]
    pub fn put_bonus(self) -> ();

    #[method(name = "Upload", args = 0)]
    pub fn upload(self) -> ();

    #[method(name = "RankingRegisterUnit", args = 0)]
    pub fn ranking_register_unit(self) -> ();

    #[method(name = "VersusRegisterUnit", args = 0)]
    pub fn versus_register_unit(self) -> ();

    #[method(name = "RelayLoad", args = 0)]
    pub fn relay_load(self) -> ();

    #[method(name = "RelayLoadError", args = 0)]
    pub fn relay_load_error(self) -> ();

    #[method(name = "RelayShowReplayPlayerName", args = 0)]
    pub fn relay_show_replay_player_name(self) -> ();

    #[method(name = "RelayHideReplayPlayerName", args = 0)]
    pub fn relay_hide_replay_player_name(self) -> ();

    #[method(name = "RelayMessageShow", args = 0)]
    pub fn relay_message_show(self) -> ();

    #[method(name = "RelayShowWinRuleForTakeOver", args = 0)]
    pub fn relay_show_win_rule_for_take_over(self) -> ();

    #[method(name = "RelaySkipReplay", args = 0)]
    pub fn relay_skip_replay(self) -> ();

    #[method(name = "RelayReplayToTakeOver", args = 0)]
    pub fn relay_replay_to_take_over(self) -> ();

    #[method(name = "IsRelay", args = 0)]
    pub fn is_relay(self) -> bool;

    #[method(name = "IsChallenge", args = 0)]
    pub fn is_challenge(self) -> bool;

    #[method(name = "VersusLoad", args = 0)]
    pub fn versus_load(self) -> ();

    #[method(name = "VersusBranch", args = 0)]
    pub fn versus_branch(self) -> ();

    #[method(name = "StartMapEdit", args = 0)]
    pub fn start_map_edit(self) -> ();

    #[method(name = "IsVersus", args = 0)]
    pub fn is_versus(self) -> bool;

    #[method(name = "TryPatch", args = 0)]
    pub fn try_patch(self) -> ();

    #[method(name = "TickPatch", args = 0)]
    pub fn tick_patch(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "OnPersistent", args = 0)]
    pub fn on_persistent(self) -> ();

    #[method(name = "GetDebugLog", args = 0)]
    pub fn get_debug_log(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "TimerStart", args = 0)]
    pub fn timer_start(self) -> ();

    #[method(name = "TimerStopActor", args = 0)]
    pub fn timer_stop_actor(self) -> ();

    #[method(name = "TimerStopCharacter", args = 0)]
    pub fn timer_stop_character(self) -> ();

    #[method(name = "TimerStop", args = 1)]
    pub fn timer_stop(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "TryGameOverRewind", args = 0)]
    pub fn try_game_over_rewind(self) -> ();

    #[method(name = "StopBgm", args = 0)]
    pub fn stop_bgm(self) -> ();

    #[method(name = "TryResume", args = 1)]
    pub fn try_resume(p: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsequence")]
impl MapSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequence/MapSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapSequence_Label {
    pub fn init() -> Self {
        Self { value: 0 }
    }

    pub fn tick() -> Self {
        Self { value: 1 }
    }

    pub fn sortie() -> Self {
        Self { value: 2 }
    }

    pub fn resume_map() -> Self {
        Self { value: 3 }
    }

    pub fn resume_sortie() -> Self {
        Self { value: 4 }
    }

    pub fn skip_sortie() -> Self {
        Self { value: 5 }
    }

    pub fn map_start() -> Self {
        Self { value: 6 }
    }

    pub fn map_begin() -> Self {
        Self { value: 7 }
    }

    pub fn turn_begin() -> Self {
        Self { value: 8 }
    }

    pub fn turn_begin_after_rewind() -> Self {
        Self { value: 9 }
    }

    pub fn turn_branch() -> Self {
        Self { value: 10 }
    }

    pub fn turn_branch_after_rewind() -> Self {
        Self { value: 11 }
    }

    pub fn turn_human() -> Self {
        Self { value: 12 }
    }

    pub fn turn_ai() -> Self {
        Self { value: 13 }
    }

    pub fn turn_link() -> Self {
        Self { value: 14 }
    }

    pub fn turn_replay() -> Self {
        Self { value: 15 }
    }

    pub fn turn_end() -> Self {
        Self { value: 16 }
    }

    pub fn complete() -> Self {
        Self { value: 17 }
    }

    pub fn game_over() -> Self {
        Self { value: 18 }
    }

    pub fn relay_unsettled() -> Self {
        Self { value: 19 }
    }

    pub fn save_data_load() -> Self {
        Self { value: 20 }
    }

    pub fn restart_load() -> Self {
        Self { value: 21 }
    }

    pub fn relay_load() -> Self {
        Self { value: 22 }
    }

    pub fn relay_load_error() -> Self {
        Self { value: 23 }
    }

    pub fn relay_skip_replay() -> Self {
        Self { value: 24 }
    }

    pub fn relay_replay_to_take_over() -> Self {
        Self { value: 25 }
    }

    pub fn versus_load() -> Self {
        Self { value: 26 }
    }

    pub fn versus_edit() -> Self {
        Self { value: 27 }
    }

    pub fn end() -> Self {
        Self { value: 28 }
    }

    pub fn tail() -> Self {
        Self { value: 29 }
    }

    pub fn patch() -> Self {
        Self { value: 30 }
    }
}
