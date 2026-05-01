
use crate::combat::fsmbuilder::FSMBuilder;
use crate::combat::fsmbuilder::IFSMBuilder;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/fsmbuilderjobintro/FSMBuilderJobIntro.md")))]
#[::unity2::class(namespace = "Combat", name = "FSMBuilderJobIntro")]
#[parent(crate::combat::fsmbuilder::FSMBuilder)]
pub struct FSMBuilderJobIntro {}

#[cfg(feature = "combat-fsmbuilderjobintro")]
#[::unity2::methods]
impl FSMBuilderJobIntro {
    #[method(name = "Build", args = 0)]
    pub fn build(self) -> ();

    #[method(name = "AddAttack", args = 1)]
    pub fn add_attack(self, p: crate::combat::phase::Phase) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-fsmbuilderjobintro")]
impl FSMBuilderJobIntro {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FSMBuilderJobIntro),
                ::core::stringify!(new),
            )
        });
        <Self as IFSMBuilderJobIntroMethods>::ctor(this);
        this
    }
}
