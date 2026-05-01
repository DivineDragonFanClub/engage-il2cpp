
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/event_systems/icancelhandler/ICancelHandler.md")))]
#[::unity2::class(namespace = "UnityEngine.EventSystems", name = "ICancelHandler")]
pub struct ICancelHandler {}

#[cfg(feature = "unity_engine-event_systems-icancelhandler")]
#[::unity2::methods]
impl ICancelHandler {
    #[method(name = "OnCancel", args = 1)]
    pub fn on_cancel(
        self,
        event_data: crate::unity_engine::event_systems::baseeventdata::BaseEventData,
    ) -> ();
}
