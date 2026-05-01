
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/playerprefs_string/PlayerPrefs_String.md")))]
#[::unity2::class(namespace = "Combat", name = "PlayerPrefs_String")]
#[parent(crate::system::object::Object)]
pub struct PlayerPrefs_String {
    #[rename(name = "key")]
    pub key: ::unity2::Il2CppString,
}

#[cfg(feature = "combat-playerprefs_string")]
#[::unity2::methods]
impl PlayerPrefs_String {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "combat-playerprefs_string")]
impl PlayerPrefs_String {
    pub fn new(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlayerPrefs_String),
                ::core::stringify!(new),
            )
        });
        <Self as IPlayerPrefs_StringMethods>::ctor(this, name);
        this
    }
}
