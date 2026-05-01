
use crate::app::procbattlecallback::IProcBattleCallback;
use crate::app::procbattlecallback::ProcBattleCallback;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dieevent/DieEvent.md")))]
#[::unity2::class(namespace = "App", name = "DieEvent")]
#[parent(crate::app::procbattlecallback::ProcBattleCallback)]
pub struct DieEvent {
    #[rename(name = "m_Unit")]
    pub m_unit: crate::app::unit::Unit,
    #[rename(name = "m_IsInCombat")]
    pub m_is_in_combat: bool,
}

#[cfg(feature = "app-dieevent")]
#[::unity2::methods]
impl DieEvent {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, unit: crate::app::unit::Unit, completed: crate::system::action::Action)
        -> ();

    #[method(name = "IsSkipCancel", args = 0)]
    pub fn is_skip_cancel(self) -> bool;

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "DieTalk", args = 0)]
    pub fn die_talk(self) -> ();

    #[method(name = "GodTalk", args = 0)]
    pub fn god_talk(self) -> ();

    #[method(name = "Event", args = 0)]
    pub fn event(self) -> ();

    #[method(name = "InitDieBgm", args = 0)]
    pub fn init_die_bgm(self) -> ();

    #[method(name = "WaitDieBgmEnd", args = 0)]
    pub fn wait_die_bgm_end(self) -> ();

    #[method(name = "PlayDieBgm", args = 0)]
    pub fn play_die_bgm(self) -> ();

    #[method(name = "StopDieBgm", args = 0)]
    pub fn stop_die_bgm(self) -> ();

    #[method(name = "IsEnable", args = 1)]
    pub fn is_enable(calculator: crate::app::battlecalculator::BattleCalculator) -> bool;

    #[method(name = "IsImportant", args = 1)]
    pub fn is_important(calculator: crate::app::battlecalculator::BattleCalculator) -> bool;

    #[method(name = "IsEnable", args = 1)]
    pub fn is_enable_2(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "TryCreateBind", args = 3)]
    pub fn try_create_bind(
        super_: crate::app::procinst::ProcInst,
        calculator: crate::app::battlecalculator::BattleCalculator,
        completed: crate::system::action::Action,
    ) -> bool;

    #[method(name = "TryCreateBind", args = 3)]
    pub fn try_create_bind_2(
        super_: crate::app::procinst::ProcInst,
        unit: crate::app::unit::Unit,
        completed: crate::system::action::Action,
    ) -> bool;
}

#[cfg(feature = "app-dieevent")]
impl DieEvent {
    pub fn new(unit: crate::app::unit::Unit, completed: crate::system::action::Action) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DieEvent),
                ::core::stringify!(new),
            )
        });
        <Self as IDieEventMethods>::ctor(this, unit, completed);
        this
    }
}
