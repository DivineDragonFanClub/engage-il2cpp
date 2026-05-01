
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/playerprefs_float/PlayerPrefs_Float.md")))]
#[::unity2::class(namespace = "Combat", name = "PlayerPrefs_Float")]
#[parent(crate::system::object::Object)]
pub struct PlayerPrefs_Float {
    #[rename(name = "key")]
    pub key: ::unity2::Il2CppString,
}

#[cfg(feature = "combat-playerprefs_float")]
#[::unity2::methods]
impl PlayerPrefs_Float {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> f32;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: f32) -> ();
}

#[cfg(feature = "combat-playerprefs_float")]
impl PlayerPrefs_Float {
    pub fn new(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(PlayerPrefs_Float),
                ::core::stringify!(new),
            )
        });
        <Self as IPlayerPrefs_FloatMethods>::ctor(this, name);
        this
    }
}
