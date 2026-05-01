
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/diagnostics/performance_counters/iperformancestopwatch_interface/IPerformanceStopwatch_Interface.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Diagnostics.PerformanceCounters",
    name = "IPerformanceStopwatch"
)]
pub struct IPerformanceStopwatch_Interface {}

#[cfg(feature = "moon_sharp-interpreter-diagnostics-performance_counters-iperformancestopwatch_interface")]
#[::unity2::methods]
impl IPerformanceStopwatch_Interface {
    #[method(name = "GetResult", args = 0)]
    pub fn get_result(
        self,
    ) -> crate::moon_sharp::interpreter::diagnostics::performanceresult::PerformanceResult;
}
