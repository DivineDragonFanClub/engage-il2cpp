
use crate::combat::decorator::Decorator;
use crate::combat::decorator::IDecorator;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/deco_rod/Deco_Rod.md")))]
#[::unity2::class(namespace = "Combat", name = "Deco_Rod")]
#[parent(crate::combat::decorator::Decorator)]
pub struct Deco_Rod {}

#[cfg(feature = "combat-deco_rod")]
#[::unity2::methods]
impl Deco_Rod {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-deco_rod")]
impl Deco_Rod {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Deco_Rod),
                ::core::stringify!(new),
            )
        });
        <Self as IDeco_RodMethods>::ctor(this);
        this
    }
}
