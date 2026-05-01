
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameracontrollerunitdetail/CameraControllerUnitDetail.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraControllerUnitDetail")]
#[parent(crate::combat::basecameracontroller::BaseCameraController)]
pub struct CameraControllerUnitDetail {
    #[rename(name = "SpeedDistance")]
    pub speed_distance: f32,
    #[rename(name = "SpeedRotate")]
    pub speed_rotate: f32,
    #[rename(name = "AccelRotate")]
    pub accel_rotate: f32,
    #[rename(name = "BrakeRotate")]
    pub brake_rotate: f32,
    #[rename(name = "TargetEmblemSpeed")]
    pub target_emblem_speed: f32,
    #[rename(name = "DistanceEmblemHideNormal")]
    pub distance_emblem_hide_normal: f32,
    #[rename(name = "DistanceEmblemHideSigurd")]
    pub distance_emblem_hide_sigurd: f32,
    #[rename(name = "FlyingEmblemUpDown")]
    pub flying_emblem_up_down: bool,
    #[rename(name = "AutoSpeed")]
    pub auto_speed: f32,
    #[rename(name = "DefaultCameraPosIndex")]
    pub default_camera_pos_index: i32,
    #[rename(name = "CameraPos")]
    pub camera_pos:
        ::unity2::Array<crate::combat::unitdetailcameraposition::UnitDetailCameraPosition>,
    #[rename(name = "m_Front")]
    pub m_front: f32,
    #[rename(name = "m_Direction")]
    pub m_direction: f32,
    #[rename(name = "m_LastRotateSpeed")]
    pub m_last_rotate_speed: f32,
    #[rename(name = "m_AutoRotate")]
    pub m_auto_rotate: bool,
    #[rename(name = "m_EmblemAlpha")]
    pub m_emblem_alpha: f32,
    #[rename(name = "m_EmblemAlphaStep")]
    pub m_emblem_alpha_step: f32,
    #[rename(name = "m_LookEmblem")]
    pub m_look_emblem: bool,
    #[rename(name = "m_LookEmblemRate")]
    pub m_look_emblem_rate: f32,
}

#[cfg(feature = "combat-cameracontrollerunitdetail")]
#[::unity2::methods]
impl CameraControllerUnitDetail {
    #[method(name = "get_Distance", args = 0)]
    pub fn get_distance(self) -> f32;

    #[method(name = "set_Distance", args = 1)]
    pub fn set_distance(self, value: f32) -> ();

    #[method(name = "get_IsJointLoaded", args = 0)]
    pub fn get_is_joint_loaded(self) -> bool;

    #[method(name = "Activate", args = 0)]
    pub fn activate(self) -> ();

    #[method(name = "GetCameraTargets", args = 0)]
    pub fn get_camera_targets(self) -> ::unity2::Array<i32>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-cameracontrollerunitdetail")]
impl CameraControllerUnitDetail {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraControllerUnitDetail),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraControllerUnitDetailMethods>::ctor(this);
        this
    }
}
