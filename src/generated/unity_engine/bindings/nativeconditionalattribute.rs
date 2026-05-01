
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/bindings/nativeconditionalattribute/NativeConditionalAttribute.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Bindings",
    name = "NativeConditionalAttribute"
)]
pub struct NativeConditionalAttribute {}

#[cfg(feature = "unity_engine-bindings-nativeconditionalattribute")]
#[::unity2::methods]
impl NativeConditionalAttribute {
    #[method(name = "set_Condition", args = 1)]
    pub fn set_condition(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "set_StubReturnStatement", args = 1)]
    pub fn set_stub_return_statement(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "set_Enabled", args = 1)]
    pub fn set_enabled(self, value: bool) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, condition: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        condition: ::unity2::Il2CppString,
        stub_return_statement: ::unity2::Il2CppString,
    ) -> ();
}

#[cfg(feature = "unity_engine-bindings-nativeconditionalattribute")]
impl NativeConditionalAttribute {
    pub fn new(condition: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NativeConditionalAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as INativeConditionalAttributeMethods>::ctor(this, condition);
        this
    }

    pub fn new_2(
        condition: ::unity2::Il2CppString,
        stub_return_statement: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NativeConditionalAttribute),
                ::core::stringify!(new_2),
            )
        });
        <Self as INativeConditionalAttributeMethods>::ctor_2(
            this,
            condition,
            stub_return_statement,
        );
        this
    }
}
