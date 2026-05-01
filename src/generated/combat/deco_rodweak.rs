
use crate::combat::deco_rod::Deco_Rod;
use crate::combat::deco_rod::IDeco_Rod;
use crate::combat::decorator::Decorator;
use crate::combat::decorator::IDecorator;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/deco_rodweak/Deco_RodWeak.md")))]
#[::unity2::class(namespace = "Combat", name = "Deco_RodWeak")]
#[parent(crate::combat::deco_rod::Deco_Rod)]
pub struct Deco_RodWeak {}

#[cfg(feature = "combat-deco_rodweak")]
#[::unity2::methods]
impl Deco_RodWeak {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsAvailable", args = 1)]
    pub fn is_available(that: crate::combat::decoratorargs::DecoratorArgs) -> bool;

    #[method(name = "OnEnemyDamage_", args = 1)]
    pub fn on_enemy_damage(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-deco_rodweak")]
impl Deco_RodWeak {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Deco_RodWeak),
                ::core::stringify!(new),
            )
        });
        <Self as IDeco_RodWeakMethods>::ctor(this);
        this
    }
}
