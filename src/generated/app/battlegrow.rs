
use crate::app::procbattlecallback::IProcBattleCallback;
use crate::app::procbattlecallback::ProcBattleCallback;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlegrow/BattleGrow.md")))]
#[::unity2::class(namespace = "App", name = "BattleGrow")]
#[parent(crate::app::procbattlecallback::ProcBattleCallback)]
pub struct BattleGrow {
    #[rename(name = "m_IsTalk")]
    pub m_is_talk: bool,
}

#[cfg(feature = "app-battlegrow")]
#[::unity2::methods]
impl BattleGrow {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        calculator: crate::app::battlecalculator::BattleCalculator,
        is_talk: bool,
    ) -> ();

    #[method(name = "GainExp", args = 0)]
    pub fn gain_exp(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        calculator: crate::app::battlecalculator::BattleCalculator,
        wait_time: f32,
    ) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind_2(
        super_: crate::app::procinst::ProcInst,
        calculator: crate::app::battlecalculator::BattleCalculator,
        is_talk: bool,
        wait_time: f32,
    ) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind_3(
        super_: crate::app::procinst::ProcInst,
        side: crate::app::battleinfoside::BattleInfoSide,
    ) -> ();
}

#[cfg(feature = "app-battlegrow")]
impl BattleGrow {
    pub fn new(calculator: crate::app::battlecalculator::BattleCalculator, is_talk: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleGrow),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleGrowMethods>::ctor(this, calculator, is_talk);
        this
    }
}
