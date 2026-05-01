
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/u2d/spriteatlas/SpriteAtlas.md")))]
#[::unity2::class(namespace = "UnityEngine.U2D", name = "SpriteAtlas")]
#[parent(crate::unity_engine::object_2::Object_2)]
pub struct SpriteAtlas {}

#[cfg(feature = "unity_engine-u2d-spriteatlas")]
#[::unity2::methods]
impl SpriteAtlas {
    #[method(name = "get_isVariant", args = 0)]
    pub fn get_is_variant(self) -> bool;

    #[method(name = "get_tag", args = 0)]
    pub fn get_tag(self) -> ::unity2::Il2CppString;

    #[method(name = "get_spriteCount", args = 0)]
    pub fn get_sprite_count(self) -> i32;

    #[method(name = "CanBindTo", args = 1)]
    pub fn can_bind_to(self, sprite: crate::unity_engine::sprite::Sprite) -> bool;

    #[method(name = "GetSprite", args = 1)]
    pub fn get_sprite(self, name: ::unity2::Il2CppString) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "GetSprites", args = 1)]
    pub fn get_sprites(self, sprites: ::unity2::Array<crate::unity_engine::sprite::Sprite>) -> i32;

    #[method(name = "GetSprites", args = 2)]
    pub fn get_sprites_2(
        self,
        sprites: ::unity2::Array<crate::unity_engine::sprite::Sprite>,
        name: ::unity2::Il2CppString,
    ) -> i32;

    #[method(name = "GetSpritesScripting", args = 1)]
    pub fn get_sprites_scripting(
        self,
        sprites: ::unity2::Array<crate::unity_engine::sprite::Sprite>,
    ) -> i32;

    #[method(name = "GetSpritesWithNameScripting", args = 2)]
    pub fn get_sprites_with_name_scripting(
        self,
        sprites: ::unity2::Array<crate::unity_engine::sprite::Sprite>,
        name: ::unity2::Il2CppString,
    ) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "unity_engine-u2d-spriteatlas")]
impl SpriteAtlas {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SpriteAtlas),
                ::core::stringify!(new),
            )
        });
        <Self as ISpriteAtlasMethods>::ctor(this);
        this
    }
}
