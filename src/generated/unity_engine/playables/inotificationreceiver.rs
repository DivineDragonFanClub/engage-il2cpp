
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/playables/inotificationreceiver/INotificationReceiver.md")))]
#[::unity2::class(namespace = "UnityEngine.Playables", name = "INotificationReceiver")]
pub struct INotificationReceiver {}

#[cfg(feature = "unity_engine-playables-inotificationreceiver")]
#[::unity2::methods]
impl INotificationReceiver {
    #[method(name = "OnNotify", args = 3)]
    pub fn on_notify(
        self,
        origin: crate::unity_engine::playables::playable::Playable,
        notification: crate::unity_engine::playables::inotification::INotification,
        context: crate::system::object::Object,
    ) -> ();
}
