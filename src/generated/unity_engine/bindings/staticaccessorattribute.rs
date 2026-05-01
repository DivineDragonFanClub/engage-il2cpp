
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/bindings/staticaccessorattribute/StaticAccessorAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine.Bindings", name = "StaticAccessorAttribute")]
pub struct StaticAccessorAttribute {}

#[cfg(feature = "unity_engine-bindings-staticaccessorattribute")]
#[::unity2::methods]
impl StaticAccessorAttribute {
    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "set_Type", args = 1)]
    pub fn set_type(
        self,
        value: crate::unity_engine::bindings::staticaccessortype::StaticAccessorType,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        name: ::unity2::Il2CppString,
        r#type: crate::unity_engine::bindings::staticaccessortype::StaticAccessorType,
    ) -> ();
}

#[cfg(feature = "unity_engine-bindings-staticaccessorattribute")]
impl StaticAccessorAttribute {
    pub fn new(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StaticAccessorAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as IStaticAccessorAttributeMethods>::ctor(this, name);
        this
    }

    pub fn new_2(
        name: ::unity2::Il2CppString,
        r#type: crate::unity_engine::bindings::staticaccessortype::StaticAccessorType,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StaticAccessorAttribute),
                ::core::stringify!(new_2),
            )
        });
        <Self as IStaticAccessorAttributeMethods>::ctor_2(this, name, r#type);
        this
    }
}
