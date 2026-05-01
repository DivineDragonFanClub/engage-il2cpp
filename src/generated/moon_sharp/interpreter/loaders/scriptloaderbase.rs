
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/loaders/scriptloaderbase/ScriptLoaderBase.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.Loaders", name = "ScriptLoaderBase")]
#[parent(crate::system::object::Object)]
pub struct ScriptLoaderBase {}

#[cfg(feature = "moon_sharp-interpreter-loaders-scriptloaderbase")]
#[::unity2::methods]
impl ScriptLoaderBase {
    #[method(name = "ScriptFileExists", args = 1)]
    pub fn script_file_exists(self, name: ::unity2::Il2CppString) -> bool;

    #[method(name = "LoadFile", args = 2)]
    pub fn load_file(
        self,
        file: ::unity2::Il2CppString,
        global_context: crate::moon_sharp::interpreter::table::Table,
    ) -> crate::system::object::Object;

    #[method(name = "ResolveModuleName", args = 2)]
    pub fn resolve_module_name(
        self,
        modname: ::unity2::Il2CppString,
        paths: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ::unity2::Il2CppString;

    #[method(name = "ResolveModuleName", args = 2)]
    pub fn resolve_module_name_2(
        self,
        modname: ::unity2::Il2CppString,
        global_context: crate::moon_sharp::interpreter::table::Table,
    ) -> ::unity2::Il2CppString;

    #[method(name = "get_ModulePaths", args = 0)]
    pub fn get_module_paths(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_ModulePaths", args = 1)]
    pub fn set_module_paths(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "UnpackStringPaths", args = 1)]
    pub fn unpack_string_paths(
        str: ::unity2::Il2CppString,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetDefaultEnvironmentPaths", args = 0)]
    pub fn get_default_environment_paths() -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "ResolveFileName", args = 2)]
    pub fn resolve_file_name(
        self,
        filename: ::unity2::Il2CppString,
        global_context: crate::moon_sharp::interpreter::table::Table,
    ) -> ::unity2::Il2CppString;

    #[method(name = "get_IgnoreLuaPathGlobal", args = 0)]
    pub fn get_ignore_lua_path_global(self) -> bool;

    #[method(name = "set_IgnoreLuaPathGlobal", args = 1)]
    pub fn set_ignore_lua_path_global(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-loaders-scriptloaderbase")]
impl ScriptLoaderBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptLoaderBase),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptLoaderBaseMethods>::ctor(this);
        this
    }
}
