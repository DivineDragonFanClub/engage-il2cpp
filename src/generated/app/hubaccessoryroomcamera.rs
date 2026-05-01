
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubaccessoryroomcamera/HubAccessoryRoomCamera.md")))]
#[::unity2::class(namespace = "App", name = "HubAccessoryRoomCamera")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubAccessoryRoomCamera {
    #[rename(name = "RotateSpeed")]
    pub rotate_speed: f32,
    #[rename(name = "RotateAccel")]
    pub rotate_accel: f32,
    #[rename(name = "RotateBrake")]
    pub rotate_brake: f32,
    #[rename(name = "CamSpeedBody")]
    pub cam_speed_body: f32,
    #[rename(name = "CamSpeedAccs")]
    pub cam_speed_accs: f32,
    #[rename(name = "MinDistBody")]
    pub min_dist_body: f32,
    #[rename(name = "MaxDistBody")]
    pub max_dist_body: f32,
    #[rename(name = "MinDistAccs")]
    pub min_dist_accs: f32,
    #[rename(name = "MaxDistAccs")]
    pub max_dist_accs: f32,
    #[rename(name = "MinFoVBody")]
    pub min_fo_v_body: f32,
    #[rename(name = "MaxFoVBody")]
    pub max_fo_v_body: f32,
    #[rename(name = "MinFoVAccs")]
    pub min_fo_v_accs: f32,
    #[rename(name = "MaxFoVAccs")]
    pub max_fo_v_accs: f32,
    #[rename(name = "HeightFixerBody")]
    pub height_fixer_body: f32,
    #[rename(name = "FarthestBodyHeight")]
    pub farthest_body_height: f32,
    #[rename(name = "HeightFixerBack")]
    pub height_fixer_back: f32,
    #[static_field]
    #[rename(name = "HeightFixerHead")]
    pub height_fixer_head: f32,
    #[static_field]
    #[rename(name = "HeightFixerFace")]
    pub height_fixer_face: f32,
    #[rename(name = "TiltHigh")]
    pub tilt_high: f32,
    #[rename(name = "TiltLow")]
    pub tilt_low: f32,
    #[rename(name = "TiltHighMax")]
    pub tilt_high_max: f32,
    #[rename(name = "TiltLowMax")]
    pub tilt_low_max: f32,
    #[rename(name = "TiltSpeed")]
    pub tilt_speed: f32,
    #[rename(name = "TiltBackSpeedHand")]
    pub tilt_back_speed_hand: f32,
    #[rename(name = "TiltBackSpeedAuto")]
    pub tilt_back_speed_auto: f32,
    #[rename(name = "CameraChangeSpeed")]
    pub camera_change_speed: f32,
    #[rename(name = "CameraChangeCurve")]
    pub camera_change_curve: f32,
    #[rename(name = "SlipSlideSizeTop")]
    pub slip_slide_size_top: f32,
    #[rename(name = "SlipSlideSizeSelect")]
    pub slip_slide_size_select: f32,
    #[rename(name = "SlipSlideSizePreview")]
    pub slip_slide_size_preview: f32,
    #[rename(name = "SlipSlideSpeed")]
    pub slip_slide_speed: f32,
    #[rename(name = "CameraBoost")]
    pub camera_boost: f32,
    #[rename(name = "EyesIKWeight")]
    pub eyes_ik_weight: f32,
    #[rename(name = "HeadIKWeight")]
    pub head_ik_weight: f32,
    #[rename(name = "BodyIKWeight")]
    pub body_ik_weight: f32,
    #[rename(name = "LookChangeSpeed")]
    pub look_change_speed: f32,
    #[rename(name = "LookChangeDegree")]
    pub look_change_degree: f32,
    #[rename(name = "m_SelectingBackId")]
    pub m_selecting_back_id: i32,
    #[rename(name = "m_IsBackUpdating")]
    pub m_is_back_updating: bool,
    #[rename(name = "m_BGHandle")]
    pub m_bg_handle: crate::app::resourcehandle_2::ResourceHandle_2,
    #[rename(name = "m_LookTarget")]
    pub m_look_target: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ViewMode")]
    pub m_view_mode: crate::app::hubaccessoryroom::HubAccessoryRoom_ViewMode,
    #[rename(name = "m_CameraPos")]
    pub m_camera_pos:
        crate::app::hubaccessoryroomcamera::HubAccessoryRoomCamera_CameraPositionParam,
    #[rename(name = "m_CameraDiff")]
    pub m_camera_diff:
        crate::app::hubaccessoryroomcamera::HubAccessoryRoomCamera_CameraPositionParam,
    #[rename(name = "m_CameraDiffRate")]
    pub m_camera_diff_rate: f32,
    #[rename(name = "m_LookCamRate")]
    pub m_look_cam_rate: f32,
    #[rename(name = "m_LastTargetPosNearest")]
    pub m_last_target_pos_nearest: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_LastTargetPosFarthest")]
    pub m_last_target_pos_farthest: crate::unity_engine::vector3::Vector3,
    #[rename(name = "Backgrounds")]
    pub backgrounds: ::unity2::Array<
        crate::app::hubaccessoryroomcamera::HubAccessoryRoomCamera_BackgroundSettings,
    >,
    #[rename(name = "MainLight")]
    pub main_light: crate::unity_engine::light::Light,
    #[rename(name = "m_LastRotSpeed")]
    pub m_last_rot_speed: f32,
}

#[cfg(feature = "app-hubaccessoryroomcamera")]
#[::unity2::methods]
impl HubAccessoryRoomCamera {
    #[method(name = "get_MinDist", args = 0)]
    pub fn get_min_dist(self) -> f32;

    #[method(name = "get_MaxDist", args = 0)]
    pub fn get_max_dist(self) -> f32;

    #[method(name = "get_MinFoV", args = 0)]
    pub fn get_min_fo_v(self) -> f32;

    #[method(name = "get_MaxFoV", args = 0)]
    pub fn get_max_fo_v(self) -> f32;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "get_Character", args = 0)]
    pub fn get_character(self) -> crate::combat::character::Character;

    #[method(name = "set_Character", args = 1)]
    pub fn set_character(self, value: crate::combat::character::Character) -> ();

    #[method(name = "get_PID", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PID", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "SetCharacter", args = 2)]
    pub fn set_character_2(
        self,
        chr: crate::combat::character::Character,
        pid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "get_ViewMode", args = 0)]
    pub fn get_view_mode(self) -> crate::app::hubaccessoryroom::HubAccessoryRoom_ViewMode;

    #[method(name = "set_ViewMode", args = 1)]
    pub fn set_view_mode(
        self,
        value: crate::app::hubaccessoryroom::HubAccessoryRoom_ViewMode,
    ) -> ();

    #[method(name = "get_IsAccs", args = 0)]
    pub fn get_is_accs(self) -> bool;

    #[method(name = "set_IsAccs", args = 1)]
    pub fn set_is_accs(self, value: bool) -> ();

    #[method(name = "get_IsCharacterChanged", args = 0)]
    pub fn get_is_character_changed(self) -> bool;

    #[method(name = "set_IsCharacterChanged", args = 1)]
    pub fn set_is_character_changed(self, value: bool) -> ();

    #[method(name = "get_IsTargetChanged", args = 0)]
    pub fn get_is_target_changed(self) -> bool;

    #[method(name = "set_IsTargetChanged", args = 1)]
    pub fn set_is_target_changed(self, value: bool) -> ();

    #[method(name = "get_IsPreviewChanged", args = 0)]
    pub fn get_is_preview_changed(self) -> bool;

    #[method(name = "set_IsPreviewChanged", args = 1)]
    pub fn set_is_preview_changed(self, value: bool) -> ();

    #[method(name = "get_CameraTilt", args = 0)]
    pub fn get_camera_tilt(self) -> f32;

    #[method(name = "set_CameraTilt", args = 1)]
    pub fn set_camera_tilt(self, value: f32) -> ();

    #[method(name = "get_SlipSlide", args = 0)]
    pub fn get_slip_slide(self) -> f32;

    #[method(name = "set_SlipSlide", args = 1)]
    pub fn set_slip_slide(self, value: f32) -> ();

    #[method(name = "get_IsLookCam", args = 0)]
    pub fn get_is_look_cam(self) -> bool;

    #[method(name = "set_IsLookCam", args = 1)]
    pub fn set_is_look_cam(self, value: bool) -> ();

    #[method(name = "get_TargetPosNearest", args = 0)]
    pub fn get_target_pos_nearest(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_TargetPosFarthest", args = 0)]
    pub fn get_target_pos_farthest(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "InitPos", args = 1)]
    pub fn init_pos(self, force: bool) -> ();

    #[method(name = "UpdateBack", args = 0)]
    pub fn update_back(self) -> ();

    #[method(name = "LoadInput", args = 0)]
    pub fn load_input(self) -> ();

    #[method(name = "UpdateCameraPos", args = 0)]
    pub fn update_camera_pos(self) -> ();

    #[method(name = "SetCameraPos", args = 0)]
    pub fn set_camera_pos(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubaccessoryroomcamera")]
impl HubAccessoryRoomCamera {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubAccessoryRoomCamera),
                ::core::stringify!(new),
            )
        });
        <Self as IHubAccessoryRoomCameraMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubaccessoryroomcamera/HubAccessoryRoomCamera_CameraPositionParam.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct HubAccessoryRoomCamera_CameraPositionParam {}

impl ::unity2::ClassIdentity for HubAccessoryRoomCamera_CameraPositionParam {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubAccessoryRoomCamera.CameraPositionParam";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubAccessoryRoomCamera_CameraPositionParam {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-hubaccessoryroomcamera")]
#[::unity2::methods(value)]
impl HubAccessoryRoomCamera_CameraPositionParam {
    #[method(name = "get_Distance", args = 0)]
    pub fn get_distance(self) -> f32;

    #[method(name = "set_Distance", args = 1)]
    pub fn set_distance(self, value: f32) -> ();

    #[method(name = "get_FoV", args = 0)]
    pub fn get_fo_v(self) -> f32;

    #[method(name = "set_FoV", args = 1)]
    pub fn set_fo_v(self, value: f32) -> ();

    #[method(name = "get_Degree", args = 0)]
    pub fn get_degree(self) -> f32;

    #[method(name = "set_Degree", args = 1)]
    pub fn set_degree(self, value: f32) -> ();

    #[method(name = "get_TargetPos", args = 0)]
    pub fn get_target_pos(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_TargetPos", args = 1)]
    pub fn set_target_pos(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "op_Multiply", args = 2)]
    pub fn op_multiply(
        param: crate::app::hubaccessoryroomcamera::HubAccessoryRoomCamera_CameraPositionParam,
        rate: f32,
    ) -> crate::app::hubaccessoryroomcamera::HubAccessoryRoomCamera_CameraPositionParam;

    #[method(name = "op_Addition", args = 2)]
    pub fn op_addition(
        left: crate::app::hubaccessoryroomcamera::HubAccessoryRoomCamera_CameraPositionParam,
        right: crate::app::hubaccessoryroomcamera::HubAccessoryRoomCamera_CameraPositionParam,
    ) -> crate::app::hubaccessoryroomcamera::HubAccessoryRoomCamera_CameraPositionParam;

    #[method(name = "op_Subtraction", args = 2)]
    pub fn op_subtraction(
        left: crate::app::hubaccessoryroomcamera::HubAccessoryRoomCamera_CameraPositionParam,
        right: crate::app::hubaccessoryroomcamera::HubAccessoryRoomCamera_CameraPositionParam,
    ) -> crate::app::hubaccessoryroomcamera::HubAccessoryRoomCamera_CameraPositionParam;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubaccessoryroomcamera/HubAccessoryRoomCamera_BackgroundSettings.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct HubAccessoryRoomCamera_BackgroundSettings {
    pub light_direction: crate::unity_engine::vector3::Vector3,
    pub light_color: crate::unity_engine::color::Color,
}

impl ::unity2::ClassIdentity for HubAccessoryRoomCamera_BackgroundSettings {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubAccessoryRoomCamera.BackgroundSettings";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubAccessoryRoomCamera_BackgroundSettings {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}
