
use crate::combat::basecameracontroller::BaseCameraController;
use crate::combat::basecameracontroller::IBaseCameraController;
use crate::combat::basetransitioncameracontroller::BaseTransitionCameraController;
use crate::combat::basetransitioncameracontroller::IBaseTransitionCameraController;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameracontrollertransition/CameraControllerTransition.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraControllerTransition")]
#[parent(crate::combat::basetransitioncameracontroller::BaseTransitionCameraController)]
pub struct CameraControllerTransition {
    #[rename(name = "StartCurve")]
    pub start_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "ReturnCurve")]
    pub return_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "RotateStartHeightStart")]
    pub rotate_start_height_start: f32,
    #[rename(name = "RotateStartHeightReturn")]
    pub rotate_start_height_return: f32,
    #[rename(name = "m_DoRotateTarget")]
    pub m_do_rotate_target: bool,
    #[rename(name = "m_RdialBlur")]
    pub m_rdial_blur:
        crate::unity_engine::rendering::universal::custom::customradialblur::CustomRadialBlur,
    #[rename(name = "WaitProgress")]
    pub wait_progress: f32,
}

#[cfg(feature = "combat-cameracontrollertransition")]
#[::unity2::methods]
impl CameraControllerTransition {
    #[method(name = "get_TransitionTimeStart", args = 0)]
    pub fn get_transition_time_start(self) -> f32;

    #[method(name = "get_TransitionTimeReturn", args = 0)]
    pub fn get_transition_time_return(self) -> f32;

    #[method(name = "get_SourceFollow", args = 0)]
    pub fn get_source_follow(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_SourceFollow", args = 1)]
    pub fn set_source_follow(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_SourceLookAt", args = 0)]
    pub fn get_source_look_at(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_SourceLookAt", args = 1)]
    pub fn set_source_look_at(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_SourceFov", args = 0)]
    pub fn get_source_fov(self) -> f32;

    #[method(name = "set_SourceFov", args = 1)]
    pub fn set_source_fov(self, value: f32) -> ();

    #[method(name = "get_TargetFov", args = 0)]
    pub fn get_target_fov(self) -> f32;

    #[method(name = "get_TargetSSS", args = 0)]
    pub fn get_target_sss(self) -> f32;

    #[method(name = "set_TargetSSS", args = 1)]
    pub fn set_target_sss(self, value: f32) -> ();

    #[method(name = "get_DetourDegree", args = 0)]
    pub fn get_detour_degree(self) -> f32;

    #[method(name = "set_DetourDegree", args = 1)]
    pub fn set_detour_degree(self, value: f32) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "StartTransition", args = 1)]
    pub fn start_transition(
        self,
        next_cam: crate::combat::basecameracontroller::BaseCameraController,
    ) -> ();

    #[method(name = "ChangeTaget", args = 1)]
    pub fn change_taget(
        self,
        next_cam: crate::combat::basecameracontroller::BaseCameraController,
    ) -> ();

    #[method(name = "ReturnTransition", args = 1)]
    pub fn return_transition(
        self,
        current_cam: crate::combat::basecameracontroller::BaseCameraController,
    ) -> ();

    #[method(name = "CheckUsable", args = 1)]
    pub fn check_usable(self, is_routine: bool) -> ();

    #[method(name = "Stabilize", args = 0)]
    pub fn stabilize(self) -> ();

    #[method(name = "Deactivate", args = 0)]
    pub fn deactivate(self) -> ();

    #[method(name = "GetSourceInfo", args = 0)]
    pub fn get_source_info(self) -> ();

    #[method(name = "SetCameraSetting", args = 1)]
    pub fn set_camera_setting(self, t: f32) -> ();

    #[method(name = "CheckDetour", args = 0)]
    pub fn check_detour(self) -> ();

    #[method(name = "GetFixedSourceFollow", args = 2)]
    pub fn get_fixed_source_follow(
        self,
        trans: f32,
        rot_height: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "RotateTargetFollow", args = 0)]
    pub fn rotate_target_follow(self) -> ();

    #[method(name = "GetCameraTargets", args = 0)]
    pub fn get_camera_targets(self) -> ::unity2::Array<i32>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-cameracontrollertransition")]
impl CameraControllerTransition {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraControllerTransition),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraControllerTransitionMethods>::ctor(this);
        this
    }
}
