
use crate::combat::fsmbuilder::FSMBuilder;
use crate::combat::fsmbuilder::IFSMBuilder;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/fsmbuilderstandard/FSMBuilderStandard.md")))]
#[::unity2::class(namespace = "Combat", name = "FSMBuilderStandard")]
#[parent(crate::combat::fsmbuilder::FSMBuilder)]
pub struct FSMBuilderStandard {
    #[rename(name = "chainSync")]
    pub chain_sync: ::unity2::Array<crate::combat::synctoken::SyncToken>,
    #[static_field]
    #[rename(name = "ChainAttackTime")]
    pub chain_attack_time: f32,
}

#[cfg(feature = "combat-fsmbuilderstandard")]
#[::unity2::methods]
impl FSMBuilderStandard {
    #[method(name = "BuildEnd", args = 0)]
    pub fn build_end(self) -> ();

    #[method(name = "BuildEnd_BothAlive", args = 0)]
    pub fn build_end_both_alive() -> ();

    #[method(name = "BuildEnd_PlayerKilled", args = 0)]
    pub fn build_end_player_killed() -> ();

    #[method(name = "BuildEnd_EnemyBossKilled", args = 0)]
    pub fn build_end_enemy_boss_killed() -> ();

    #[method(name = "BuildEnd_EnemyKilled", args = 0)]
    pub fn build_end_enemy_killed() -> ();

    #[method(name = "BuildMain", args = 0)]
    pub fn build_main(self) -> ();

    #[method(name = "BuildStandardPhase", args = 1)]
    pub fn build_standard_phase(phase: crate::combat::phase::Phase) -> ();

    #[method(name = "BuildStart", args = 0)]
    pub fn build_start(self) -> ();

    #[method(name = "Start_Training", args = 0)]
    pub fn start_training() -> ();

    #[method(name = "Start_Talk", args = 0)]
    pub fn start_talk() -> ();

    #[method(name = "Start_Default", args = 0)]
    pub fn start_default() -> ();

    #[method(name = "AppendStartAnim", args = 2)]
    pub fn append_start_anim(run_start: bool, start_time: f32) -> ();

    #[method(name = "AppendDragonStone", args = 0)]
    pub fn append_dragon_stone() -> bool;

    #[method(name = "Start_WithDive", args = 0)]
    pub fn start_with_dive() -> ();

    #[method(name = "AllocChainSync", args = 0)]
    pub fn alloc_chain_sync(self) -> ();

    #[method(name = "BuildChainAttackPhase", args = 2)]
    pub fn build_chain_attack_phase(self, phase: crate::combat::phase::Phase, chain_num: i32)
        -> ();

    #[method(name = "BuildChainFSM", args = 3)]
    pub fn build_chain_fsm(
        self,
        phase: crate::combat::phase::Phase,
        chain: crate::combat::character::Character,
        chain_num: i32,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-fsmbuilderstandard")]
impl FSMBuilderStandard {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FSMBuilderStandard),
                ::core::stringify!(new),
            )
        });
        <Self as IFSMBuilderStandardMethods>::ctor(this);
        this
    }
}
