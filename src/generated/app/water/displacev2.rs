
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/water/displacev2/DisplaceV2.md")))]
#[::unity2::class(namespace = "App.Water", name = "DisplaceV2")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DisplaceV2 {}

#[cfg(feature = "app-water-displacev2")]
#[::unity2::methods]
impl DisplaceV2 {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnEnable", args = 0)]
    pub fn on_enable(self) -> ();

    #[method(name = "OnDisable", args = 0)]
    pub fn on_disable(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-water-displacev2")]
impl DisplaceV2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DisplaceV2),
                ::core::stringify!(new),
            )
        });
        <Self as IDisplaceV2Methods>::ctor(this);
        this
    }
}
