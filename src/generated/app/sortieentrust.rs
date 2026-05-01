
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentrust/SortieEntrust.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrust")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrust {}

#[cfg(feature = "app-sortieentrust")]
#[::unity2::methods]
impl SortieEntrust {
    #[method(name = "Execute", args = 1)]
    pub fn execute(
        unit: crate::app::unit::Unit,
    ) -> crate::app::sortieentrustresult::SortieEntrustResult;

    #[method(name = "ExecuteAllSortieUnits", args = 0)]
    pub fn execute_all_sortie_units() -> crate::app::sortieentrustresult::SortieEntrustResult;

    #[method(name = "ShowWarning", args = 2)]
    pub fn show_warning(
        super_: crate::app::procinst::ProcInst,
        result: crate::app::sortieentrustresult::SortieEntrustResult,
    ) -> ();

    #[method(name = "ShowWarningAllSortieUnits", args = 2)]
    pub fn show_warning_all_sortie_units(
        super_: crate::app::procinst::ProcInst,
        result: crate::app::sortieentrustresult::SortieEntrustResult,
    ) -> ();

    #[method(name = "ShowWarningCommon", args = 2)]
    pub fn show_warning_common(
        super_: crate::app::procinst::ProcInst,
        mid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortieentrust")]
impl SortieEntrust {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrust),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrustMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentrust/SortieEntrust_Executor.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrust.Executor")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrust_Executor {
    #[rename(name = "m_IsAll")]
    pub m_is_all: bool,
    #[rename(name = "m_UnitDataHolder")]
    pub m_unit_data_holder: crate::app::sortieentrustunitdataholder::SortieEntrustUnitDataHolder,
    #[rename(name = "m_StockDataHoler")]
    pub m_stock_data_holer: crate::app::sortieentruststockdataholder::SortieEntrustStockDataHolder,
}

#[cfg(feature = "app-sortieentrust")]
#[::unity2::methods]
impl SortieEntrust_Executor {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Execute", args = 1)]
    pub fn execute(
        self,
        unit: crate::app::unit::Unit,
    ) -> crate::app::sortieentrustresult::SortieEntrustResult;

    #[method(name = "Prepare", args = 1)]
    pub fn prepare(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "Calc", args = 0)]
    pub fn calc(self) -> ();

    #[method(name = "CalcOne", args = 1)]
    pub fn calc_one(
        self,
        unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData,
    ) -> ();

    #[method(name = "Reflect", args = 0)]
    pub fn reflect(self) -> ();

    #[method(name = "CheckInventroyGlut", args = 0)]
    pub fn check_inventroy_glut(self) -> bool;

    #[method(name = "CheckInventroyShortage", args = 0)]
    pub fn check_inventroy_shortage(self) -> bool;

    #[method(name = "IsTarget", args = 1)]
    pub fn is_target(self, unit: crate::app::unit::Unit) -> bool;
}

#[cfg(feature = "app-sortieentrust")]
impl SortieEntrust_Executor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrust_Executor),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrust_ExecutorMethods>::ctor(this);
        this
    }
}
