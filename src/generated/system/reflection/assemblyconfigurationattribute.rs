
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/assemblyconfigurationattribute/AssemblyConfigurationAttribute.md")))]
#[::unity2::class(
    namespace = "System.Reflection",
    name = "AssemblyConfigurationAttribute"
)]
pub struct AssemblyConfigurationAttribute {
    #[rename(name = "m_configuration")]
    pub m_configuration: ::unity2::Il2CppString,
}

#[cfg(feature = "system-reflection-assemblyconfigurationattribute")]
#[::unity2::methods]
impl AssemblyConfigurationAttribute {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, configuration: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "system-reflection-assemblyconfigurationattribute")]
impl AssemblyConfigurationAttribute {
    pub fn new(configuration: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssemblyConfigurationAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IAssemblyConfigurationAttributeMethods>::ctor(this, configuration);
        this
    }
}
