
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/water/waterbasev2/WaterBaseV2.md")))]
#[::unity2::class(namespace = "App.Water", name = "WaterBaseV2")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct WaterBaseV2 {
    #[rename(name = "sharedMaterial")]
    pub shared_material: crate::unity_engine::material::Material,
    #[rename(name = "edgeBlend")]
    pub edge_blend: bool,
    #[rename(name = "reflection")]
    pub reflection: bool,
}

#[cfg(feature = "app-water-waterbasev2")]
#[::unity2::methods]
impl WaterBaseV2 {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-water-waterbasev2")]
impl WaterBaseV2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(WaterBaseV2),
                ::core::stringify!(new),
            )
        });
        <Self as IWaterBaseV2Methods>::ctor(this);
        this
    }
}
