
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/iscrollhandler/IScrollHandler.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "IScrollHandler")]
pub struct IScrollHandler {}

#[cfg(feature = "unity_engine-event_systems-iscrollhandler")]
#[::unity2::methods]
impl IScrollHandler {
    #[method(name = "OnScroll", args = 1)]
    pub fn on_scroll(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();
}
