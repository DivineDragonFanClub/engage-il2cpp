
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/railendmarkerreciever/RailEndMarkerReciever.md")))]
#[::unity2::class(namespace = "App", name = "RailEndMarkerReciever")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RailEndMarkerReciever {
    #[rename(name = "m_CameraComponent")]
    pub m_camera_component: crate::app::dragonridecamera::DragonRideCamera,
}

#[cfg(feature = "app-railendmarkerreciever")]
#[::unity2::methods]
impl RailEndMarkerReciever {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnNotify", args = 3)]
    pub fn on_notify(
        self,
        origin: crate::unity_engine::playables::playable::Playable,
        notification: crate::unity_engine::playables::inotification::INotification,
        context: crate::system::object::Object,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-railendmarkerreciever")]
impl RailEndMarkerReciever {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RailEndMarkerReciever),
                ::core::stringify!(new),
            )
        });
        <Self as IRailEndMarkerRecieverMethods>::ctor(this);
        this
    }
}
