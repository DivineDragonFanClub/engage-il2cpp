
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/bindings/visibletoothermodulesattribute/VisibleToOtherModulesAttribute.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Bindings",
    name = "VisibleToOtherModulesAttribute"
)]
pub struct VisibleToOtherModulesAttribute {}

#[cfg(feature = "unity_engine-bindings-visibletoothermodulesattribute")]
#[::unity2::methods]
impl VisibleToOtherModulesAttribute {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, modules: ::unity2::Array<::unity2::Il2CppString>) -> ();
}

#[cfg(feature = "unity_engine-bindings-visibletoothermodulesattribute")]
impl VisibleToOtherModulesAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VisibleToOtherModulesAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IVisibleToOtherModulesAttributeMethods>::ctor(this);
        this
    }

    pub fn new_2(modules: ::unity2::Array<::unity2::Il2CppString>) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VisibleToOtherModulesAttribute),
                ::core::stringify!(new_2),
            )
        });
        <Self as IVisibleToOtherModulesAttributeMethods>::ctor_2(this, modules);
        this
    }
}
