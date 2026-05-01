
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubcamera/HubCamera.md")))]
#[::unity2::class(namespace = "App", name = "HubCamera")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct HubCamera {
    #[rename(name = "m_CameraTargetHeight")]
    pub m_camera_target_height: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_OffsetCurveX")]
    pub m_offset_curve_x: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_DistanceCurve")]
    pub m_distance_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_FovCurve")]
    pub m_fov_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_TalkCurve")]
    pub m_talk_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_NearClip")]
    pub m_near_clip: f32,
    #[rename(name = "m_RotateSpeed")]
    pub m_rotate_speed: f32,
    #[rename(name = "m_PitchSpeed")]
    pub m_pitch_speed: f32,
    #[rename(name = "m_RotateFollowSpeed")]
    pub m_rotate_follow_speed: f32,
    #[rename(name = "m_FollowRate")]
    pub m_follow_rate: f32,
    #[rename(name = "m_PredictionLimitAngle")]
    pub m_prediction_limit_angle: f32,
    #[rename(name = "m_SideOffsetIndex")]
    pub m_side_offset_index: f32,
    #[static_field]
    #[rename(name = "m_DefaultAngleX")]
    pub m_default_angle_x: f32,
    #[rename(name = "m_Camera")]
    pub m_camera: crate::unity_engine::camera::Camera,
    #[rename(name = "PlayerController")]
    pub player_controller: crate::app::hubplayercontroller::HubPlayerController,
    #[rename(name = "SideLength")]
    pub side_length: f32,
    #[rename(name = "m_AngleX")]
    pub m_angle_x: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_TargetPosition")]
    pub m_target_position: crate::app::interpolatorvector3::InterpolatorVector3,
    #[rename(name = "m_Position")]
    pub m_position: crate::app::interpolatorvector3::InterpolatorVector3,
    #[rename(name = "m_AngleY")]
    pub m_angle_y: crate::app::interpolatorrotation::InterpolatorRotation,
    #[rename(name = "m_Distance")]
    pub m_distance: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_Fov")]
    pub m_fov: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_DistanceRatio")]
    pub m_distance_ratio: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_HeightRatio")]
    pub m_height_ratio: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_TalkTail")]
    pub m_talk_tail: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_SideOffset")]
    pub m_side_offset: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_PredictionPitch")]
    pub m_prediction_pitch: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "ObjectCollisionLayerMask")]
    pub object_collision_layer_mask: i32,
    #[rename(name = "HeightLayerMask")]
    pub height_layer_mask: i32,
    #[rename(name = "hresults")]
    pub hresults: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
    #[rename(name = "rayhits")]
    pub rayhits: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
    #[rename(name = "m_zoom")]
    pub m_zoom: f32,
    #[rename(name = "TalkCameraMoveTime")]
    pub talk_camera_move_time: f32,
    #[rename(name = "TalkCameraReturnTime")]
    pub talk_camera_return_time: f32,
    #[rename(name = "NormalTalkDistanceRatio")]
    pub normal_talk_distance_ratio: f32,
    #[rename(name = "NormalTalkAngleX")]
    pub normal_talk_angle_x: f32,
    #[rename(name = "NormalTalkAngleY")]
    pub normal_talk_angle_y: f32,
    #[rename(name = "NormalTalkZoom")]
    pub normal_talk_zoom: f32,
    #[rename(name = "NormalTalkSeparateDistance")]
    pub normal_talk_separate_distance: f32,
    #[rename(name = "NormalTalkOffsetY")]
    pub normal_talk_offset_y: f32,
    #[rename(name = "ShopTalkDistanceRatio")]
    pub shop_talk_distance_ratio: f32,
    #[rename(name = "ShopTalkAngleX")]
    pub shop_talk_angle_x: f32,
    #[rename(name = "ShopTalkAngleY")]
    pub shop_talk_angle_y: f32,
    #[rename(name = "ShopTalkZoom")]
    pub shop_talk_zoom: f32,
    #[rename(name = "ShopTalkSeparateDistance")]
    pub shop_talk_separate_distance: f32,
    #[rename(name = "ShopTalkOffsetY")]
    pub shop_talk_offset_y: f32,
}

#[cfg(feature = "app-hubcamera")]
#[::unity2::methods]
impl HubCamera {
    #[method(name = "get_IsStop", args = 0)]
    pub fn get_is_stop(self) -> bool;

    #[method(name = "set_IsStop", args = 1)]
    pub fn set_is_stop(self, value: bool) -> ();

    #[method(name = "get_AngleY", args = 0)]
    pub fn get_angle_y(self) -> f32;

    #[method(name = "get_IsAdjustPosition", args = 0)]
    pub fn get_is_adjust_position(self) -> bool;

    #[method(name = "get_IsAdjustPositionSub", args = 0)]
    pub fn get_is_adjust_position_sub(self) -> bool;

    #[method(name = "get_IsAdjustHeight", args = 0)]
    pub fn get_is_adjust_height(self) -> bool;

    #[method(name = "get_IsPredictionHeight", args = 0)]
    pub fn get_is_prediction_height(self) -> bool;

    #[method(name = "get_PredictionHeightTime", args = 0)]
    pub fn get_prediction_height_time(self) -> f32;

    #[method(name = "get_IsMaximumZoom", args = 0)]
    pub fn get_is_maximum_zoom(self) -> bool;

    #[method(name = "get_ZoomSpeed", args = 0)]
    pub fn get_zoom_speed(self) -> f32;

    #[method(name = "get_ZoomInterpolateTime", args = 0)]
    pub fn get_zoom_interpolate_time(self) -> f32;

    #[method(name = "get_ZoomDistanceTime", args = 0)]
    pub fn get_zoom_distance_time(self) -> f32;

    #[method(name = "get_CameraSpeed", args = 0)]
    pub fn get_camera_speed(self) -> f32;

    #[method(name = "get_CameraRadius", args = 0)]
    pub fn get_camera_radius(self) -> f32;

    #[method(name = "get_CameraPositionTime", args = 0)]
    pub fn get_camera_position_time(self) -> f32;

    #[method(name = "get_PitchParam", args = 0)]
    pub fn get_pitch_param(self) -> f32;

    #[method(name = "get_PitchBlankParam", args = 0)]
    pub fn get_pitch_blank_param(self) -> f32;

    #[method(name = "get_PitchHeightParam", args = 0)]
    pub fn get_pitch_height_param(self) -> f32;

    #[method(name = "get_ZoomParam", args = 0)]
    pub fn get_zoom_param(self) -> f32;

    #[method(name = "set_ZoomParam", args = 1)]
    pub fn set_zoom_param(self, value: f32) -> ();

    #[method(name = "get_CameraRoateteParamName", args = 0)]
    pub fn get_camera_roatete_param_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Reset", args = 1)]
    pub fn reset(self, zoom_step: i32) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "UpdateKey", args = 0)]
    pub fn update_key(self) -> ();

    #[method(name = "CompareNear", args = 2)]
    pub fn compare_near(
        a: crate::unity_engine::raycasthit::RaycastHit,
        b: crate::unity_engine::raycasthit::RaycastHit,
    ) -> i32;

    #[method(name = "GetHitOffsetPos", args = 5)]
    pub fn get_hit_offset_pos(
        pos: crate::unity_engine::vector3::Vector3,
        target: crate::unity_engine::vector3::Vector3,
        dir: crate::unity_engine::vector3::Vector3,
        offset: crate::unity_engine::vector3::Vector3,
        hit: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Commit", args = 0)]
    pub fn commit(self) -> ();

    #[method(name = "Reset", args = 1)]
    pub fn reset_2(self, y: f32) -> ();

    #[method(name = "SetAngleY", args = 1)]
    pub fn set_angle_y(self, y: f32) -> ();

    #[method(name = "SetAngleX", args = 1)]
    pub fn set_angle_x(self, x: f32) -> ();

    #[method(name = "Instant", args = 0)]
    pub fn instant(self) -> ();

    #[method(name = "GetParamTime", args = 1)]
    pub fn get_param_time(name: ::unity2::Il2CppString) -> f32;

    #[method(name = "GetZoomRatio", args = 0)]
    pub fn get_zoom_ratio(self) -> f32;

    #[method(name = "SetZoom", args = 1)]
    pub fn set_zoom(self, zoom: f32) -> ();

    #[method(name = "CheckScroll", args = 0)]
    pub fn check_scroll(self) -> bool;

    #[method(name = "CheckScrollStrictly", args = 0)]
    pub fn check_scroll_strictly(self) -> bool;

    #[method(name = "StartManualCamera_NormalTalk", args = 2)]
    pub fn start_manual_camera_normal_talk(
        self,
        player: crate::unity_engine::vector3::Vector3,
        target: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "StartManualCamera_FaceMainTalk", args = 3)]
    pub fn start_manual_camera_face_main_talk(
        self,
        player: crate::unity_engine::vector3::Vector3,
        target: crate::unity_engine::vector3::Vector3,
        forward: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "StartManualCamera_ShopTalk", args = 2)]
    pub fn start_manual_camera_shop_talk(
        self,
        player: crate::unity_engine::vector3::Vector3,
        target: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "StartManualCamera", args = 9)]
    pub fn start_manual_camera(
        self,
        source: crate::unity_engine::vector3::Vector3,
        target: crate::unity_engine::vector3::Vector3,
        ratio: f32,
        angle_x: f32,
        angle_y: f32,
        zoom: f32,
        separate_distance: f32,
        offset_y: f32,
        move_time: f32,
    ) -> ();

    #[method(name = "Intersect", args = 8)]
    pub fn intersect(
        self,
        source: crate::unity_engine::vector3::Vector3,
        target: crate::unity_engine::vector3::Vector3,
        ratio: f32,
        angle_x: f32,
        angle_y: f32,
        zoom: f32,
        separate_distance: f32,
        offset_y: f32,
    ) -> bool;

    #[method(name = "ReturnManualCamera", args = 0)]
    pub fn return_manual_camera(self) -> ();

    #[method(name = "ReturnManualCamera", args = 1)]
    pub fn return_manual_camera_2(self, return_time: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-hubcamera")]
impl HubCamera {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubCamera),
                ::core::stringify!(new),
            )
        });
        <Self as IHubCameraMethods>::ctor(this);
        this
    }
}
