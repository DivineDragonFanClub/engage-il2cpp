
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::playables::playableasset::IPlayableAsset;
use crate::unity_engine::playables::playableasset::PlayableAsset;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomcaptionclip/MyRoomCaptionClip.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomCaptionClip")]
#[parent(crate::unity_engine::playables::playableasset::PlayableAsset)]
pub struct MyRoomCaptionClip {
    #[rename(name = "Template")]
    pub template: crate::app::myroomcaptionmid::MyRoomCaptionMID,
}

#[cfg(feature = "app-myroomcaptionclip")]
#[::unity2::methods]
impl MyRoomCaptionClip {
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

#[cfg(feature = "app-myroomcaptionclip")]
impl MyRoomCaptionClip {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomCaptionClip),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomCaptionClipMethods>::ctor(this);
        this
    }
}
