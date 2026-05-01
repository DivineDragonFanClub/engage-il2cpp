
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/godfacepicture/GodFacePicture.md")))]
#[::unity2::class(namespace = "App", name = "GodFacePicture")]
#[parent(crate::system::object::Object)]
pub struct GodFacePicture {
    #[static_field]
    #[rename(name = "Path")]
    pub path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_FacePicture")]
    pub s_face_picture: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
}

#[cfg(feature = "app-godfacepicture")]
#[::unity2::methods]
impl GodFacePicture {
    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async() -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading() -> bool;

    #[method(name = "IsLoaded", args = 0)]
    pub fn is_loaded() -> bool;

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(god_data: crate::app::goddata::GodData) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "GetPath", args = 1)]
    pub fn get_path(god_data: crate::app::goddata::GodData) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-godfacepicture")]
impl GodFacePicture {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GodFacePicture),
                ::core::stringify!(new),
            )
        });
        <Self as IGodFacePictureMethods>::ctor(this);
        this
    }
}
