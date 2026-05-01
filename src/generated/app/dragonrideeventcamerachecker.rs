
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonrideeventcamerachecker/DragonRideEventCameraChecker.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideEventCameraChecker")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DragonRideEventCameraChecker {
    #[static_field]
    #[rename(name = "waitTime")]
    pub wait_time: f64,
    #[rename(name = "m_TargetObject")]
    pub m_target_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-dragonrideeventcamerachecker")]
#[::unity2::methods]
impl DragonRideEventCameraChecker {
    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-dragonrideeventcamerachecker")]
impl DragonRideEventCameraChecker {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideEventCameraChecker),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideEventCameraCheckerMethods>::ctor(this);
        this
    }
}
