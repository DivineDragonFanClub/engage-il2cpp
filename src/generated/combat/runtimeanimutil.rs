
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/runtimeanimutil/RuntimeAnimUtil.md")))]
#[::unity2::class(namespace = "Combat", name = "RuntimeAnimUtil")]
#[parent(crate::system::object::Object)]
pub struct RuntimeAnimUtil {}

#[cfg(feature = "combat-runtimeanimutil")]
#[::unity2::methods]
impl RuntimeAnimUtil {
    #[method(name = "IsAttack", args = 1)]
    pub fn is_attack(hash: i32) -> bool;

    #[method(name = "IsDamage", args = 1)]
    pub fn is_damage(hash: i32) -> bool;

    #[method(name = "IsDamageMid", args = 1)]
    pub fn is_damage_mid(hash: i32) -> bool;

    #[method(name = "IsGuard", args = 1)]
    pub fn is_guard(hash: i32) -> bool;

    #[method(name = "IsParry", args = 1)]
    pub fn is_parry(hash: i32) -> bool;

    #[method(name = "IsEvasion", args = 1)]
    pub fn is_evasion(hash: i32) -> bool;

    #[method(name = "IsDie", args = 1)]
    pub fn is_die(hash: i32) -> bool;

    #[method(name = "IsDamageSideAll", args = 1)]
    pub fn is_damage_side_all(hash: i32) -> bool;

    #[method(name = "IsIdle", args = 2)]
    pub fn is_idle(
        state: crate::unity_engine::animatorstateinfo::AnimatorStateInfo,
        backward_cancel_nt: f32,
    ) -> bool;
}
