
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/combatsequence/CombatSequence.md")))]
#[::unity2::class(namespace = "Combat", name = "CombatSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: combat :: combatsequence :: CombatSequence >)]
pub struct CombatSequence {
    #[rename(name = "m_MapCamera")]
    pub m_map_camera: crate::unity_engine::camera::Camera,
}

#[cfg(feature = "combat-combatsequence")]
#[::unity2::methods]
impl CombatSequence {
    #[method(name = "get_Calculator", args = 0)]
    pub fn get_calculator(self) -> crate::app::battlecalculator::BattleCalculator;

    #[method(name = "set_Calculator", args = 1)]
    pub fn set_calculator(self, value: crate::app::battlecalculator::BattleCalculator) -> ();

    #[method(name = "get_SimCalculator", args = 0)]
    pub fn get_sim_calculator(self) -> crate::app::battlecalculator::BattleCalculator;

    #[method(name = "set_SimCalculator", args = 1)]
    pub fn set_sim_calculator(self, value: crate::app::battlecalculator::BattleCalculator) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        calculator: crate::app::battlecalculator::BattleCalculator,
        sim_calculator: crate::app::battlecalculator::BattleCalculator,
        callback: crate::app::procvoidmethod::ProcVoidMethod,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        calculator: crate::app::battlecalculator::BattleCalculator,
        sim_calculator: crate::app::battlecalculator::BattleCalculator,
    ) -> ();

    #[method(name = "TimerStart", args = 0)]
    pub fn timer_start(self) -> ();

    #[method(name = "TimerStop", args = 0)]
    pub fn timer_stop(self) -> ();

    #[method(name = "CreateBorder", args = 0)]
    pub fn create_border(self) -> ();

    #[method(name = "DeleteBorder", args = 0)]
    pub fn delete_border(self) -> ();

    #[method(name = "LoadVoice", args = 0)]
    pub fn load_voice(self) -> ();

    #[method(name = "LoadVoiceImpl", args = 1)]
    pub fn load_voice_impl(self, battle_side_type: crate::app::battleside::BattleSide_Type) -> ();

    #[method(name = "UnloadVoice", args = 0)]
    pub fn unload_voice(self) -> ();

    #[method(name = "UnloadVoiceImpl", args = 1)]
    pub fn unload_voice_impl(self, battle_side_type: crate::app::battleside::BattleSide_Type)
        -> ();

    #[method(name = "LoadScene", args = 0)]
    pub fn load_scene(self) -> ();

    #[method(name = "WaitLoading", args = 0)]
    pub fn wait_loading(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "UnloadScene", args = 0)]
    pub fn unload_scene(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnPersistent", args = 0)]
    pub fn on_persistent(self) -> ();

    #[method(name = "BoundToCombatCamera", args = 0)]
    pub fn bound_to_combat_camera(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ReturnToMapCamera", args = 0)]
    pub fn return_to_map_camera(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ToPreBgm", args = 0)]
    pub fn to_pre_bgm(self) -> ();

    #[method(name = "ToMainBgm", args = 0)]
    pub fn to_main_bgm(self) -> ();

    #[method(name = "ReturnBgm", args = 0)]
    pub fn return_bgm(self) -> ();

    #[method(name = "IsComebackWithTransitionCamera", args = 0)]
    pub fn is_comeback_with_transition_camera(self) -> bool;

    #[method(name = "DieTalk", args = 0)]
    pub fn die_talk(self) -> ();

    #[method(name = "ShowGrowth", args = 0)]
    pub fn show_growth(self) -> ();

    #[method(name = "MoveCursor", args = 0)]
    pub fn move_cursor(self) -> ();

    #[method(name = "BackToGround", args = 0)]
    pub fn back_to_ground(self) -> ();

    #[method(name = "WaitForBackToGround", args = 0)]
    pub fn wait_for_back_to_ground(self) -> ();

    #[method(name = "BackToGroundFadeIn", args = 0)]
    pub fn back_to_ground_fade_in(self) -> ();
}

#[cfg(feature = "combat-combatsequence")]
impl CombatSequence {
    pub fn new(
        calculator: crate::app::battlecalculator::BattleCalculator,
        sim_calculator: crate::app::battlecalculator::BattleCalculator,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CombatSequence),
                ::core::stringify!(new),
            )
        });
        <Self as ICombatSequenceMethods>::ctor(this, calculator, sim_calculator);
        this
    }
}
