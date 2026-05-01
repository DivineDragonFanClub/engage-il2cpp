
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/ienddraghandler/IEndDragHandler.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "IEndDragHandler")]
pub struct IEndDragHandler {}

#[cfg(feature = "unity_engine-event_systems-ienddraghandler")]
#[::unity2::methods]
impl IEndDragHandler {
    #[method(name = "OnEndDrag", args = 1)]
    pub fn on_end_drag(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();
}
