
use crate::moon_sharp::interpreter::tree::expression::Expression;
use crate::moon_sharp::interpreter::tree::expression::IExpression;
use crate::moon_sharp::interpreter::tree::nodebase::INodeBase;
use crate::moon_sharp::interpreter::tree::nodebase::NodeBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/expressions/dynamicexprexpression/DynamicExprExpression.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Tree.Expressions",
    name = "DynamicExprExpression"
)]
#[parent(crate::moon_sharp::interpreter::tree::expression::Expression)]
pub struct DynamicExprExpression {
    #[rename(name = "m_Exp")]
    pub m_exp: crate::moon_sharp::interpreter::tree::expression::Expression,
}

#[cfg(feature = "moon_sharp-interpreter-tree-expressions-dynamicexprexpression")]
#[::unity2::methods]
impl DynamicExprExpression {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        exp: crate::moon_sharp::interpreter::tree::expression::Expression,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> ();

    #[method(name = "Eval", args = 1)]
    pub fn eval(
        self,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "Compile", args = 1)]
    pub fn compile(
        self,
        bc: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> ();

    #[method(name = "FindDynamic", args = 1)]
    pub fn find_dynamic(
        self,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;
}

#[cfg(feature = "moon_sharp-interpreter-tree-expressions-dynamicexprexpression")]
impl DynamicExprExpression {
    pub fn new(
        exp: crate::moon_sharp::interpreter::tree::expression::Expression,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DynamicExprExpression),
                ::core::stringify!(new),
            )
        });
        <Self as IDynamicExprExpressionMethods>::ctor(this, exp, lcontext);
        this
    }
}
