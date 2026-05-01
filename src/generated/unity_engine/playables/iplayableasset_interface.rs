
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/iplayableasset_interface/IPlayableAsset_Interface.md")))]
#[::unity2::class(namespace = "UnityEngine.Playables", name = "IPlayableAsset")]
pub struct IPlayableAsset_Interface {}

#[cfg(feature = "unity_engine-playables-iplayableasset_interface")]
#[::unity2::methods]
impl IPlayableAsset_Interface {
    #[method(name = "CreatePlayable", args = 2)]
    pub fn create_playable(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        owner: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = "get_duration", args = 0)]
    pub fn get_duration(self) -> f64;
}
