
use crate::combat::decorator::Decorator;
use crate::combat::decorator::IDecorator;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/deco_skill/Deco_Skill.md")))]
#[::unity2::class(namespace = "Combat", name = "Deco_Skill")]
#[parent(crate::combat::decorator::Decorator)]
pub struct Deco_Skill {
    #[rename(name = "m_Pair")]
    pub m_pair: crate::combat::skillstack::SkillStack_Packet,
    #[rename(name = "m_bNameShown")]
    pub m_b_name_shown: bool,
}

#[cfg(feature = "combat-deco_skill")]
#[::unity2::methods]
impl Deco_Skill {
    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Pair", args = 1)]
    pub fn set_pair(self, value: crate::combat::skillstack::SkillStack_Packet) -> ();

    #[method(name = "IsAvailable", args = 2)]
    pub fn is_available(
        that: crate::combat::decoratorargs::DecoratorArgs,
        pair: crate::combat::skillstack::SkillStack_Packet,
    ) -> bool;

    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnEnterAttack", args = 0)]
    pub fn on_enter_attack(self) -> ();

    #[method(name = "OnEnemyDamage_", args = 1)]
    pub fn on_enemy_damage(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "DrawSkillDamages", args = 2)]
    pub fn draw_skill_damages(
        phase: crate::combat::phase::Phase,
        skills: crate::combat::skillstack::SkillStack,
    ) -> ();

    #[method(name = "IsSubspace", args = 1)]
    pub fn is_subspace(c: crate::combat::character::Character) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-deco_skill")]
impl Deco_Skill {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Deco_Skill),
                ::core::stringify!(new),
            )
        });
        <Self as IDeco_SkillMethods>::ctor(this);
        this
    }
}
