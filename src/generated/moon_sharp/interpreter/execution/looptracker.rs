
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/execution/looptracker/LoopTracker.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Execution", name = "LoopTracker")]
#[parent(crate::system::object::Object)]
pub struct LoopTracker {
    #[rename(name = "Loops")]
    pub loops: crate::moon_sharp::interpreter::data_structs::faststack_1::FastStack_1<
        crate::moon_sharp::interpreter::execution::iloop::ILoop,
    >,
}

#[cfg(feature = "moon_sharp-interpreter-execution-looptracker")]
#[::unity2::methods]
impl LoopTracker {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-execution-looptracker")]
impl LoopTracker {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LoopTracker),
                ::core::stringify!(new),
            )
        });
        <Self as ILoopTrackerMethods>::ctor(this);
        this
    }
}
