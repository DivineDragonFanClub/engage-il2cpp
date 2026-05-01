
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/ipropertypreview/IPropertyPreview.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "IPropertyPreview")]
pub struct IPropertyPreview {}

#[cfg(feature = "unity_engine-timeline-ipropertypreview")]
#[::unity2::methods]
impl IPropertyPreview {
    #[method(name = "GatherProperties", args = 2)]
    pub fn gather_properties(
        self,
        director: crate::unity_engine::playables::playabledirector::PlayableDirector,
        driver: crate::unity_engine::timeline::ipropertycollector::IPropertyCollector,
    ) -> ();
}
