
use crate::moon_sharp::interpreter::tree::nodebase::INodeBase;
use crate::moon_sharp::interpreter::tree::nodebase::NodeBase;
use crate::moon_sharp::interpreter::tree::statement::IStatement;
use crate::moon_sharp::interpreter::tree::statement::Statement;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/statements/labelstatement/LabelStatement.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Tree.Statements",
    name = "LabelStatement"
)]
#[parent(crate::moon_sharp::interpreter::tree::statement::Statement)]
pub struct LabelStatement {
    #[rename(name = "m_Gotos")]
    pub m_gotos: crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::tree::statements::gotostatement::GotoStatement,
    >,
    #[rename(name = "m_StackFrame")]
    pub m_stack_frame:
        crate::moon_sharp::interpreter::execution::runtimescopeblock::RuntimeScopeBlock,
}

#[cfg(feature = "moon_sharp-interpreter-tree-statements-labelstatement")]
#[::unity2::methods]
impl LabelStatement {
    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Label", args = 1)]
    pub fn set_label(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Address", args = 0)]
    pub fn get_address(self) -> i32;

    #[method(name = "set_Address", args = 1)]
    pub fn set_address(self, value: i32) -> ();

    #[method(name = "get_SourceRef", args = 0)]
    pub fn get_source_ref(self) -> crate::moon_sharp::interpreter::debugging::sourceref::SourceRef;

    #[method(name = "set_SourceRef", args = 1)]
    pub fn set_source_ref(
        self,
        value: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    ) -> ();

    #[method(name = "get_NameToken", args = 0)]
    pub fn get_name_token(self) -> crate::moon_sharp::interpreter::tree::token::Token;

    #[method(name = "set_NameToken", args = 1)]
    pub fn set_name_token(self, value: crate::moon_sharp::interpreter::tree::token::Token) -> ();

    #[method(name = "get_DefinedVarsCount", args = 0)]
    pub fn get_defined_vars_count(self) -> i32;

    #[method(name = "set_DefinedVarsCount", args = 1)]
    pub fn set_defined_vars_count(self, value: i32) -> ();

    #[method(name = "get_LastDefinedVarName", args = 0)]
    pub fn get_last_defined_var_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LastDefinedVarName", args = 1)]
    pub fn set_last_defined_var_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> ();

    #[method(name = "SetDefinedVars", args = 2)]
    pub fn set_defined_vars(
        self,
        defined_vars_count: i32,
        last_defined_vars_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "RegisterGoto", args = 1)]
    pub fn register_goto(
        self,
        gotostat: crate::moon_sharp::interpreter::tree::statements::gotostatement::GotoStatement,
    ) -> ();

    #[method(name = "Compile", args = 1)]
    pub fn compile(
        self,
        bc: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> ();

    #[method(name = "SetScope", args = 1)]
    pub fn set_scope(
        self,
        runtime_scope_block : crate :: moon_sharp :: interpreter :: execution :: runtimescopeblock :: RuntimeScopeBlock,
    ) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-tree-statements-labelstatement")]
impl LabelStatement {
    pub fn new(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(LabelStatement),
                ::core::stringify!(new),
            )
        });
        <Self as ILabelStatementMethods>::ctor(this, lcontext);
        this
    }
}
