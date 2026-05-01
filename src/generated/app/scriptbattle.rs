
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::scriptutil::IScriptUtil;
use crate::app::scriptutil::ScriptUtil;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/scriptbattle/ScriptBattle.md")))]
#[::unity2::class(namespace = "App", name = "ScriptBattle")]
#[parent(crate::app::scriptutil::ScriptUtil)]
pub struct ScriptBattle {}

#[cfg(feature = "app-scriptbattle")]
#[::unity2::methods]
impl ScriptBattle {
    #[method(name = "CombatStart", args = 1)]
    pub fn combat_start(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "Regist", args = 1)]
    pub fn regist(script: crate::app::eventscript::EventScript) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-scriptbattle")]
impl ScriptBattle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptBattle),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptBattleMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/scriptbattle/ScriptBattle_ScriptCombatSequence.md")))]
#[::unity2::class(namespace = "App", name = "ScriptBattle.ScriptCombatSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct ScriptBattle_ScriptCombatSequence {
    #[rename(name = "m_PersonA")]
    pub m_person_a: crate::app::persondata::PersonData,
    #[rename(name = "m_PersonB")]
    pub m_person_b: crate::app::persondata::PersonData,
    #[rename(name = "m_UnitA")]
    pub m_unit_a: crate::app::unit::Unit,
    #[rename(name = "m_UnitB")]
    pub m_unit_b: crate::app::unit::Unit,
    #[rename(name = "m_Temporary")]
    pub m_temporary: crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
    #[rename(name = "m_Info")]
    pub m_info: crate::app::battleinfo::BattleInfo,
    #[rename(name = "m_SimInfo")]
    pub m_sim_info: crate::app::battleinfo::BattleInfo,
    #[rename(name = "m_Calculator")]
    pub m_calculator: crate::app::battlecalculator::BattleCalculator,
    #[rename(name = "m_SimCalculator")]
    pub m_sim_calculator: crate::app::battlecalculator::BattleCalculator,
}

#[cfg(feature = "app-scriptbattle")]
#[::unity2::methods]
impl ScriptBattle_ScriptCombatSequence {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        person_a: crate::app::persondata::PersonData,
        person_b: crate::app::persondata::PersonData,
    ) -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "BattleStart", args = 0)]
    pub fn battle_start(self) -> ();

    #[method(name = "TryGetUnitOrCreate", args = 1)]
    pub fn try_get_unit_or_create(
        self,
        person: crate::app::persondata::PersonData,
    ) -> crate::app::unit::Unit;

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        person_a: crate::app::persondata::PersonData,
        person_b: crate::app::persondata::PersonData,
    ) -> ();
}

#[cfg(feature = "app-scriptbattle")]
impl ScriptBattle_ScriptCombatSequence {
    pub fn new(
        person_a: crate::app::persondata::PersonData,
        person_b: crate::app::persondata::PersonData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptBattle_ScriptCombatSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptBattle_ScriptCombatSequenceMethods>::ctor(this, person_a, person_b);
        this
    }
}
