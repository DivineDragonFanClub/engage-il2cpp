
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/phasearray/PhaseArray.md")))]
#[::unity2::class(namespace = "Combat", name = "PhaseArray")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: combat :: phase :: Phase >)]
pub struct PhaseArray {
    #[rename(name = "m_CurrentIndex")]
    pub m_current_index: i32,
}

#[cfg(feature = "combat-phasearray")]
#[::unity2::methods]
impl PhaseArray {
    #[method(name = "get_Current", args = 0)]
    pub fn get_current(self) -> crate::combat::phase::Phase;

    #[method(name = "get_Last", args = 0)]
    pub fn get_last(self) -> crate::combat::phase::Phase;

    #[method(name = "get_AttackRight", args = 0)]
    pub fn get_attack_right(self) -> i32;

    #[method(name = "set_AttackRight", args = 1)]
    pub fn set_attack_right(self, value: i32) -> ();

    #[method(name = "get_HasCounter", args = 0)]
    pub fn get_has_counter(self) -> bool;

    #[method(name = "Add", args = 1)]
    pub fn add(self, phase: crate::combat::phase::Phase) -> ();

    #[method(name = "Next", args = 0)]
    pub fn next(self) -> ();

    #[method(name = "SetCurrentToLast", args = 0)]
    pub fn set_current_to_last(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "UpdateAttackRight", args = 0)]
    pub fn update_attack_right(self) -> ();

    #[method(name = "PostProcess", args = 4)]
    pub fn post_process(
        self,
        style: crate::combat::combatstyle::CombatStyle,
        game_status: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
        gs_chain_atk: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
        gs_dragonic: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
    ) -> ();

    #[method(name = "MarkFirstAndLastAttack", args = 0)]
    pub fn mark_first_and_last_attack(self) -> ();

    #[method(name = "FindDetailFirst", args = 1)]
    pub fn find_detail_first(
        self,
        detail: crate::combat::phase::Phase_Detail,
    ) -> crate::combat::phase::Phase;

    #[method(name = "DecideAnimationHash", args = 3)]
    pub fn decide_animation_hash(
        self,
        style: crate::combat::combatstyle::CombatStyle,
        game_status: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
        gs_chain_atk: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
    ) -> ();

    #[method(name = "MarkUseAnims", args = 3)]
    pub fn mark_use_anims(
        self,
        game_status: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
        gs_chain_atk: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
        gs_dragonic: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
    ) -> ();

    #[method(name = "RegisterPreloadAnim", args = 2)]
    pub fn register_preload_anim(
        r#use: crate::combat::preloadanims::PreloadAnims,
        gs: crate::combat::charactergamestatus::CharacterGameStatus,
    ) -> ();

    #[method(name = "GetBrokenSide", args = 0)]
    pub fn get_broken_side(self) -> i32;
}

#[cfg(feature = "combat-phasearray")]
impl PhaseArray {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PhaseArray),
                ::core::stringify!(new),
            )
        });
        <Self as IPhaseArrayMethods>::ctor(this);
        this
    }
}
