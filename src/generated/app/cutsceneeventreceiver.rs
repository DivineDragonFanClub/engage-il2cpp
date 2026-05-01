
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cutsceneeventreceiver/CutSceneEventReceiver.md")))]
#[::unity2::class(namespace = "App", name = "CutSceneEventReceiver")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CutSceneEventReceiver {}

#[cfg(feature = "app-cutsceneeventreceiver")]
#[::unity2::methods]
impl CutSceneEventReceiver {
    #[method(name = "OnNotify", args = 3)]
    pub fn on_notify(
        self,
        origin: crate::unity_engine::playables::playable::Playable,
        notification: crate::unity_engine::playables::inotification::INotification,
        context: crate::system::object::Object,
    ) -> ();

    #[method(name = "OnNotifySpringResestMarker", args = 1)]
    pub fn on_notify_spring_resest_marker(
        self,
        spring_marker: crate::app::cutscenespringresetmarker::CutSceneSpringResetMarker,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-cutsceneeventreceiver")]
impl CutSceneEventReceiver {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CutSceneEventReceiver),
                ::core::stringify!(new),
            )
        });
        <Self as ICutSceneEventReceiverMethods>::ctor(this);
        this
    }
}
