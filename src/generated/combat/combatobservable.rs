
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combatobservable/CombatObservable.md")))]
#[::unity2::class(namespace = "Combat", name = "CombatObservable")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct CombatObservable {}

#[cfg(feature = "combat-combatobservable")]
#[::unity2::methods]
impl CombatObservable {
    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-combatobservable")]
impl CombatObservable {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CombatObservable),
                ::core::stringify!(new),
            )
        });
        <Self as ICombatObservableMethods>::ctor(this);
        this
    }
}
