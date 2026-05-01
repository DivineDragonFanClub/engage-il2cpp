
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/ilayerable/ILayerable.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "ILayerable")]
pub struct ILayerable {}

#[cfg(feature = "unity_engine-timeline-ilayerable")]
#[::unity2::methods]
impl ILayerable {
    #[method(name = "CreateLayerMixer", args = 3)]
    pub fn create_layer_mixer(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        go: crate::unity_engine::gameobject::GameObject,
        input_count: i32,
    ) -> crate::unity_engine::playables::playable::Playable;
}
