
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomeventreceiver/MyRoomEventReceiver.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomEventReceiver")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MyRoomEventReceiver {}

#[cfg(feature = "app-myroomeventreceiver")]
#[::unity2::methods]
impl MyRoomEventReceiver {
    #[method(name = "get_AdjustedTime", args = 0)]
    pub fn get_adjusted_time(self) -> f64;

    #[method(name = "OnNotify", args = 3)]
    pub fn on_notify(
        self,
        origin: crate::unity_engine::playables::playable::Playable,
        notification: crate::unity_engine::playables::inotification::INotification,
        context: crate::system::object::Object,
    ) -> ();

    #[method(name = "OnNotifyEffectMarker", args = 1)]
    pub fn on_notify_effect_marker(
        self,
        effect_marker: crate::app::myroomeffectmarker::MyRoomEffectMarker,
    ) -> ();

    #[method(name = "OnNotifySEMarker", args = 1)]
    pub fn on_notify_se_marker(self, se_marker: crate::app::myroomsemarker::MyRoomSEMarker) -> ();

    #[method(name = "OnNotifyCameraMarker", args = 1)]
    pub fn on_notify_camera_marker(
        self,
        camera_marker: crate::app::myroomcameramarker::MyRoomCameraMarker,
    ) -> ();

    #[method(name = "OnNotifyLookingCameraMarker", args = 1)]
    pub fn on_notify_looking_camera_marker(
        self,
        looking_camera_marker: crate::app::myroomlookingcameramarker::MyRoomLookingCameraMarker,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomeventreceiver")]
impl MyRoomEventReceiver {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomEventReceiver),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomEventReceiverMethods>::ctor(this);
        this
    }
}
