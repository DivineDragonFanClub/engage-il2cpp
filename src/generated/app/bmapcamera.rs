
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/bmapcamera/BmapCamera.md")))]
#[::unity2::class(namespace = "App", name = "BmapCamera")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: bmapcamera :: BmapCamera >)]
pub struct BmapCamera {
    #[rename(name = "m_Camera")]
    pub m_camera: crate::unity_engine::camera::Camera,
}

#[cfg(feature = "app-bmapcamera")]
#[::unity2::methods]
impl BmapCamera {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "GetCamera", args = 0)]
    pub fn get_camera(self) -> crate::unity_engine::camera::Camera;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-bmapcamera")]
impl BmapCamera {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BmapCamera),
                ::core::stringify!(new),
            )
        });
        <Self as IBmapCameraMethods>::ctor(this);
        this
    }
}
