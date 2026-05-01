
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combatstaging/CombatStaging.md")))]
#[::unity2::class(namespace = "Combat", name = "CombatStaging")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CombatStaging {}

#[cfg(feature = "combat-combatstaging")]
#[::unity2::methods]
impl CombatStaging {
    #[method(name = "Attach", args = 2)]
    pub fn attach(self, side: i32, chr: crate::combat::character::Character) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-combatstaging")]
impl CombatStaging {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CombatStaging),
                ::core::stringify!(new),
            )
        });
        <Self as ICombatStagingMethods>::ctor(this);
        this
    }
}
