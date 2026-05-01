
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/cameracontrollerbehind/CameraControllerBehind.md")))]
#[::unity2::class(namespace = "Combat", name = "CameraControllerBehind")]
#[parent(crate::combat::basecameracontroller::BaseCameraController)]
pub struct CameraControllerBehind {
    #[rename(name = "Distance")]
    pub distance: f32,
    #[rename(name = "CameraHeight")]
    pub camera_height: f32,
    #[rename(name = "CameraLookUp")]
    pub camera_look_up: f32,
    #[rename(name = "DistanceRate")]
    pub distance_rate: f32,
    #[rename(name = "SidePlayer")]
    pub side_player: bool,
    #[rename(name = "NearUnitSize")]
    pub near_unit_size: f32,
    #[rename(name = "FarUnitSize")]
    pub far_unit_size: f32,
    #[rename(name = "CameraUpThreshold")]
    pub camera_up_threshold: f32,
    #[rename(name = "m_BaseDegree")]
    pub m_base_degree: f32,
    #[rename(name = "m_LastDegree")]
    pub m_last_degree: f32,
    #[rename(name = "m_LastLookHeight")]
    pub m_last_look_height: f32,
    #[rename(name = "m_DoCameraLookUp")]
    pub m_do_camera_look_up: bool,
    #[rename(name = "isValid")]
    pub is_valid: bool,
    #[rename(name = "m_IsInverse")]
    pub m_is_inverse: bool,
    #[rename(name = "m_IsSideRev")]
    pub m_is_side_rev: bool,
}

#[cfg(feature = "combat-cameracontrollerbehind")]
#[::unity2::methods]
impl CameraControllerBehind {
    #[method(name = "get_WidthOnCamera", args = 0)]
    pub fn get_width_on_camera(self) -> f32;

    #[method(name = "set_WidthOnCamera", args = 1)]
    pub fn set_width_on_camera(self, value: f32) -> ();

    #[method(name = "get_NearSide", args = 0)]
    pub fn get_near_side(self) -> i32;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Initilize", args = 0)]
    pub fn initilize(self) -> ();

    #[method(name = "CheckUsable", args = 1)]
    pub fn check_usable(self, is_routine: bool) -> ();

    #[method(name = "Activate", args = 0)]
    pub fn activate(self) -> ();

    #[method(name = "Stabilize", args = 0)]
    pub fn stabilize(self) -> ();

    #[method(name = "SetInverse", args = 2)]
    pub fn set_inverse(self, inv_side: bool, inv_camera: bool) -> ();

    #[method(name = "GetCameraTargets", args = 0)]
    pub fn get_camera_targets(self) -> ::unity2::Array<i32>;

    #[method(name = "IsDive", args = 0)]
    pub fn is_dive(self) -> bool;

    #[method(name = "IsDragon", args = 0)]
    pub fn is_dragon(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-cameracontrollerbehind")]
impl CameraControllerBehind {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraControllerBehind),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraControllerBehindMethods>::ctor(this);
        this
    }
}
