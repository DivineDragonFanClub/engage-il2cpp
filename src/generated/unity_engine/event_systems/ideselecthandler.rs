
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/ideselecthandler/IDeselectHandler.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "IDeselectHandler")]
pub struct IDeselectHandler {}

#[cfg(feature = "unity_engine-event_systems-ideselecthandler")]
#[::unity2::methods]
impl IDeselectHandler {
    #[method(name = "OnDeselect", args = 1)]
    pub fn on_deselect(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();
}
