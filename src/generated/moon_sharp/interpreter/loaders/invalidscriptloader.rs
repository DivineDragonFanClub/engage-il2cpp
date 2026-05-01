
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/loaders/invalidscriptloader/InvalidScriptLoader.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Loaders",
    name = "InvalidScriptLoader"
)]
#[parent(crate::system::object::Object)]
pub struct InvalidScriptLoader {
    #[rename(name = "m_Error")]
    pub m_error: ::unity2::Il2CppString,
}

#[cfg(feature = "moon_sharp-interpreter-loaders-invalidscriptloader")]
#[::unity2::methods]
impl InvalidScriptLoader {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, frameworkname: ::unity2::Il2CppString) -> ();

    #[method(name = "LoadFile", args = 2)]
    pub fn load_file(
        self,
        file: ::unity2::Il2CppString,
        global_context: crate::moon_sharp::interpreter::table::Table,
    ) -> crate::system::object::Object;

    #[method(name = "ResolveFileName", args = 2)]
    pub fn resolve_file_name(
        self,
        filename: ::unity2::Il2CppString,
        global_context: crate::moon_sharp::interpreter::table::Table,
    ) -> ::unity2::Il2CppString;

    #[method(name = "ResolveModuleName", args = 2)]
    pub fn resolve_module_name(
        self,
        modname: ::unity2::Il2CppString,
        global_context: crate::moon_sharp::interpreter::table::Table,
    ) -> ::unity2::Il2CppString;
}

#[cfg(feature = "moon_sharp-interpreter-loaders-invalidscriptloader")]
impl InvalidScriptLoader {
    pub fn new(frameworkname: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvalidScriptLoader),
                ::core::stringify!(new),
            )
        });
        <Self as IInvalidScriptLoaderMethods>::ctor(this, frameworkname);
        this
    }
}
