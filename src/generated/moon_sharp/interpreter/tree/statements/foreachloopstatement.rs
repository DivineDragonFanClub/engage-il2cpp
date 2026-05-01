
use crate::moon_sharp::interpreter::tree::nodebase::INodeBase;
use crate::moon_sharp::interpreter::tree::nodebase::NodeBase;
use crate::moon_sharp::interpreter::tree::statement::IStatement;
use crate::moon_sharp::interpreter::tree::statement::Statement;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/statements/foreachloopstatement/ForEachLoopStatement.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Tree.Statements",
    name = "ForEachLoopStatement"
)]
#[parent(crate::moon_sharp::interpreter::tree::statement::Statement)]
pub struct ForEachLoopStatement {
    #[rename(name = "m_StackFrame")]
    pub m_stack_frame:
        crate::moon_sharp::interpreter::execution::runtimescopeblock::RuntimeScopeBlock,
    #[rename(name = "m_Names")]
    pub m_names: ::unity2::Array<crate::moon_sharp::interpreter::symbolref::SymbolRef>,
    #[rename(name = "m_NameExps")]
    pub m_name_exps: ::unity2::Array<crate::moon_sharp::interpreter::tree::ivariable::IVariable>,
    #[rename(name = "m_RValues")]
    pub m_r_values: crate::moon_sharp::interpreter::tree::expression::Expression,
    #[rename(name = "m_Block")]
    pub m_block: crate::moon_sharp::interpreter::tree::statement::Statement,
    #[rename(name = "m_RefFor")]
    pub m_ref_for: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    #[rename(name = "m_RefEnd")]
    pub m_ref_end: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
}

#[cfg(feature = "moon_sharp-interpreter-tree-statements-foreachloopstatement")]
#[::unity2::methods]
impl ForEachLoopStatement {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        first_name_token: crate::moon_sharp::interpreter::tree::token::Token,
        for_token: crate::moon_sharp::interpreter::tree::token::Token,
    ) -> ();

    #[method(name = "Compile", args = 1)]
    pub fn compile(
        self,
        bc: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-tree-statements-foreachloopstatement")]
impl ForEachLoopStatement {
    pub fn new(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        first_name_token: crate::moon_sharp::interpreter::tree::token::Token,
        for_token: crate::moon_sharp::interpreter::tree::token::Token,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ForEachLoopStatement),
                ::core::stringify!(new),
            )
        });
        <Self as IForEachLoopStatementMethods>::ctor(this, lcontext, first_name_token, for_token);
        this
    }
}
