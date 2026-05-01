
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/finishstyleex/FinishStyleEx.md")))]
#[::unity2::class(namespace = "Combat", name = "FinishStyleEx")]
#[parent(crate::system::object::Object)]
pub struct FinishStyleEx {}

#[cfg(feature = "combat-finishstyleex")]
#[::unity2::methods]
impl FinishStyleEx {
    #[method(name = "IsEnemyGroupKilled", args = 1)]
    pub fn is_enemy_group_killed(s: crate::combat::finishstyle::FinishStyle) -> bool;

    #[method(name = "IsSlow", args = 1)]
    pub fn is_slow(s: crate::combat::finishstyle::FinishStyle) -> bool;
}
