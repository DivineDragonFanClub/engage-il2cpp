
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/boundingboxdrawer/BoundingBoxDrawer.md")))]
#[::unity2::class(namespace = "App", name = "BoundingBoxDrawer")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct BoundingBoxDrawer {
    #[rename(name = "m_OnlySelected")]
    pub m_only_selected: bool,
    #[rename(name = "m_Alpha")]
    pub m_alpha: f32,
}

#[cfg(feature = "app-boundingboxdrawer")]
#[::unity2::methods]
impl BoundingBoxDrawer {
    #[method(name = "DrawBoundingBox", args = 3)]
    pub fn draw_bounding_box(
        bounds: crate::unity_engine::bounds::Bounds,
        go: crate::unity_engine::gameobject::GameObject,
        alpha: f32,
    ) -> ();

    #[method(name = "DrawBoundingBox", args = 2)]
    pub fn draw_bounding_box_2(go: crate::unity_engine::gameobject::GameObject, alpha: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-boundingboxdrawer")]
impl BoundingBoxDrawer {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BoundingBoxDrawer),
                ::core::stringify!(new),
            )
        });
        <Self as IBoundingBoxDrawerMethods>::ctor(this);
        this
    }
}
