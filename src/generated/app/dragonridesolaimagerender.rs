
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridesolaimagerender/DragonRideSolaImageRender.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideSolaImageRender")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DragonRideSolaImageRender {
    #[rename(name = "m_Camera")]
    pub m_camera: crate::unity_engine::camera::Camera,
    #[rename(name = "m_Sola")]
    pub m_sola: crate::combat::character::Character,
}

#[cfg(feature = "app-dragonridesolaimagerender")]
#[::unity2::methods]
impl DragonRideSolaImageRender {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "SetLayer", args = 2)]
    pub fn set_layer(self, obj: crate::unity_engine::gameobject::GameObject, set_layer: i32) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "PlaySolaAnime", args = 1)]
    pub fn play_sola_anime(self, name: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-dragonridesolaimagerender")]
impl DragonRideSolaImageRender {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideSolaImageRender),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideSolaImageRenderMethods>::ctor(this);
        this
    }
}
