
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::module::IModule;
use crate::system::reflection::module::Module;
use crate::system::reflection::runtimemodule::IRuntimeModule;
use crate::system::reflection::runtimemodule::RuntimeModule;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/monomodule/MonoModule.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "MonoModule")]
#[parent(crate::system::reflection::runtimemodule::RuntimeModule)]
pub struct MonoModule {}

#[cfg(feature = "system-reflection-monomodule")]
#[::unity2::methods]
impl MonoModule {
    #[method(name = "get_Assembly", args = 0)]
    pub fn get_assembly(self) -> crate::system::reflection::assembly::Assembly;

    #[method(name = "get_ScopeName", args = 0)]
    pub fn get_scope_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsResource", args = 0)]
    pub fn is_resource(self) -> bool;

    #[method(name = "GetCustomAttributes", args = 2)]
    pub fn get_custom_attributes(
        self,
        attribute_type: ::unity2::SystemType,
        inherit: bool,
    ) -> ::unity2::Array<crate::system::object::Object>;

    #[method(name = "IsDefined", args = 2)]
    pub fn is_defined(self, attribute_type: ::unity2::SystemType, inherit: bool) -> bool;

    #[method(name = "GetRuntimeAssembly", args = 0)]
    pub fn get_runtime_assembly(
        self,
    ) -> crate::system::reflection::runtimeassembly::RuntimeAssembly;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-monomodule")]
impl MonoModule {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MonoModule),
                ::core::stringify!(new),
            )
        });
        <Self as IMonoModuleMethods>::ctor(this);
        this
    }
}
