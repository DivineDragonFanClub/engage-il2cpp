
use crate::combat::basecombatlocation::BaseCombatLocation;
use crate::combat::basecombatlocation::IBaseCombatLocation;
use crate::combat::combatlocationcrosscut::CombatLocationCrosscut;
use crate::combat::combatlocationcrosscut::ICombatLocationCrosscut;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combatlocationengageattack/CombatLocationEngageAttack.md")))]
#[::unity2::class(namespace = "Combat", name = "CombatLocationEngageAttack")]
#[parent(crate::combat::combatlocationcrosscut::CombatLocationCrosscut)]
pub struct CombatLocationEngageAttack {}

#[cfg(feature = "combat-combatlocationengageattack")]
#[::unity2::methods]
impl CombatLocationEngageAttack {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, record: crate::combat::combatrecord::CombatRecord) -> ();

    #[method(name = "SetRoughPos", args = 1)]
    pub fn set_rough_pos(self, try_count: i32) -> ();

    #[method(name = "SetBattlePatern", args = 1)]
    pub fn set_battle_patern(self, pattern: i32) -> ();

    #[method(name = "CalcLocation", args = 0)]
    pub fn calc_location(self) -> ();
}

#[cfg(feature = "combat-combatlocationengageattack")]
impl CombatLocationEngageAttack {
    pub fn new(record: crate::combat::combatrecord::CombatRecord) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CombatLocationEngageAttack),
                ::core::stringify!(new),
            )
        });
        <Self as ICombatLocationEngageAttackMethods>::ctor(this, record);
        this
    }
}
