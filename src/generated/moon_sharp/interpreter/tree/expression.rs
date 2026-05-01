
use crate::moon_sharp::interpreter::tree::nodebase::INodeBase;
use crate::moon_sharp::interpreter::tree::nodebase::NodeBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/expression/Expression.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Tree", name = "Expression")]
#[parent(crate::moon_sharp::interpreter::tree::nodebase::NodeBase)]
pub struct Expression {}

#[cfg(feature = "moon_sharp-interpreter-tree-expression")]
#[::unity2::methods]
impl Expression {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> ();

    #[method(name = "GetFriendlyDebugName", args = 0)]
    pub fn get_friendly_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "Eval", args = 1)]
    pub fn eval(
        self,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "FindDynamic", args = 1)]
    pub fn find_dynamic(
        self,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "ExprListAfterFirstExpr", args = 2)]
    pub fn expr_list_after_first_expr(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        expr1: crate::moon_sharp::interpreter::tree::expression::Expression,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::tree::expression::Expression,
    >;

    #[method(name = "ExprList", args = 1)]
    pub fn expr_list(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::tree::expression::Expression,
    >;

    #[method(name = "Expr", args = 1)]
    pub fn expr(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> crate::moon_sharp::interpreter::tree::expression::Expression;

    #[method(name = "SubExpr", args = 2)]
    pub fn sub_expr(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        is_primary: bool,
    ) -> crate::moon_sharp::interpreter::tree::expression::Expression;

    #[method(name = "SimpleExp", args = 1)]
    pub fn simple_exp(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> crate::moon_sharp::interpreter::tree::expression::Expression;

    #[method(name = "PrimaryExp", args = 1)]
    pub fn primary_exp(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> crate::moon_sharp::interpreter::tree::expression::Expression;

    #[method(name = "PrefixExp", args = 1)]
    pub fn prefix_exp(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> crate::moon_sharp::interpreter::tree::expression::Expression;
}

#[cfg(feature = "moon_sharp-interpreter-tree-expression")]
impl Expression {
    pub fn new(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Expression),
                ::core::stringify!(new),
            )
        });
        <Self as IExpressionMethods>::ctor(this, lcontext);
        this
    }
}
