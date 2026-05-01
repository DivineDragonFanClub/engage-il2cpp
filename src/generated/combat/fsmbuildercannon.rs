
use crate::combat::fsmbuilder::FSMBuilder;
use crate::combat::fsmbuilder::IFSMBuilder;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/fsmbuildercannon/FSMBuilderCannon.md")))]
#[::unity2::class(namespace = "Combat", name = "FSMBuilderCannon")]
#[parent(crate::combat::fsmbuilder::FSMBuilder)]
pub struct FSMBuilderCannon {
    #[rename(name = "m_Cannon")]
    pub m_cannon: crate::app::mapobject::MapObject,
}

#[cfg(feature = "combat-fsmbuildercannon")]
#[::unity2::methods]
impl FSMBuilderCannon {
    #[method(name = "Build", args = 0)]
    pub fn build(self) -> ();

    #[method(name = "Rotate", args = 0)]
    pub fn rotate() -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "BuildSkipover", args = 0)]
    pub fn build_skipover(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-fsmbuildercannon")]
impl FSMBuilderCannon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FSMBuilderCannon),
                ::core::stringify!(new),
            )
        });
        <Self as IFSMBuilderCannonMethods>::ctor(this);
        this
    }
}
