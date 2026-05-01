
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/charactersound/CharacterSound.md")))]
#[::unity2::class(namespace = "Combat", name = "CharacterSound")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CharacterSound {
    #[rename(name = "_cp")]
    pub cp: crate::combat::character::Character,
    #[rename(name = "m_LoadObj")]
    pub m_load_obj: crate::app::gamesound::GameSound_ResultLoad,
    #[rename(name = "m_VoiceID")]
    pub m_voice_id: ::unity2::Il2CppString,
    #[rename(name = "m_bLoaded")]
    pub m_b_loaded: bool,
    #[rename(name = "m_bWinVoicePlayed")]
    pub m_b_win_voice_played: bool,
    #[rename(name = "m_IsVoiceLoad")]
    pub m_is_voice_load: bool,
}

#[cfg(feature = "combat-charactersound")]
#[::unity2::methods]
impl CharacterSound {
    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "get_VoiceCancelTimes", args = 0)]
    pub fn get_voice_cancel_times(self) -> i32;

    #[method(name = "set_VoiceCancelTimes", args = 1)]
    pub fn set_voice_cancel_times(self, value: i32) -> ();

    #[method(name = "get_WalkingSpeed", args = 0)]
    pub fn get_walking_speed(self) -> f32;

    #[method(name = "set_WalkingSpeed", args = 1)]
    pub fn set_walking_speed(self, value: f32) -> ();

    #[method(name = "get_IsMute", args = 0)]
    pub fn get_is_mute(self) -> bool;

    #[method(name = "set_IsMute", args = 1)]
    pub fn set_is_mute(self, value: bool) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "MyStart", args = 0)]
    pub fn my_start(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "get_IsVoiceReady", args = 0)]
    pub fn get_is_voice_ready(self) -> bool;

    #[method(name = "LoadVoice", args = 0)]
    pub fn load_voice(self) -> ();

    #[method(name = "UnloadVoice", args = 0)]
    pub fn unload_voice(self) -> ();

    #[method(name = "Play", args = 1)]
    pub fn play(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "PlayWithDetail", args = 1)]
    pub fn play_with_detail(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "PlayVoice", args = 1)]
    pub fn play_voice(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "PlayVoice_ArenaName", args = 1)]
    pub fn play_voice_arena_name(self, event_name: ::unity2::Il2CppString) -> ();

    #[method(name = "CalculateMobVoiceID", args = 0)]
    pub fn calculate_mob_voice_id(self) -> ::unity2::Il2CppString;

    #[method(name = "Footstep", args = 0)]
    pub fn footstep(self) -> ();

    #[method(name = "Swing", args = 0)]
    pub fn swing(self) -> ();

    #[method(name = "Flap", args = 0)]
    pub fn flap(self) -> ();

    #[method(name = "WeaponOpen", args = 0)]
    pub fn weapon_open(self) -> ();

    #[method(name = "WeaponClose", args = 0)]
    pub fn weapon_close(self) -> ();

    #[method(name = "Shoot", args = 0)]
    pub fn shoot(self) -> ();

    #[method(name = "PlayWinVoice", args = 0)]
    pub fn play_win_voice(self) -> ();

    #[method(name = "OnDie", args = 0)]
    pub fn on_die(self) -> ();

    #[method(name = "SelectWinVoice", args = 1)]
    pub fn select_win_voice(chr: crate::combat::character::Character) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-charactersound")]
impl CharacterSound {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CharacterSound),
                ::core::stringify!(new),
            )
        });
        <Self as ICharacterSoundMethods>::ctor(this);
        this
    }
}
