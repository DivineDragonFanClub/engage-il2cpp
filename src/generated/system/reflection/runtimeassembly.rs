
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::assembly::Assembly;
use crate::system::reflection::assembly::IAssembly;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/runtimeassembly/RuntimeAssembly.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "RuntimeAssembly")]
#[parent(crate::system::reflection::assembly::Assembly)]
pub struct RuntimeAssembly {}

#[cfg(feature = "system-reflection-runtimeassembly")]
#[::unity2::methods]
impl RuntimeAssembly {
    #[method(name = "GetName", args = 1)]
    pub fn get_name(
        self,
        copied_name: bool,
    ) -> crate::system::reflection::assemblyname::AssemblyName;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-runtimeassembly")]
impl RuntimeAssembly {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RuntimeAssembly),
                ::core::stringify!(new),
            )
        });
        <Self as IRuntimeAssemblyMethods>::ctor(this);
        this
    }
}
