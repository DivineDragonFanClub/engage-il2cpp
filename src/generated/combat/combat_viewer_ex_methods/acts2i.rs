
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combat_viewer_ex_methods/acts2i/ActS2I.md")))]
#[::unity2::class(namespace = "Combat.CombatViewerExMethods", name = "ActS2I")]
#[parent(crate::system::object::Object)]
pub struct ActS2I {
    #[static_field]
    #[rename(name = "items")]
    pub items: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "hashes")]
    pub hashes: ::unity2::Array<i32>,
}

#[cfg(feature = "combat-combat_viewer_ex_methods-acts2i")]
#[::unity2::methods]
impl ActS2I {
    #[method(name = "get_Popup", args = 0)]
    pub fn get_popup() -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "ToTableIndex", args = 1)]
    pub fn to_table_index(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "ToName", args = 1)]
    pub fn to_name(i: i32) -> ::unity2::Il2CppString;

    #[method(name = "ToHash", args = 1)]
    pub fn to_hash(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "IsEnd", args = 1)]
    pub fn is_end(s: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsChainAttack", args = 1)]
    pub fn is_chain_attack(s: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsAttack", args = 1)]
    pub fn is_attack(s: ::unity2::Il2CppString) -> bool;

    #[method(name = "HasWild", args = 1)]
    pub fn has_wild(s: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsDamage", args = 1)]
    pub fn is_damage(s: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsEvade", args = 1)]
    pub fn is_evade(s: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsGuard", args = 1)]
    pub fn is_guard(s: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsParry", args = 1)]
    pub fn is_parry(s: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsDie", args = 1)]
    pub fn is_die(s: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsSkill", args = 1)]
    pub fn is_skill(s: ::unity2::Il2CppString) -> bool;

    #[method(name = "SkillName", args = 1)]
    pub fn skill_name(s: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "IsDamageGroup", args = 1)]
    pub fn is_damage_group(s: ::unity2::Il2CppString) -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
