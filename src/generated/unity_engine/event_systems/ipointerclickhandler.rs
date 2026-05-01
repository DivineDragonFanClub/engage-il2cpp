
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/ipointerclickhandler/IPointerClickHandler.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "IPointerClickHandler")]
pub struct IPointerClickHandler {}

#[cfg(feature = "unity_engine-event_systems-ipointerclickhandler")]
#[::unity2::methods]
impl IPointerClickHandler {
    #[method(name = "OnPointerClick", args = 1)]
    pub fn on_pointer_click(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();
}
