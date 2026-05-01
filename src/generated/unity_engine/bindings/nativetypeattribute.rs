
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/bindings/nativetypeattribute/NativeTypeAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine.Bindings", name = "NativeTypeAttribute")]
pub struct NativeTypeAttribute {}

#[cfg(feature = "unity_engine-bindings-nativetypeattribute")]
#[::unity2::methods]
impl NativeTypeAttribute {
    #[method(name = "set_Header", args = 1)]
    pub fn set_header(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "set_IntermediateScriptingStructName", args = 1)]
    pub fn set_intermediate_scripting_struct_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "set_CodegenOptions", args = 1)]
    pub fn set_codegen_options(
        self,
        value: crate::unity_engine::bindings::codegenoptions::CodegenOptions,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(
        self,
        codegen_options: crate::unity_engine::bindings::codegenoptions::CodegenOptions,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_3(self, header: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_4(
        self,
        codegen_options: crate::unity_engine::bindings::codegenoptions::CodegenOptions,
        intermediate_struct_name: ::unity2::Il2CppString,
    ) -> ();
}

#[cfg(feature = "unity_engine-bindings-nativetypeattribute")]
impl NativeTypeAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NativeTypeAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as INativeTypeAttributeMethods>::ctor(this);
        this
    }

    pub fn new_2(
        codegen_options: crate::unity_engine::bindings::codegenoptions::CodegenOptions,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NativeTypeAttribute),
                ::core::stringify!(new_2),
            )
        });
        <Self as INativeTypeAttributeMethods>::ctor_2(this, codegen_options);
        this
    }

    pub fn new_3(header: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NativeTypeAttribute),
                ::core::stringify!(new_3),
            )
        });
        <Self as INativeTypeAttributeMethods>::ctor_3(this, header);
        this
    }

    pub fn new_4(
        codegen_options: crate::unity_engine::bindings::codegenoptions::CodegenOptions,
        intermediate_struct_name: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NativeTypeAttribute),
                ::core::stringify!(new_4),
            )
        });
        <Self as INativeTypeAttributeMethods>::ctor_4(
            this,
            codegen_options,
            intermediate_struct_name,
        );
        this
    }
}
