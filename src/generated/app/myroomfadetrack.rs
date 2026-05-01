
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::playables::playableasset::IPlayableAsset;
use crate::unity_engine::playables::playableasset::PlayableAsset;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use crate::unity_engine::timeline::trackasset::ITrackAsset;
use crate::unity_engine::timeline::trackasset::TrackAsset;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomfadetrack/MyRoomFadeTrack.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomFadeTrack")]
#[parent(crate::unity_engine::timeline::trackasset::TrackAsset)]
pub struct MyRoomFadeTrack {}

#[cfg(feature = "app-myroomfadetrack")]
#[::unity2::methods]
impl MyRoomFadeTrack {
    #[method(name = "CreateTrackMixer", args = 3)]
    pub fn create_track_mixer(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        go: crate::unity_engine::gameobject::GameObject,
        input_count: i32,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomfadetrack")]
impl MyRoomFadeTrack {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomFadeTrack),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomFadeTrackMethods>::ctor(this);
        this
    }
}
