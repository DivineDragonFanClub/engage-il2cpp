
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::playables::playablebehaviour::IPlayableBehaviour;
use crate::unity_engine::playables::playablebehaviour::PlayableBehaviour;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomphasebehaviour/MyRoomPhaseBehaviour.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomPhaseBehaviour")]
#[parent(crate::unity_engine::playables::playablebehaviour::PlayableBehaviour)]
pub struct MyRoomPhaseBehaviour {}

#[cfg(feature = "app-myroomphasebehaviour")]
#[::unity2::methods]
impl MyRoomPhaseBehaviour {
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

#[cfg(feature = "app-myroomphasebehaviour")]
impl MyRoomPhaseBehaviour {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomPhaseBehaviour),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomPhaseBehaviourMethods>::ctor(this);
        this
    }
}
