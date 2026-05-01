
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomfaderender/MyRoomFadeRender.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomFadeRender")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MyRoomFadeRender {
    #[rename(name = "m_color")]
    pub m_color: crate::unity_engine::color::Color,
    #[rename(name = "m_layer")]
    pub m_layer: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_image")]
    pub m_image: crate::unity_engine::ui::image::Image,
}

#[cfg(feature = "app-myroomfaderender")]
#[::unity2::methods]
impl MyRoomFadeRender {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "SetColor", args = 1)]
    pub fn set_color(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "LateUpdate", args = 0)]
    pub fn late_update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomfaderender")]
impl MyRoomFadeRender {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomFadeRender),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomFadeRenderMethods>::ctor(this);
        this
    }
}
