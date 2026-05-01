
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardflags/ProfileCardFlags.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardFlags")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: profilecardflags :: ProfileCardFlags >)]
pub struct ProfileCardFlags {
    #[rename(name = "m_Dictionary")]
    pub m_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<i32, bool>,
}

#[cfg(feature = "app-profilecardflags")]
#[::unity2::methods]
impl ProfileCardFlags {
    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "Entry", args = 2)]
    pub fn entry(self, data: crate::app::profilecardbgdata::ProfileCardBgData, value: bool) -> ();

    #[method(name = "Entry", args = 2)]
    pub fn entry_2(
        self,
        data: crate::app::profilecardframedata::ProfileCardFrameData,
        value: bool,
    ) -> ();

    #[method(name = "Entry", args = 2)]
    pub fn entry_3(
        self,
        data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
        value: bool,
    ) -> ();

    #[method(name = "Entry", args = 2)]
    pub fn entry_4(
        self,
        data: crate::app::profilecardtextcolordata::ProfileCardTextColorData,
        value: bool,
    ) -> ();

    #[method(name = "Entry", args = 2)]
    pub fn entry_5(
        self,
        data: crate::app::profilecardcharacterstampdata::ProfileCardCharacterStampData,
        value: bool,
    ) -> ();

    #[method(name = "Entry", args = 2)]
    pub fn entry_6(
        self,
        data: crate::app::profilecardstampdata::ProfileCardStampData,
        value: bool,
    ) -> ();

    #[method(name = "Entry", args = 2)]
    pub fn entry_7(
        self,
        data: crate::app::profilecardtitledata::ProfileCardTitleData,
        value: bool,
    ) -> ();

    #[method(name = "Entry", args = 2)]
    pub fn entry_8(
        self,
        data: crate::app::profilecardfavoritecharacterdata::ProfileCardFavoriteCharacterData,
        value: bool,
    ) -> ();

    #[method(name = "Entry", args = 2)]
    pub fn entry_9(
        self,
        data: crate::app::profilecardfavoritemapdata::ProfileCardFavoriteMapData,
        value: bool,
    ) -> ();

    #[method(name = "Entry", args = 2)]
    pub fn entry_10(
        self,
        data: crate::app::profilecardcommentdata::ProfileCardCommentData,
        value: bool,
    ) -> ();

    #[method(name = "Entry", args = 2)]
    pub fn entry_11(
        self,
        data: crate::app::profilecardthemeofeditmapdata::ProfileCardThemeOfEditMapData,
        value: bool,
    ) -> ();

    #[method(name = "Entry", args = 2)]
    pub fn entry_12(self, hash: i32, value: bool) -> ();

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist(self, data: crate::app::profilecardbgdata::ProfileCardBgData) -> bool;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist_2(self, data: crate::app::profilecardframedata::ProfileCardFrameData) -> bool;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist_3(
        self,
        data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    ) -> bool;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist_4(
        self,
        data: crate::app::profilecardtextcolordata::ProfileCardTextColorData,
    ) -> bool;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist_5(
        self,
        data: crate::app::profilecardcharacterstampdata::ProfileCardCharacterStampData,
    ) -> bool;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist_6(self, data: crate::app::profilecardstampdata::ProfileCardStampData) -> bool;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist_7(self, data: crate::app::profilecardtitledata::ProfileCardTitleData) -> bool;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist_8(
        self,
        data: crate::app::profilecardfavoritecharacterdata::ProfileCardFavoriteCharacterData,
    ) -> bool;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist_9(
        self,
        data: crate::app::profilecardfavoritemapdata::ProfileCardFavoriteMapData,
    ) -> bool;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist_10(
        self,
        data: crate::app::profilecardcommentdata::ProfileCardCommentData,
    ) -> bool;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist_11(
        self,
        data: crate::app::profilecardthemeofeditmapdata::ProfileCardThemeOfEditMapData,
    ) -> bool;

    #[method(name = "IsExist", args = 1)]
    pub fn is_exist_12(self, hash: i32) -> bool;

    #[method(name = "Get", args = 1)]
    pub fn get(self, data: crate::app::profilecardbgdata::ProfileCardBgData) -> bool;

    #[method(name = "Get", args = 1)]
    pub fn get_2(self, data: crate::app::profilecardframedata::ProfileCardFrameData) -> bool;

    #[method(name = "Get", args = 1)]
    pub fn get_3(self, data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData) -> bool;

    #[method(name = "Get", args = 1)]
    pub fn get_4(
        self,
        data: crate::app::profilecardtextcolordata::ProfileCardTextColorData,
    ) -> bool;

    #[method(name = "Get", args = 1)]
    pub fn get_5(
        self,
        data: crate::app::profilecardcharacterstampdata::ProfileCardCharacterStampData,
    ) -> bool;

    #[method(name = "Get", args = 1)]
    pub fn get_6(self, data: crate::app::profilecardstampdata::ProfileCardStampData) -> bool;

    #[method(name = "Get", args = 1)]
    pub fn get_7(self, data: crate::app::profilecardtitledata::ProfileCardTitleData) -> bool;

    #[method(name = "Get", args = 1)]
    pub fn get_8(
        self,
        data: crate::app::profilecardfavoritecharacterdata::ProfileCardFavoriteCharacterData,
    ) -> bool;

    #[method(name = "Get", args = 1)]
    pub fn get_9(
        self,
        data: crate::app::profilecardfavoritemapdata::ProfileCardFavoriteMapData,
    ) -> bool;

    #[method(name = "Get", args = 1)]
    pub fn get_10(self, data: crate::app::profilecardcommentdata::ProfileCardCommentData) -> bool;

    #[method(name = "Get", args = 1)]
    pub fn get_11(
        self,
        data: crate::app::profilecardthemeofeditmapdata::ProfileCardThemeOfEditMapData,
    ) -> bool;

    #[method(name = "Get", args = 1)]
    pub fn get_12(self, hash: i32) -> bool;

    #[method(name = "Set", args = 2)]
    pub fn set(self, data: crate::app::profilecardbgdata::ProfileCardBgData, value: bool) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_2(
        self,
        data: crate::app::profilecardframedata::ProfileCardFrameData,
        value: bool,
    ) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_3(
        self,
        data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
        value: bool,
    ) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_4(
        self,
        data: crate::app::profilecardtextcolordata::ProfileCardTextColorData,
        value: bool,
    ) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_5(
        self,
        data: crate::app::profilecardcharacterstampdata::ProfileCardCharacterStampData,
        value: bool,
    ) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_6(
        self,
        data: crate::app::profilecardstampdata::ProfileCardStampData,
        value: bool,
    ) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_7(
        self,
        data: crate::app::profilecardtitledata::ProfileCardTitleData,
        value: bool,
    ) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_8(
        self,
        data: crate::app::profilecardfavoritecharacterdata::ProfileCardFavoriteCharacterData,
        value: bool,
    ) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_9(
        self,
        data: crate::app::profilecardfavoritemapdata::ProfileCardFavoriteMapData,
        value: bool,
    ) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_10(
        self,
        data: crate::app::profilecardcommentdata::ProfileCardCommentData,
        value: bool,
    ) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_11(
        self,
        data: crate::app::profilecardthemeofeditmapdata::ProfileCardThemeOfEditMapData,
        value: bool,
    ) -> ();

    #[method(name = "Set", args = 2)]
    pub fn set_12(self, hash: i32, value: bool) -> ();

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();
}

#[cfg(feature = "app-profilecardflags")]
impl ProfileCardFlags {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardFlags),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardFlagsMethods>::ctor(this);
        this
    }
}
