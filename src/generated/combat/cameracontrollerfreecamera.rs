
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameracontrollerfreecamera/CameraControllerFreeCamera.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraControllerFreeCamera")]
#[parent(crate::combat::basecameracontroller::BaseCameraController)]
pub struct CameraControllerFreeCamera {
    #[rename(name = "CameraPosFromChara")]
    pub camera_pos_from_chara: crate::unity_engine::vector3::Vector3,
    #[rename(name = "LookDirection")]
    pub look_direction: crate::unity_engine::vector3::Vector3,
}

#[cfg(feature = "combat-cameracontrollerfreecamera")]
#[::unity2::methods]
impl CameraControllerFreeCamera {
    #[method(name = "Activate", args = 0)]
    pub fn activate(self) -> ();

    #[method(name = "GetCameraTargets", args = 0)]
    pub fn get_camera_targets(self) -> ::unity2::Array<i32>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-cameracontrollerfreecamera")]
impl CameraControllerFreeCamera {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraControllerFreeCamera),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraControllerFreeCameraMethods>::ctor(this);
        this
    }
}
