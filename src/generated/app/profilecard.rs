
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecard/ProfileCard_Achievement.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ProfileCard_Achievement {
    pub m_acheve_catetory: crate::app::achievedata::AchieveData_Categories,
    pub m_count: i32,
}

impl ::unity2::ClassIdentity for ProfileCard_Achievement {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ProfileCard.Achievement";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ProfileCard_Achievement {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-profilecard")]
#[::unity2::methods(value)]
impl ProfileCard_Achievement {
    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecard/ProfileCard.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCard")]
#[parent(crate::system::object::Object)]
pub struct ProfileCard {
    #[static_field]
    #[rename(name = "Version")]
    pub version: i32,
    #[static_field]
    #[rename(name = "MessageMax")]
    pub message_max: i32,
    #[static_field]
    #[rename(name = "StampMax")]
    pub stamp_max: i32,
    #[static_field]
    #[rename(name = "SortieCountMax")]
    pub sortie_count_max: i32,
    #[static_field]
    #[rename(name = "PageCount")]
    pub page_count: i32,
    #[static_field]
    #[rename(name = "StampMaxVer5")]
    pub stamp_max_ver5: i32,
    #[rename(name = "m_UserName")]
    pub m_user_name: ::unity2::Il2CppString,
    #[rename(name = "m_Lang")]
    pub m_lang: crate::app::language::Language_Langs,
    #[rename(name = "m_Title")]
    pub m_title: crate::app::profilecardtitledata::ProfileCardTitleData,
    #[rename(name = "m_Bg")]
    pub m_bg: crate::app::profilecardbgdata::ProfileCardBgData,
    #[rename(name = "m_Frame")]
    pub m_frame: crate::app::profilecardframedata::ProfileCardFrameData,
    #[rename(name = "m_TextDeco")]
    pub m_text_deco: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    #[rename(name = "m_TextColor")]
    pub m_text_color: crate::app::profilecardtextcolordata::ProfileCardTextColorData,
    #[rename(name = "m_CharacterStamp")]
    pub m_character_stamp: crate::app::profilecardcharacterstampdata::ProfileCardCharacterStampData,
    #[rename(name = "m_Comment")]
    pub m_comment: ::unity2::Array<crate::app::profilecardcommentdata::ProfileCardCommentData>,
    #[rename(name = "m_CommentString")]
    pub m_comment_string: ::unity2::Il2CppString,
    #[rename(name = "m_PlayTime")]
    pub m_play_time: f32,
    #[rename(name = "m_BirthMonth")]
    pub m_birth_month: u8,
    #[rename(name = "m_BirthDay")]
    pub m_birth_day: u8,
    #[rename(name = "m_Difficulty")]
    pub m_difficulty: crate::app::difficulty::Difficulty,
    #[rename(name = "m_GameMode")]
    pub m_game_mode: crate::app::gamemode::GameMode,
    #[rename(name = "m_Gender")]
    pub m_gender: crate::app::gender::Gender,
    #[rename(name = "m_FavoriteCharacter")]
    pub m_favorite_character:
        crate::app::profilecardfavoritecharacterdata::ProfileCardFavoriteCharacterData,
    #[rename(name = "m_FavoriteMap")]
    pub m_favorite_map: crate::app::profilecardfavoritemapdata::ProfileCardFavoriteMapData,
    #[rename(name = "m_RelayPlayingCount")]
    pub m_relay_playing_count: u32,
    #[rename(name = "m_VersusThemeOfEditMap")]
    pub m_versus_theme_of_edit_map:
        crate::app::profilecardthemeofeditmapdata::ProfileCardThemeOfEditMapData,
    #[rename(name = "m_VersusRate")]
    pub m_versus_rate: i32,
    #[rename(name = "m_VersusPlayingCount")]
    pub m_versus_playing_count: u32,
    #[rename(name = "m_OwnerID")]
    pub m_owner_id: u64,
    #[rename(name = "m_IsPublic")]
    pub m_is_public: bool,
    #[rename(name = "m_FreeStamp")]
    pub m_free_stamp: ::unity2::Array<
        crate::system::collections::generic::list_1::List_1<
            crate::app::profilecard::ProfileCard_FreeStamp,
        >,
    >,
    #[rename(name = "m_SortieCount")]
    pub m_sortie_count: ::unity2::Array<crate::app::profilecard::ProfileCard_SortieCount>,
    #[rename(name = "m_Achievements")]
    pub m_achievements: ::unity2::Array<crate::app::profilecard::ProfileCard_Achievement>,
    #[rename(name = "m_Images")]
    pub m_images: ::unity2::Array<u8>,
}

#[cfg(feature = "app-profilecard")]
#[::unity2::methods]
impl ProfileCard {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, src: crate::app::profilecard::ProfileCard) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Copy", args = 1)]
    pub fn copy(self, src: crate::app::profilecard::ProfileCard) -> ();

    #[method(name = "GetImage", args = 0)]
    pub fn get_image(self) -> ::unity2::Array<u8>;

    #[method(name = "SetImage", args = 1)]
    pub fn set_image(self, image: ::unity2::Array<u8>) -> ();

    #[method(name = "GetUserName", args = 0)]
    pub fn get_user_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCheckedUserName", args = 0)]
    pub fn get_checked_user_name(self) -> ::unity2::Il2CppString;

    #[method(name = "SetUserName", args = 1)]
    pub fn set_user_name(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Lang", args = 0)]
    pub fn get_lang(self) -> crate::app::language::Language_Langs;

    #[method(name = "set_Lang", args = 1)]
    pub fn set_lang(self, value: crate::app::language::Language_Langs) -> ();

    #[method(name = "get_Title", args = 0)]
    pub fn get_title(self) -> crate::app::profilecardtitledata::ProfileCardTitleData;

    #[method(name = "set_Title", args = 1)]
    pub fn set_title(self, value: crate::app::profilecardtitledata::ProfileCardTitleData) -> ();

    #[method(name = "GetTitleName", args = 0)]
    pub fn get_title_name(self) -> ::unity2::Il2CppString;

    #[method(name = "get_Bg", args = 0)]
    pub fn get_bg(self) -> crate::app::profilecardbgdata::ProfileCardBgData;

    #[method(name = "set_Bg", args = 1)]
    pub fn set_bg(self, value: crate::app::profilecardbgdata::ProfileCardBgData) -> ();

    #[method(name = "get_Frame", args = 0)]
    pub fn get_frame(self) -> crate::app::profilecardframedata::ProfileCardFrameData;

    #[method(name = "set_Frame", args = 1)]
    pub fn set_frame(self, value: crate::app::profilecardframedata::ProfileCardFrameData) -> ();

    #[method(name = "get_TextDeco", args = 0)]
    pub fn get_text_deco(self) -> crate::app::profilecardtextdecodata::ProfileCardTextDecoData;

    #[method(name = "set_TextDeco", args = 1)]
    pub fn set_text_deco(
        self,
        value: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    ) -> ();

    #[method(name = "get_TextColor", args = 0)]
    pub fn get_text_color(self) -> crate::app::profilecardtextcolordata::ProfileCardTextColorData;

    #[method(name = "set_TextColor", args = 1)]
    pub fn set_text_color(
        self,
        value: crate::app::profilecardtextcolordata::ProfileCardTextColorData,
    ) -> ();

    #[method(name = "get_CharacterStamp", args = 0)]
    pub fn get_character_stamp(
        self,
    ) -> crate::app::profilecardcharacterstampdata::ProfileCardCharacterStampData;

    #[method(name = "set_CharacterStamp", args = 1)]
    pub fn set_character_stamp(
        self,
        value: crate::app::profilecardcharacterstampdata::ProfileCardCharacterStampData,
    ) -> ();

    #[method(name = "get_Comment", args = 0)]
    pub fn get_comment(
        self,
    ) -> ::unity2::Array<crate::app::profilecardcommentdata::ProfileCardCommentData>;

    #[method(name = "set_Comment", args = 1)]
    pub fn set_comment(
        self,
        value: ::unity2::Array<crate::app::profilecardcommentdata::ProfileCardCommentData>,
    ) -> ();

    #[method(name = "get_CommentString", args = 0)]
    pub fn get_comment_string(self) -> ::unity2::Il2CppString;

    #[method(name = "InitializeComment", args = 0)]
    pub fn initialize_comment(self) -> ();

    #[method(name = "GetComment", args = 1)]
    pub fn get_comment_2(
        self,
        i: i32,
    ) -> crate::app::profilecardcommentdata::ProfileCardCommentData;

    #[method(name = "GetCommentMessage", args = 1)]
    pub fn get_comment_message(self, i: i32) -> ::unity2::Il2CppString;

    #[method(name = "SetComment", args = 2)]
    pub fn set_comment_2(self, i: i32, pccid: ::unity2::Il2CppString) -> ();

    #[method(name = "GenerateCommentString", args = 0)]
    pub fn generate_comment_string(self) -> ();

    #[method(name = "get_PlayTime", args = 0)]
    pub fn get_play_time(self) -> f32;

    #[method(name = "set_PlayTime", args = 1)]
    pub fn set_play_time(self, value: f32) -> ();

    #[method(name = "get_PlayTimeHour", args = 0)]
    pub fn get_play_time_hour(self) -> i32;

    #[method(name = "set_PlayTimeHour", args = 1)]
    pub fn set_play_time_hour(self, value: i32) -> ();

    #[method(name = "get_PlayTimeMinute", args = 0)]
    pub fn get_play_time_minute(self) -> i32;

    #[method(name = "set_PlayTimeMinute", args = 1)]
    pub fn set_play_time_minute(self, value: i32) -> ();

    #[method(name = "get_BirthMonth", args = 0)]
    pub fn get_birth_month(self) -> u8;

    #[method(name = "set_BirthMonth", args = 1)]
    pub fn set_birth_month(self, value: u8) -> ();

    #[method(name = "get_BirthDay", args = 0)]
    pub fn get_birth_day(self) -> u8;

    #[method(name = "set_BirthDay", args = 1)]
    pub fn set_birth_day(self, value: u8) -> ();

    #[method(name = "get_Difficulty", args = 0)]
    pub fn get_difficulty(self) -> crate::app::difficulty::Difficulty;

    #[method(name = "set_Difficulty", args = 1)]
    pub fn set_difficulty(self, value: crate::app::difficulty::Difficulty) -> ();

    #[method(name = "get_GameMode", args = 0)]
    pub fn get_game_mode(self) -> crate::app::gamemode::GameMode;

    #[method(name = "set_GameMode", args = 1)]
    pub fn set_game_mode(self, value: crate::app::gamemode::GameMode) -> ();

    #[method(name = "get_Gender", args = 0)]
    pub fn get_gender(self) -> crate::app::gender::Gender;

    #[method(name = "set_Gender", args = 1)]
    pub fn set_gender(self, value: crate::app::gender::Gender) -> ();

    #[method(name = "get_FavoriteCharacter", args = 0)]
    pub fn get_favorite_character(
        self,
    ) -> crate::app::profilecardfavoritecharacterdata::ProfileCardFavoriteCharacterData;

    #[method(name = "set_FavoriteCharacter", args = 1)]
    pub fn set_favorite_character(
        self,
        value: crate::app::profilecardfavoritecharacterdata::ProfileCardFavoriteCharacterData,
    ) -> ();

    #[method(name = "SetFavoriteCharacter", args = 1)]
    pub fn set_favorite_character_2(self, pcfcid: ::unity2::Il2CppString) -> ();

    #[method(name = "get_FavoriteMap", args = 0)]
    pub fn get_favorite_map(
        self,
    ) -> crate::app::profilecardfavoritemapdata::ProfileCardFavoriteMapData;

    #[method(name = "set_FavoriteMap", args = 1)]
    pub fn set_favorite_map(
        self,
        value: crate::app::profilecardfavoritemapdata::ProfileCardFavoriteMapData,
    ) -> ();

    #[method(name = "get_RelayPlayingCount", args = 0)]
    pub fn get_relay_playing_count(self) -> u32;

    #[method(name = "set_RelayPlayingCount", args = 1)]
    pub fn set_relay_playing_count(self, value: u32) -> ();

    #[method(name = "get_VersusThemeOfEditMap", args = 0)]
    pub fn get_versus_theme_of_edit_map(
        self,
    ) -> crate::app::profilecardthemeofeditmapdata::ProfileCardThemeOfEditMapData;

    #[method(name = "set_VersusThemeOfEditMap", args = 1)]
    pub fn set_versus_theme_of_edit_map(
        self,
        value: crate::app::profilecardthemeofeditmapdata::ProfileCardThemeOfEditMapData,
    ) -> ();

    #[method(name = "get_VersusRate", args = 0)]
    pub fn get_versus_rate(self) -> i32;

    #[method(name = "set_VersusRate", args = 1)]
    pub fn set_versus_rate(self, value: i32) -> ();

    #[method(name = "get_VersusPlayingCount", args = 0)]
    pub fn get_versus_playing_count(self) -> u32;

    #[method(name = "set_VersusPlayingCount", args = 1)]
    pub fn set_versus_playing_count(self, value: u32) -> ();

    #[method(name = "get_OwnerID", args = 0)]
    pub fn get_owner_id(self) -> u64;

    #[method(name = "set_OwnerID", args = 1)]
    pub fn set_owner_id(self, value: u64) -> ();

    #[method(name = "get_IsPublic", args = 0)]
    pub fn get_is_public(self) -> bool;

    #[method(name = "set_IsPublic", args = 1)]
    pub fn set_is_public(self, value: bool) -> ();

    #[method(name = "get_FreeStampListArray", args = 0)]
    pub fn get_free_stamp_list_array(
        self,
    ) -> ::unity2::Array<
        crate::system::collections::generic::list_1::List_1<
            crate::app::profilecard::ProfileCard_FreeStamp,
        >,
    >;

    #[method(name = "GetFreeStamp", args = 2)]
    pub fn get_free_stamp(
        self,
        page: i32,
        index: i32,
    ) -> crate::app::profilecard::ProfileCard_FreeStamp;

    #[method(name = "GetFreeStampCount", args = 1)]
    pub fn get_free_stamp_count(self, is_front: bool) -> i32;

    #[method(name = "GetFreeStampCount", args = 1)]
    pub fn get_free_stamp_count_2(self, page: i32) -> i32;

    #[method(name = "AddFreeStamp", args = 6)]
    pub fn add_free_stamp(
        self,
        stamp_data: crate::app::profilecardstampdata::ProfileCardStampData,
        position: crate::unity_engine::vector2::Vector2,
        rotation: f32,
        scale_x: f32,
        scale_y: f32,
        is_front: bool,
    ) -> crate::app::profilecard::ProfileCard_FreeStamp;

    #[method(name = "RemoveFreeStamp", args = 2)]
    pub fn remove_free_stamp(
        self,
        free_stamp: crate::app::profilecard::ProfileCard_FreeStamp,
        is_front: bool,
    ) -> ();

    #[method(name = "ClearFreeStamp", args = 1)]
    pub fn clear_free_stamp(self, is_front: bool) -> ();

    #[method(name = "CopyFreeStamp", args = 1)]
    pub fn copy_free_stamp(
        self,
        free_stamp_list_array: ::unity2::Array<
            crate::system::collections::generic::list_1::List_1<
                crate::app::profilecard::ProfileCard_FreeStamp,
            >,
        >,
    ) -> ();

    #[method(name = "GetSortieCount", args = 1)]
    pub fn get_sortie_count(self, index: i32) -> crate::app::profilecard::ProfileCard_SortieCount;

    #[method(name = "SetSortieCount", args = 4)]
    pub fn set_sortie_count(
        self,
        index: i32,
        person_data: crate::app::persondata::PersonData,
        job_data: crate::app::jobdata::JobData,
        count: i32,
    ) -> ();

    #[method(name = "SortSortieCount", args = 0)]
    pub fn sort_sortie_count(self) -> ();

    #[method(name = "ClearSortieCount", args = 0)]
    pub fn clear_sortie_count(self) -> ();

    #[method(name = "GetAchievement", args = 1)]
    pub fn get_achievement(self, category: crate::app::achievedata::AchieveData_Categories) -> i32;

    #[method(name = "SetAchievement", args = 2)]
    pub fn set_achievement(
        self,
        categoty: crate::app::achievedata::AchieveData_Categories,
        count: i32,
    ) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-profilecard")]
impl ProfileCard {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCard),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardMethods>::ctor(this);
        this
    }

    pub fn new_2(src: crate::app::profilecard::ProfileCard) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCard),
                ::core::stringify!(new_2),
            )
        });
        <Self as IProfileCardMethods>::ctor_2(this, src);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecard/ProfileCard_FreeStamp.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCard.FreeStamp")]
#[parent(crate::system::object::Object)]
pub struct ProfileCard_FreeStamp {
    #[rename(name = "m_Stamp")]
    pub m_stamp: crate::app::profilecardstampdata::ProfileCardStampData,
    #[rename(name = "m_X")]
    pub m_x: i32,
    #[rename(name = "m_Y")]
    pub m_y: i32,
    #[rename(name = "m_Rotation")]
    pub m_rotation: f32,
    #[rename(name = "m_ScaleX")]
    pub m_scale_x: f32,
    #[rename(name = "m_ScaleY")]
    pub m_scale_y: f32,
}

#[cfg(feature = "app-profilecard")]
#[::unity2::methods]
impl ProfileCard_FreeStamp {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor_2(self, free_stamp: crate::app::profilecard::ProfileCard_FreeStamp) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Add", args = 7)]
    pub fn add(
        free_stamp_list: crate::system::collections::generic::list_1::List_1<
            crate::app::profilecard::ProfileCard_FreeStamp,
        >,
        stamp_data: crate::app::profilecardstampdata::ProfileCardStampData,
        x: i32,
        y: i32,
        rotation: f32,
        scale_x: f32,
        scale_y: f32,
    ) -> crate::app::profilecard::ProfileCard_FreeStamp;

    #[method(name = "Add", args = 2)]
    pub fn add_2(
        free_stamp_list: crate::system::collections::generic::list_1::List_1<
            crate::app::profilecard::ProfileCard_FreeStamp,
        >,
        free_stamp: crate::app::profilecard::ProfileCard_FreeStamp,
    ) -> bool;

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 2)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();
}

#[cfg(feature = "app-profilecard")]
impl ProfileCard_FreeStamp {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCard_FreeStamp),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCard_FreeStampMethods>::ctor(this);
        this
    }

    pub fn new_2(free_stamp: crate::app::profilecard::ProfileCard_FreeStamp) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCard_FreeStamp),
                ::core::stringify!(new_2),
            )
        });
        <Self as IProfileCard_FreeStampMethods>::ctor_2(this, free_stamp);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecard/ProfileCard_SortieCount.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ProfileCard_SortieCount {
    pub m_person: crate::app::persondata::PersonData,
    pub m_job: crate::app::jobdata::JobData,
    pub m_count: i32,
}

impl ::unity2::ClassIdentity for ProfileCard_SortieCount {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ProfileCard.SortieCount";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ProfileCard_SortieCount {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-profilecard")]
#[::unity2::methods(value)]
impl ProfileCard_SortieCount {
    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Serialize", args = 1)]
    pub fn serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "Deserialize", args = 2)]
    pub fn deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();
}
