
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/cameracontroller/CameraController.md")))]
#[::unity2::class(namespace = "App", name = "CameraController")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: cameracontroller :: CameraController >)]
pub struct CameraController {
    #[rename(name = "m_Hash")]
    pub m_hash: i32,
    #[rename(name = "m_Original")]
    pub m_original: crate::app::cameraparameter::CameraParameter,
    #[rename(name = "m_Operation")]
    pub m_operation: crate::app::cameraparameter::CameraParameter,
    #[rename(name = "m_Updated")]
    pub m_updated: bool,
    #[rename(name = "m_NearClipPlane")]
    pub m_near_clip_plane: f32,
    #[rename(name = "m_FarClipPlane")]
    pub m_far_clip_plane: f32,
    #[rename(name = "m_MouseX")]
    pub m_mouse_x: f32,
    #[rename(name = "m_MouseY")]
    pub m_mouse_y: f32,
}

#[cfg(feature = "app-cameracontroller")]
#[::unity2::methods]
impl CameraController {
    #[method(name = "get_NearClipPlane", args = 0)]
    pub fn get_near_clip_plane(self) -> f32;

    #[method(name = "set_NearClipPlane", args = 1)]
    pub fn set_near_clip_plane(self, value: f32) -> ();

    #[method(name = "get_FarClipPlane", args = 0)]
    pub fn get_far_clip_plane(self) -> f32;

    #[method(name = "set_FarClipPlane", args = 1)]
    pub fn set_far_clip_plane(self, value: f32) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = "TryChange", args = 1)]
    pub fn try_change(self, camera: crate::unity_engine::camera::Camera) -> bool;

    #[method(name = "TryInitialize", args = 1)]
    pub fn try_initialize(self, camera: crate::unity_engine::camera::Camera) -> bool;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-cameracontroller")]
impl CameraController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CameraController),
                ::core::stringify!(new),
            )
        });
        <Self as ICameraControllerMethods>::ctor(this);
        this
    }
}
