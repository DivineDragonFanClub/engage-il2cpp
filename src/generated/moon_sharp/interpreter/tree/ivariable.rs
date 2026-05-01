
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/ivariable/IVariable.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Tree", name = "IVariable")]
pub struct IVariable {}

#[cfg(feature = "moon_sharp-interpreter-tree-ivariable")]
#[::unity2::methods]
impl IVariable {
    #[method(name = "CompileAssignment", args = 3)]
    pub fn compile_assignment(
        self,
        bc: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
        stackofs: i32,
        tupleidx: i32,
    ) -> ();
}
