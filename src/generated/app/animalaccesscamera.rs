
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/animalaccesscamera/AnimalAccessCamera.md")))]
#[::unity2::class(namespace = "App", name = "AnimalAccessCamera")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct AnimalAccessCamera {
    #[rename(name = "m_Camera")]
    pub m_camera: crate::unity_engine::camera::Camera,
    #[rename(name = "m_AngleX")]
    pub m_angle_x: crate::app::interpolatorrotation::InterpolatorRotation,
    #[rename(name = "m_AngleY")]
    pub m_angle_y: crate::app::interpolatorrotation::InterpolatorRotation,
}

#[cfg(feature = "app-animalaccesscamera")]
#[::unity2::methods]
impl AnimalAccessCamera {
    #[method(name = "get_Target", args = 0)]
    pub fn get_target(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_Target", args = 1)]
    pub fn set_target(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_Distance", args = 0)]
    pub fn get_distance(self) -> f32;

    #[method(name = "set_Distance", args = 1)]
    pub fn set_distance(self, value: f32) -> ();

    #[method(name = "get_InitAngle", args = 0)]
    pub fn get_init_angle(self) -> f32;

    #[method(name = "set_InitAngle", args = 1)]
    pub fn set_init_angle(self, value: f32) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "UpdateKey", args = 0)]
    pub fn update_key(self) -> ();

    #[method(name = "Commit", args = 0)]
    pub fn commit(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-animalaccesscamera")]
impl AnimalAccessCamera {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimalAccessCamera),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimalAccessCameraMethods>::ctor(this);
        this
    }
}
