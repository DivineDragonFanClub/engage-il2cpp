
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talkfacecameraadjustor/TalkFaceCameraAdjustor.md")))]
#[::unity2::class(namespace = "App", name = "TalkFaceCameraAdjustor")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct TalkFaceCameraAdjustor {
    #[rename(name = "m_CameraTransform")]
    pub m_camera_transform: crate::unity_engine::transform::Transform,
    #[rename(name = "m_CameraHeightOffset")]
    pub m_camera_height_offset: f32,
    #[rename(name = "m_DefaultCameraOffset")]
    pub m_default_camera_offset: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Tmp")]
    pub m_tmp: crate::unity_engine::vector3::Vector3,
}

#[cfg(feature = "app-talkfacecameraadjustor")]
#[::unity2::methods]
impl TalkFaceCameraAdjustor {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-talkfacecameraadjustor")]
impl TalkFaceCameraAdjustor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkFaceCameraAdjustor),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkFaceCameraAdjustorMethods>::ctor(this);
        this
    }
}
