
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardvisualassets/ProfileCardVisualAssets.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardVisualAssets")]
#[parent(crate::system::object::Object)]
pub struct ProfileCardVisualAssets {
    #[static_field]
    #[rename(name = "BgPath")]
    pub bg_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "BgThumbPath")]
    pub bg_thumb_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FramePath")]
    pub frame_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FrameThumbPath")]
    pub frame_thumb_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "TextDecoPath")]
    pub text_deco_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "TextDecoThumbPath")]
    pub text_deco_thumb_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "StampPath")]
    pub stamp_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "TexturesPath")]
    pub textures_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FrameThumbEmpty")]
    pub frame_thumb_empty: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "TextureFaceEmpty")]
    pub texture_face_empty: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "TexturePhotoEmpty")]
    pub texture_photo_empty: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_BgThumbAtlas")]
    pub s_bg_thumb_atlas: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_FrameThumbAtlas")]
    pub s_frame_thumb_atlas: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_TextDecoAtlas")]
    pub s_text_deco_atlas: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_TextDecoThumbAtlas")]
    pub s_text_deco_thumb_atlas: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_StampAtlas")]
    pub s_stamp_atlas: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
    #[static_field]
    #[rename(name = "s_TexturesAtlas")]
    pub s_textures_atlas: crate::app::spriteatlasmanager_2::SpriteAtlasManager_2,
}

#[cfg(feature = "app-profilecardvisualassets")]
#[::unity2::methods]
impl ProfileCardVisualAssets {
    #[method(name = "LoadAsync", args = 0)]
    pub fn load_async() -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading() -> bool;

    #[method(name = "GetBg", args = 2)]
    pub fn get_bg(
        bg_data: crate::app::profilecardbgdata::ProfileCardBgData,
        completed: crate::system::action_1::Action_1<crate::unity_engine::sprite::Sprite>,
    ) -> ();

    #[method(name = "ReleaseBg", args = 1)]
    pub fn release_bg(bg_data: crate::app::profilecardbgdata::ProfileCardBgData) -> ();

    #[method(name = "TryGetBgThumb", args = 1)]
    pub fn try_get_bg_thumb(
        bg_data: crate::app::profilecardbgdata::ProfileCardBgData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "GetFrame", args = 2)]
    pub fn get_frame(
        frame_data: crate::app::profilecardframedata::ProfileCardFrameData,
        completed: crate::system::action_1::Action_1<crate::unity_engine::sprite::Sprite>,
    ) -> ();

    #[method(name = "ReleaseFrame", args = 1)]
    pub fn release_frame(frame_data: crate::app::profilecardframedata::ProfileCardFrameData) -> ();

    #[method(name = "TryGetFrameThumb", args = 1)]
    pub fn try_get_frame_thumb(
        frame_data: crate::app::profilecardframedata::ProfileCardFrameData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetFrameThumbEmpty", args = 0)]
    pub fn try_get_frame_thumb_empty() -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetTextDecoNameBg", args = 1)]
    pub fn try_get_text_deco_name_bg(
        deco_data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetTextDecoCommentBg", args = 1)]
    pub fn try_get_text_deco_comment_bg(
        deco_data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetTextDecoTitleLargeBg", args = 1)]
    pub fn try_get_text_deco_title_large_bg(
        deco_data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetTextDecoTitleSmallBg", args = 1)]
    pub fn try_get_text_deco_title_small_bg(
        deco_data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetTextDecoColumnBg", args = 1)]
    pub fn try_get_text_deco_column_bg(
        deco_data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetTextDecoLine", args = 1)]
    pub fn try_get_text_deco_line(
        deco_data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetTextDecoUnitNameBg", args = 1)]
    pub fn try_get_text_deco_unit_name_bg(
        deco_data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetTextDecoAchievement1Bg", args = 1)]
    pub fn try_get_text_deco_achievement1_bg(
        deco_data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetTextDecoAchievement2Bg", args = 1)]
    pub fn try_get_text_deco_achievement2_bg(
        deco_data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetTextDecoThumb", args = 1)]
    pub fn try_get_text_deco_thumb(
        text_deco_data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetStamp", args = 1)]
    pub fn try_get_stamp(
        stamp_data: crate::app::profilecardstampdata::ProfileCardStampData,
    ) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetTextures", args = 1)]
    pub fn try_get_textures(name: ::unity2::Il2CppString) -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetTextureFaceEmpty", args = 0)]
    pub fn try_get_texture_face_empty() -> crate::unity_engine::sprite::Sprite;

    #[method(name = "TryGetTexturePhotoEmpty", args = 0)]
    pub fn try_get_texture_photo_empty() -> crate::unity_engine::sprite::Sprite;

    #[method(name = "Unload", args = 0)]
    pub fn unload() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-profilecardvisualassets")]
impl ProfileCardVisualAssets {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardVisualAssets),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardVisualAssetsMethods>::ctor(this);
        this
    }
}
