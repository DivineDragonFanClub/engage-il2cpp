
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/system/reflection/assemblydelaysignattribute/AssemblyDelaySignAttribute.md")))]
#[::unity2::class(namespace = "System.Reflection", name = "AssemblyDelaySignAttribute")]
pub struct AssemblyDelaySignAttribute {
    #[rename(name = "m_delaySign")]
    pub m_delay_sign: bool,
}

#[cfg(feature = "system-reflection-assemblydelaysignattribute")]
#[::unity2::methods]
impl AssemblyDelaySignAttribute {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, delay_sign: bool) -> ();
}

#[cfg(feature = "system-reflection-assemblydelaysignattribute")]
impl AssemblyDelaySignAttribute {
    pub fn new(delay_sign: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AssemblyDelaySignAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IAssemblyDelaySignAttributeMethods>::ctor(this, delay_sign);
        this
    }
}
