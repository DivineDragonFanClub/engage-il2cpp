
use crate::combat::basecombatlocation::BaseCombatLocation;
use crate::combat::basecombatlocation::IBaseCombatLocation;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combatlocationsky/CombatLocationSky.md")))]
#[::unity2::class(namespace = "Combat", name = "CombatLocationSky")]
#[parent(crate::combat::basecombatlocation::BaseCombatLocation)]
pub struct CombatLocationSky {
    #[rename(name = "RotateTable")]
    pub rotate_table: ::unity2::Array<i32>,
}

#[cfg(feature = "combat-combatlocationsky")]
#[::unity2::methods]
impl CombatLocationSky {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, record: crate::combat::combatrecord::CombatRecord) -> ();

    #[method(name = "get_RoughPosCount", args = 0)]
    pub fn get_rough_pos_count(self) -> i32;

    #[method(name = "SetRoughPos", args = 1)]
    pub fn set_rough_pos(self, try_count: i32) -> ();

    #[method(name = "get_PatternCount", args = 0)]
    pub fn get_pattern_count(self) -> i32;

    #[method(name = "SetBattlePatern", args = 1)]
    pub fn set_battle_patern(self, pattern: i32) -> ();

    #[method(name = "CalcLocation", args = 0)]
    pub fn calc_location(self) -> ();
}

#[cfg(feature = "combat-combatlocationsky")]
impl CombatLocationSky {
    pub fn new(record: crate::combat::combatrecord::CombatRecord) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CombatLocationSky),
                ::core::stringify!(new),
            )
        });
        <Self as ICombatLocationSkyMethods>::ctor(this, record);
        this
    }
}
