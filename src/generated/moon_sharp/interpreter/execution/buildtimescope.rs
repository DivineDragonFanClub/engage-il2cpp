
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/execution/buildtimescope/BuildTimeScope.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Execution", name = "BuildTimeScope")]
#[parent(crate::system::object::Object)]
pub struct BuildTimeScope {
    #[rename(name = "m_Frames")]
    pub m_frames: crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::execution::scopes::buildtimescopeframe::BuildTimeScopeFrame,
    >,
    #[rename(name = "m_ClosureBuilders")]
    pub m_closure_builders: crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::execution::iclosurebuilder::IClosureBuilder,
    >,
}

#[cfg(feature = "moon_sharp-interpreter-execution-buildtimescope")]
#[::unity2::methods]
impl BuildTimeScope {
    #[method(name = "PushFunction", args = 2)]
    pub fn push_function(
        self,
        closure_builder : crate :: moon_sharp :: interpreter :: execution :: iclosurebuilder :: IClosureBuilder,
        has_var_args: bool,
    ) -> ();

    #[method(name = "PushBlock", args = 0)]
    pub fn push_block(self) -> ();

    #[method(name = "PopBlock", args = 0)]
    pub fn pop_block(
        self,
    ) -> crate::moon_sharp::interpreter::execution::runtimescopeblock::RuntimeScopeBlock;

    #[method(name = "PopFunction", args = 0)]
    pub fn pop_function(
        self,
    ) -> crate::moon_sharp::interpreter::execution::runtimescopeframe::RuntimeScopeFrame;

    #[method(name = "Find", args = 1)]
    pub fn find(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "CreateGlobalReference", args = 1)]
    pub fn create_global_reference(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "ForceEnvUpValue", args = 0)]
    pub fn force_env_up_value(self) -> ();

    #[method(name = "CreateUpValue", args = 4)]
    pub fn create_up_value(
        self,
        build_time_scope: crate::moon_sharp::interpreter::execution::buildtimescope::BuildTimeScope,
        symb: crate::moon_sharp::interpreter::symbolref::SymbolRef,
        closured_frame: i32,
        current_frame: i32,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "DefineLocal", args = 1)]
    pub fn define_local(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "TryDefineLocal", args = 1)]
    pub fn try_define_local(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "CurrentFunctionHasVarArgs", args = 0)]
    pub fn current_function_has_var_args(self) -> bool;

    #[method(name = "DefineLabel", args = 1)]
    pub fn define_label(
        self,
        label: crate::moon_sharp::interpreter::tree::statements::labelstatement::LabelStatement,
    ) -> ();

    #[method(name = "RegisterGoto", args = 1)]
    pub fn register_goto(
        self,
        gotostat: crate::moon_sharp::interpreter::tree::statements::gotostatement::GotoStatement,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-execution-buildtimescope")]
impl BuildTimeScope {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BuildTimeScope),
                ::core::stringify!(new),
            )
        });
        <Self as IBuildTimeScopeMethods>::ctor(this);
        this
    }
}
