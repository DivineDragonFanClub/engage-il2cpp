
use crate::app::water::displacev2::DisplaceV2;
use crate::app::water::displacev2::IDisplaceV2;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/water/gerstnerdisplacev2/GerstnerDisplaceV2.md")))]
#[::unity2::class(namespace = "App.Water", name = "GerstnerDisplaceV2")]
#[parent(crate::app::water::displacev2::DisplaceV2)]
pub struct GerstnerDisplaceV2 {}

#[cfg(feature = "app-water-gerstnerdisplacev2")]
#[::unity2::methods]
impl GerstnerDisplaceV2 {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-water-gerstnerdisplacev2")]
impl GerstnerDisplaceV2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GerstnerDisplaceV2),
                ::core::stringify!(new),
            )
        });
        <Self as IGerstnerDisplaceV2Methods>::ctor(this);
        this
    }
}
