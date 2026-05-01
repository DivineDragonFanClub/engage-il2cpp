
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/diagnostics/performancestatistics/PerformanceStatistics.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Diagnostics",
    name = "PerformanceStatistics"
)]
#[parent(crate::system::object::Object)]
pub struct PerformanceStatistics {
# [rename (name = "m_Stopwatches")] pub m_stopwatches : :: unity2 :: Array < crate :: moon_sharp :: interpreter :: diagnostics :: performance_counters :: iperformancestopwatch_interface :: IPerformanceStopwatch_Interface > ,
# [static_field] # [rename (name = "m_GlobalStopwatches")] pub m_global_stopwatches : :: unity2 :: Array < crate :: moon_sharp :: interpreter :: diagnostics :: performance_counters :: iperformancestopwatch_interface :: IPerformanceStopwatch_Interface > ,
# [rename (name = "m_Enabled")] pub m_enabled : bool ,
}

#[cfg(feature = "moon_sharp-interpreter-diagnostics-performancestatistics")]
#[::unity2::methods]
impl PerformanceStatistics {
    #[method(name = "get_Enabled", args = 0)]
    pub fn get_enabled(self) -> bool;

    #[method(name = "set_Enabled", args = 1)]
    pub fn set_enabled(self, value: bool) -> ();

    #[method(name = "GetPerformanceCounterResult", args = 1)]
    pub fn get_performance_counter_result(
        self,
        pc: crate::moon_sharp::interpreter::diagnostics::performancecounter::PerformanceCounter,
    ) -> crate::moon_sharp::interpreter::diagnostics::performanceresult::PerformanceResult;

    #[method(name = "GetPerformanceLog", args = 0)]
    pub fn get_performance_log(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "moon_sharp-interpreter-diagnostics-performancestatistics")]
impl PerformanceStatistics {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PerformanceStatistics),
                ::core::stringify!(new),
            )
        });
        <Self as IPerformanceStatisticsMethods>::ctor(this);
        this
    }
}
