
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/timelineundo/TimelineUndo.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "TimelineUndo")]
#[parent(crate::system::object::Object)]
pub struct TimelineUndo {}

#[cfg(feature = "unity_engine-timeline-timelineundo")]
#[::unity2::methods]
impl TimelineUndo {
    #[method(name = "PushDestroyUndo", args = 3)]
    pub fn push_destroy_undo(
        timeline: crate::unity_engine::timeline::timelineasset::TimelineAsset,
        thing_to_dirty: crate::unity_engine::object_2::Object_2,
        object_to_destroy: crate::unity_engine::object_2::Object_2,
    ) -> ();
}
