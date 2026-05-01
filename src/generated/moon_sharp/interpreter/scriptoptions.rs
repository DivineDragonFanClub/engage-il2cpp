
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/scriptoptions/ScriptOptions.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "ScriptOptions")]
#[parent(crate::system::object::Object)]
pub struct ScriptOptions {}

#[cfg(feature = "moon_sharp-interpreter-scriptoptions")]
#[::unity2::methods]
impl ScriptOptions {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        defaults: crate::moon_sharp::interpreter::scriptoptions::ScriptOptions,
    ) -> ();

    #[method(name = "get_ScriptLoader", args = 0)]
    pub fn get_script_loader(
        self,
    ) -> crate::moon_sharp::interpreter::loaders::iscriptloader::IScriptLoader;

    #[method(name = "set_ScriptLoader", args = 1)]
    pub fn set_script_loader(
        self,
        value: crate::moon_sharp::interpreter::loaders::iscriptloader::IScriptLoader,
    ) -> ();

    #[method(name = "get_DebugPrint", args = 0)]
    pub fn get_debug_print(self) -> crate::system::action_1::Action_1<::unity2::Il2CppString>;

    #[method(name = "set_DebugPrint", args = 1)]
    pub fn set_debug_print(
        self,
        value: crate::system::action_1::Action_1<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "get_DebugInput", args = 0)]
    pub fn get_debug_input(
        self,
    ) -> crate::system::func_2::Func_2<::unity2::Il2CppString, ::unity2::Il2CppString>;

    #[method(name = "set_DebugInput", args = 1)]
    pub fn set_debug_input(
        self,
        value: crate::system::func_2::Func_2<::unity2::Il2CppString, ::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "get_UseLuaErrorLocations", args = 0)]
    pub fn get_use_lua_error_locations(self) -> bool;

    #[method(name = "set_UseLuaErrorLocations", args = 1)]
    pub fn set_use_lua_error_locations(self, value: bool) -> ();

    #[method(name = "get_ColonOperatorClrCallbackBehaviour", args = 0)]
    pub fn get_colon_operator_clr_callback_behaviour(
        self,
    ) -> crate::moon_sharp::interpreter::colonoperatorbehaviour::ColonOperatorBehaviour;

    #[method(name = "set_ColonOperatorClrCallbackBehaviour", args = 1)]
    pub fn set_colon_operator_clr_callback_behaviour(
        self,
        value: crate::moon_sharp::interpreter::colonoperatorbehaviour::ColonOperatorBehaviour,
    ) -> ();

    #[method(name = "get_Stdin", args = 0)]
    pub fn get_stdin(self) -> crate::system::io::stream::Stream;

    #[method(name = "set_Stdin", args = 1)]
    pub fn set_stdin(self, value: crate::system::io::stream::Stream) -> ();

    #[method(name = "get_Stdout", args = 0)]
    pub fn get_stdout(self) -> crate::system::io::stream::Stream;

    #[method(name = "set_Stdout", args = 1)]
    pub fn set_stdout(self, value: crate::system::io::stream::Stream) -> ();

    #[method(name = "get_Stderr", args = 0)]
    pub fn get_stderr(self) -> crate::system::io::stream::Stream;

    #[method(name = "set_Stderr", args = 1)]
    pub fn set_stderr(self, value: crate::system::io::stream::Stream) -> ();

    #[method(name = "get_TailCallOptimizationThreshold", args = 0)]
    pub fn get_tail_call_optimization_threshold(self) -> i32;

    #[method(name = "set_TailCallOptimizationThreshold", args = 1)]
    pub fn set_tail_call_optimization_threshold(self, value: i32) -> ();

    #[method(name = "get_CheckThreadAccess", args = 0)]
    pub fn get_check_thread_access(self) -> bool;

    #[method(name = "set_CheckThreadAccess", args = 1)]
    pub fn set_check_thread_access(self, value: bool) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-scriptoptions")]
impl ScriptOptions {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptOptions),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptOptionsMethods>::ctor(this);
        this
    }

    pub fn new_2(defaults: crate::moon_sharp::interpreter::scriptoptions::ScriptOptions) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptOptions),
                ::core::stringify!(new_2),
            )
        });
        <Self as IScriptOptionsMethods>::ctor_2(this, defaults);
        this
    }
}
