
use crate::combat::basecombatlocation::BaseCombatLocation;
use crate::combat::basecombatlocation::IBaseCombatLocation;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combatlocationlastboss/CombatLocationLastBoss.md")))]
#[::unity2::class(namespace = "Combat", name = "CombatLocationLastBoss")]
#[parent(crate::combat::basecombatlocation::BaseCombatLocation)]
pub struct CombatLocationLastBoss {
    #[rename(name = "m_IsCrossCut")]
    pub m_is_cross_cut: bool,
    #[rename(name = "m_BossCenter")]
    pub m_boss_center: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_BossSize")]
    pub m_boss_size: f32,
    #[rename(name = "m_BattleDist")]
    pub m_battle_dist: f32,
}

#[cfg(feature = "combat-combatlocationlastboss")]
#[::unity2::methods]
impl CombatLocationLastBoss {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, record: crate::combat::combatrecord::CombatRecord) -> ();

    #[method(name = "get_RoughPosCount", args = 0)]
    pub fn get_rough_pos_count(self) -> i32;

    #[method(name = "SetRoughPos", args = 1)]
    pub fn set_rough_pos(self, try_count: i32) -> ();

    #[method(name = "SetRoughPosCrosscut", args = 1)]
    pub fn set_rough_pos_crosscut(self, try_count: i32) -> ();

    #[method(name = "SetRoughPosStandard", args = 1)]
    pub fn set_rough_pos_standard(self, try_count: i32) -> ();

    #[method(name = "get_PatternCount", args = 0)]
    pub fn get_pattern_count(self) -> i32;

    #[method(name = "SetBattlePatern", args = 1)]
    pub fn set_battle_patern(self, pattern: i32) -> ();

    #[method(name = "CalcLocation", args = 0)]
    pub fn calc_location(self) -> ();
}

#[cfg(feature = "combat-combatlocationlastboss")]
impl CombatLocationLastBoss {
    pub fn new(record: crate::combat::combatrecord::CombatRecord) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CombatLocationLastBoss),
                ::core::stringify!(new),
            )
        });
        <Self as ICombatLocationLastBossMethods>::ctor(this, record);
        this
    }
}
