
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/autocamerabase/AutoCameraBase.md")))]
#[::unity2::class(namespace = "Combat", name = "AutoCameraBase")]
#[parent(crate::combat::basecameracontroller::BaseCameraController)]
pub struct AutoCameraBase {
    #[static_field]
    #[rename(name = "MinLongitude")]
    pub min_longitude: f32,
    #[static_field]
    #[rename(name = "MaxLongitude")]
    pub max_longitude: f32,
    #[static_field]
    #[rename(name = "MinLatitude")]
    pub min_latitude: f32,
    #[static_field]
    #[rename(name = "MaxLatitude")]
    pub max_latitude: f32,
    #[static_field]
    #[rename(name = "HalfLatitude")]
    pub half_latitude: f32,
    #[rename(name = "m_Longitude")]
    pub m_longitude: f32,
    #[rename(name = "m_Latitude")]
    pub m_latitude: f32,
    #[rename(name = "m_MinDistance")]
    pub m_min_distance: f32,
    #[rename(name = "m_FocusSide")]
    pub m_focus_side: f32,
    #[rename(name = "m_LastViewDir")]
    pub m_last_view_dir: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_FOV")]
    pub m_fov: f32,
    #[rename(name = "m_CameraLookUp")]
    pub m_camera_look_up: crate::unity_engine::vector3::Vector3,
    #[static_field]
    #[rename(name = "HalfDegree")]
    pub half_degree: f32,
    #[rename(name = "HalfDegreeDot")]
    pub half_degree_dot: f32,
}

#[cfg(feature = "combat-autocamerabase")]
#[::unity2::methods]
impl AutoCameraBase {
    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "CheckUsable", args = 1)]
    pub fn check_usable(self, is_routine: bool) -> ();

    #[method(name = "IsSimilarAngle", args = 2)]
    pub fn is_similar_angle(
        self,
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
    ) -> bool;

    #[method(name = "CalcFitDistance", args = 2)]
    pub fn calc_fit_distance(xdist: f32, fov_y_deg: f32) -> f32;

    #[method(name = "Reprojection", args = 4)]
    pub fn reprojection(
        cam: crate::unity_engine::camera::Camera,
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
        t: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Reprojection", args = 4)]
    pub fn reprojection_2(
        mvp: crate::unity_engine::matrix4x4::Matrix4x4,
        a: crate::unity_engine::vector3::Vector3,
        b: crate::unity_engine::vector3::Vector3,
        t: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetCameraTargets", args = 0)]
    pub fn get_camera_targets(self) -> ::unity2::Array<i32>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-autocamerabase")]
impl AutoCameraBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AutoCameraBase),
                ::core::stringify!(new),
            )
        });
        <Self as IAutoCameraBaseMethods>::ctor(this);
        this
    }
}
