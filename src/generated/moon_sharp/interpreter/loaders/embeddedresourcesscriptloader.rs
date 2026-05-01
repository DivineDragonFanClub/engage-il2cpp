
use crate::moon_sharp::interpreter::loaders::scriptloaderbase::IScriptLoaderBase;
use crate::moon_sharp::interpreter::loaders::scriptloaderbase::ScriptLoaderBase;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/loaders/embeddedresourcesscriptloader/EmbeddedResourcesScriptLoader.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Loaders",
    name = "EmbeddedResourcesScriptLoader"
)]
#[parent(crate::moon_sharp::interpreter::loaders::scriptloaderbase::ScriptLoaderBase)]
pub struct EmbeddedResourcesScriptLoader {
    #[rename(name = "m_ResourceAssembly")]
    pub m_resource_assembly: crate::system::reflection::assembly::Assembly,
    #[rename(name = "m_ResourceNames")]
    pub m_resource_names:
        crate::system::collections::generic::hashset_1::HashSet_1<::unity2::Il2CppString>,
    #[rename(name = "m_Namespace")]
    pub m_namespace: ::unity2::Il2CppString,
}

#[cfg(feature = "moon_sharp-interpreter-loaders-embeddedresourcesscriptloader")]
#[::unity2::methods]
impl EmbeddedResourcesScriptLoader {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, resource_assembly: crate::system::reflection::assembly::Assembly) -> ();

    #[method(name = "FileNameToResource", args = 1)]
    pub fn file_name_to_resource(self, file: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "ScriptFileExists", args = 1)]
    pub fn script_file_exists(self, name: ::unity2::Il2CppString) -> bool;

    #[method(name = "LoadFile", args = 2)]
    pub fn load_file(
        self,
        file: ::unity2::Il2CppString,
        global_context: crate::moon_sharp::interpreter::table::Table,
    ) -> crate::system::object::Object;
}

#[cfg(feature = "moon_sharp-interpreter-loaders-embeddedresourcesscriptloader")]
impl EmbeddedResourcesScriptLoader {
    pub fn new(resource_assembly: crate::system::reflection::assembly::Assembly) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EmbeddedResourcesScriptLoader),
                ::core::stringify!(new),
            )
        });
        <Self as IEmbeddedResourcesScriptLoaderMethods>::ctor(this, resource_assembly);
        this
    }
}
