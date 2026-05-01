
use crate::combat::fsmbuilder::FSMBuilder;
use crate::combat::fsmbuilder::IFSMBuilder;
use crate::combat::fsmbuilderstandard::FSMBuilderStandard;
use crate::combat::fsmbuilderstandard::IFSMBuilderStandard;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/fsmbuilderengattack/FSMBuilderEngAttack.md")))]
#[::unity2::class(namespace = "Combat", name = "FSMBuilderEngAttack")]
#[parent(crate::combat::fsmbuilderstandard::FSMBuilderStandard)]
pub struct FSMBuilderEngAttack {}

#[cfg(feature = "combat-fsmbuilderengattack")]
#[::unity2::methods]
impl FSMBuilderEngAttack {
    #[method(name = "Build", args = 0)]
    pub fn build(self) -> ();

    #[method(name = "BuildStart", args = 0)]
    pub fn build_start(self) -> ();

    #[method(name = "BuildMain", args = 0)]
    pub fn build_main(self) -> ();

    #[method(name = "BuildAllForOne", args = 0)]
    pub fn build_all_for_one(self) -> ();

    #[method(name = "BuildEnd", args = 0)]
    pub fn build_end(self) -> ();

    #[method(name = "MakeLastSituation", args = 0)]
    pub fn make_last_situation() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-fsmbuilderengattack")]
impl FSMBuilderEngAttack {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FSMBuilderEngAttack),
                ::core::stringify!(new),
            )
        });
        <Self as IFSMBuilderEngAttackMethods>::ctor(this);
        this
    }
}
