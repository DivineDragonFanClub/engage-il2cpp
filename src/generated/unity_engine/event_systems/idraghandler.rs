
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/idraghandler/IDragHandler.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "IDragHandler")]
pub struct IDragHandler {}

#[cfg(feature = "unity_engine-event_systems-idraghandler")]
#[::unity2::methods]
impl IDragHandler {
    #[method(name = "OnDrag", args = 1)]
    pub fn on_drag(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();
}
