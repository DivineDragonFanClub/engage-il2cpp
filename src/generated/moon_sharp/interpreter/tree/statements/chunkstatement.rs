
use crate::moon_sharp::interpreter::tree::nodebase::INodeBase;
use crate::moon_sharp::interpreter::tree::nodebase::NodeBase;
use crate::moon_sharp::interpreter::tree::statement::IStatement;
use crate::moon_sharp::interpreter::tree::statement::Statement;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/statements/chunkstatement/ChunkStatement.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Tree.Statements",
    name = "ChunkStatement"
)]
#[parent(crate::moon_sharp::interpreter::tree::statement::Statement)]
pub struct ChunkStatement {
    #[rename(name = "m_Block")]
    pub m_block: crate::moon_sharp::interpreter::tree::statement::Statement,
    #[rename(name = "m_StackFrame")]
    pub m_stack_frame:
        crate::moon_sharp::interpreter::execution::runtimescopeframe::RuntimeScopeFrame,
    #[rename(name = "m_Env")]
    pub m_env: crate::moon_sharp::interpreter::symbolref::SymbolRef,
    #[rename(name = "m_VarArgs")]
    pub m_var_args: crate::moon_sharp::interpreter::symbolref::SymbolRef,
}

#[cfg(feature = "moon_sharp-interpreter-tree-statements-chunkstatement")]
#[::unity2::methods]
impl ChunkStatement {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> ();

    #[method(name = "Compile", args = 1)]
    pub fn compile(
        self,
        bc: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> ();

    #[method(name = "CreateUpvalue", args = 2)]
    pub fn create_upvalue(
        self,
        scope: crate::moon_sharp::interpreter::execution::buildtimescope::BuildTimeScope,
        symbol: crate::moon_sharp::interpreter::symbolref::SymbolRef,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;
}

#[cfg(feature = "moon_sharp-interpreter-tree-statements-chunkstatement")]
impl ChunkStatement {
    pub fn new(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ChunkStatement),
                ::core::stringify!(new),
            )
        });
        <Self as IChunkStatementMethods>::ctor(this, lcontext);
        this
    }
}
