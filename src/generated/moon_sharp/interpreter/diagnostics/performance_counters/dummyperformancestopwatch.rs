
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/diagnostics/performance_counters/dummyperformancestopwatch/DummyPerformanceStopwatch.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Diagnostics.PerformanceCounters",
    name = "DummyPerformanceStopwatch"
)]
#[parent(crate::system::object::Object)]
pub struct DummyPerformanceStopwatch {
# [static_field] # [rename (name = "Instance")] pub instance : crate :: moon_sharp :: interpreter :: diagnostics :: performance_counters :: dummyperformancestopwatch :: DummyPerformanceStopwatch ,
# [rename (name = "m_Result")] pub m_result : crate :: moon_sharp :: interpreter :: diagnostics :: performanceresult :: PerformanceResult ,
}

#[cfg(feature = "moon_sharp-interpreter-diagnostics-performance_counters-dummyperformancestopwatch")]
#[::unity2::methods]
impl DummyPerformanceStopwatch {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetResult", args = 0)]
    pub fn get_result(
        self,
    ) -> crate::moon_sharp::interpreter::diagnostics::performanceresult::PerformanceResult;

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "moon_sharp-interpreter-diagnostics-performance_counters-dummyperformancestopwatch")]
impl DummyPerformanceStopwatch {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DummyPerformanceStopwatch),
                ::core::stringify!(new),
            )
        });
        <Self as IDummyPerformanceStopwatchMethods>::ctor(this);
        this
    }
}
