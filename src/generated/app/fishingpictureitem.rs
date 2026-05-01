
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingpictureitem/FishingPictureItem.md")))]
#[::unity2::class(namespace = "App", name = "FishingPictureItem")]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct FishingPictureItem {
    #[static_field]
    #[rename(name = "cTextureAtlasPath")]
    pub c_texture_atlas_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTextureRankLarge")]
    pub c_texture_rank_large: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTextureRankMiddle")]
    pub c_texture_rank_middle: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTextureRankSmall")]
    pub c_texture_rank_small: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTextureGyotakuLarge")]
    pub c_texture_gyotaku_large: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTextureGyotakuMiddle")]
    pub c_texture_gyotaku_middle: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "cTextureGyotakuSmall")]
    pub c_texture_gyotaku_small: ::unity2::Il2CppString,
    #[rename(name = "cTextureGyotakuList")]
    pub c_texture_gyotaku_list: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_InfoWindow")]
    pub m_info_window: crate::unity_engine::transform::Transform,
    #[rename(name = "m_Gyotaku")]
    pub m_gyotaku: crate::unity_engine::transform::Transform,
    #[rename(name = "m_FishData")]
    pub m_fish_data: crate::app::fishingfishdata::FishingFishData,
    #[rename(name = "m_Sprites")]
    pub m_sprites: ::unity2::Array<crate::unity_engine::sprite::Sprite>,
}

#[cfg(feature = "app-fishingpictureitem")]
#[::unity2::methods]
impl FishingPictureItem {
    #[method(name = "get_FishSize", args = 0)]
    pub fn get_fish_size(self) -> i32;

    #[method(name = "set_FishSize", args = 1)]
    pub fn set_fish_size(self, value: i32) -> ();

    #[method(name = "get_IsCrown", args = 0)]
    pub fn get_is_crown(self) -> bool;

    #[method(name = "set_IsCrown", args = 1)]
    pub fn set_is_crown(self, value: bool) -> ();

    #[method(name = "get_IsUnknown", args = 0)]
    pub fn get_is_unknown(self) -> bool;

    #[method(name = "set_IsUnknown", args = 1)]
    pub fn set_is_unknown(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        set_data: crate::app::fishingfishdata::FishingFishData,
        set_menu: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "OnDeselect", args = 0)]
    pub fn on_deselect(self) -> ();

    #[method(name = "OnClose", args = 0)]
    pub fn on_close(self) -> ();
}

#[cfg(feature = "app-fishingpictureitem")]
impl FishingPictureItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingPictureItem),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingPictureItemMethods>::ctor(this);
        this
    }

    pub fn new_2(
        set_data: crate::app::fishingfishdata::FishingFishData,
        set_menu: crate::unity_engine::gameobject::GameObject,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingPictureItem),
                ::core::stringify!(new_2),
            )
        });
        <Self as IFishingPictureItemMethods>::ctor_2(this, set_data, set_menu);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingpictureitem/FishingPictureItem_SpriteKind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct FishingPictureItem_SpriteKind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for FishingPictureItem_SpriteKind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "FishingPictureItem.SpriteKind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FishingPictureItem_SpriteKind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl FishingPictureItem_SpriteKind {
    pub fn size() -> Self {
        Self { value: 0 }
    }

    pub fn gyotaku() -> Self {
        Self { value: 1 }
    }

    pub fn unknown() -> Self {
        Self { value: 2 }
    }
}
