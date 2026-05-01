
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonrideexplode/DragonRideExplode.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideExplode")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct DragonRideExplode {
    #[rename(name = "m_Type")]
    pub m_type: i32,
}

#[cfg(feature = "app-dragonrideexplode")]
#[::unity2::methods]
impl DragonRideExplode {
    #[method(name = "SetExplodeStart", args = 1)]
    pub fn set_explode_start(self, r#type: i32) -> ();

    #[method(name = "OnParticleSystemStopped", args = 0)]
    pub fn on_particle_system_stopped(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dragonrideexplode")]
impl DragonRideExplode {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideExplode),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideExplodeMethods>::ctor(this);
        this
    }
}
