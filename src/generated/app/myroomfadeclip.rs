
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::playables::playableasset::IPlayableAsset;
use crate::unity_engine::playables::playableasset::PlayableAsset;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomfadeclip/MyRoomFadeClip.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomFadeClip")]
#[parent(crate::unity_engine::playables::playableasset::PlayableAsset)]
pub struct MyRoomFadeClip {
    #[rename(name = "Template")]
    pub template: crate::app::myroomfadecolor::MyRoomFadeColor,
}

#[cfg(feature = "app-myroomfadeclip")]
#[::unity2::methods]
impl MyRoomFadeClip {
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

#[cfg(feature = "app-myroomfadeclip")]
impl MyRoomFadeClip {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomFadeClip),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomFadeClipMethods>::ctor(this);
        this
    }
}
