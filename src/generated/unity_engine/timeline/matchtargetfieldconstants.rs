
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/matchtargetfieldconstants/MatchTargetFieldConstants.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "MatchTargetFieldConstants")]
#[parent(crate::system::object::Object)]
pub struct MatchTargetFieldConstants {
    #[static_field]
    #[rename(name = "All")]
    pub all: crate::unity_engine::timeline::matchtargetfields::MatchTargetFields,
    #[static_field]
    #[rename(name = "None")]
    pub none: crate::unity_engine::timeline::matchtargetfields::MatchTargetFields,
    #[static_field]
    #[rename(name = "Position")]
    pub position: crate::unity_engine::timeline::matchtargetfields::MatchTargetFields,
    #[static_field]
    #[rename(name = "Rotation")]
    pub rotation: crate::unity_engine::timeline::matchtargetfields::MatchTargetFields,
}

#[cfg(feature = "unity_engine-timeline-matchtargetfieldconstants")]
#[::unity2::methods]
impl MatchTargetFieldConstants {
    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
