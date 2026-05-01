
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/symbolref/SymbolRef.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter", name = "SymbolRef")]
#[parent(crate::system::object::Object)]
pub struct SymbolRef {
    #[static_field]
    #[rename(name = "s_DefaultEnv")]
    pub s_default_env: crate::moon_sharp::interpreter::symbolref::SymbolRef,
    #[rename(name = "i_Type")]
    pub i_type: crate::moon_sharp::interpreter::symbolreftype::SymbolRefType,
    #[rename(name = "i_Env")]
    pub i_env: crate::moon_sharp::interpreter::symbolref::SymbolRef,
    #[rename(name = "i_Index")]
    pub i_index: i32,
    #[rename(name = "i_Name")]
    pub i_name: ::unity2::Il2CppString,
}

#[cfg(feature = "moon_sharp-interpreter-symbolref")]
#[::unity2::methods]
impl SymbolRef {
    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> crate::moon_sharp::interpreter::symbolreftype::SymbolRefType;

    #[method(name = "get_Index", args = 0)]
    pub fn get_index(self) -> i32;

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Environment", args = 0)]
    pub fn get_environment(self) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "get_DefaultEnv", args = 0)]
    pub fn get_default_env() -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "Global", args = 2)]
    pub fn global(
        name: ::unity2::Il2CppString,
        env_symbol: crate::moon_sharp::interpreter::symbolref::SymbolRef,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "Local", args = 2)]
    pub fn local(
        name: ::unity2::Il2CppString,
        index: i32,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "Upvalue", args = 2)]
    pub fn upvalue(
        name: ::unity2::Il2CppString,
        index: i32,
    ) -> crate::moon_sharp::interpreter::symbolref::SymbolRef;

    #[method(name = "ToString", args = 0)]
    pub fn to_string(self) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "moon_sharp-interpreter-symbolref")]
impl SymbolRef {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SymbolRef),
                ::core::stringify!(new),
            )
        });
        <Self as ISymbolRefMethods>::ctor(this);
        this
    }
}
