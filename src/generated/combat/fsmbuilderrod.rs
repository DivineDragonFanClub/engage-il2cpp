
use crate::combat::fsmbuilder::FSMBuilder;
use crate::combat::fsmbuilder::IFSMBuilder;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/fsmbuilderrod/FSMBuilderRod.md")))]
#[::unity2::class(namespace = "Combat", name = "FSMBuilderRod")]
#[parent(crate::combat::fsmbuilder::FSMBuilder)]
pub struct FSMBuilderRod {}

#[cfg(feature = "combat-fsmbuilderrod")]
#[::unity2::methods]
impl FSMBuilderRod {
    #[method(name = "Build", args = 0)]
    pub fn build(self) -> ();

    #[method(name = "BuildSkipover", args = 0)]
    pub fn build_skipover(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-fsmbuilderrod")]
impl FSMBuilderRod {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FSMBuilderRod),
                ::core::stringify!(new),
            )
        });
        <Self as IFSMBuilderRodMethods>::ctor(this);
        this
    }
}
