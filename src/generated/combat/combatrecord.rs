
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combatrecord/CombatRecord.md")))]
#[::unity2::class(namespace = "Combat", name = "CombatRecord")]
#[parent(crate::system::object::Object)]
pub struct CombatRecord {
    #[rename(name = "finishStyle")]
    pub finish_style: crate::combat::finishstyle::FinishStyle,
}

#[cfg(feature = "combat-combatrecord")]
#[::unity2::methods]
impl CombatRecord {
    #[method(name = "op_Implicit", args = 1)]
    pub fn op_implicit(a: crate::combat::combatrecord::CombatRecord) -> bool;

    #[method(name = "get_IsEnemyAttack", args = 0)]
    pub fn get_is_enemy_attack(self) -> bool;

    #[method(name = "set_IsEnemyAttack", args = 1)]
    pub fn set_is_enemy_attack(self, value: bool) -> ();

    #[method(name = "get_CombatStyle", args = 0)]
    pub fn get_combat_style(self) -> crate::combat::combatstyle::CombatStyle;

    #[method(name = "set_CombatStyle", args = 1)]
    pub fn set_combat_style(self, value: crate::combat::combatstyle::CombatStyle) -> ();

    #[method(name = "get_Calculator", args = 0)]
    pub fn get_calculator(self) -> crate::app::battlecalculator::BattleCalculator;

    #[method(name = "set_Calculator", args = 1)]
    pub fn set_calculator(self, value: crate::app::battlecalculator::BattleCalculator) -> ();

    #[method(name = "get_SimCalculator", args = 0)]
    pub fn get_sim_calculator(self) -> crate::app::battlecalculator::BattleCalculator;

    #[method(name = "set_SimCalculator", args = 1)]
    pub fn set_sim_calculator(self, value: crate::app::battlecalculator::BattleCalculator) -> ();

    #[method(name = "get_GameStatus", args = 0)]
    pub fn get_game_status(
        self,
    ) -> ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>;

    #[method(name = "set_GameStatus", args = 1)]
    pub fn set_game_status(
        self,
        value: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
    ) -> ();

    #[method(name = "get_GameStatusChainAtk", args = 0)]
    pub fn get_game_status_chain_atk(
        self,
    ) -> ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>;

    #[method(name = "set_GameStatusChainAtk", args = 1)]
    pub fn set_game_status_chain_atk(
        self,
        value: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
    ) -> ();

    #[method(name = "get_GameStatusDragonize", args = 0)]
    pub fn get_game_status_dragonize(
        self,
    ) -> ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>;

    #[method(name = "set_GameStatusDragonize", args = 1)]
    pub fn set_game_status_dragonize(
        self,
        value: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
    ) -> ();

    #[method(name = "get_Location", args = 0)]
    pub fn get_location(self) -> crate::combat::basecombatlocation::BaseCombatLocation;

    #[method(name = "set_Location", args = 1)]
    pub fn set_location(self, value: crate::combat::basecombatlocation::BaseCombatLocation) -> ();

    #[method(name = "get_PassiveSkills", args = 0)]
    pub fn get_passive_skills(self) -> crate::combat::skillstack::SkillStack;

    #[method(name = "set_PassiveSkills", args = 1)]
    pub fn set_passive_skills(self, value: crate::combat::skillstack::SkillStack) -> ();

    #[method(name = "get_PhaseArray", args = 0)]
    pub fn get_phase_array(self) -> crate::combat::phasearray::PhaseArray;

    #[method(name = "set_PhaseArray", args = 1)]
    pub fn set_phase_array(self, value: crate::combat::phasearray::PhaseArray) -> ();

    #[method(name = "get_CurrentPhase", args = 0)]
    pub fn get_current_phase(self) -> crate::combat::phase::Phase;

    #[method(name = "get_LastPhase", args = 0)]
    pub fn get_last_phase(self) -> crate::combat::phase::Phase;

    #[method(name = "get_AttackRight", args = 0)]
    pub fn get_attack_right(self) -> i32;

    #[method(name = "get_HasCounter", args = 0)]
    pub fn get_has_counter(self) -> bool;

    #[method(name = "UpdateAttackRight", args = 0)]
    pub fn update_attack_right(self) -> ();

    #[method(name = "get_MapDistance", args = 0)]
    pub fn get_map_distance(self) -> i32;

    #[method(name = "set_MapDistance", args = 1)]
    pub fn set_map_distance(self, value: i32) -> ();

    #[method(name = "get_MapDistance1or2", args = 0)]
    pub fn get_map_distance1or2(self) -> i32;

    #[method(name = "get_ChainAttackCount", args = 0)]
    pub fn get_chain_attack_count(self) -> i32;

    #[method(name = "set_ChainAttackCount", args = 1)]
    pub fn set_chain_attack_count(self, value: i32) -> ();

    #[method(name = "get_FinishStyle", args = 0)]
    pub fn get_finish_style(self) -> crate::combat::finishstyle::FinishStyle;

    #[method(name = "get_IsChainAttack", args = 0)]
    pub fn get_is_chain_attack(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "CalcFinishStyle", args = 0)]
    pub fn calc_finish_style(self) -> crate::combat::finishstyle::FinishStyle;

    #[method(name = "PostProcess", args = 0)]
    pub fn post_process(self) -> ();

    #[method(name = "PreloadPassiveSkillsEffects", args = 0)]
    pub fn preload_passive_skills_effects(self) -> ();

    #[method(name = "MakeDecorators", args = 0)]
    pub fn make_decorators(self) -> ();

    #[method(name = "FixForSelfHeal", args = 0)]
    pub fn fix_for_self_heal(self) -> ();

    #[method(name = "FixForDance", args = 0)]
    pub fn fix_for_dance(self) -> ();

    #[method(name = "スターラッシュ", args = 0)]
    pub fn _unnamed(self) -> ();

    #[method(name = "ImportFromGame", args = 2)]
    pub fn import_from_game(
        self,
        calc: crate::app::battlecalculator::BattleCalculator,
        sim_calc: crate::app::battlecalculator::BattleCalculator,
    ) -> ();

    #[method(name = "ImportOnestepParameters", args = 5)]
    pub fn import_onestep_parameters(
        calc: crate::app::battlecalculator::BattleCalculator,
        src: crate::app::battlescene::BattleScene,
        phase: crate::combat::phase::Phase,
        s0: i32,
        s1: i32,
    ) -> ();

    #[method(name = "GetDistance", args = 1)]
    pub fn get_distance(calc: crate::app::battlecalculator::BattleCalculator) -> i32;

    #[method(name = "ImportMasterUnitsFromGame", args = 1)]
    pub fn import_master_units_from_game(
        self,
        calc: crate::app::battlecalculator::BattleCalculator,
    ) -> ();

    #[method(name = "ImportGrandewFromGame", args = 1)]
    pub fn import_grandew_from_game(
        self,
        calc: crate::app::battlecalculator::BattleCalculator,
    ) -> ();

    #[method(name = "ImportChainUnitsFromGame", args = 1)]
    pub fn import_chain_units_from_game(
        self,
        calc: crate::app::battlecalculator::BattleCalculator,
    ) -> ();

    #[method(name = "CreateForJobIntro", args = 1)]
    pub fn create_for_job_intro(self, data: crate::app::jobintrodata::JobIntroData) -> ();

    #[method(name = "CreatePhaseArrayForJobIntro", args = 2)]
    pub fn create_phase_array_for_job_intro(
        self,
        data: crate::app::jobintrodata::JobIntroData,
        is_rod: bool,
    ) -> ();

    #[method(name = "ImportFromViewer", args = 1)]
    pub fn import_from_viewer(self, viewer: crate::combat::combatviewer::CombatViewer) -> ();
}

#[cfg(feature = "combat-combatrecord")]
impl CombatRecord {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CombatRecord),
                ::core::stringify!(new),
            )
        });
        <Self as ICombatRecordMethods>::ctor(this);
        this
    }
}
