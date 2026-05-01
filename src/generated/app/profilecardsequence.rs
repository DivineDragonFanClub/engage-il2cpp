
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardsequence/ProfileCardSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ProfileCardSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ProfileCardSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ProfileCardSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ProfileCardSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ProfileCardSequence_Label {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn top() -> Self {
        Self { value: 1 }
    }

    pub fn my_card() -> Self {
        Self { value: 2 }
    }

    pub fn my_card_main() -> Self {
        Self { value: 3 }
    }

    pub fn my_card_save() -> Self {
        Self { value: 4 }
    }

    pub fn my_card_end() -> Self {
        Self { value: 5 }
    }

    pub fn select_edit() -> Self {
        Self { value: 6 }
    }

    pub fn title() -> Self {
        Self { value: 7 }
    }

    pub fn comment() -> Self {
        Self { value: 8 }
    }

    pub fn comment_list() -> Self {
        Self { value: 9 }
    }

    pub fn character() -> Self {
        Self { value: 10 }
    }

    pub fn relay_map() -> Self {
        Self { value: 11 }
    }

    pub fn bg() -> Self {
        Self { value: 12 }
    }

    pub fn frame() -> Self {
        Self { value: 13 }
    }

    pub fn text_deco() -> Self {
        Self { value: 14 }
    }

    pub fn text_color() -> Self {
        Self { value: 15 }
    }

    pub fn character_stamp() -> Self {
        Self { value: 16 }
    }

    pub fn stamp() -> Self {
        Self { value: 17 }
    }

    pub fn album() -> Self {
        Self { value: 18 }
    }

    pub fn album_main() -> Self {
        Self { value: 19 }
    }

    pub fn album_list() -> Self {
        Self { value: 20 }
    }

    pub fn album_end() -> Self {
        Self { value: 21 }
    }

    pub fn photo() -> Self {
        Self { value: 22 }
    }

    pub fn public_setting() -> Self {
        Self { value: 23 }
    }

    pub fn stamp_visibility_setting() -> Self {
        Self { value: 24 }
    }

    pub fn end() -> Self {
        Self { value: 25 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardsequence/ProfileCardSequence.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct ProfileCardSequence {
    #[rename(name = "m_TopMenuResult")]
    pub m_top_menu_result: crate::app::profilecardtopmenu::ProfileCardTopMenu_Result2,
    #[rename(name = "m_MyCardMenuResult")]
    pub m_my_card_menu_result: crate::app::profilecardmycardmenu::ProfileCardMyCardMenu_Result2,
    #[rename(name = "m_SelectEditMenuResult")]
    pub m_select_edit_menu_result:
        crate::app::profilecardselecteditmenu::ProfileCardSelectEditMenu_Result2,
    #[rename(name = "m_CommentIndexMenuResult")]
    pub m_comment_index_menu_result:
        crate::app::profilecardcommentindexmenu::ProfileCardCommentIndexMenu_Result2,
    #[rename(name = "m_AlbumMenuResult")]
    pub m_album_menu_result: crate::app::profilecardalbummenu::ProfileCardAlbumMenu_Result2,
    #[rename(name = "m_AlbumListMenuResult")]
    pub m_album_list_menu_result:
        crate::app::profilecardalbumlistmenu::ProfileCardAlbumListMenu_Result2,
    #[rename(name = "m_ProfileCardMyCardRoot")]
    pub m_profile_card_my_card_root: crate::app::profilecardmycardroot::ProfileCardMyCardRoot,
    #[rename(name = "m_ProfileCardAlbumRoot")]
    pub m_profile_card_album_root: crate::app::profilecardalbumroot::ProfileCardAlbumRoot,
    #[rename(name = "m_ProfileCardTemp")]
    pub m_profile_card_temp: crate::app::profilecard::ProfileCard,
    #[rename(name = "m_AlbumIndex")]
    pub m_album_index: i32,
    #[rename(name = "m_EnabledPhoto")]
    pub m_enabled_photo: bool,
    #[rename(name = "m_CommentIndex")]
    pub m_comment_index: i32,
    #[rename(name = "m_CommentTempArray")]
    pub m_comment_temp_array:
        ::unity2::Array<crate::app::profilecardcommentdata::ProfileCardCommentData>,
    #[rename(name = "m_NeededUpdateCardOnBuild")]
    pub m_needed_update_card_on_build: bool,
    #[rename(name = "m_BgData")]
    pub m_bg_data: crate::app::profilecardbgdata::ProfileCardBgData,
    #[rename(name = "m_FrameData")]
    pub m_frame_data: crate::app::profilecardframedata::ProfileCardFrameData,
    #[rename(name = "m_LoadingBgData")]
    pub m_loading_bg_data: bool,
    #[rename(name = "m_LoadingFrameData")]
    pub m_loading_frame_data: bool,
    #[rename(name = "m_DisposeEventHandler")]
    pub m_dispose_event_handler:
        crate::app::profilecardsequence::ProfileCardSequence_DisposeEventHandler,
}

#[cfg(feature = "app-profilecardsequence")]
#[::unity2::methods]
impl ProfileCardSequence {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        enabled_photo: bool,
        dispose_event_handler : crate :: app :: profilecardsequence :: ProfileCardSequence_DisposeEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        enabled_photo: bool,
        dispose_event_handler : crate :: app :: profilecardsequence :: ProfileCardSequence_DisposeEventHandler,
    ) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "StartSequence", args = 0)]
    pub fn start_sequence(self) -> ();

    #[method(name = "CreateTopMenu", args = 0)]
    pub fn create_top_menu(self) -> ();

    #[method(name = "IsLoadingCardContantsResources", args = 0)]
    pub fn is_loading_card_contants_resources(self) -> bool;

    #[method(name = "LoadMyCardContent", args = 0)]
    pub fn load_my_card_content(self) -> ();

    #[method(name = "IsLoadingCardContent", args = 0)]
    pub fn is_loading_card_content(self) -> bool;

    #[method(name = "UnloadCardContent", args = 0)]
    pub fn unload_card_content(self) -> ();

    #[method(name = "IsAsianLanguage", args = 0)]
    pub fn is_asian_language(self) -> bool;

    #[method(name = "ShowContentsNotice", args = 0)]
    pub fn show_contents_notice(self) -> ();

    #[method(name = "ConvertMyCardLanguage", args = 0)]
    pub fn convert_my_card_language(self) -> ();

    #[method(name = "CreateMyCardRoot", args = 0)]
    pub fn create_my_card_root(self) -> ();

    #[method(name = "ShowMyCardRoot", args = 0)]
    pub fn show_my_card_root(self) -> ();

    #[method(name = "HideMyCardRoot", args = 0)]
    pub fn hide_my_card_root(self) -> ();

    #[method(name = "CreateMyCardMenu", args = 0)]
    pub fn create_my_card_menu(self) -> ();

    #[method(name = "CreateSelectEditMenu", args = 0)]
    pub fn create_select_edit_menu(self) -> ();

    #[method(name = "CreateTitleMenu", args = 0)]
    pub fn create_title_menu(self) -> ();

    #[method(name = "CreateCommentMenu", args = 0)]
    pub fn create_comment_menu(self) -> ();

    #[method(name = "CreateCommentListMenu", args = 0)]
    pub fn create_comment_list_menu(self) -> ();

    #[method(name = "CreateFavoriteCharacterMenu", args = 0)]
    pub fn create_favorite_character_menu(self) -> ();

    #[method(name = "CreateFavoriteRelayMap", args = 0)]
    pub fn create_favorite_relay_map(self) -> ();

    #[method(name = "CreateVisualMenu", args = 0)]
    pub fn create_visual_menu(self) -> ();

    #[method(name = "CreateBgMenu", args = 0)]
    pub fn create_bg_menu(self) -> ();

    #[method(name = "CreateFrameMenu", args = 0)]
    pub fn create_frame_menu(self) -> ();

    #[method(name = "CreateTextDecoMenu", args = 0)]
    pub fn create_text_deco_menu(self) -> ();

    #[method(name = "CreateTextColorMenu", args = 0)]
    pub fn create_text_color_menu(self) -> ();

    #[method(name = "CreateCharacterStampMenu", args = 0)]
    pub fn create_character_stamp_menu(self) -> ();

    #[method(name = "CreateStampMenu", args = 0)]
    pub fn create_stamp_menu(self) -> ();

    #[method(name = "ScaleCardRootLarge", args = 0)]
    pub fn scale_card_root_large(self) -> ();

    #[method(name = "IsCardRootScalingLarge", args = 0)]
    pub fn is_card_root_scaling_large(self) -> bool;

    #[method(name = "ScaleCardRootSmall", args = 0)]
    pub fn scale_card_root_small(self) -> ();

    #[method(name = "IsCardRootScalingSmall", args = 0)]
    pub fn is_card_root_scaling_small(self) -> bool;

    #[method(name = "CreatePhotoSequence", args = 0)]
    pub fn create_photo_sequence(self) -> ();

    #[method(name = "SaveMyCardEdit", args = 0)]
    pub fn save_my_card_edit(self) -> ();

    #[method(name = "CloseMyCardRoot", args = 0)]
    pub fn close_my_card_root(self) -> ();

    #[method(name = "IsClosingMyCardRoot", args = 0)]
    pub fn is_closing_my_card_root(self) -> bool;

    #[method(name = "DestroyMyCardRoot", args = 0)]
    pub fn destroy_my_card_root(self) -> ();

    #[method(name = "LoadAlbumTopContent", args = 0)]
    pub fn load_album_top_content(self) -> ();

    #[method(name = "CreateAlbumRoot", args = 0)]
    pub fn create_album_root(self) -> ();

    #[method(name = "CreateAlbumListMenu", args = 0)]
    pub fn create_album_list_menu(self) -> ();

    #[method(name = "CreateAlbumMenu", args = 0)]
    pub fn create_album_menu(self) -> ();

    #[method(name = "CloseAlubumRoot", args = 0)]
    pub fn close_alubum_root(self) -> ();

    #[method(name = "IsClosingAlbumRoot", args = 0)]
    pub fn is_closing_album_root(self) -> bool;

    #[method(name = "DestroyAlbumRoot", args = 0)]
    pub fn destroy_album_root(self) -> ();

    #[method(name = "ScaleAlbumCardRootSmallImmediately", args = 0)]
    pub fn scale_album_card_root_small_immediately(self) -> ();

    #[method(name = "HideAlbumCardRoot", args = 0)]
    pub fn hide_album_card_root(self) -> ();

    #[method(name = "ShowAlbumCardRoot", args = 0)]
    pub fn show_album_card_root(self) -> ();

    #[method(name = "ScaleAlbumCardRootLarge", args = 0)]
    pub fn scale_album_card_root_large(self) -> ();

    #[method(name = "IsAlbumCardRootScalingLarge", args = 0)]
    pub fn is_album_card_root_scaling_large(self) -> bool;

    #[method(name = "ScaleAlbumCardRootSmall", args = 0)]
    pub fn scale_album_card_root_small(self) -> ();

    #[method(name = "IsAlbumCardRootScalingSmall", args = 0)]
    pub fn is_album_card_root_scaling_small(self) -> bool;

    #[method(name = "CreatePublicSetting", args = 0)]
    pub fn create_public_setting(self) -> ();

    #[method(name = "CreateStampVisibilitySetting", args = 0)]
    pub fn create_stamp_visibility_setting(self) -> ();

    #[method(name = "EndSequence", args = 0)]
    pub fn end_sequence(self) -> ();

    #[method(name = "UnloadResources", args = 0)]
    pub fn unload_resources(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();
}

#[cfg(feature = "app-profilecardsequence")]
impl ProfileCardSequence {
    pub fn new(
        enabled_photo: bool,
        dispose_event_handler : crate :: app :: profilecardsequence :: ProfileCardSequence_DisposeEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardSequenceMethods>::ctor(this, enabled_photo, dispose_event_handler);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardsequence/ProfileCardSequence_DisposeEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardSequence.DisposeEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardSequence_DisposeEventHandler {}

#[cfg(feature = "app-profilecardsequence")]
#[::unity2::methods]
impl ProfileCardSequence_DisposeEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-profilecardsequence")]
impl ProfileCardSequence_DisposeEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardSequence_DisposeEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardSequence_DisposeEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
