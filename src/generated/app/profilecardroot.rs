
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardroot/ProfileCardRoot.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ProfileCardRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_RectTransform")]
    pub m_rect_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_CanvasGroup")]
    pub m_canvas_group: crate::unity_engine::canvasgroup::CanvasGroup,
    #[rename(name = "m_FrameImage")]
    pub m_frame_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_BgImage")]
    pub m_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_FrontPage")]
    pub m_front_page: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_UserName")]
    pub m_user_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Hyphen")]
    pub m_hyphen: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Title")]
    pub m_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_TitleBgImage")]
    pub m_title_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Photo")]
    pub m_photo: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CharacterStamp")]
    pub m_character_stamp: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Comment")]
    pub m_comment: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CommentBgImage")]
    pub m_comment_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_PlayerInfoCaption")]
    pub m_player_info_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_PlayerInfoCaptionBgImage")]
    pub m_player_info_caption_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_PlayTimeCaption")]
    pub m_play_time_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_PlayTime")]
    pub m_play_time: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_PlayTimeBgImage")]
    pub m_play_time_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_BirthdayCaption")]
    pub m_birthday_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Birthday")]
    pub m_birthday: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BirthdayBgImane")]
    pub m_birthday_bg_imane: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_DifficultyCaption")]
    pub m_difficulty_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Difficulty")]
    pub m_difficulty: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_DifficultyBgImage")]
    pub m_difficulty_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_ModeCaption")]
    pub m_mode_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Mode")]
    pub m_mode: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ModeBgImage")]
    pub m_mode_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_FavoriteCharacterCaption")]
    pub m_favorite_character_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_FavoriteCharacter")]
    pub m_favorite_character: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_FavoriteCharacterBgImage")]
    pub m_favorite_character_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_FavoriteMapCaption")]
    pub m_favorite_map_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_FavoriteMap")]
    pub m_favorite_map: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_FavoriteMapBgImage")]
    pub m_favorite_map_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_RelayCaption")]
    pub m_relay_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RelayCaptionBgImage")]
    pub m_relay_caption_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_RelayPlayCountCaption")]
    pub m_relay_play_count_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RelayPlayCount")]
    pub m_relay_play_count: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RelayPlayCountBgImage")]
    pub m_relay_play_count_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_VersusCaption")]
    pub m_versus_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_VersusCaptionBgImage")]
    pub m_versus_caption_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_VersusMapThemeCaption")]
    pub m_versus_map_theme_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_VersusMapTheme")]
    pub m_versus_map_theme: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_VersusMapThemeBgImage")]
    pub m_versus_map_theme_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_VersusPlayCountCaption")]
    pub m_versus_play_count_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_VersusPlayCount")]
    pub m_versus_play_count: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_VersusPlayCountBgImage")]
    pub m_versus_play_count_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_VersusRateCaption")]
    pub m_versus_rate_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_VersusRate")]
    pub m_versus_rate: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_VersusRateBgImage")]
    pub m_versus_rate_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_InfoLineImage")]
    pub m_info_line_image: ::unity2::Array<crate::unity_engine::ui::image::Image>,
    #[rename(name = "m_FrontStampRoot")]
    pub m_front_stamp_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_FrontStampOrigin")]
    pub m_front_stamp_origin: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_BackPage")]
    pub m_back_page: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SortieCountCaption")]
    pub m_sortie_count_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_SortieCountCaptionBgImage")]
    pub m_sortie_count_caption_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SortieCount")]
    pub m_sortie_count:
        ::unity2::Array<crate::app::profilecardroot::ProfileCardRoot_SortieCountInfo>,
    #[rename(name = "m_AchievementCaption")]
    pub m_achievement_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_AchievementCaptionBgImage")]
    pub m_achievement_caption_bg_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Achievement")]
    pub m_achievement:
        ::unity2::Array<crate::app::profilecardroot::ProfileCardRoot_AchievementInfo>,
    #[rename(name = "m_BackStampRoot")]
    pub m_back_stamp_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_BackStampOrigin")]
    pub m_back_stamp_origin: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_BgHighlightAnimator")]
    pub m_bg_highlight_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_FrameHighlightAnimator")]
    pub m_frame_highlight_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_TextDecoHighlightAnimator")]
    pub m_text_deco_highlight_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_TextHighlightAnimator")]
    pub m_text_highlight_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_TitleHightlightAnimator")]
    pub m_title_hightlight_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_PhotoHighlightAnimator")]
    pub m_photo_highlight_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_CharaStampHighlightAnimator")]
    pub m_chara_stamp_highlight_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_CommentHighlightAnimator")]
    pub m_comment_highlight_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_FavoriteCharacterHighlightAnimator")]
    pub m_favorite_character_highlight_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_FavoriteRelayMapHighlightAnimator")]
    pub m_favorite_relay_map_highlight_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_OpenPositionY")]
    pub m_open_position_y: f32,
    #[rename(name = "m_SmallPosition")]
    pub m_small_position: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_SmallScale")]
    pub m_small_scale: crate::unity_engine::vector2::Vector2,
    #[static_field]
    #[rename(name = "m_CharacterStampEmptyImageName")]
    pub m_character_stamp_empty_image_name: ::unity2::Il2CppString,
    #[rename(name = "m_IsFront")]
    pub m_is_front: bool,
    #[rename(name = "m_IsEFIGS")]
    pub m_is_efigs: bool,
    #[rename(name = "m_StampIsVisible")]
    pub m_stamp_is_visible: bool,
    #[rename(name = "m_BgData")]
    pub m_bg_data: crate::app::profilecardbgdata::ProfileCardBgData,
    #[rename(name = "m_BgDataLoading")]
    pub m_bg_data_loading: crate::app::profilecardbgdata::ProfileCardBgData,
    #[rename(name = "m_FrameData")]
    pub m_frame_data: crate::app::profilecardframedata::ProfileCardFrameData,
    #[rename(name = "m_FrameDataLoading")]
    pub m_frame_data_loading: crate::app::profilecardframedata::ProfileCardFrameData,
    #[rename(name = "m_PhotoTexture2d")]
    pub m_photo_texture2d: crate::unity_engine::texture2d::Texture2D,
    #[rename(name = "m_PhotoSprite")]
    pub m_photo_sprite: crate::unity_engine::sprite::Sprite,
    #[rename(name = "m_StartHandler")]
    pub m_start_handler: crate::app::profilecardroot::ProfileCardRoot_StartHandler,
}

#[cfg(feature = "app-profilecardroot")]
#[::unity2::methods]
impl ProfileCardRoot {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "SetStartHandler", args = 1)]
    pub fn set_start_handler(
        self,
        start_handler: crate::app::profilecardroot::ProfileCardRoot_StartHandler,
    ) -> ();

    #[method(name = "IsOpening", args = 0)]
    pub fn is_opening(self) -> bool;

    #[method(name = "SetAlpha", args = 1)]
    pub fn set_alpha(self, alpha: f32) -> ();

    #[method(name = "UpdateCard", args = 1)]
    pub fn update_card(self, card: crate::app::profilecard::ProfileCard) -> ();

    #[method(name = "UpdataUserName", args = 1)]
    pub fn updata_user_name(self, user_name: ::unity2::Il2CppString) -> ();

    #[method(name = "UpdateTitle", args = 1)]
    pub fn update_title(
        self,
        title_data: crate::app::profilecardtitledata::ProfileCardTitleData,
    ) -> ();

    #[method(name = "UpdateHyphen", args = 0)]
    pub fn update_hyphen(self) -> ();

    #[method(name = "UpdatePhoto", args = 1)]
    pub fn update_photo(self, image: ::unity2::Array<u8>) -> ();

    #[method(name = "DestroyPhoto", args = 0)]
    pub fn destroy_photo(self) -> ();

    #[method(name = "UpdateComment", args = 1)]
    pub fn update_comment(self, comment_string: ::unity2::Il2CppString) -> ();

    #[method(name = "UpdatePlayTime", args = 1)]
    pub fn update_play_time(self, play_time: f32) -> ();

    #[method(name = "UpdatePlayTime", args = 2)]
    pub fn update_play_time_2(self, hour: i32, minute: i32) -> ();

    #[method(name = "UpdateFavoriteCharacter", args = 1)]
    pub fn update_favorite_character(
        self,
        character_data : crate :: app :: profilecardfavoritecharacterdata :: ProfileCardFavoriteCharacterData,
    ) -> ();

    #[method(name = "UpdateFavoriteMap", args = 1)]
    pub fn update_favorite_map(
        self,
        map_data: crate::app::profilecardfavoritemapdata::ProfileCardFavoriteMapData,
    ) -> ();

    #[method(name = "UpdateBg", args = 1)]
    pub fn update_bg(self, pcbgid: ::unity2::Il2CppString) -> ();

    #[method(name = "UpdateBg", args = 1)]
    pub fn update_bg_2(self, bg_data: crate::app::profilecardbgdata::ProfileCardBgData) -> ();

    #[method(name = "UpdateFrame", args = 1)]
    pub fn update_frame(self, pcfid: ::unity2::Il2CppString) -> ();

    #[method(name = "UpdateFrame", args = 1)]
    pub fn update_frame_2(
        self,
        frame_data: crate::app::profilecardframedata::ProfileCardFrameData,
    ) -> ();

    #[method(name = "UpdateCharacterStamp", args = 1)]
    pub fn update_character_stamp(self, id: ::unity2::Il2CppString) -> ();

    #[method(name = "UpdateCharacterStamp", args = 1)]
    pub fn update_character_stamp_2(
        self,
        stamp_data: crate::app::profilecardcharacterstampdata::ProfileCardCharacterStampData,
    ) -> ();

    #[method(name = "UpdateTextDeco", args = 1)]
    pub fn update_text_deco(self, pctdid: ::unity2::Il2CppString) -> ();

    #[method(name = "UpdateTextDeco", args = 1)]
    pub fn update_text_deco_2(
        self,
        deco_data: crate::app::profilecardtextdecodata::ProfileCardTextDecoData,
    ) -> ();

    #[method(name = "UpdateTextColor", args = 1)]
    pub fn update_text_color(self, pctcid: ::unity2::Il2CppString) -> ();

    #[method(name = "UpdateTextColor", args = 1)]
    pub fn update_text_color_2(
        self,
        text_color_data: crate::app::profilecardtextcolordata::ProfileCardTextColorData,
    ) -> ();

    #[method(name = "UpdateFreeStamp", args = 1)]
    pub fn update_free_stamp(
        self,
        free_stamp_list_array: ::unity2::Array<
            crate::system::collections::generic::list_1::List_1<
                crate::app::profilecard::ProfileCard_FreeStamp,
            >,
        >,
    ) -> ();

    #[method(name = "SetFreeStampVisibility", args = 1)]
    pub fn set_free_stamp_visibility(self, visible: bool) -> ();

    #[method(name = "AddFreeStamp", args = 2)]
    pub fn add_free_stamp(
        self,
        free_stamp: crate::app::profilecard::ProfileCard_FreeStamp,
        page: i32,
    ) -> ();

    #[method(name = "RemoveFreeStamp", args = 2)]
    pub fn remove_free_stamp(
        self,
        free_stamp: crate::app::profilecard::ProfileCard_FreeStamp,
        page: i32,
    ) -> ();

    #[method(name = "ClearFreeStamp", args = 0)]
    pub fn clear_free_stamp(self) -> ();

    #[method(name = "ClearFreeStamp", args = 1)]
    pub fn clear_free_stamp_2(self, page: i32) -> ();

    #[method(name = "GetStampObject", args = 1)]
    pub fn get_stamp_object(
        self,
        index: i32,
    ) -> crate::app::profilecardstampobject::ProfileCardStampObject;

    #[method(name = "GetStamp", args = 1)]
    pub fn get_stamp(self, index: i32) -> crate::app::profilecard::ProfileCard_FreeStamp;

    #[method(name = "SetStampFrameActive", args = 2)]
    pub fn set_stamp_frame_active(self, stamp_index: i32, actived: bool) -> ();

    #[method(name = "IsStampExist", args = 1)]
    pub fn is_stamp_exist(
        self,
        rect: crate::app::profilecardroot::ProfileCardRoot_RectInfo,
    ) -> bool;

    #[method(name = "GetOverlappedStamp", args = 1)]
    pub fn get_overlapped_stamp(
        self,
        rect: crate::app::profilecardroot::ProfileCardRoot_RectInfo,
    ) -> crate::app::profilecard::ProfileCard_FreeStamp;

    #[method(name = "GetOverlappedStampIndex", args = 1)]
    pub fn get_overlapped_stamp_index(
        self,
        rect: crate::app::profilecardroot::ProfileCardRoot_RectInfo,
    ) -> i32;

    #[method(name = "Contain", args = 1)]
    pub fn contain(self, rect_transform: crate::unity_engine::recttransform::RectTransform)
        -> bool;

    #[method(name = "GetRect", args = 0)]
    pub fn get_rect(self) -> crate::unity_engine::rect::Rect;

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "SwitchPages", args = 0)]
    pub fn switch_pages(self) -> ();

    #[method(name = "IsSwithcingPages", args = 0)]
    pub fn is_swithcing_pages(self) -> bool;

    #[method(name = "IsFront", args = 0)]
    pub fn is_front(self) -> bool;

    #[method(name = "SwitchScale", args = 0)]
    pub fn switch_scale(self) -> ();

    #[method(name = "ScaleSmall", args = 0)]
    pub fn scale_small(self) -> ();

    #[method(name = "ScaleSmallImmediately", args = 0)]
    pub fn scale_small_immediately(self) -> ();

    #[method(name = "ScaleLarge", args = 0)]
    pub fn scale_large(self) -> ();

    #[method(name = "IsScalingSmall", args = 0)]
    pub fn is_scaling_small(self) -> bool;

    #[method(name = "IsScalingLarge", args = 0)]
    pub fn is_scaling_large(self) -> bool;

    #[method(name = "IsScaledSmall", args = 0)]
    pub fn is_scaled_small(self) -> bool;

    #[method(name = "BlinkBg", args = 1)]
    pub fn blink_bg(self, on: bool) -> ();

    #[method(name = "BlinkFrame", args = 1)]
    pub fn blink_frame(self, on: bool) -> ();

    #[method(name = "BlinkTextDeco", args = 1)]
    pub fn blink_text_deco(self, on: bool) -> ();

    #[method(name = "BlinkText", args = 1)]
    pub fn blink_text(self, on: bool) -> ();

    #[method(name = "BlinkTitle", args = 1)]
    pub fn blink_title(self, on: bool) -> ();

    #[method(name = "BlinkPhoto", args = 1)]
    pub fn blink_photo(self, on: bool) -> ();

    #[method(name = "BlinkCharacterStamp", args = 1)]
    pub fn blink_character_stamp(self, on: bool) -> ();

    #[method(name = "BlinkComment", args = 1)]
    pub fn blink_comment(self, on: bool) -> ();

    #[method(name = "BlinkFavoriteCharacter", args = 1)]
    pub fn blink_favorite_character(self, on: bool) -> ();

    #[method(name = "BlinkFavoriteRelayMap", args = 1)]
    pub fn blink_favorite_relay_map(self, on: bool) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "IsClosing", args = 0)]
    pub fn is_closing(self) -> bool;

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-profilecardroot")]
impl ProfileCardRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardRootMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardroot/ProfileCardRoot_RectInfo.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct ProfileCardRoot_RectInfo {
    pub pos: crate::unity_engine::vector2::Vector2,
    pub harf_size: crate::unity_engine::vector2::Vector2,
    pub x_axis: crate::unity_engine::vector2::Vector2,
    pub y_axis: crate::unity_engine::vector2::Vector2,
}

impl ::unity2::ClassIdentity for ProfileCardRoot_RectInfo {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ProfileCardRoot.RectInfo";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ProfileCardRoot_RectInfo {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-profilecardroot")]
#[::unity2::methods(value)]
impl ProfileCardRoot_RectInfo {
    #[method(name = ".ctor", args = 4)]
    pub fn ctor(
        self,
        pos: crate::unity_engine::vector2::Vector2,
        harf_size: crate::unity_engine::vector2::Vector2,
        x_axis: crate::unity_engine::vector2::Vector2,
        y_axis: crate::unity_engine::vector2::Vector2,
    ) -> ();

    #[method(name = "GetAbsDot", args = 2)]
    pub fn get_abs_dot(
        v1: crate::unity_engine::vector2::Vector2,
        v2: crate::unity_engine::vector2::Vector2,
    ) -> f32;

    #[method(name = "IsOverlap", args = 2)]
    pub fn is_overlap(
        rect1: crate::app::profilecardroot::ProfileCardRoot_RectInfo,
        rect2: crate::app::profilecardroot::ProfileCardRoot_RectInfo,
    ) -> bool;

    #[method(name = "GetDistance", args = 2)]
    pub fn get_distance(
        pos: crate::unity_engine::vector2::Vector2,
        rect: crate::app::profilecardroot::ProfileCardRoot_RectInfo,
    ) -> f32;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardroot/ProfileCardRoot_SortieCountInfo.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardRoot.SortieCountInfo")]
#[parent(crate::system::object::Object)]
pub struct ProfileCardRoot_SortieCountInfo {
    #[rename(name = "m_RootObject")]
    pub m_root_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_UnitIcon")]
    pub m_unit_icon: crate::app::uniticon::UnitIcon,
    #[rename(name = "m_UnitName")]
    pub m_unit_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_SortieCount")]
    pub m_sortie_count: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BgImage")]
    pub m_bg_image: crate::unity_engine::ui::image::Image,
}

#[cfg(feature = "app-profilecardroot")]
#[::unity2::methods]
impl ProfileCardRoot_SortieCountInfo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-profilecardroot")]
impl ProfileCardRoot_SortieCountInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardRoot_SortieCountInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardRoot_SortieCountInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardroot/ProfileCardRoot_AchievementInfo.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardRoot.AchievementInfo")]
#[parent(crate::system::object::Object)]
pub struct ProfileCardRoot_AchievementInfo {
    #[rename(name = "m_Icon")]
    pub m_icon: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Caption")]
    pub m_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_Count")]
    pub m_count: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_BgImage")]
    pub m_bg_image: crate::unity_engine::ui::image::Image,
}

#[cfg(feature = "app-profilecardroot")]
#[::unity2::methods]
impl ProfileCardRoot_AchievementInfo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-profilecardroot")]
impl ProfileCardRoot_AchievementInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardRoot_AchievementInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardRoot_AchievementInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardroot/ProfileCardRoot_StartHandler.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardRoot.StartHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardRoot_StartHandler {}

#[cfg(feature = "app-profilecardroot")]
#[::unity2::methods]
impl ProfileCardRoot_StartHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-profilecardroot")]
impl ProfileCardRoot_StartHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardRoot_StartHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardRoot_StartHandlerMethods>::ctor(this, object, method);
        this
    }
}
