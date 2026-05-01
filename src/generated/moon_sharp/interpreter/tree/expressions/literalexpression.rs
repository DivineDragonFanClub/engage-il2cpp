
use crate::moon_sharp::interpreter::tree::expression::Expression;
use crate::moon_sharp::interpreter::tree::expression::IExpression;
use crate::moon_sharp::interpreter::tree::nodebase::INodeBase;
use crate::moon_sharp::interpreter::tree::nodebase::NodeBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/expressions/literalexpression/LiteralExpression.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Tree.Expressions",
    name = "LiteralExpression"
)]
#[parent(crate::moon_sharp::interpreter::tree::expression::Expression)]
pub struct LiteralExpression {
    #[rename(name = "m_Value")]
    pub m_value: crate::moon_sharp::interpreter::dynvalue::DynValue,
}

#[cfg(feature = "moon_sharp-interpreter-tree-expressions-literalexpression")]
#[::unity2::methods]
impl LiteralExpression {
    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        t: crate::moon_sharp::interpreter::tree::token::Token,
    ) -> ();

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

#[cfg(feature = "moon_sharp-interpreter-tree-expressions-literalexpression")]
impl LiteralExpression {
    pub fn new(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LiteralExpression),
                ::core::stringify!(new),
            )
        });
        <Self as ILiteralExpressionMethods>::ctor(this, lcontext, value);
        this
    }

    pub fn new_2(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        t: crate::moon_sharp::interpreter::tree::token::Token,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LiteralExpression),
                ::core::stringify!(new_2),
            )
        });
        <Self as ILiteralExpressionMethods>::ctor_2(this, lcontext, t);
        this
    }
}
