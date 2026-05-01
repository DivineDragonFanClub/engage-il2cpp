
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameracontrollersolo/CameraControllerSolo.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraControllerSolo")]
#[parent(crate::combat::basecameracontroller::BaseCameraController)]
pub struct CameraControllerSolo {
    #[rename(name = "Distance")]
    pub distance: f32,
    #[rename(name = "TargetSide")]
    pub target_side: i32,
    #[rename(name = "FollowJoint")]
    pub follow_joint: crate::combat::camerapositiondata::CameraPositionData_TargetJoint,
    #[rename(name = "FollowVector")]
    pub follow_vector: crate::unity_engine::vector3::Vector3,
    #[rename(name = "FollowVectorBigDragonLeg")]
    pub follow_vector_big_dragon_leg: crate::unity_engine::vector3::Vector3,
    #[rename(name = "FollowVectorBigDragonFly")]
    pub follow_vector_big_dragon_fly: crate::unity_engine::vector3::Vector3,
    #[rename(name = "LookAtJoint")]
    pub look_at_joint: crate::combat::camerapositiondata::CameraPositionData_TargetJoint,
    #[rename(name = "LookAtVector")]
    pub look_at_vector: crate::unity_engine::vector3::Vector3,
    #[rename(name = "LookAtVectorBigDragonLeg")]
    pub look_at_vector_big_dragon_leg: crate::unity_engine::vector3::Vector3,
    #[rename(name = "LookAtVectorBigDragonFly")]
    pub look_at_vector_big_dragon_fly: crate::unity_engine::vector3::Vector3,
    #[rename(name = "LookCenterAndLookAt")]
    pub look_center_and_look_at: bool,
    #[rename(name = "UnusableCheck")]
    pub unusable_check: bool,
    #[rename(name = "m_IsCameraInverse")]
    pub m_is_camera_inverse: bool,
    #[rename(name = "m_IsSideInverse")]
    pub m_is_side_inverse: bool,
}

#[cfg(feature = "combat-cameracontrollersolo")]
#[::unity2::methods]
impl CameraControllerSolo {
    #[method(name = "get_CameraTarget", args = 0)]
    pub fn get_camera_target(self) -> i32;

    #[method(name = "GetCombatVector", args = 1)]
    pub fn get_combat_vector(
        self,
        vec: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "CheckUsable", args = 1)]
    pub fn check_usable(self, is_routine: bool) -> ();

    #[method(name = "SetInverse", args = 2)]
    pub fn set_inverse(self, inv_side: bool, inv_camera: bool) -> ();

    #[method(name = "Activate", args = 0)]
    pub fn activate(self) -> ();

    #[method(name = "Deactivate", args = 0)]
    pub fn deactivate(self) -> ();

    #[method(name = "GetCameraTargets", args = 0)]
    pub fn get_camera_targets(self) -> ::unity2::Array<i32>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-cameracontrollersolo")]
impl CameraControllerSolo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraControllerSolo),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraControllerSoloMethods>::ctor(this);
        this
    }
}
