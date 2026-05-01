
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::playables::playablebehaviour::IPlayableBehaviour;
use crate::unity_engine::playables::playablebehaviour::PlayableBehaviour;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomvoicebehaviour/MyRoomVoiceBehaviour.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomVoiceBehaviour")]
#[parent(crate::unity_engine::playables::playablebehaviour::PlayableBehaviour)]
pub struct MyRoomVoiceBehaviour {}

#[cfg(feature = "app-myroomvoicebehaviour")]
#[::unity2::methods]
impl MyRoomVoiceBehaviour {
    #[method(name = "get_VoiceSituation", args = 0)]
    pub fn get_voice_situation(self) -> crate::app::gamesound::GameSound_WakeupVoiceSituation;

    #[method(name = "set_VoiceSituation", args = 1)]
    pub fn set_voice_situation(
        self,
        value: crate::app::gamesound::GameSound_WakeupVoiceSituation,
    ) -> ();

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> crate::app::reliancedata::RelianceData_Level;

    #[method(name = "get_Pattern", args = 0)]
    pub fn get_pattern(self) -> crate::app::gamesound::GameSound_WakeupVoicePattern;

    #[method(name = "get_Character", args = 0)]
    pub fn get_character(self) -> crate::combat::character::Character;

    #[method(name = "get_VoiceID", args = 0)]
    pub fn get_voice_id(self) -> ::unity2::Il2CppString;

    #[method(name = "get_EventName", args = 0)]
    pub fn get_event_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_EventName", args = 1)]
    pub fn set_event_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "OnBehaviourPlay", args = 2)]
    pub fn on_behaviour_play(
        self,
        playable: crate::unity_engine::playables::playable::Playable,
        info: crate::unity_engine::playables::framedata::FrameData,
    ) -> ();

    #[method(name = "OnBehaviourPause", args = 2)]
    pub fn on_behaviour_pause(
        self,
        playable: crate::unity_engine::playables::playable::Playable,
        info: crate::unity_engine::playables::framedata::FrameData,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomvoicebehaviour")]
impl MyRoomVoiceBehaviour {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomVoiceBehaviour),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomVoiceBehaviourMethods>::ctor(this);
        this
    }
}
