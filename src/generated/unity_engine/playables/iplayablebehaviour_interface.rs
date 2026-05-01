
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/iplayablebehaviour_interface/IPlayableBehaviour_Interface.md")))]
#[::unity2::class(namespace = "UnityEngine.Playables", name = "IPlayableBehaviour")]
pub struct IPlayableBehaviour_Interface {}

#[cfg(feature = "unity_engine-playables-iplayablebehaviour_interface")]
#[::unity2::methods]
impl IPlayableBehaviour_Interface {
    #[method(name = "OnGraphStart", args = 1)]
    pub fn on_graph_start(self, playable: crate::unity_engine::playables::playable::Playable)
        -> ();

    #[method(name = "OnGraphStop", args = 1)]
    pub fn on_graph_stop(self, playable: crate::unity_engine::playables::playable::Playable) -> ();

    #[method(name = "OnPlayableCreate", args = 1)]
    pub fn on_playable_create(
        self,
        playable: crate::unity_engine::playables::playable::Playable,
    ) -> ();

    #[method(name = "OnPlayableDestroy", args = 1)]
    pub fn on_playable_destroy(
        self,
        playable: crate::unity_engine::playables::playable::Playable,
    ) -> ();

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

    #[method(name = "PrepareFrame", args = 2)]
    pub fn prepare_frame(
        self,
        playable: crate::unity_engine::playables::playable::Playable,
        info: crate::unity_engine::playables::framedata::FrameData,
    ) -> ();

    #[method(name = "ProcessFrame", args = 3)]
    pub fn process_frame(
        self,
        playable: crate::unity_engine::playables::playable::Playable,
        info: crate::unity_engine::playables::framedata::FrameData,
        player_data: crate::system::object::Object,
    ) -> ();
}
