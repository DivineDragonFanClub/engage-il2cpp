
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::assembly::Assembly;
use crate::system::reflection::assembly::IAssembly;
use crate::system::reflection::runtimeassembly::IRuntimeAssembly;
use crate::system::reflection::runtimeassembly::RuntimeAssembly;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/monoassembly/MonoAssembly.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "MonoAssembly")]
#[parent(crate::system::reflection::runtimeassembly::RuntimeAssembly)]
pub struct MonoAssembly {}

#[cfg(feature = "system-reflection-monoassembly")]
#[::unity2::methods]
impl MonoAssembly {
    #[method(name = "GetType", args = 3)]
    pub fn get_type(
        self,
        name: ::unity2::Il2CppString,
        throw_on_error: bool,
        ignore_case: bool,
    ) -> ::unity2::SystemType;

    #[method(name = "GetModule", args = 1)]
    pub fn get_module(
        self,
        name: ::unity2::Il2CppString,
    ) -> crate::system::reflection::module::Module;

    #[method(name = "GetModules", args = 1)]
    pub fn get_modules(
        self,
        get_resource_modules: bool,
    ) -> ::unity2::Array<crate::system::reflection::module::Module>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-monoassembly")]
impl MonoAssembly {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MonoAssembly),
                ::core::stringify!(new),
            )
        });
        <Self as IMonoAssemblyMethods>::ctor(this);
        this
    }
}
