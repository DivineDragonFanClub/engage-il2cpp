
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/arenacombatsequence/ArenaCombatSequence.md")))]
#[::unity2::class(namespace = "Combat", name = "ArenaCombatSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: combat :: arenacombatsequence :: ArenaCombatSequence >)]
pub struct ArenaCombatSequence {
# [rename (name = "StartTelopPath")] pub start_telop_path : :: unity2 :: Il2CppString ,
# [rename (name = "m_RenderCamera")] pub m_render_camera : crate :: unity_engine :: camera :: Camera ,
# [rename (name = "m_RenderCameraData")] pub m_render_camera_data : crate :: unity_engine :: rendering :: universal :: universaladditionalcameradata :: UniversalAdditionalCameraData ,
# [rename (name = "RenderCameraIndex_CharaMap")] pub render_camera_index_chara_map : i32 ,
# [rename (name = "RenderCameraIndex_Chara")] pub render_camera_index_chara : i32 ,
# [rename (name = "m_RelianceController")] pub m_reliance_controller : crate :: app :: reliancepopupcontroller :: ReliancePopUpController ,
# [rename (name = "m_BondController")] pub m_bond_controller : crate :: app :: bondpopupcontroller :: BondPopUpController ,
}

#[cfg(feature = "combat-arenacombatsequence")]
#[::unity2::methods]
impl ArenaCombatSequence {
    #[method(name = "get_ResultTelopPath", args = 0)]
    pub fn get_result_telop_path(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Calculator", args = 0)]
    pub fn get_calculator(self) -> crate::app::battlecalculator::BattleCalculator;

    #[method(name = "get_SimCalculator", args = 0)]
    pub fn get_sim_calculator(self) -> crate::app::battlecalculator::BattleCalculator;

    #[method(name = "get_BondExp", args = 0)]
    pub fn get_bond_exp(self) -> i32;

    #[method(name = "get_IsEmblemBattle", args = 0)]
    pub fn get_is_emblem_battle(self) -> bool;

    #[method(name = "get_IsSpecialBattle", args = 0)]
    pub fn get_is_special_battle(self) -> bool;

    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        calculator: crate::app::battlecalculator::BattleCalculator,
        sim_calculator: crate::app::battlecalculator::BattleCalculator,
        is_emblem_battle: bool,
        is_special: bool,
        bond_exp: i32,
    ) -> ();

    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        calculator: crate::app::battlecalculator::BattleCalculator,
        sim_calculator: crate::app::battlecalculator::BattleCalculator,
        is_emblem_battle: bool,
        is_special: bool,
        bond_exp: i32,
    ) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnPersistent", args = 0)]
    pub fn on_persistent(self) -> ();

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "WaitBegin", args = 0)]
    pub fn wait_begin(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "StartFight", args = 0)]
    pub fn start_fight(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "WaitFinish", args = 0)]
    pub fn wait_finish(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Result", args = 0)]
    pub fn result(self) -> ();

    #[method(name = "Grow1", args = 0)]
    pub fn grow1(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Grow2", args = 0)]
    pub fn grow2(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Grow3", args = 0)]
    pub fn grow3(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> crate::system::collections::ienumerator::IEnumerator;
}

#[cfg(feature = "combat-arenacombatsequence")]
impl ArenaCombatSequence {
    pub fn new(
        calculator: crate::app::battlecalculator::BattleCalculator,
        sim_calculator: crate::app::battlecalculator::BattleCalculator,
        is_emblem_battle: bool,
        is_special: bool,
        bond_exp: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ArenaCombatSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IArenaCombatSequenceMethods>::ctor(
            this,
            calculator,
            sim_calculator,
            is_emblem_battle,
            is_special,
            bond_exp,
        );
        this
    }
}
