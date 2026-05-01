
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/dynamicexpression/DynamicExpression.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "DynamicExpression")]
#[parent(crate::system::object::Object)]
pub struct DynamicExpression {
# [rename (name = "m_Exp")] pub m_exp : crate :: moon_sharp :: interpreter :: tree :: expressions :: dynamicexprexpression :: DynamicExprExpression ,
# [rename (name = "m_Constant")] pub m_constant : crate :: moon_sharp :: interpreter :: dynvalue :: DynValue ,
# [rename (name = "ExpressionCode")] pub expression_code : :: unity2 :: Il2CppString ,
}

#[cfg(feature = "moon_sharp-interpreter-dynamicexpression")]
#[::unity2::methods]
impl DynamicExpression {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        s: crate::moon_sharp::interpreter::script::Script,
        str_expr: ::unity2::Il2CppString,
        expr : crate :: moon_sharp :: interpreter :: tree :: expressions :: dynamicexprexpression :: DynamicExprExpression,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        s: crate::moon_sharp::interpreter::script::Script,
        str_expr: ::unity2::Il2CppString,
        constant: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ();

    #[method(name = "Evaluate", args = 1)]
    pub fn evaluate(
        self,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "FindSymbol", args = 1)]
    pub fn find_symbol(
        self,
        context: crate::moon_sharp::interpreter::scriptexecutioncontext::ScriptExecutionContext,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "get_OwnerScript", args = 0)]
    pub fn get_owner_script(self) -> crate::moon_sharp::interpreter::script::Script;

    #[method(name = "set_OwnerScript", args = 1)]
    pub fn set_owner_script(self, value: crate::moon_sharp::interpreter::script::Script) -> ();

    #[method(name = "IsConstant", args = 0)]
    pub fn is_constant(self) -> bool;

    #[method(name = "GetHashCode", args = 0)]
    pub fn get_hash_code(self) -> i32;

    #[method(name = "Equals", args = 1)]
    pub fn equals(self, obj: crate::system::object::Object) -> bool;
}

#[cfg(feature = "moon_sharp-interpreter-dynamicexpression")]
impl DynamicExpression {
    pub fn new(
        s: crate::moon_sharp::interpreter::script::Script,
        str_expr: ::unity2::Il2CppString,
        expr : crate :: moon_sharp :: interpreter :: tree :: expressions :: dynamicexprexpression :: DynamicExprExpression,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DynamicExpression),
                ::core::stringify!(new),
            )
        });
        <Self as IDynamicExpressionMethods>::ctor(this, s, str_expr, expr);
        this
    }

    pub fn new_2(
        s: crate::moon_sharp::interpreter::script::Script,
        str_expr: ::unity2::Il2CppString,
        constant: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DynamicExpression),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDynamicExpressionMethods>::ctor_2(this, s, str_expr, constant);
        this
    }
}
