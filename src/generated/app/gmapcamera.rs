
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapcamera/GmapCamera_DistanceMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GmapCamera_DistanceMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GmapCamera_DistanceMode {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GmapCamera.DistanceMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GmapCamera_DistanceMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GmapCamera_DistanceMode {
    pub fn near() -> Self {
        Self { value: 0 }
    }

    pub fn middle() -> Self {
        Self { value: 1 }
    }

    pub fn far() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapcamera/GmapCamera.md")))]
#[::unity2::class(namespace = "App", name = "GmapCamera")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: gmapcamera :: GmapCamera >)]
pub struct GmapCamera {
    #[rename(name = "m_SphereCenter")]
    pub m_sphere_center: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_DefaultAngleX")]
    pub m_default_angle_x: f32,
    #[rename(name = "m_DefaultDistance")]
    pub m_default_distance: f32,
    #[rename(name = "m_AngleCurve")]
    pub m_angle_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_DistanceCurve")]
    pub m_distance_curve: crate::unity_engine::animationcurve::AnimationCurve,
    #[rename(name = "m_ZoomSpeed")]
    pub m_zoom_speed: f32,
    #[rename(name = "m_Camera")]
    pub m_camera: crate::unity_engine::camera::Camera,
    #[rename(name = "m_ZoomParam")]
    pub m_zoom_param: ::unity2::Array<f32>,
    #[rename(name = "m_ZoomDir")]
    pub m_zoom_dir: f32,
    #[rename(name = "m_Position")]
    pub m_position: crate::app::interpolatorvector3::InterpolatorVector3,
    #[rename(name = "m_AngleX")]
    pub m_angle_x: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_Distance")]
    pub m_distance: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_IsRStickZoom")]
    pub m_is_r_stick_zoom: bool,
    #[rename(name = "m_PrevZoom")]
    pub m_prev_zoom: f32,
}

#[cfg(feature = "app-gmapcamera")]
#[::unity2::methods]
impl GmapCamera {
    #[method(name = "get_ZoomDistance", args = 0)]
    pub fn get_zoom_distance(self) -> f32;

    #[method(name = "set_ZoomDistance", args = 1)]
    pub fn set_zoom_distance(self, value: f32) -> ();

    #[method(name = "get_Position", args = 0)]
    pub fn get_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "get_CameraPosition", args = 0)]
    pub fn get_camera_position(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_IsCameraTracking", args = 1)]
    pub fn set_is_camera_tracking(self, value: bool) -> ();

    #[method(name = "get_IsCameraTracking", args = 0)]
    pub fn get_is_camera_tracking(self) -> bool;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "UpdateKey", args = 0)]
    pub fn update_key(self) -> ();

    #[method(name = "Commit", args = 0)]
    pub fn commit(self) -> ();

    #[method(name = "Instant", args = 0)]
    pub fn instant(self) -> ();

    #[method(name = "ActiveCamera", args = 0)]
    pub fn active_camera(self) -> ();

    #[method(name = "GetCameraDistanceMode", args = 0)]
    pub fn get_camera_distance_mode(self) -> crate::app::gmapcamera::GmapCamera_DistanceMode;

    #[method(name = "GetBackMode", args = 1)]
    pub fn get_back_mode(
        self,
        mode: crate::app::gmapcamera::GmapCamera_DistanceMode,
    ) -> crate::app::gmapcamera::GmapCamera_DistanceMode;

    #[method(name = "GetZoomMode", args = 1)]
    pub fn get_zoom_mode(
        self,
        mode: crate::app::gmapcamera::GmapCamera_DistanceMode,
    ) -> crate::app::gmapcamera::GmapCamera_DistanceMode;

    #[method(name = "GetNextMode", args = 2)]
    pub fn get_next_mode(
        self,
        mode: crate::app::gmapcamera::GmapCamera_DistanceMode,
        dir: f32,
    ) -> crate::app::gmapcamera::GmapCamera_DistanceMode;

    #[method(name = "GetParamTime", args = 1)]
    pub fn get_param_time(name: ::unity2::Il2CppString) -> f32;

    #[method(name = "GetMoveableRect", args = 0)]
    pub fn get_moveable_rect() -> crate::unity_engine::rect::Rect;

    #[method(name = "IsMovableArea", args = 3)]
    pub fn is_movable_area(
        position: crate::unity_engine::vector3::Vector3,
        out_area_flag: crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_DirFlagField,
        ignore_flag: crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_DirFlagField,
    ) -> bool;

    #[method(name = "GetDisableDir", args = 2)]
    pub fn get_disable_dir(
        position: crate::unity_engine::vector3::Vector3,
        ignore_flag: crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_DirFlagField,
    ) -> crate::app::gmapsequence::GmapSequence_GmapFreeCameraSequence_DirFlagField;

    #[method(name = "SetPosition", args = 2)]
    pub fn set_position(
        self,
        position: crate::unity_engine::vector3::Vector3,
        is_free_camera: bool,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Zoom", args = 1)]
    pub fn zoom(self, value: f32) -> ();

    #[method(name = "SetZoomParam", args = 1)]
    pub fn set_zoom_param(self, index: i32) -> ();

    #[method(name = "SetLookOver", args = 1)]
    pub fn set_look_over(
        self,
        look_over_camera_position: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "CancelLookOver", args = 1)]
    pub fn cancel_look_over(self, return_position: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "CheckScroll", args = 0)]
    pub fn check_scroll(self) -> bool;

    #[method(name = "CheckScrollStrictly", args = 0)]
    pub fn check_scroll_strictly(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapcamera")]
impl GmapCamera {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapCamera),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapCameraMethods>::ctor(this);
        this
    }
}
