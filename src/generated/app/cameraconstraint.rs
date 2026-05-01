
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cameraconstraint/CameraConstraint.md")))]
#[::unity2::class(namespace = "App", name = "CameraConstraint")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CameraConstraint {
    #[rename(name = "m_Target")]
    pub m_target: crate::unity_engine::camera::Camera,
    #[rename(name = "m_Camera")]
    pub m_camera: crate::unity_engine::camera::Camera,
    #[rename(name = "m_Position")]
    pub m_position: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Rotation")]
    pub m_rotation: crate::unity_engine::quaternion::Quaternion,
}

#[cfg(feature = "app-cameraconstraint")]
#[::unity2::methods]
impl CameraConstraint {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-cameraconstraint")]
impl CameraConstraint {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraConstraint),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraConstraintMethods>::ctor(this);
        this
    }
}
