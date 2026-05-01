
use crate::moon_sharp::interpreter::loaders::scriptloaderbase::IScriptLoaderBase;
use crate::moon_sharp::interpreter::loaders::scriptloaderbase::ScriptLoaderBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/loaders/unityassetsscriptloader/UnityAssetsScriptLoader.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Loaders",
    name = "UnityAssetsScriptLoader"
)]
#[parent(crate::moon_sharp::interpreter::loaders::scriptloaderbase::ScriptLoaderBase)]
pub struct UnityAssetsScriptLoader {
    #[rename(name = "m_Resources")]
    pub m_resources: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        ::unity2::Il2CppString,
    >,
    #[static_field]
    #[rename(name = "DEFAULT_PATH")]
    pub default_path: ::unity2::Il2CppString,
}

#[cfg(feature = "moon_sharp-interpreter-loaders-unityassetsscriptloader")]
#[::unity2::methods]
impl UnityAssetsScriptLoader {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, assets_path: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        script_to_code_map: crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            ::unity2::Il2CppString,
        >,
    ) -> ();

    #[method(name = "LoadResourcesWithReflection", args = 1)]
    pub fn load_resources_with_reflection(self, assets_path: ::unity2::Il2CppString) -> ();

    #[method(name = "GetFileName", args = 1)]
    pub fn get_file_name(self, filename: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "LoadFile", args = 2)]
    pub fn load_file(
        self,
        file: ::unity2::Il2CppString,
        global_context: crate::moon_sharp::interpreter::table::Table,
    ) -> crate::system::object::Object;

    #[method(name = "ScriptFileExists", args = 1)]
    pub fn script_file_exists(self, file: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetLoadedScripts", args = 0)]
    pub fn get_loaded_scripts(self) -> ::unity2::Array<::unity2::Il2CppString>;
}

#[cfg(feature = "moon_sharp-interpreter-loaders-unityassetsscriptloader")]
impl UnityAssetsScriptLoader {
    pub fn new(assets_path: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnityAssetsScriptLoader),
                ::core::stringify!(new),
            )
        });
        <Self as IUnityAssetsScriptLoaderMethods>::ctor(this, assets_path);
        this
    }

    pub fn new_2(
        script_to_code_map: crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            ::unity2::Il2CppString,
        >,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnityAssetsScriptLoader),
                ::core::stringify!(new_2),
            )
        });
        <Self as IUnityAssetsScriptLoaderMethods>::ctor_2(this, script_to_code_map);
        this
    }
}
