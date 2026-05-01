
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/fontupdatetracker/FontUpdateTracker.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "FontUpdateTracker")]
#[parent(crate::system::object::Object)]
pub struct FontUpdateTracker {
    #[static_field]
    #[rename(name = "m_Tracked")]
    pub m_tracked: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::unity_engine::font::Font,
        crate::system::collections::generic::hashset_1::HashSet_1<
            crate::unity_engine::ui::text::Text,
        >,
    >,
}

#[cfg(feature = "unity_engine-ui-fontupdatetracker")]
#[::unity2::methods]
impl FontUpdateTracker {
    #[method(name = "TrackText", args = 1)]
    pub fn track_text(t: crate::unity_engine::ui::text::Text) -> ();

    #[method(name = "RebuildForFont", args = 1)]
    pub fn rebuild_for_font(f: crate::unity_engine::font::Font) -> ();

    #[method(name = "UntrackText", args = 1)]
    pub fn untrack_text(t: crate::unity_engine::ui::text::Text) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
