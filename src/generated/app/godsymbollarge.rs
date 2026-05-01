
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godsymbollarge/GodSymbolLarge.md")))]
#[::unity2::class(namespace = "App", name = "GodSymbolLarge")]
#[parent(crate::system::object::Object)]
pub struct GodSymbolLarge {
    #[static_field]
    #[rename(name = "Path")]
    pub path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_GodSymbolLarge")]
    pub s_god_symbol_large: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
}

#[cfg(feature = "app-godsymbollarge")]
#[::unity2::methods]
impl GodSymbolLarge {
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

#[cfg(feature = "app-godsymbollarge")]
impl GodSymbolLarge {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodSymbolLarge),
                ::core::stringify!(new),
            )
        });
        <Self as IGodSymbolLargeMethods>::ctor(this);
        this
    }
}
