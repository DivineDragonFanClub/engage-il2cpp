
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/repl/replinterpreter/ReplInterpreter.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.REPL", name = "ReplInterpreter")]
#[parent(crate::system::object::Object)]
pub struct ReplInterpreter {
    #[rename(name = "m_Script")]
    pub m_script: crate::moon_sharp::interpreter::script::Script,
    #[rename(name = "m_CurrentCommand")]
    pub m_current_command: ::unity2::Il2CppString,
}

#[cfg(feature = "moon_sharp-interpreter-repl-replinterpreter")]
#[::unity2::methods]
impl ReplInterpreter {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, script: crate::moon_sharp::interpreter::script::Script) -> ();

    #[method(name = "get_HandleDynamicExprs", args = 0)]
    pub fn get_handle_dynamic_exprs(self) -> bool;

    #[method(name = "set_HandleDynamicExprs", args = 1)]
    pub fn set_handle_dynamic_exprs(self, value: bool) -> ();

    #[method(name = "get_HandleClassicExprsSyntax", args = 0)]
    pub fn get_handle_classic_exprs_syntax(self) -> bool;

    #[method(name = "set_HandleClassicExprsSyntax", args = 1)]
    pub fn set_handle_classic_exprs_syntax(self, value: bool) -> ();

    #[method(name = "get_HasPendingCommand", args = 0)]
    pub fn get_has_pending_command(self) -> bool;

    #[method(name = "get_CurrentPendingCommand", args = 0)]
    pub fn get_current_pending_command(self) -> ::unity2::Il2CppString;

    #[method(name = "get_ClassicPrompt", args = 0)]
    pub fn get_classic_prompt(self) -> ::unity2::Il2CppString;

    #[method(name = "Evaluate", args = 1)]
    pub fn evaluate(
        self,
        input: ::unity2::Il2CppString,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;
}

#[cfg(feature = "moon_sharp-interpreter-repl-replinterpreter")]
impl ReplInterpreter {
    pub fn new(script: crate::moon_sharp::interpreter::script::Script) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReplInterpreter),
                ::core::stringify!(new),
            )
        });
        <Self as IReplInterpreterMethods>::ctor(this, script);
        this
    }
}
