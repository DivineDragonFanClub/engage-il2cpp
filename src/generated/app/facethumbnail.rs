
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/facethumbnail/FaceThumbnail.md")))]
#[::unity2::class(namespace = "App", name = "FaceThumbnail")]
#[parent(crate::system::object::Object)]
pub struct FaceThumbnail {
    #[static_field]
    #[rename(name = "Path")]
    pub path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_FaceThumb")]
    pub s_face_thumb: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
}

#[cfg(feature = "app-facethumbnail")]
#[::unity2::methods]
impl FaceThumbnail {
    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async() -> ();

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(unit: crate::app::unit::Unit) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "Get", args = 1)]
    pub fn get_2(person: crate::app::persondata::PersonData)
        -> crate::unity_engine::sprite::Sprite;

    #[method(name = "Get", args = 1)]
    pub fn get_3(god: crate::app::goddata::GodData) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "GetPath", args = 1)]
    pub fn get_path(god: crate::app::goddata::GodData) -> ::unity2::Il2CppString;

    #[method(name = "GetDLCGodPath", args = 1)]
    pub fn get_dlc_god_path(god_ascii_name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "Get", args = 1)]
    pub fn get_4(ring: crate::app::ringdata::RingData) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "GetPath", args = 1)]
    pub fn get_path_2(unit: crate::app::unit::Unit) -> ::unity2::Il2CppString;

    #[method(name = "GetPath", args = 1)]
    pub fn get_path_3(person: crate::app::persondata::PersonData) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-facethumbnail")]
impl FaceThumbnail {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FaceThumbnail),
                ::core::stringify!(new),
            )
        });
        <Self as IFaceThumbnailMethods>::ctor(this);
        this
    }
}
