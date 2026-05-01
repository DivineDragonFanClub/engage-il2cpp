
use crate::combat::characterassett_1::CharacterAssetT_1;
use crate::combat::characterassett_1::ICharacterAssetT_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/animasset/AnimAsset.md")))]
#[::unity2::class(namespace = "Combat", name = "AnimAsset")]
# [parent (crate :: combat :: characterassett_1 :: CharacterAssetT_1 < crate :: unity_engine :: animationclip :: AnimationClip >)]
pub struct AnimAsset {}

#[cfg(feature = "combat-animasset")]
#[::unity2::methods]
impl AnimAsset {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Hash", args = 0)]
    pub fn get_hash(self) -> i32;

    #[method(name = "set_Hash", args = 1)]
    pub fn set_hash(self, value: i32) -> ();

    #[method(name = "SetNameAndHash", args = 2)]
    pub fn set_name_and_hash(self, name: ::unity2::Il2CppString, hash: i32) -> ();
}

#[cfg(feature = "combat-animasset")]
impl AnimAsset {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimAsset),
                ::core::stringify!(new),
            )
        });
        <Self as IAnimAssetMethods>::ctor(this);
        this
    }

    pub fn new_2(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AnimAsset),
                ::core::stringify!(new_2),
            )
        });
        <Self as IAnimAssetMethods>::ctor_2(this, name);
        this
    }
}
