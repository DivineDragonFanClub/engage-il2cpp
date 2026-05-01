
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/chapterdata/ChapterData_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ChapterData_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ChapterData_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ChapterData.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ChapterData_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ChapterData_Flags {
    pub fn sally() -> Self {
        Self { value: 1 }
    }

    pub fn can_back() -> Self {
        Self { value: 2 }
    }

    pub fn sight() -> Self {
        Self { value: 4 }
    }

    pub fn kizuna() -> Self {
        Self { value: 8 }
    }

    pub fn hub() -> Self {
        Self { value: 16 }
    }

    pub fn gmap() -> Self {
        Self { value: 32 }
    }

    pub fn r#continue() -> Self {
        Self { value: 64 }
    }

    pub fn serious() -> Self {
        Self { value: 128 }
    }

    pub fn casual() -> Self {
        Self { value: 256 }
    }

    pub fn challenge() -> Self {
        Self { value: 512 }
    }

    pub fn relay() -> Self {
        Self { value: 1024 }
    }

    pub fn versus() -> Self {
        Self { value: 2048 }
    }

    pub fn test_map() -> Self {
        Self { value: 4096 }
    }

    pub fn opposition() -> Self {
        Self { value: 8192 }
    }

    pub fn high_rank_item() -> Self {
        Self { value: 16384 }
    }

    pub fn can_slope() -> Self {
        Self { value: 32768 }
    }

    pub fn side_story() -> Self {
        Self { value: 1073741824 }
    }

    pub fn scenario() -> Self {
        Self { value: -2147483648 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/chapterdata/ChapterData.md")))]
#[::unity2::class(namespace = "App", name = "ChapterData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: chapterdata :: ChapterData >)]
pub struct ChapterData {
    #[rename(name = "m_PrefixlessCid")]
    pub m_prefixless_cid: ::unity2::Il2CppString,
    #[rename(name = "m_ClearedFlagName")]
    pub m_cleared_flag_name: ::unity2::Il2CppString,
    #[rename(name = "m_GmapSpotFlagName")]
    pub m_gmap_spot_flag_name: ::unity2::Il2CppString,
    #[rename(name = "m_PlaceName")]
    pub m_place_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-chapterdata")]
#[::unity2::methods]
impl ChapterData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Cid", args = 0)]
    pub fn get_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Cid", args = 1)]
    pub fn set_cid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Alpha", args = 0)]
    pub fn get_alpha(self) -> f32;

    #[method(name = "set_Alpha", args = 1)]
    pub fn set_alpha(self, value: f32) -> ();

    #[method(name = "get_Mess", args = 0)]
    pub fn get_mess(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Mess", args = 1)]
    pub fn set_mess(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Event", args = 0)]
    pub fn get_event(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Event", args = 1)]
    pub fn set_event(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Field", args = 0)]
    pub fn get_field(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Field", args = 1)]
    pub fn set_field(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ScriptBmap", args = 0)]
    pub fn get_script_bmap(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ScriptBmap", args = 1)]
    pub fn set_script_bmap(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ScriptEncount", args = 0)]
    pub fn get_script_encount(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ScriptEncount", args = 1)]
    pub fn set_script_encount(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ScriptKizuna", args = 0)]
    pub fn get_script_kizuna(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ScriptKizuna", args = 1)]
    pub fn set_script_kizuna(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ChapterTitle", args = 0)]
    pub fn get_chapter_title(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ChapterTitle", args = 1)]
    pub fn set_chapter_title(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Terrain", args = 0)]
    pub fn get_terrain(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Terrain", args = 1)]
    pub fn set_terrain(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Dispos", args = 0)]
    pub fn get_dispos(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Dispos", args = 1)]
    pub fn set_dispos(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_NextChapter", args = 0)]
    pub fn get_next_chapter(self) -> ::unity2::Il2CppString;

    #[method(name = "set_NextChapter", args = 1)]
    pub fn set_next_chapter(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_GmapSpot", args = 0)]
    pub fn get_gmap_spot(self) -> ::unity2::Il2CppString;

    #[method(name = "set_GmapSpot", args = 1)]
    pub fn set_gmap_spot(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_GmapSpotState", args = 0)]
    pub fn get_gmap_spot_state(self) -> crate::app::gmapspot::GmapSpot_State;

    #[method(name = "set_GmapSpotState", args = 1)]
    pub fn set_gmap_spot_state(self, value: crate::app::gmapspot::GmapSpot_State) -> ();

    #[method(name = "get_GmapSpotOpenCondition", args = 0)]
    pub fn get_gmap_spot_open_condition(self) -> ::unity2::Il2CppString;

    #[method(name = "set_GmapSpotOpenCondition", args = 1)]
    pub fn set_gmap_spot_open_condition(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_GmapSpotEncount", args = 0)]
    pub fn get_gmap_spot_encount(self) -> crate::app::gmapspot::GmapSpot_EncountType;

    #[method(name = "set_GmapSpotEncount", args = 1)]
    pub fn set_gmap_spot_encount(self, value: crate::app::gmapspot::GmapSpot_EncountType) -> ();

    #[method(name = "get_EncountJobs", args = 0)]
    pub fn get_encount_jobs(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_EncountJobs", args = 1)]
    pub fn set_encount_jobs(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_Reward", args = 0)]
    pub fn get_reward(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Reward", args = 1)]
    pub fn set_reward(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HoldLevel", args = 0)]
    pub fn get_hold_level(self) -> u8;

    #[method(name = "set_HoldLevel", args = 1)]
    pub fn set_hold_level(self, value: u8) -> ();

    #[method(name = "get_Progress", args = 0)]
    pub fn get_progress(self) -> u8;

    #[method(name = "set_Progress", args = 1)]
    pub fn set_progress(self, value: u8) -> ();

    #[method(name = "get_Flag", args = 0)]
    pub fn get_flag(self) -> crate::app::chapterdata::ChapterData_Flags;

    #[method(name = "set_Flag", args = 1)]
    pub fn set_flag(self, value: crate::app::chapterdata::ChapterData_Flags) -> ();

    #[method(name = "get_SoundFieldSituation", args = 0)]
    pub fn get_sound_field_situation(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SoundFieldSituation", args = 1)]
    pub fn set_sound_field_situation(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PlayerPhaseBgm", args = 0)]
    pub fn get_player_phase_bgm(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PlayerPhaseBgm", args = 1)]
    pub fn set_player_phase_bgm(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_EnemyPhaseBgm", args = 0)]
    pub fn get_enemy_phase_bgm(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EnemyPhaseBgm", args = 1)]
    pub fn set_enemy_phase_bgm(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AllyPhaseBgm", args = 0)]
    pub fn get_ally_phase_bgm(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AllyPhaseBgm", args = 1)]
    pub fn set_ally_phase_bgm(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PlayerEncountBgm", args = 0)]
    pub fn get_player_encount_bgm(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PlayerEncountBgm", args = 1)]
    pub fn set_player_encount_bgm(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_EnemyEncountBgm", args = 0)]
    pub fn get_enemy_encount_bgm(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EnemyEncountBgm", args = 1)]
    pub fn set_enemy_encount_bgm(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_SortieBgm", args = 0)]
    pub fn get_sortie_bgm(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SortieBgm", args = 1)]
    pub fn set_sortie_bgm(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_KizunaBgm", args = 0)]
    pub fn get_kizuna_bgm(self) -> ::unity2::Il2CppString;

    #[method(name = "set_KizunaBgm", args = 1)]
    pub fn set_kizuna_bgm(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Help", args = 0)]
    pub fn get_help(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Help", args = 1)]
    pub fn set_help(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RecommendedLevel", args = 0)]
    pub fn get_recommended_level(self) -> u8;

    #[method(name = "set_RecommendedLevel", args = 1)]
    pub fn set_recommended_level(self, value: u8) -> ();

    #[method(name = "get_Nation", args = 0)]
    pub fn get_nation(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Nation", args = 1)]
    pub fn set_nation(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_NationName", args = 0)]
    pub fn get_nation_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_NationName", args = 1)]
    pub fn set_nation_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_NetKillBonusIndex", args = 0)]
    pub fn get_net_kill_bonus_index(self) -> u8;

    #[method(name = "set_NetKillBonusIndex", args = 1)]
    pub fn set_net_kill_bonus_index(self, value: u8) -> ();

    #[method(name = "get_NetRankingIndex", args = 0)]
    pub fn get_net_ranking_index(self) -> u8;

    #[method(name = "set_NetRankingIndex", args = 1)]
    pub fn set_net_ranking_index(self, value: u8) -> ();

    #[method(name = "get_Hub", args = 0)]
    pub fn get_hub(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Hub", args = 1)]
    pub fn set_hub(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetPlaceName", args = 0)]
    pub fn get_place_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetPrefixlessCid", args = 0)]
    pub fn get_prefixless_cid(self) -> ::unity2::Il2CppString;

    #[method(name = "IsFlag", args = 1)]
    pub fn is_flag(self, flags: crate::app::chapterdata::ChapterData_Flags) -> bool;

    #[method(name = "GetClearedFlagName", args = 0)]
    pub fn get_cleared_flag_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetGmapSpotFlagName", args = 0)]
    pub fn get_gmap_spot_flag_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsScenario", args = 0)]
    pub fn is_scenario(self) -> bool;

    #[method(name = "IsDlc", args = 0)]
    pub fn is_dlc(self) -> bool;

    #[method(name = "IsDlcGod", args = 0)]
    pub fn is_dlc_god(self) -> bool;

    #[method(name = "IsDlcEvil", args = 0)]
    pub fn is_dlc_evil(self) -> bool;

    #[method(name = "IsLastEvil", args = 0)]
    pub fn is_last_evil(self) -> bool;

    #[method(name = "IsEncountableType", args = 0)]
    pub fn is_encountable_type(self) -> bool;

    #[method(name = "IsTraining", args = 0)]
    pub fn is_training(self) -> bool;

    #[method(name = "IsUnknown", args = 0)]
    pub fn is_unknown(self) -> bool;

    #[method(name = "IsInvalid", args = 0)]
    pub fn is_invalid(self) -> bool;

    #[method(name = "TryGetSpotState", args = 1)]
    pub fn try_get_spot_state(self, state: crate::app::gmapspot::GmapSpot_State) -> bool;

    #[method(name = "TrySetSpotState", args = 1)]
    pub fn try_set_spot_state(self, state: crate::app::gmapspot::GmapSpot_State) -> bool;

    #[method(name = "ReplaceLabel", args = 1)]
    pub fn replace_label(self, name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetNextActivatingSubChapters", args = 0)]
    pub fn get_next_activating_sub_chapters(
        self,
    ) -> ::unity2::Array<crate::app::chapterdata::ChapterData>;

    #[method(name = "GetEncountJob", args = 1)]
    pub fn get_encount_job(self, random: crate::app::random_2::Random_2) -> ::unity2::Il2CppString;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetPrefixlessNation", args = 0)]
    pub fn get_prefixless_nation(self) -> ::unity2::Il2CppString;

    #[method(name = "GetNationLevel", args = 0)]
    pub fn get_nation_level(self) -> i32;

    #[method(name = "ForEach", args = 2)]
    pub fn for_each(
        chapter: crate::app::chapterdata::ChapterData,
        func: crate::system::action_1::Action_1<crate::app::chapterdata::ChapterData>,
    ) -> ();

    #[method(name = "Serialize", args = 2)]
    pub fn serialize(
        stream: crate::app::stream_2::Stream_2,
        chapter: crate::app::chapterdata::ChapterData,
    ) -> ();

    #[method(name = "TrySerialize", args = 2)]
    pub fn try_serialize(
        stream: crate::app::stream_2::Stream_2,
        chapter: crate::app::chapterdata::ChapterData,
    ) -> ();

    #[method(name = "TryDeserialize", args = 1)]
    pub fn try_deserialize(
        stream: crate::app::stream_2::Stream_2,
    ) -> crate::app::chapterdata::ChapterData;

    #[method(name = "GetLastChapter", args = 0)]
    pub fn get_last_chapter() -> crate::app::chapterdata::ChapterData;
}

#[cfg(feature = "app-chapterdata")]
impl ChapterData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChapterData),
                ::core::stringify!(new),
            )
        });
        <Self as IChapterDataMethods>::ctor(this);
        this
    }
}
