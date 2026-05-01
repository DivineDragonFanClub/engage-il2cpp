
use crate::moon_sharp::interpreter::tree::expression::Expression;
use crate::moon_sharp::interpreter::tree::expression::IExpression;
use crate::moon_sharp::interpreter::tree::nodebase::INodeBase;
use crate::moon_sharp::interpreter::tree::nodebase::NodeBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/expressions/exprlistexpression/ExprListExpression.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Tree.Expressions",
    name = "ExprListExpression"
)]
#[parent(crate::moon_sharp::interpreter::tree::expression::Expression)]
pub struct ExprListExpression {
    #[rename(name = "expressions")]
    pub expressions: crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::tree::expression::Expression,
    >,
}

#[cfg(feature = "moon_sharp-interpreter-tree-expressions-exprlistexpression")]
#[::unity2::methods]
impl ExprListExpression {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        exps: crate::system::collections::generic::list_1::List_1<
            crate::moon_sharp::interpreter::tree::expression::Expression,
        >,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> ();

    #[method(name = "GetExpressions", args = 0)]
    pub fn get_expressions(
        self,
    ) -> ::unity2::Array<crate::moon_sharp::interpreter::tree::expression::Expression>;

    #[method(name = "Compile", args = 1)]
    pub fn compile(
        self,
        bc: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> ();

    #[method(name = "Eval", args = 1)]
    pub fn eval(
        self,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;
}

#[cfg(feature = "moon_sharp-interpreter-tree-expressions-exprlistexpression")]
impl ExprListExpression {
    pub fn new(
        exps: crate::system::collections::generic::list_1::List_1<
            crate::moon_sharp::interpreter::tree::expression::Expression,
        >,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ExprListExpression),
                ::core::stringify!(new),
            )
        });
        <Self as IExprListExpressionMethods>::ctor(this, exps, lcontext);
        this
    }
}
