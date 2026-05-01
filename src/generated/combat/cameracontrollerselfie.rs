
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameracontrollerselfie/CameraControllerSelfie.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraControllerSelfie")]
#[parent(crate::combat::basecameracontroller::BaseCameraController)]
pub struct CameraControllerSelfie {}

#[cfg(feature = "combat-cameracontrollerselfie")]
#[::unity2::methods]
impl CameraControllerSelfie {
    #[method(name = "get_IsSideInverse", args = 0)]
    pub fn get_is_side_inverse(self) -> bool;

    #[method(name = "set_IsSideInverse", args = 1)]
    pub fn set_is_side_inverse(self, value: bool) -> ();

    #[method(name = "get_IsCameraInverse", args = 0)]
    pub fn get_is_camera_inverse(self) -> bool;

    #[method(name = "set_IsCameraInverse", args = 1)]
    pub fn set_is_camera_inverse(self, value: bool) -> ();

    #[method(name = "get_TargetSide", args = 0)]
    pub fn get_target_side(self) -> i32;

    #[method(name = "Activate", args = 0)]
    pub fn activate(self) -> ();

    #[method(name = "Deactivate", args = 0)]
    pub fn deactivate(self) -> ();

    #[method(name = "SetInverse", args = 2)]
    pub fn set_inverse(self, inv_side: bool, inv_camera: bool) -> ();

    #[method(name = "GetCameraTargets", args = 0)]
    pub fn get_camera_targets(self) -> ::unity2::Array<i32>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-cameracontrollerselfie")]
impl CameraControllerSelfie {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraControllerSelfie),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraControllerSelfieMethods>::ctor(this);
        this
    }
}
