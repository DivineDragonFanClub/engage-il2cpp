
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/commonsprite/CommonSprite.md")))]
#[::unity2::class(namespace = "App", name = "CommonSprite")]
#[parent(crate::system::object::Object)]
pub struct CommonSprite {
    #[static_field]
    #[rename(name = "s_AtlasManager")]
    pub s_atlas_manager: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
}

#[cfg(feature = "app-commonsprite")]
#[::unity2::methods]
impl CommonSprite {
    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async() -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = "TryGet", args = 1)]
    pub fn try_get(sprite_name: ::unity2::Il2CppString) -> crate::unity_engine::sprite::Sprite;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-commonsprite")]
impl CommonSprite {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CommonSprite),
                ::core::stringify!(new),
            )
        });
        <Self as ICommonSpriteMethods>::ctor(this);
        this
    }
}
