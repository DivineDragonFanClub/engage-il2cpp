
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/idrophandler/IDropHandler.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "IDropHandler")]
pub struct IDropHandler {}

#[cfg(feature = "unity_engine-event_systems-idrophandler")]
#[::unity2::methods]
impl IDropHandler {
    #[method(name = "OnDrop", args = 1)]
    pub fn on_drop(
        self,
        event_data: crate::unity_engine::event_systems::pointereventdata::PointerEventData,
    ) -> ();
}
