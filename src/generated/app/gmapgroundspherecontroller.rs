
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gmapgroundspherecontroller/GmapGroundSphereController.md")))]
#[::unity2::class(namespace = "App", name = "GmapGroundSphereController")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct GmapGroundSphereController {
    #[rename(name = "m_MeshRenderer")]
    pub m_mesh_renderer: crate::unity_engine::meshrenderer::MeshRenderer,
    #[rename(name = "m_GrandSunkTexture")]
    pub m_grand_sunk_texture: crate::unity_engine::texture2d::Texture2D,
    #[rename(name = "m_GrandAscendTexture")]
    pub m_grand_ascend_texture: crate::unity_engine::texture2d::Texture2D,
}

#[cfg(feature = "app-gmapgroundspherecontroller")]
#[::unity2::methods]
impl GmapGroundSphereController {
    #[method(name = "ChangeTexture", args = 1)]
    pub fn change_texture(self, is_ascend: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gmapgroundspherecontroller")]
impl GmapGroundSphereController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GmapGroundSphereController),
                ::core::stringify!(new),
            )
        });
        <Self as IGmapGroundSphereControllerMethods>::ctor(this);
        this
    }
}
