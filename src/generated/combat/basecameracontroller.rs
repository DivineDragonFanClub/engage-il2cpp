
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/basecameracontroller/BaseCameraController.md")))]
#[::unity2::class(namespace = "Combat", name = "BaseCameraController")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct BaseCameraController {
    #[rename(name = "LookupDegree")]
    pub lookup_degree: f32,
    #[rename(name = "TripodHeight")]
    pub tripod_height: f32,
    #[rename(name = "MoveSpeedCurve")]
    pub move_speed_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "ChaseSpeedCurve")]
    pub chase_speed_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "IsShakable")]
    pub is_shakable: bool,
    #[rename(name = "Controlable")]
    pub controlable: bool,
    #[rename(name = "CameraBackByEmblem")]
    pub camera_back_by_emblem: f32,
    #[rename(name = "ImmobileCamera")]
    pub immobile_camera: bool,
}

#[cfg(feature = "combat-basecameracontroller")]
#[::unity2::methods]
impl BaseCameraController {
    #[method(name = "get_Follow", args = 0)]
    pub fn get_follow(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "set_Follow", args = 1)]
    pub fn set_follow(self, value: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "get_LookAt", args = 0)]
    pub fn get_look_at(self) -> crate::unity_engine::transform::Transform;

    #[method(name = "set_LookAt", args = 1)]
    pub fn set_look_at(self, value: crate::unity_engine::transform::Transform) -> ();

    #[method(name = "get_BaseFollow", args = 0)]
    pub fn get_base_follow(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_BaseFollow", args = 1)]
    pub fn set_base_follow(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_BaseLookAt", args = 0)]
    pub fn get_base_look_at(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_BaseLookAt", args = 1)]
    pub fn set_base_look_at(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_IsUsable", args = 0)]
    pub fn get_is_usable(self) -> bool;

    #[method(name = "set_IsUsable", args = 1)]
    pub fn set_is_usable(self, value: bool) -> ();

    #[method(name = "get_Positions", args = 0)]
    pub fn get_positions(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::combat::cameraposition::CameraPosition,
    >;

    #[method(name = "set_Positions", args = 1)]
    pub fn set_positions(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::combat::cameraposition::CameraPosition,
        >,
    ) -> ();

    #[method(name = "get_SSS", args = 0)]
    pub fn get_sss(self) -> f32;

    #[method(name = "set_SSS", args = 1)]
    pub fn set_sss(self, value: f32) -> ();

    #[method(name = "get_MoveQuick", args = 0)]
    pub fn get_move_quick(self) -> bool;

    #[method(name = "set_MoveQuick", args = 1)]
    pub fn set_move_quick(self, value: bool) -> ();

    #[method(name = "get_PosData", args = 0)]
    pub fn get_pos_data(self) -> crate::combat::camerapositiondata::CameraPositionData;

    #[method(name = "set_PosData", args = 1)]
    pub fn set_pos_data(self, value: crate::combat::camerapositiondata::CameraPositionData) -> ();

    #[method(name = "get_IsUpdated", args = 0)]
    pub fn get_is_updated(self) -> bool;

    #[method(name = "set_IsUpdated", args = 1)]
    pub fn set_is_updated(self, value: bool) -> ();

    #[method(name = "get_RotateByTransition", args = 0)]
    pub fn get_rotate_by_transition(self) -> f32;

    #[method(name = "set_RotateByTransition", args = 1)]
    pub fn set_rotate_by_transition(self, value: f32) -> ();

    #[method(name = "SetInverse", args = 2)]
    pub fn set_inverse(self, inv_side: bool, inv_camera: bool) -> ();

    #[method(name = "get_Switch", args = 0)]
    pub fn get_switch(self) -> crate::combat::cameraswitch::CameraSwitch;

    #[method(name = "set_Switch", args = 1)]
    pub fn set_switch(self, value: crate::combat::cameraswitch::CameraSwitch) -> ();

    #[method(name = "get_IsActiveVCam", args = 0)]
    pub fn get_is_active_v_cam(self) -> bool;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "UpdatePosition", args = 1)]
    pub fn update_position(self, force: bool) -> ();

    #[method(name = "Stabilize", args = 0)]
    pub fn stabilize(self) -> ();

    #[method(name = "CheckUsable", args = 1)]
    pub fn check_usable(self, is_routine: bool) -> ();

    #[method(name = "Activate", args = 0)]
    pub fn activate(self) -> ();

    #[method(name = "Deactivate", args = 0)]
    pub fn deactivate(self) -> ();

    #[method(name = "SetFollowLookupPos", args = 2)]
    pub fn set_follow_lookup_pos(
        self,
        follow_pos: crate::unity_engine::vector3::Vector3,
        lookat_pos: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "FixHeight", args = 3)]
    pub fn fix_height(
        self,
        current: crate::unity_engine::vector3::Vector3,
        next_pos: crate::unity_engine::vector3::Vector3,
        delta: f32,
    ) -> f32;

    #[method(name = "CheckCollision", args = 0)]
    pub fn check_collision(self) -> bool;

    #[method(name = "CheckObstacle", args = 4)]
    pub fn check_obstacle(
        self,
        side: i32,
        check_enemy: bool,
        no_check_range: f32,
        is_debug: bool,
    ) -> bool;

    #[method(name = "CheckObstacleImpl", args = 3)]
    pub fn check_obstacle_impl(self, side: i32, no_check_range: f32, is_debug: bool) -> bool;

    #[method(name = "DistanceToCamera", args = 1)]
    pub fn distance_to_camera(self, pos: crate::unity_engine::vector3::Vector3) -> f32;

    #[method(name = "GetCameraSpeed", args = 3)]
    pub fn get_camera_speed(
        self,
        curve: crate::unity_engine::animationcurve::AnimationCurve,
        current_pos: crate::unity_engine::vector3::Vector3,
        best_pos: crate::unity_engine::vector3::Vector3,
    ) -> f32;

    #[method(name = "GetCameraTargets", args = 0)]
    pub fn get_camera_targets(self) -> ::unity2::Array<i32>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-basecameracontroller")]
impl BaseCameraController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BaseCameraController),
                ::core::stringify!(new),
            )
        });
        <Self as IBaseCameraControllerMethods>::ctor(this);
        this
    }
}
