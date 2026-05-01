
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::reflection::module::IModule;
use crate::system::reflection::module::Module;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/runtimemodule/RuntimeModule.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "RuntimeModule")]
#[parent(crate::system::reflection::module::Module)]
pub struct RuntimeModule {}

#[cfg(feature = "system-reflection-runtimemodule")]
#[::unity2::methods]
impl RuntimeModule {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "system-reflection-runtimemodule")]
impl RuntimeModule {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RuntimeModule),
                ::core::stringify!(new),
            )
        });
        <Self as IRuntimeModuleMethods>::ctor(this);
        this
    }
}
