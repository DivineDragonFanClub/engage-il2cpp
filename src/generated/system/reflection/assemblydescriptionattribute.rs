
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/assemblydescriptionattribute/AssemblyDescriptionAttribute.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "AssemblyDescriptionAttribute")]
pub struct AssemblyDescriptionAttribute {
    #[rename(name = "m_description")]
    pub m_description: ::unity2::Il2CppString,
}

#[cfg(feature = "system-reflection-assemblydescriptionattribute")]
#[::unity2::methods]
impl AssemblyDescriptionAttribute {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, description: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "system-reflection-assemblydescriptionattribute")]
impl AssemblyDescriptionAttribute {
    pub fn new(description: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssemblyDescriptionAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IAssemblyDescriptionAttributeMethods>::ctor(this, description);
        this
    }
}
