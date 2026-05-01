
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/execution/runtimescopeframe/RuntimeScopeFrame.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Execution",
    name = "RuntimeScopeFrame"
)]
#[parent(crate::system::object::Object)]
pub struct RuntimeScopeFrame {}

#[cfg(feature = "moon_sharp-interpreter-execution-runtimescopeframe")]
#[::unity2::methods]
impl RuntimeScopeFrame {
    #[method(name = "get_DebugSymbols", args = 0)]
    pub fn get_debug_symbols(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::moon_sharp::interpreter::symbolref::SymbolRef,
    >;

    #[method(name = "set_DebugSymbols", args = 1)]
    pub fn set_debug_symbols(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::moon_sharp::interpreter::symbolref::SymbolRef,
        >,
    ) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_ToFirstBlock", args = 0)]
    pub fn get_to_first_block(self) -> i32;

    #[method(name = "set_ToFirstBlock", args = 1)]
    pub fn set_to_first_block(self, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "moon_sharp-interpreter-execution-runtimescopeframe")]
impl RuntimeScopeFrame {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RuntimeScopeFrame),
                ::core::stringify!(new),
            )
        });
        <Self as IRuntimeScopeFrameMethods>::ctor(this);
        this
    }
}
