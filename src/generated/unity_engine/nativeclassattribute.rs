
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/nativeclassattribute/NativeClassAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "NativeClassAttribute")]
pub struct NativeClassAttribute {}

#[cfg(feature = "unity_engine-nativeclassattribute")]
#[::unity2::methods]
impl NativeClassAttribute {
    #[method(name = "set_QualifiedNativeName", args = 1)]
    pub fn set_qualified_native_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "set_Declaration", args = 1)]
    pub fn set_declaration(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, qualified_cpp_name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        qualified_cpp_name: ::unity2::Il2CppString,
        declaration: ::unity2::Il2CppString,
    ) -> ();
}

#[cfg(feature = "unity_engine-nativeclassattribute")]
impl NativeClassAttribute {
    pub fn new(qualified_cpp_name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NativeClassAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as INativeClassAttributeMethods>::ctor(this, qualified_cpp_name);
        this
    }

    pub fn new_2(
        qualified_cpp_name: ::unity2::Il2CppString,
        declaration: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NativeClassAttribute),
                ::core::stringify!(new_2),
            )
        });
        <Self as INativeClassAttributeMethods>::ctor_2(this, qualified_cpp_name, declaration);
        this
    }
}
