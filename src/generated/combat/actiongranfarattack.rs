
use crate::combat::actionbase::ActionBase;
use crate::combat::actionbase::IActionBase;
use crate::combat::actiondisposerholder::ActionDisposerHolder;
use crate::combat::actiondisposerholder::IActionDisposerHolder;
use crate::combat::actiongranbase::ActionGranBase;
use crate::combat::actiongranbase::IActionGranBase;
use crate::combat::state::IState;
use crate::combat::state::State;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/actiongranfarattack/ActionGranFarAttack.md")))]
#[::unity2::class(namespace = "Combat", name = "ActionGranFarAttack")]
#[parent(crate::combat::actiongranbase::ActionGranBase)]
pub struct ActionGranFarAttack {
    #[rename(name = "m_WorldHitTime")]
    pub m_world_hit_time: f32,
}

#[cfg(feature = "combat-actiongranfarattack")]
#[::unity2::methods]
impl ActionGranFarAttack {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, chr: crate::combat::character::Character, world_hit_time: f32) -> ();

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();
}

#[cfg(feature = "combat-actiongranfarattack")]
impl ActionGranFarAttack {
    pub fn new(chr: crate::combat::character::Character, world_hit_time: f32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ActionGranFarAttack),
                ::core::stringify!(new),
            )
        });
        <Self as IActionGranFarAttackMethods>::ctor(this, chr, world_hit_time);
        this
    }
}
