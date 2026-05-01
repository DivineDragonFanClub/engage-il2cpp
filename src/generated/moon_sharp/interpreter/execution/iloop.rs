
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/execution/iloop/ILoop.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Execution", name = "ILoop")]
pub struct ILoop {}

#[cfg(feature = "moon_sharp-interpreter-execution-iloop")]
#[::unity2::methods]
impl ILoop {
    #[method(name = "CompileBreak", args = 1)]
    pub fn compile_break(
        self,
        bc: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> ();

    #[method(name = "IsBoundary", args = 0)]
    pub fn is_boundary(self) -> bool;
}
