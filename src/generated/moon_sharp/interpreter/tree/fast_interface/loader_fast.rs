
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/fast_interface/loader_fast/Loader_Fast.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Tree.Fast_Interface",
    name = "Loader_Fast"
)]
#[parent(crate::system::object::Object)]
pub struct Loader_Fast {}

#[cfg(feature = "moon_sharp-interpreter-tree-fast_interface-loader_fast")]
#[::unity2::methods]
impl Loader_Fast {
    #[method(name = "LoadDynamicExpr", args = 2)]
    pub fn load_dynamic_expr (script : crate :: moon_sharp :: interpreter :: script :: Script , source : crate :: moon_sharp :: interpreter :: debugging :: sourcecode :: SourceCode) -> crate :: moon_sharp :: interpreter :: tree :: expressions :: dynamicexprexpression :: DynamicExprExpression ;

    #[method(name = "CreateLoadingContext", args = 2)]
    pub fn create_loading_context(
        script: crate::moon_sharp::interpreter::script::Script,
        source: crate::moon_sharp::interpreter::debugging::sourcecode::SourceCode,
    ) -> crate::moon_sharp::interpreter::execution::scriptloadingcontext::ScriptLoadingContext;

    #[method(name = "LoadChunk", args = 3)]
    pub fn load_chunk(
        script: crate::moon_sharp::interpreter::script::Script,
        source: crate::moon_sharp::interpreter::debugging::sourcecode::SourceCode,
        bytecode: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> i32;

    #[method(name = "LoadFunction", args = 4)]
    pub fn load_function(
        script: crate::moon_sharp::interpreter::script::Script,
        source: crate::moon_sharp::interpreter::debugging::sourcecode::SourceCode,
        bytecode: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
        uses_global_env: bool,
    ) -> i32;
}
