
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/ipointeruphandler/IPointerUpHandler.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "IPointerUpHandler")]
pub struct IPointerUpHandler {}

#[cfg(feature = "unity_engine-event_systems-ipointeruphandler")]
#[::unity2::methods]
impl IPointerUpHandler {
    #[method(name = "OnPointerUp", args = 1)]
    pub fn on_pointer_up(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();
}
