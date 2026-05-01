
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/commonringfaceframe/CommonRingFaceFrame.md")))]
#[::unity2::class(namespace = "App", name = "CommonRingFaceFrame")]
#[parent(crate::system::object::Object)]
pub struct CommonRingFaceFrame {
    #[static_field]
    #[rename(name = "Path")]
    pub path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_CommonRingFaceFrame")]
    pub s_common_ring_face_frame: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
}

#[cfg(feature = "app-commonringfaceframe")]
#[::unity2::methods]
impl CommonRingFaceFrame {
    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async() -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading() -> bool;

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = "Get", args = 1)]
    pub fn get(rank: crate::app::ringdata::RingData_Ranks) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "Get", args = 1)]
    pub fn get_2(icon_name: ::unity2::Il2CppString) -> crate::unity_engine::sprite::Sprite;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-commonringfaceframe")]
impl CommonRingFaceFrame {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(CommonRingFaceFrame),
                ::core::stringify!(new),
            )
        });
        <Self as ICommonRingFaceFrameMethods>::ctor(this);
        this
    }
}
