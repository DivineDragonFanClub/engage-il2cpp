
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/loop/Loop.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Tree", name = "Loop")]
#[parent(crate::system::object::Object)]
pub struct Loop {
    #[rename(name = "Scope")]
    pub scope: crate::moon_sharp::interpreter::execution::runtimescopeblock::RuntimeScopeBlock,
    #[rename(name = "BreakJumps")]
    pub break_jumps: crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    >,
}

#[cfg(feature = "moon_sharp-interpreter-tree-loop")]
#[::unity2::methods]
impl Loop {
    #[method(name = "CompileBreak", args = 1)]
    pub fn compile_break(
        self,
        bc: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> ();

    #[method(name = "IsBoundary", args = 0)]
    pub fn is_boundary(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-tree-loop")]
impl Loop {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Loop),
                ::core::stringify!(new),
            )
        });
        <Self as ILoopMethods>::ctor(this);
        this
    }
}
