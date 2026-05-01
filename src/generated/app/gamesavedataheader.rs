
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gamesavedataheader/GameSaveDataHeader.md")))]
#[::unity2::class(namespace = "App", name = "GameSaveDataHeader")]
#[parent(crate::system::object::Object)]
pub struct GameSaveDataHeader {
    #[static_field]
    #[rename(name = "Size")]
    pub size: i32,
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[rename(name = "m_GameVersion")]
    pub m_game_version: i32,
    #[rename(name = "m_Status")]
    pub m_status: crate::app::gameuserdata::GameUserData_StatusField,
    #[rename(name = "m_Chapter")]
    pub m_chapter: crate::app::chapterdata::ChapterData,
    #[rename(name = "m_NextChapter")]
    pub m_next_chapter: crate::app::chapterdata::ChapterData,
    #[rename(name = "m_Sequence")]
    pub m_sequence: crate::app::gameuserdata::GameUserData_Sequences,
    #[rename(name = "m_GameMode")]
    pub m_game_mode: crate::app::gamemode::GameMode,
    #[rename(name = "m_Difficulty")]
    pub m_difficulty: crate::app::difficulty::Difficulty,
    #[rename(name = "m_EvilDifficulty")]
    pub m_evil_difficulty: crate::app::difficulty::Difficulty,
    #[rename(name = "m_Turn")]
    pub m_turn: i32,
    #[rename(name = "m_ContentsIndex")]
    pub m_contents_index: i32,
    #[rename(name = "m_Identifier")]
    pub m_identifier: u64,
    #[rename(name = "m_PlayTime")]
    pub m_play_time: f32,
    #[rename(name = "m_ChallengeRoute")]
    pub m_challenge_route: i32,
    #[rename(name = "m_ChallengeStage")]
    pub m_challenge_stage: i32,
    #[rename(name = "m_UnitEdit")]
    pub m_unit_edit: crate::app::unitedit::UnitEdit,
}

#[cfg(feature = "app-gamesavedataheader")]
#[::unity2::methods]
impl GameSaveDataHeader {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> bool;

    #[method(name = "get_GameVersion", args = 0)]
    pub fn get_game_version(self) -> i32;

    #[method(name = "get_Status", args = 0)]
    pub fn get_status(self) -> crate::app::gameuserdata::GameUserData_StatusField;

    #[method(name = "get_Chapter", args = 0)]
    pub fn get_chapter(self) -> crate::app::chapterdata::ChapterData;

    #[method(name = "get_Sequence", args = 0)]
    pub fn get_sequence(self) -> crate::app::gameuserdata::GameUserData_Sequences;

    #[method(name = "get_Difficulty", args = 0)]
    pub fn get_difficulty(self) -> crate::app::difficulty::Difficulty;

    #[method(name = "get_GameMode", args = 0)]
    pub fn get_game_mode(self) -> crate::app::gamemode::GameMode;

    #[method(name = "get_Turn", args = 0)]
    pub fn get_turn(self) -> i32;

    #[method(name = "get_PlayTime", args = 0)]
    pub fn get_play_time(self) -> f32;

    #[method(name = "get_ContentsIndex", args = 0)]
    pub fn get_contents_index(self) -> i32;

    #[method(name = "get_Identifier", args = 0)]
    pub fn get_identifier(self) -> u64;

    #[method(name = "get_NextChapter", args = 0)]
    pub fn get_next_chapter(self) -> crate::app::chapterdata::ChapterData;

    #[method(name = "GetChapterPrefix", args = 0)]
    pub fn get_chapter_prefix(self) -> ::unity2::Il2CppString;

    #[method(name = "GetChapterName", args = 0)]
    pub fn get_chapter_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetPlaceName", args = 0)]
    pub fn get_place_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetChapterForDisplay", args = 0)]
    pub fn get_chapter_for_display(self) -> crate::app::chapterdata::ChapterData;

    #[method(name = "GetHeroName", args = 0)]
    pub fn get_hero_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetHeroGender", args = 0)]
    pub fn get_hero_gender(self) -> crate::app::gender::Gender;

    #[method(name = "IsEncountMap", args = 0)]
    pub fn is_encount_map(self) -> bool;

    #[method(name = "IsChallengeMap", args = 0)]
    pub fn is_challenge_map(self) -> bool;

    #[method(name = "IsTemporary", args = 0)]
    pub fn is_temporary(self) -> bool;

    #[method(name = "IsCompleted", args = 0)]
    pub fn is_completed(self) -> bool;

    #[method(name = "IsEvilCompleted", args = 0)]
    pub fn is_evil_completed(self) -> bool;

    #[method(name = "get_EvilDifficulty", args = 0)]
    pub fn get_evil_difficulty(self) -> crate::app::difficulty::Difficulty;

    #[method(name = "IsEvilMap", args = 0)]
    pub fn is_evil_map(self) -> bool;
}

#[cfg(feature = "app-gamesavedataheader")]
impl GameSaveDataHeader {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameSaveDataHeader),
                ::core::stringify!(new),
            )
        });
        <Self as IGameSaveDataHeaderMethods>::ctor(this);
        this
    }
}
