
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::bindings::nativemethodattribute::INativeMethodAttribute;
use crate::unity_engine::bindings::nativemethodattribute::NativeMethodAttribute;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/bindings/nativepropertyattribute/NativePropertyAttribute.md")))]
#[::unity2::class(namespace = "UnityEngine.Bindings", name = "NativePropertyAttribute")]
#[parent(crate::unity_engine::bindings::nativemethodattribute::NativeMethodAttribute)]
pub struct NativePropertyAttribute {}

#[cfg(feature = "unity_engine-bindings-nativepropertyattribute")]
#[::unity2::methods]
impl NativePropertyAttribute {
    #[method(name = "set_TargetType", args = 1)]
    pub fn set_target_type(
        self,
        value: crate::unity_engine::bindings::targettype::TargetType,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_3(
        self,
        name: ::unity2::Il2CppString,
        is_free: bool,
        target_type: crate::unity_engine::bindings::targettype::TargetType,
    ) -> ();
}

#[cfg(feature = "unity_engine-bindings-nativepropertyattribute")]
impl NativePropertyAttribute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NativePropertyAttribute),
                ::core::stringify!(new),
            )
        });
        <Self as INativePropertyAttributeMethods>::ctor(this);
        this
    }

    pub fn new_2(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NativePropertyAttribute),
                ::core::stringify!(new_2),
            )
        });
        <Self as INativePropertyAttributeMethods>::ctor_2(this, name);
        this
    }

    pub fn new_3(
        name: ::unity2::Il2CppString,
        is_free: bool,
        target_type: crate::unity_engine::bindings::targettype::TargetType,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NativePropertyAttribute),
                ::core::stringify!(new_3),
            )
        });
        <Self as INativePropertyAttributeMethods>::ctor_3(this, name, is_free, target_type);
        this
    }
}
