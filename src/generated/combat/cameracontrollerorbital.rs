
use crate::combat::basecameracontroller::BaseCameraController;
use crate::combat::basecameracontroller::IBaseCameraController;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameracontrollerorbital/CameraControllerOrbital.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraControllerOrbital")]
#[parent(crate::combat::basecameracontroller::BaseCameraController)]
pub struct CameraControllerOrbital {
    #[rename(name = "Target")]
    pub target: crate::combat::camerapositiondata::CameraPositionData_TargetJoint,
    #[rename(name = "Distance")]
    pub distance: f32,
    #[rename(name = "StartDegree")]
    pub start_degree: f32,
    #[rename(name = "RotateSpeed")]
    pub rotate_speed: f32,
    #[rename(name = "CameraHeight")]
    pub camera_height: f32,
    #[rename(name = "FOVWalk")]
    pub fov_walk: f32,
    #[rename(name = "FOVRide")]
    pub fov_ride: f32,
    #[rename(name = "m_Rotate")]
    pub m_rotate: f32,
    #[rename(name = "m_LookAt")]
    pub m_look_at: crate::combat::fxz::FXZ,
}

#[cfg(feature = "combat-cameracontrollerorbital")]
#[::unity2::methods]
impl CameraControllerOrbital {
    #[method(name = "Activate", args = 0)]
    pub fn activate(self) -> ();

    #[method(name = "GetCameraTargets", args = 0)]
    pub fn get_camera_targets(self) -> ::unity2::Array<i32>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-cameracontrollerorbital")]
impl CameraControllerOrbital {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraControllerOrbital),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraControllerOrbitalMethods>::ctor(this);
        this
    }
}
