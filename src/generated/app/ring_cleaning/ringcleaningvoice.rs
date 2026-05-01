
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ring_cleaning/ringcleaningvoice/RingCleaningVoice.md")))]
#[::unity2::class(namespace = "App.RingCleaning", name = "RingCleaningVoice")]
#[parent(crate::system::object::Object)]
pub struct RingCleaningVoice {
    #[static_field]
    #[rename(name = "SeEventNames")]
    pub se_event_names: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "SeIndex")]
    pub se_index: i32,
    #[static_field]
    #[rename(name = "SeStrongEventNames")]
    pub se_strong_event_names: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "SeStrongIndex")]
    pub se_strong_index: i32,
}

#[cfg(feature = "app-ring_cleaning-ringcleaningvoice")]
#[::unity2::methods]
impl RingCleaningVoice {
    #[method(name = "get_IsPlayableFinishVoice", args = 0)]
    pub fn get_is_playable_finish_voice() -> bool;

    #[method(name = "set_IsPlayableFinishVoice", args = 1)]
    pub fn set_is_playable_finish_voice(value: bool) -> ();

    #[method(name = "get_GodUnit", args = 0)]
    pub fn get_god_unit() -> crate::app::godunit::GodUnit;

    #[method(name = "set_GodUnit", args = 1)]
    pub fn set_god_unit(value: crate::app::godunit::GodUnit) -> ();

    #[method(name = "get_GodSoundID", args = 0)]
    pub fn get_god_sound_id() -> ::unity2::Il2CppString;

    #[method(name = "set_GodSoundID", args = 1)]
    pub fn set_god_sound_id(value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_GodChara", args = 0)]
    pub fn get_god_chara() -> crate::combat::character::Character;

    #[method(name = "set_GodChara", args = 1)]
    pub fn set_god_chara(value: crate::combat::character::Character) -> ();

    #[method(name = "Setup", args = 1)]
    pub fn setup(god: crate::app::godunit::GodUnit) -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = "IsFinish", args = 0)]
    pub fn is_finish() -> bool;

    #[method(name = "PlayVoice", args = 1)]
    pub fn play_voice(
        situation: crate::app::ringcleaningvoicedata::RingCleaningVoiceData_Situation,
    ) -> crate::app::ringcleaningvoicedata::RingCleaningVoiceData;

    #[method(name = "PlayStartVoice", args = 0)]
    pub fn play_start_voice() -> crate::app::ringcleaningvoicedata::RingCleaningVoiceData;

    #[method(name = "PlayCleaningCriticalPointVoice", args = 0)]
    pub fn play_cleaning_critical_point_voice(
    ) -> crate::app::ringcleaningvoicedata::RingCleaningVoiceData;

    #[method(name = "PlayStrongCleaningNotCriticalPointVoice", args = 0)]
    pub fn play_strong_cleaning_not_critical_point_voice(
    ) -> crate::app::ringcleaningvoicedata::RingCleaningVoiceData;

    #[method(name = "PlayStrongCleaningCriticalPointVoice", args = 0)]
    pub fn play_strong_cleaning_critical_point_voice(
    ) -> crate::app::ringcleaningvoicedata::RingCleaningVoiceData;

    #[method(name = "PlayFinishCleaningVoice", args = 0)]
    pub fn play_finish_cleaning_voice() -> ();

    #[method(name = "get_NowSeEvent", args = 0)]
    pub fn get_now_se_event() -> ::unity2::Il2CppString;

    #[method(name = "PlayCleaningSE", args = 0)]
    pub fn play_cleaning_se() -> ();

    #[method(name = "PlayStartStrongSE", args = 0)]
    pub fn play_start_strong_se() -> ();

    #[method(name = "PlayStrongCleaningSE", args = 0)]
    pub fn play_strong_cleaning_se() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-ring_cleaning-ringcleaningvoice")]
impl RingCleaningVoice {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingCleaningVoice),
                ::core::stringify!(new),
            )
        });
        <Self as IRingCleaningVoiceMethods>::ctor(this);
        this
    }
}
