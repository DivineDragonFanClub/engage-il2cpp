
use crate::app::procbattlecallback::IProcBattleCallback;
use crate::app::procbattlecallback::ProcBattleCallback;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battletalkevent/BattleTalkEvent.md")))]
#[::unity2::class(namespace = "App", name = "BattleTalkEvent")]
#[parent(crate::app::procbattlecallback::ProcBattleCallback)]
pub struct BattleTalkEvent {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_Target")]
    pub m_target: crate::app::unit::Unit,
}

#[cfg(feature = "app-battletalkevent")]
#[::unity2::methods]
impl BattleTalkEvent {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        calculator: crate::app::battlecalculator::BattleCalculator,
        unit: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        completed: crate::system::action::Action,
    ) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "IsEnable", args = 1)]
    pub fn is_enable(calculator: crate::app::battlecalculator::BattleCalculator) -> bool;

    #[method(name = "IsEnable", args = 3)]
    pub fn is_enable_2(
        calculator: crate::app::battlecalculator::BattleCalculator,
        unit: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
    ) -> bool;

    #[method(name = "TryCreateBind", args = 3)]
    pub fn try_create_bind(
        super_: crate::app::procinst::ProcInst,
        calculator: crate::app::battlecalculator::BattleCalculator,
        completed: crate::system::action::Action,
    ) -> bool;

    #[method(name = "TryCreateBind", args = 5)]
    pub fn try_create_bind_2(
        super_: crate::app::procinst::ProcInst,
        calculator: crate::app::battlecalculator::BattleCalculator,
        unit: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        completed: crate::system::action::Action,
    ) -> bool;
}

#[cfg(feature = "app-battletalkevent")]
impl BattleTalkEvent {
    pub fn new(
        calculator: crate::app::battlecalculator::BattleCalculator,
        unit: crate::app::unit::Unit,
        target: crate::app::unit::Unit,
        completed: crate::system::action::Action,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleTalkEvent),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleTalkEventMethods>::ctor(this, calculator, unit, target, completed);
        this
    }
}
