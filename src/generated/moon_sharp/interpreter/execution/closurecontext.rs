
use crate::system::collections::generic::list_1::IList_1;
use crate::system::collections::generic::list_1::List_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/execution/closurecontext/ClosureContext.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Execution", name = "ClosureContext")]
# [parent (crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: moon_sharp :: interpreter :: dynvalue :: DynValue >)]
pub struct ClosureContext {}

#[cfg(feature = "moon_sharp-interpreter-execution-closurecontext")]
#[::unity2::methods]
impl ClosureContext {
    #[method(name = "get_Symbols", args = 0)]
    pub fn get_symbols(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Symbols", args = 1)]
    pub fn set_symbols(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        symbols: ::unity2::Array<crate::moon_sharp::interpreter::symbolref::SymbolRef>,
        values: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor_2(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-execution-closurecontext")]
impl ClosureContext {
    pub fn new(
        symbols: ::unity2::Array<crate::moon_sharp::interpreter::symbolref::SymbolRef>,
        values: crate::system::collections::generic::ienumerable_1::IEnumerable_1<
            crate::moon_sharp::interpreter::dynvalue::DynValue,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClosureContext),
                ::core::stringify!(new),
            )
        });
        <Self as IClosureContextMethods>::ctor(this, symbols, values);
        this
    }

    pub fn new_2() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClosureContext),
                ::core::stringify!(new_2),
            )
        });
        <Self as IClosureContextMethods>::ctor_2(this);
        this
    }
}
