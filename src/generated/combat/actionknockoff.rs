
use crate::combat::actionbase::ActionBase;
use crate::combat::actionbase::IActionBase;
use crate::combat::actiondisposerholder::ActionDisposerHolder;
use crate::combat::actiondisposerholder::IActionDisposerHolder;
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actionknockoff/ActionKnockoff.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionKnockoff")]
#[parent(crate::combat::actiondisposerholder::ActionDisposerHolder)]
pub struct ActionKnockoff {
    #[rename(name = "played")]
    pub played: bool,
}

#[cfg(feature = "combat-actionknockoff")]
#[::unity2::methods]
impl ActionKnockoff {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        chr: crate::combat::character::Character,
        phase: crate::combat::phase::Phase,
        world_hit_time: f32,
    ) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();
}

#[cfg(feature = "combat-actionknockoff")]
impl ActionKnockoff {
    pub fn new(
        chr: crate::combat::character::Character,
        phase: crate::combat::phase::Phase,
        world_hit_time: f32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionKnockoff),
                ::core::stringify!(new),
            )
        });
        <Self as IActionKnockoffMethods>::ctor(this, chr, phase, world_hit_time);
        this
    }
}
