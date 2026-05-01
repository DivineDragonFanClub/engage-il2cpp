
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fortunetellingcard/FortuneTellingCard.md")))]
#[::unity2::class(namespace = "App", name = "FortuneTellingCard")]
#[parent(crate::system::object::Object)]
pub struct FortuneTellingCard {
    #[static_field]
    #[rename(name = "Path")]
    pub path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_FortuneTellingCard")]
    pub s_fortune_telling_card: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
}

#[cfg(feature = "app-fortunetellingcard")]
#[::unity2::methods]
impl FortuneTellingCard {
    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async() -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(icon_name: ::unity2::Il2CppString) -> crate::unity_engine::sprite::Sprite;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-fortunetellingcard")]
impl FortuneTellingCard {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FortuneTellingCard),
                ::core::stringify!(new),
            )
        });
        <Self as IFortuneTellingCardMethods>::ctor(this);
        this
    }
}
