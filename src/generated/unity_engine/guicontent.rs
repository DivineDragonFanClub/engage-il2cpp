
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/guicontent/GUIContent.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "GUIContent")]
#[parent(crate::system::object::Object)]
pub struct GUIContent {
    #[rename(name = "m_Text")]
    pub m_text: ::unity2::Il2CppString,
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::texture::Texture,
    #[rename(name = "m_Tooltip")]
    pub m_tooltip: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_Text")]
    pub s_text: crate::unity_engine::guicontent::GUIContent,
    #[static_field]
    #[rename(name = "s_Image")]
    pub s_image: crate::unity_engine::guicontent::GUIContent,
    #[static_field]
    #[rename(name = "s_TextImage")]
    pub s_text_image: crate::unity_engine::guicontent::GUIContent,
    #[static_field]
    #[rename(name = "none")]
    pub none: crate::unity_engine::guicontent::GUIContent,
}

#[cfg(feature = "unity_engine-guicontent")]
#[::unity2::methods]
impl GUIContent {
    #[method(name = "get_text", args = 0)]
    pub fn get_text(self) -> ::unity2::Il2CppString;

    #[method(name = "set_text", args = 1)]
    pub fn set_text(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "set_image", args = 1)]
    pub fn set_image(self, value: crate::unity_engine::texture::Texture) -> ();

    #[method(name = "get_tooltip", args = 0)]
    pub fn get_tooltip(self) -> ::unity2::Il2CppString;

    #[method(name = "set_tooltip", args = 1)]
    pub fn set_tooltip(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_3(
        self,
        text: ::unity2::Il2CppString,
        image: crate::unity_engine::texture::Texture,
        tooltip: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_4(self, src: crate::unity_engine::guicontent::GUIContent) -> ();

    #[method(name = "Temp", args = 1)]
    pub fn temp(t: ::unity2::Il2CppString) -> crate::unity_engine::guicontent::GUIContent;

    #[method(name = "ClearStaticCache", args = 0)]
    pub fn clear_static_cache() -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-guicontent")]
impl GUIContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUIContent),
                ::core::stringify!(new),
            )
        });
        <Self as IGUIContentMethods>::ctor(this);
        this
    }

    pub fn new_2(text: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUIContent),
                ::core::stringify!(new_2),
            )
        });
        <Self as IGUIContentMethods>::ctor_2(this, text);
        this
    }

    pub fn new_3(
        text: ::unity2::Il2CppString,
        image: crate::unity_engine::texture::Texture,
        tooltip: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUIContent),
                ::core::stringify!(new_3),
            )
        });
        <Self as IGUIContentMethods>::ctor_3(this, text, image, tooltip);
        this
    }

    pub fn new_4(src: crate::unity_engine::guicontent::GUIContent) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GUIContent),
                ::core::stringify!(new_4),
            )
        });
        <Self as IGUIContentMethods>::ctor_4(this, src);
        this
    }
}
