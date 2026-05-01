
use crate::moon_sharp::interpreter::tree::nodebase::INodeBase;
use crate::moon_sharp::interpreter::tree::nodebase::NodeBase;
use crate::moon_sharp::interpreter::tree::statement::IStatement;
use crate::moon_sharp::interpreter::tree::statement::Statement;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/statements/scopeblockstatement/ScopeBlockStatement.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Tree.Statements",
    name = "ScopeBlockStatement"
)]
#[parent(crate::moon_sharp::interpreter::tree::statement::Statement)]
pub struct ScopeBlockStatement {
    #[rename(name = "m_Block")]
    pub m_block: crate::moon_sharp::interpreter::tree::statement::Statement,
    #[rename(name = "m_StackFrame")]
    pub m_stack_frame:
        crate::moon_sharp::interpreter::execution::runtimescopeblock::RuntimeScopeBlock,
    #[rename(name = "m_Do")]
    pub m_do: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    #[rename(name = "m_End")]
    pub m_end: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
}

#[cfg(feature = "moon_sharp-interpreter-tree-statements-scopeblockstatement")]
#[::unity2::methods]
impl ScopeBlockStatement {
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
}

#[cfg(feature = "moon_sharp-interpreter-tree-statements-scopeblockstatement")]
impl ScopeBlockStatement {
    pub fn new(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScopeBlockStatement),
                ::core::stringify!(new),
            )
        });
        <Self as IScopeBlockStatementMethods>::ctor(this, lcontext);
        this
    }
}
