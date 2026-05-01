
use crate::moon_sharp::interpreter::tree::nodebase::INodeBase;
use crate::moon_sharp::interpreter::tree::nodebase::NodeBase;
use crate::moon_sharp::interpreter::tree::statement::IStatement;
use crate::moon_sharp::interpreter::tree::statement::Statement;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/statements/assignmentstatement/AssignmentStatement.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Tree.Statements",
    name = "AssignmentStatement"
)]
#[parent(crate::moon_sharp::interpreter::tree::statement::Statement)]
pub struct AssignmentStatement {
    #[rename(name = "m_LValues")]
    pub m_l_values: crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::tree::ivariable::IVariable,
    >,
    #[rename(name = "m_RValues")]
    pub m_r_values: crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::tree::expression::Expression,
    >,
    #[rename(name = "m_Ref")]
    pub m_ref: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
}

#[cfg(feature = "moon_sharp-interpreter-tree-statements-assignmentstatement")]
#[::unity2::methods]
impl AssignmentStatement {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        start_token: crate::moon_sharp::interpreter::tree::token::Token,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        first_expression: crate::moon_sharp::interpreter::tree::expression::Expression,
        first: crate::moon_sharp::interpreter::tree::token::Token,
    ) -> ();

    #[method(name = "CheckVar", args = 2)]
    pub fn check_var(
        self,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        first_expression: crate::moon_sharp::interpreter::tree::expression::Expression,
    ) -> crate::moon_sharp::interpreter::tree::ivariable::IVariable;

    #[method(name = "Compile", args = 1)]
    pub fn compile(
        self,
        bc: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-tree-statements-assignmentstatement")]
impl AssignmentStatement {
    pub fn new(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        start_token: crate::moon_sharp::interpreter::tree::token::Token,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssignmentStatement),
                ::core::stringify!(new),
            )
        });
        <Self as IAssignmentStatementMethods>::ctor(this, lcontext, start_token);
        this
    }

    pub fn new_2(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        first_expression: crate::moon_sharp::interpreter::tree::expression::Expression,
        first: crate::moon_sharp::interpreter::tree::token::Token,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssignmentStatement),
                ::core::stringify!(new_2),
            )
        });
        <Self as IAssignmentStatementMethods>::ctor_2(this, lcontext, first_expression, first);
        this
    }
}
