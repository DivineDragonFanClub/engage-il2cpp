
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::playables::playableasset::IPlayableAsset;
use crate::unity_engine::playables::playableasset::PlayableAsset;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomphaseclip/MyRoomPhaseClip.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomPhaseClip")]
#[parent(crate::unity_engine::playables::playableasset::PlayableAsset)]
pub struct MyRoomPhaseClip {
    #[rename(name = "Situation")]
    pub situation: crate::app::gamesound::GameSound_WakeupVoiceSituation,
}

#[cfg(feature = "app-myroomphaseclip")]
#[::unity2::methods]
impl MyRoomPhaseClip {
    #[method(name = "get_clipCaps", args = 0)]
    pub fn get_clip_caps(self) -> crate::unity_engine::timeline::clipcaps::ClipCaps;

    #[method(name = "CreatePlayable", args = 2)]
    pub fn create_playable(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        owner: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomphaseclip")]
impl MyRoomPhaseClip {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomPhaseClip),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomPhaseClipMethods>::ctor(this);
        this
    }
}
