
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameracontrollerside/CameraControllerSide.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraControllerSide")]
#[parent(crate::combat::basecameracontroller::BaseCameraController)]
pub struct CameraControllerSide {
    #[rename(name = "DistanceCurve")]
    pub distance_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "CameraHeight")]
    pub camera_height: f32,
    #[rename(name = "CameraHeighWithTarget")]
    pub camera_heigh_with_target: bool,
}

#[cfg(feature = "combat-cameracontrollerside")]
#[::unity2::methods]
impl CameraControllerSide {
    #[method(name = "get_MaxDistance", args = 0)]
    pub fn get_max_distance(self) -> f32;

    #[method(name = "CheckUsable", args = 1)]
    pub fn check_usable(self, is_routine: bool) -> ();

    #[method(name = "GetCameraTargets", args = 0)]
    pub fn get_camera_targets(self) -> ::unity2::Array<i32>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-cameracontrollerside")]
impl CameraControllerSide {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraControllerSide),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraControllerSideMethods>::ctor(this);
        this
    }
}
