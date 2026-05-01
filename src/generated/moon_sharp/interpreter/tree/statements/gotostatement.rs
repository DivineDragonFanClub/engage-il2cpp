
use crate::moon_sharp::interpreter::tree::nodebase::INodeBase;
use crate::moon_sharp::interpreter::tree::nodebase::NodeBase;
use crate::moon_sharp::interpreter::tree::statement::IStatement;
use crate::moon_sharp::interpreter::tree::statement::Statement;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/tree/statements/gotostatement/GotoStatement.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Tree.Statements",
    name = "GotoStatement"
)]
#[parent(crate::moon_sharp::interpreter::tree::statement::Statement)]
pub struct GotoStatement {
    #[rename(name = "m_Jump")]
    pub m_jump: crate::moon_sharp::interpreter::execution::vm::instruction::Instruction,
    #[rename(name = "m_LabelAddress")]
    pub m_label_address: i32,
}

#[cfg(feature = "moon_sharp-interpreter-tree-statements-gotostatement")]
#[::unity2::methods]
impl GotoStatement {
    #[method(name = "get_SourceRef", args = 0)]
    pub fn get_source_ref(self) -> crate::moon_sharp::interpreter::debugging::sourceref::SourceRef;

    #[method(name = "set_SourceRef", args = 1)]
    pub fn set_source_ref(
        self,
        value: crate::moon_sharp::interpreter::debugging::sourceref::SourceRef,
    ) -> ();

    #[method(name = "get_GotoToken", args = 0)]
    pub fn get_goto_token(self) -> crate::moon_sharp::interpreter::tree::token::Token;

    #[method(name = "set_GotoToken", args = 1)]
    pub fn set_goto_token(self, value: crate::moon_sharp::interpreter::tree::token::Token) -> ();

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Label", args = 1)]
    pub fn set_label(self, value: ::unity2::Il2CppString) -> ();

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

    #[method(name = "Compile", args = 1)]
    pub fn compile(
        self,
        bc: crate::moon_sharp::interpreter::execution::vm::bytecode::ByteCode,
    ) -> ();

    #[method(name = "SetDefinedVars", args = 2)]
    pub fn set_defined_vars(
        self,
        defined_vars_count: i32,
        last_defined_vars_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "SetAddress", args = 1)]
    pub fn set_address(self, label_address: i32) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-tree-statements-gotostatement")]
impl GotoStatement {
    pub fn new(
        lcontext : crate :: moon_sharp :: interpreter :: execution :: scriptloadingcontext :: ScriptLoadingContext,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GotoStatement),
                ::core::stringify!(new),
            )
        });
        <Self as IGotoStatementMethods>::ctor(this, lcontext);
        this
    }
}
