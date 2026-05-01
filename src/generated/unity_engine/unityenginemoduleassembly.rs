
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/unityenginemoduleassembly/UnityEngineModuleAssembly.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "UnityEngineModuleAssembly")]
pub struct UnityEngineModuleAssembly {}

#[cfg(feature = "unity_engine-unityenginemoduleassembly")]
#[::unity2::methods]
impl UnityEngineModuleAssembly {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-unityenginemoduleassembly")]
impl UnityEngineModuleAssembly {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(UnityEngineModuleAssembly),
                ::core::stringify!(new),
            )
        });
        <Self as IUnityEngineModuleAssemblyMethods>::ctor(this);
        this
    }
}
