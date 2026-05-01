
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapcollision/MapCollision.md")))]
#[::unity2::class(namespace = "App", name = "MapCollision")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct MapCollision {
    #[rename(name = "m_MapObject")]
    pub m_map_object: crate::app::mapobject::MapObject,
    #[rename(name = "m_Renderers")]
    pub m_renderers: ::unity2::Array<crate::unity_engine::renderer::Renderer>,
    #[rename(name = "m_Alpha")]
    pub m_alpha: f32,
    #[rename(name = "m_DisableCloseTransparent")]
    pub m_disable_close_transparent: bool,
}

#[cfg(feature = "app-mapcollision")]
#[::unity2::methods]
impl MapCollision {
    #[method(name = "GetRoot", args = 1)]
    pub fn get_root(
        transform: crate::unity_engine::transform::Transform,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "GetMapObject", args = 0)]
    pub fn get_map_object(self) -> crate::app::mapobject::MapObject;

    #[method(name = "GetRenderers", args = 0)]
    pub fn get_renderers(self) -> ::unity2::Array<crate::unity_engine::renderer::Renderer>;

    #[method(name = "SetAlpha", args = 1)]
    pub fn set_alpha(self, alpha: f32) -> ();

    #[method(name = "GetAlpha", args = 0)]
    pub fn get_alpha(self) -> f32;

    #[method(name = "CommitAlpha", args = 0)]
    pub fn commit_alpha(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapcollision")]
impl MapCollision {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapCollision),
                ::core::stringify!(new),
            )
        });
        <Self as IMapCollisionMethods>::ctor(this);
        this
    }
}
