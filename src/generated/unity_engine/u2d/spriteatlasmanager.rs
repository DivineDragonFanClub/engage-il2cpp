
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/u2d/spriteatlasmanager/SpriteAtlasManager.md")))]
#[::unity2::class(namespace = "UnityEngine.U2D", name = "SpriteAtlasManager")]
#[parent(crate::system::object::Object)]
pub struct SpriteAtlasManager {
    #[static_field]
    #[rename(name = "atlasRequested")]
    pub atlas_requested: crate::system::action_2::Action_2<
        ::unity2::Il2CppString,
        crate::system::action_1::Action_1<crate::unity_engine::u2d::spriteatlas::SpriteAtlas>,
    >,
    #[static_field]
    #[rename(name = "atlasRegistered")]
    pub atlas_registered:
        crate::system::action_1::Action_1<crate::unity_engine::u2d::spriteatlas::SpriteAtlas>,
}

#[cfg(feature = "unity_engine-u2d-spriteatlasmanager")]
#[::unity2::methods]
impl SpriteAtlasManager {
    #[method(name = "RequestAtlas", args = 1)]
    pub fn request_atlas(tag: ::unity2::Il2CppString) -> bool;

    #[method(name = "add_atlasRegistered", args = 1)]
    pub fn add_atlas_registered(
        value: crate::system::action_1::Action_1<
            crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
        >,
    ) -> ();

    #[method(name = "remove_atlasRegistered", args = 1)]
    pub fn remove_atlas_registered(
        value: crate::system::action_1::Action_1<
            crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
        >,
    ) -> ();

    #[method(name = "PostRegisteredAtlas", args = 1)]
    pub fn post_registered_atlas(
        sprite_atlas: crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    ) -> ();

    #[method(name = "Register", args = 1)]
    pub fn register(sprite_atlas: crate::unity_engine::u2d::spriteatlas::SpriteAtlas) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
