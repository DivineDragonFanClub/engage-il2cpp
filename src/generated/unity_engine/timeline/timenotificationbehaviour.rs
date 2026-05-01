
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::playables::playablebehaviour::IPlayableBehaviour;
use crate::unity_engine::playables::playablebehaviour::PlayableBehaviour;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/timenotificationbehaviour/TimeNotificationBehaviour.md")))]
#[::unity2::class(namespace = "UnityEngine.Timeline", name = "TimeNotificationBehaviour")]
#[parent(crate::unity_engine::playables::playablebehaviour::PlayableBehaviour)]
pub struct TimeNotificationBehaviour {
# [rename (name = "m_Notifications")] pub m_notifications : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: timeline :: timenotificationbehaviour :: TimeNotificationBehaviour_NotificationEntry > ,
# [rename (name = "m_PreviousTime")] pub m_previous_time : f64 ,
# [rename (name = "m_NeedSortNotifications")] pub m_need_sort_notifications : bool ,
# [rename (name = "m_TimeSource")] pub m_time_source : crate :: unity_engine :: playables :: playable :: Playable ,
}

#[cfg(feature = "unity_engine-timeline-timenotificationbehaviour")]
#[::unity2::methods]
impl TimeNotificationBehaviour {
    #[method(name = "set_timeSource", args = 1)]
    pub fn set_time_source(self, value: crate::unity_engine::playables::playable::Playable) -> ();

    #[method(name = "Create", args = 3)]
    pub fn create(
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        duration: f64,
        loop_mode: crate::unity_engine::playables::directorwrapmode::DirectorWrapMode,
    ) -> crate::unity_engine::playables::scriptplayable_1::ScriptPlayable_1<
        crate::unity_engine::timeline::timenotificationbehaviour::TimeNotificationBehaviour,
    >;

    #[method(name = "AddNotification", args = 3)]
    pub fn add_notification(
        self,
        time: f64,
        payload: crate::unity_engine::playables::inotification::INotification,
        flags: crate::unity_engine::timeline::notificationflags::NotificationFlags,
    ) -> ();

    #[method(name = "OnGraphStart", args = 1)]
    pub fn on_graph_start(self, playable: crate::unity_engine::playables::playable::Playable)
        -> ();

    #[method(name = "OnBehaviourPause", args = 2)]
    pub fn on_behaviour_pause(
        self,
        playable: crate::unity_engine::playables::playable::Playable,
        info: crate::unity_engine::playables::framedata::FrameData,
    ) -> ();

    #[method(name = "PrepareFrame", args = 2)]
    pub fn prepare_frame(
        self,
        playable: crate::unity_engine::playables::playable::Playable,
        info: crate::unity_engine::playables::framedata::FrameData,
    ) -> ();

    #[method(name = "SortNotifications", args = 0)]
    pub fn sort_notifications(self) -> ();

    #[method(name = "CanRestoreNotification", args = 4)]
    pub fn can_restore_notification(
        e : crate :: unity_engine :: timeline :: timenotificationbehaviour :: TimeNotificationBehaviour_NotificationEntry,
        info: crate::unity_engine::playables::framedata::FrameData,
        current_time: f64,
        previous_time: f64,
    ) -> bool;

    #[method(name = "TriggerNotificationsInRange", args = 5)]
    pub fn trigger_notifications_in_range(
        self,
        start: f64,
        end: f64,
        info: crate::unity_engine::playables::framedata::FrameData,
        playable: crate::unity_engine::playables::playable::Playable,
        check_state: bool,
    ) -> ();

    #[method(name = "SyncDurationWithExternalSource", args = 1)]
    pub fn sync_duration_with_external_source(
        self,
        playable: crate::unity_engine::playables::playable::Playable,
    ) -> ();

    #[method(name = "Trigger_internal", args = 3)]
    pub fn trigger_internal(
        playable: crate::unity_engine::playables::playable::Playable,
        output: crate::unity_engine::playables::playableoutput::PlayableOutput,
        e : crate :: unity_engine :: timeline :: timenotificationbehaviour :: TimeNotificationBehaviour_NotificationEntry,
    ) -> ();

    #[method(name = "Restore_internal", args = 1)]
    pub fn restore_internal(
        e : crate :: unity_engine :: timeline :: timenotificationbehaviour :: TimeNotificationBehaviour_NotificationEntry,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-timeline-timenotificationbehaviour")]
impl TimeNotificationBehaviour {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TimeNotificationBehaviour),
                ::core::stringify!(new),
            )
        });
        <Self as ITimeNotificationBehaviourMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/timeline/timenotificationbehaviour/TimeNotificationBehaviour_NotificationEntry.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct TimeNotificationBehaviour_NotificationEntry {
    pub time: f64,
    pub payload: crate::unity_engine::playables::inotification::INotification,
    pub notification_fired: bool,
    pub flags: crate::unity_engine::timeline::notificationflags::NotificationFlags,
}

impl ::unity2::ClassIdentity for TimeNotificationBehaviour_NotificationEntry {
    const NAMESPACE: &'static str = "UnityEngine.Timeline";

    const NAME: &'static str = "TimeNotificationBehaviour.NotificationEntry";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TimeNotificationBehaviour_NotificationEntry {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "unity_engine-timeline-timenotificationbehaviour")]
#[::unity2::methods(value)]
impl TimeNotificationBehaviour_NotificationEntry {
    #[method(name = "get_triggerInEditor", args = 0)]
    pub fn get_trigger_in_editor(self) -> bool;

    #[method(name = "get_prewarm", args = 0)]
    pub fn get_prewarm(self) -> bool;

    #[method(name = "get_triggerOnce", args = 0)]
    pub fn get_trigger_once(self) -> bool;
}
