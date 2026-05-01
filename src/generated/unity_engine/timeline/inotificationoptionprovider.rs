
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/inotificationoptionprovider/INotificationOptionProvider.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Timeline",
    name = "INotificationOptionProvider"
)]
pub struct INotificationOptionProvider {}

#[cfg(feature = "unity_engine-timeline-inotificationoptionprovider")]
#[::unity2::methods]
impl INotificationOptionProvider {
    #[method(name = "get_flags", args = 0)]
    pub fn get_flags(self) -> crate::unity_engine::timeline::notificationflags::NotificationFlags;
}
