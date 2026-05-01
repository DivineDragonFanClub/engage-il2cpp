
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/spriteatlasmanager_2/SpriteAtlasManager_2.md")))]
#[::unity2::class(namespace = "App", name = "SpriteAtlasManager")]
#[parent(crate::system::object::Object)]
pub struct SpriteAtlasManager_2 {
    #[rename(name = "m_Handle")]
    pub m_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    >,
    #[rename(name = "m_SpriteAtlas")]
    pub m_sprite_atlas: crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    #[rename(name = "m_CacheTable")]
    pub m_cache_table: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::unity_engine::sprite::Sprite,
    >,
}

#[cfg(feature = "app-spriteatlasmanager_2")]
#[::unity2::methods]
impl SpriteAtlasManager_2 {
    #[method(name = "LoadAsync", args = 1)]
    pub fn load_async(self, path: ::unity2::Il2CppString) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "IsLoaded", args = 0)]
    pub fn is_loaded(self) -> bool;

    #[method(name = "Unload", args = 0)]
    pub fn unload(self) -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(self, name: ::unity2::Il2CppString) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGet", args = 1)]
    pub fn try_get(self, name: ::unity2::Il2CppString) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "ClearCache", args = 0)]
    pub fn clear_cache(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-spriteatlasmanager_2")]
impl SpriteAtlasManager_2 {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SpriteAtlasManager_2),
                ::core::stringify!(new),
            )
        });
        <Self as ISpriteAtlasManager_2Methods>::ctor(this);
        this
    }
}
