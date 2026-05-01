
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/water/reflectioncamera/ReflectionCamera.md")))]
#[::unity2::class(namespace = "App.Water", name = "ReflectionCamera")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ReflectionCamera {
    #[rename(name = "m_id")]
    pub m_id: i16,
    #[rename(name = "m_camera")]
    pub m_camera: crate::unity_engine::camera::Camera,
    #[rename(name = "m_cameraOriginal")]
    pub m_camera_original: crate::unity_engine::camera::Camera,
    #[rename(name = "farClip")]
    pub far_clip: f32,
    #[rename(name = "clipPlaneOffset")]
    pub clip_plane_offset: f32,
}

#[cfg(feature = "app-water-reflectioncamera")]
#[::unity2::methods]
impl ReflectionCamera {
    #[method(name = "CameraSetting", args = 1)]
    pub fn camera_setting(self, cam: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "SetRenderTextureSize", args = 2)]
    pub fn set_render_texture_size(self, width: f32, height: f32) -> ();

    #[method(name = "CreateTexture", args = 0)]
    pub fn create_texture(self) -> ();

    #[method(name = "RenderReflection", args = 1)]
    pub fn render_reflection(self, transform_mesh: crate::unity_engine::transform::Transform)
        -> ();

    #[method(name = "SetCameraTransform", args = 1)]
    pub fn set_camera_transform(
        self,
        transform_mesh: crate::unity_engine::transform::Transform,
    ) -> ();

    #[method(name = "CalculateObliqueMatrix", args = 2)]
    pub fn calculate_oblique_matrix(
        self,
        projection: crate::unity_engine::matrix4x4::Matrix4x4,
        clip_plane: crate::unity_engine::vector4::Vector4,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "CalculateReflectionMatrix", args = 2)]
    pub fn calculate_reflection_matrix(
        self,
        reflection_mat: crate::unity_engine::matrix4x4::Matrix4x4,
        plane: crate::unity_engine::vector4::Vector4,
    ) -> crate::unity_engine::matrix4x4::Matrix4x4;

    #[method(name = "Sgn", args = 1)]
    pub fn sgn(self, a: f32) -> f32;

    #[method(name = "CameraSpacePlane", args = 4)]
    pub fn camera_space_plane(
        self,
        cam: crate::unity_engine::camera::Camera,
        pos: crate::unity_engine::vector3::Vector3,
        normal: crate::unity_engine::vector3::Vector3,
        side_sign: f32,
    ) -> crate::unity_engine::vector4::Vector4;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-water-reflectioncamera")]
impl ReflectionCamera {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReflectionCamera),
                ::core::stringify!(new),
            )
        });
        <Self as IReflectionCameraMethods>::ctor(this);
        this
    }
}
