
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/notificationutilities/NotificationUtilities.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "NotificationUtilities")]
#[parent(crate::system::object::Object)]
pub struct NotificationUtilities {}

#[cfg(feature = "unity_engine-timeline-notificationutilities")]
#[::unity2::methods]
impl NotificationUtilities {
    #[method(name = "CreateNotificationsPlayable", args = 3)]
    pub fn create_notifications_playable(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        markers: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
            crate::unity_engine::timeline::imarker_interface::IMarker_Interface,
        >,
        go: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::playables::scriptplayable_1::ScriptPlayable_1<
        crate::unity_engine::timeline::timenotificationbehaviour::TimeNotificationBehaviour,
    >;

    #[method(name = "TrackTypeSupportsNotifications", args = 1)]
    pub fn track_type_supports_notifications(r#type: ::unity2::SystemType) -> bool;
}
