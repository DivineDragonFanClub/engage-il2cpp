
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/addressable_assets/invalidkeyexception/InvalidKeyException.md")))]
#[::unity2::class(
    namespace = "UnityEngine.AddressableAssets",
    name = "InvalidKeyException"
)]
pub struct InvalidKeyException {}

#[cfg(feature = "unity_engine-addressable_assets-invalidkeyexception")]
#[::unity2::methods]
impl InvalidKeyException {
    #[method(name = "get_Key", args = 0)]
    pub fn get_key(self) -> crate::system::object::Object;

    #[method(name = "set_Key", args = 1)]
    pub fn set_key(self, value: crate::system::object::Object) -> ();

    #[method(name = "get_Type", args = 0)]
    pub fn get_type(self) -> ::unity2::SystemType;

    #[method(name = "set_Type", args = 1)]
    pub fn set_type(self, value: ::unity2::SystemType) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, key: crate::system::object::Object) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, key: crate::system::object::Object, r#type: ::unity2::SystemType) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor_3(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_4(self, message: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Message", args = 0)]
    pub fn get_message(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "unity_engine-addressable_assets-invalidkeyexception")]
impl InvalidKeyException {
    pub fn new(key: crate::system::object::Object) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvalidKeyException),
                ::core::stringify!(new),
            )
        });
        <Self as IInvalidKeyExceptionMethods>::ctor(this, key);
        this
    }

    pub fn new_2(key: crate::system::object::Object, r#type: ::unity2::SystemType) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvalidKeyException),
                ::core::stringify!(new_2),
            )
        });
        <Self as IInvalidKeyExceptionMethods>::ctor_2(this, key, r#type);
        this
    }

    pub fn new_3() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvalidKeyException),
                ::core::stringify!(new_3),
            )
        });
        <Self as IInvalidKeyExceptionMethods>::ctor_3(this);
        this
    }

    pub fn new_4(message: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(InvalidKeyException),
                ::core::stringify!(new_4),
            )
        });
        <Self as IInvalidKeyExceptionMethods>::ctor_4(this, message);
        this
    }
}
