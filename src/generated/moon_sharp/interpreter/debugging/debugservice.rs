
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/debugging/debugservice/DebugService.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Debugging", name = "DebugService")]
#[parent(crate::system::object::Object)]
pub struct DebugService {
    #[rename(name = "m_Processor")]
    pub m_processor: crate::moon_sharp::interpreter::execution::vm::processor::Processor,
}

#[cfg(feature = "moon_sharp-interpreter-debugging-debugservice")]
#[::unity2::methods]
impl DebugService {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        script: crate::moon_sharp::interpreter::script::Script,
        processor: crate::moon_sharp::interpreter::execution::vm::processor::Processor,
    ) -> ();

    #[method(name = "get_OwnerScript", args = 0)]
    pub fn get_owner_script(self) -> crate::moon_sharp::interpreter::script::Script;

    #[method(name = "set_OwnerScript", args = 1)]
    pub fn set_owner_script(self, value: crate::moon_sharp::interpreter::script::Script) -> ();

    #[method(name = "ResetBreakPoints", args = 2)]
    pub fn reset_break_points(
        self,
        src: crate::moon_sharp::interpreter::debugging::sourcecode::SourceCode,
        lines: crate::system::collections::generic::hashset_1::HashSet_1<i32>,
    ) -> crate::system::collections::generic::hashset_1::HashSet_1<i32>;
}

#[cfg(feature = "moon_sharp-interpreter-debugging-debugservice")]
impl DebugService {
    pub fn new(
        script: crate::moon_sharp::interpreter::script::Script,
        processor: crate::moon_sharp::interpreter::execution::vm::processor::Processor,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DebugService),
                ::core::stringify!(new),
            )
        });
        <Self as IDebugServiceMethods>::ctor(this, script, processor);
        this
    }
}
