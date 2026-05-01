
use crate::moon_sharp::interpreter::tree::expression::Expression;
use crate::moon_sharp::interpreter::tree::expression::IExpression;
use crate::moon_sharp::interpreter::tree::nodebase::INodeBase;
use crate::moon_sharp::interpreter::tree::nodebase::NodeBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/expressions/adjustmentexpression/AdjustmentExpression.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Tree.Expressions",
    name = "AdjustmentExpression"
)]
#[parent(crate::moon_sharp::interpreter::tree::expression::Expression)]
pub struct AdjustmentExpression {
    #[rename(name = "expression")]
    pub expression: crate::moon_sharp::interpreter::tree::expression::Expression,
}

#[cfg(feature = "moon_sharp-interpreter-tree-expressions-adjustmentexpression")]
#[::unity2::methods]
impl AdjustmentExpression {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        exp: crate::moon_sharp::interpreter::tree::expression::Expression,
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

#[cfg(feature = "moon_sharp-interpreter-tree-expressions-adjustmentexpression")]
impl AdjustmentExpression {
    pub fn new(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
        exp: crate::moon_sharp::interpreter::tree::expression::Expression,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AdjustmentExpression),
                ::core::stringify!(new),
            )
        });
        <Self as IAdjustmentExpressionMethods>::ctor(this, lcontext, exp);
        this
    }
}
