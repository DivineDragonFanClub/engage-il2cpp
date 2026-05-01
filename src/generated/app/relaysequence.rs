
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaysequence/RelaySequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RelaySequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RelaySequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RelaySequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RelaySequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RelaySequence_Label {
    pub fn entry() -> Self {
        Self { value: 0 }
    }

    pub fn mode_menu() -> Self {
        Self { value: 1 }
    }

    pub fn new() -> Self {
        Self { value: 2 }
    }

    pub fn new_dispos_player_counts() -> Self {
        Self { value: 3 }
    }

    pub fn new_map_select() -> Self {
        Self { value: 4 }
    }

    pub fn new_setup() -> Self {
        Self { value: 5 }
    }

    pub fn take_over() -> Self {
        Self { value: 6 }
    }

    pub fn take_over_mode_select() -> Self {
        Self { value: 7 }
    }

    pub fn take_over_search() -> Self {
        Self { value: 8 }
    }

    pub fn take_over_search_dispos_player_counts() -> Self {
        Self { value: 9 }
    }

    pub fn take_over_postsearch_branch() -> Self {
        Self { value: 10 }
    }

    pub fn take_over_random_select() -> Self {
        Self { value: 11 }
    }

    pub fn take_over_data_code() -> Self {
        Self { value: 12 }
    }

    pub fn take_over_predownload_meta_with_data_code() -> Self {
        Self { value: 13 }
    }

    pub fn take_over_data_code_dispos_player_counts() -> Self {
        Self { value: 14 }
    }

    pub fn take_over_download_meta_with_data_code() -> Self {
        Self { value: 15 }
    }

    pub fn take_over_show() -> Self {
        Self { value: 16 }
    }

    pub fn take_over_data_menu() -> Self {
        Self { value: 17 }
    }

    pub fn take_over_download() -> Self {
        Self { value: 18 }
    }

    pub fn replay() -> Self {
        Self { value: 19 }
    }

    pub fn replay_data_menu() -> Self {
        Self { value: 20 }
    }

    pub fn replay_download() -> Self {
        Self { value: 21 }
    }

    pub fn map_sequence() -> Self {
        Self { value: 22 }
    }

    pub fn award() -> Self {
        Self { value: 23 }
    }

    pub fn no_ticket() -> Self {
        Self { value: 24 }
    }

    pub fn no_selectable_map() -> Self {
        Self { value: 25 }
    }

    pub fn not_found_map_for_take_over_random() -> Self {
        Self { value: 26 }
    }

    pub fn not_found_map_for_take_over_data_code() -> Self {
        Self { value: 27 }
    }

    pub fn cant_take_over_self_create() -> Self {
        Self { value: 28 }
    }

    pub fn cant_take_over_end() -> Self {
        Self { value: 29 }
    }

    pub fn cant_take_over_other_playing() -> Self {
        Self { value: 30 }
    }

    pub fn cant_take_over_already_played() -> Self {
        Self { value: 31 }
    }

    pub fn cant_take_over_lack_of_unit() -> Self {
        Self { value: 32 }
    }

    pub fn cant_take_over_unknown() -> Self {
        Self { value: 33 }
    }

    pub fn not_found_map_for_replay() -> Self {
        Self { value: 34 }
    }

    pub fn no_entered_map() -> Self {
        Self { value: 35 }
    }

    pub fn cant_get_by_deleting_for_take_over() -> Self {
        Self { value: 36 }
    }

    pub fn cant_get_by_deleting_for_replay() -> Self {
        Self { value: 37 }
    }

    pub fn invalid_download_data_for_take_over() -> Self {
        Self { value: 38 }
    }

    pub fn invalid_download_data_for_replay() -> Self {
        Self { value: 39 }
    }

    pub fn restore_and_return_to_mode_menu() -> Self {
        Self { value: 40 }
    }

    pub fn restore_and_return_to_take_over_mode_select() -> Self {
        Self { value: 41 }
    }

    pub fn shutdown() -> Self {
        Self { value: 42 }
    }

    pub fn end() -> Self {
        Self { value: 43 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaysequence/RelaySequence_RelaySequenceNet.md")))]
#[::unity2::class(namespace = "App", name = "RelaySequence.RelaySequenceNet")]
# [parent (crate :: app :: relaysequence :: RelaySequence_RelaySequenceBase_1 < crate :: app :: relaysequence :: RelaySequence_RelaySequenceNet >)]
pub struct RelaySequence_RelaySequenceNet {}

#[cfg(feature = "app-relaysequence")]
#[::unity2::methods]
impl RelaySequence_RelaySequenceNet {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "TakeOverSearchImpl", args = 0)]
    pub fn take_over_search_impl(self) -> ();

    #[method(name = "TakeOverPostsearchImpl", args = 0)]
    pub fn take_over_postsearch_impl(self) -> ();

    #[method(name = "TakeOverDownloadMetaWithDataCodeImpl", args = 0)]
    pub fn take_over_download_meta_with_data_code_impl(self) -> ();

    #[method(name = "TakeOverPostdownloadMetaWithDataCodeImpl", args = 0)]
    pub fn take_over_postdownload_meta_with_data_code_impl(self) -> ();

    #[method(name = "TakeOverSetPlayingImpl", args = 0)]
    pub fn take_over_set_playing_impl(self) -> ();

    #[method(name = "TakeOverPostsetPlayingImpl", args = 0)]
    pub fn take_over_postset_playing_impl(self) -> ();

    #[method(name = "ReplaySearchEnteredImpl", args = 0)]
    pub fn replay_search_entered_impl(self) -> ();

    #[method(name = "ReplaySearchAnyImpl", args = 0)]
    pub fn replay_search_any_impl(self) -> ();

    #[method(name = "ReplayPostsearchImpl", args = 0)]
    pub fn replay_postsearch_impl(self) -> ();

    #[method(name = "DownloadImpl", args = 1)]
    pub fn download_impl(self, with_meta_data: bool) -> ();

    #[method(name = "PostdownloadTakeOverImpl", args = 0)]
    pub fn postdownload_take_over_impl(self) -> ();

    #[method(name = "PostdownloadReplayImpl", args = 0)]
    pub fn postdownload_replay_impl(self) -> ();

    #[method(name = "UploadImpl", args = 0)]
    pub fn upload_impl(self) -> ();

    #[method(name = "PostuploadImpl", args = 0)]
    pub fn postupload_impl(self) -> ();

    #[method(name = "CloseWaitMessageImpl", args = 2)]
    pub fn close_wait_message_impl(
        self,
        super_: crate::app::procinst::ProcInst,
        is_success: bool,
    ) -> ();

    #[method(name = "DeleteSelectedFromSearchResults", args = 0)]
    pub fn delete_selected_from_search_results(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relaysequence")]
impl RelaySequence_RelaySequenceNet {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelaySequence_RelaySequenceNet),
                ::core::stringify!(new),
            )
        });
        <Self as IRelaySequence_RelaySequenceNetMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaysequence/RelaySequence_RelaySequenceBase_1.md")))]
#[::unity2::class(namespace = "App", name = "RelaySequence.RelaySequenceBase`1")]
pub struct RelaySequence_RelaySequenceBase_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_SearchResults")]
    pub m_search_results: crate::system::collections::generic::list_1::List_1<
        crate::app::relayservermetadata::RelayServerMetaData,
    >,
    #[rename(name = "m_DownloadMetaResult")]
    pub m_download_meta_result: crate::app::relayservermetadata::RelayServerMetaData,
    #[rename(name = "m_DataCode")]
    pub m_data_code: ::unity2::Il2CppString,
    #[rename(name = "m_ReplayCache")]
    pub m_replay_cache: crate::app::relayreplaycache::RelayReplayCache,
    #[rename(name = "m_IsPublic")]
    pub m_is_public: bool,
    #[rename(name = "m_Mode")]
    pub m_mode: crate::app::relay::Relay_Modes,
    #[rename(name = "m_TakeOverMode")]
    pub m_take_over_mode: crate::app::relay::Relay_TakeOverModes,
    #[rename(name = "m_Cid")]
    pub m_cid: ::unity2::Il2CppString,
    #[rename(name = "m_IsAwarded")]
    pub m_is_awarded: bool,
    #[rename(name = "m_Bg")]
    pub m_bg: crate::app::menubg::MenuBg,
}

#[cfg(feature = "app-relaysequence")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> RelaySequence_RelaySequenceBase_1<T0> {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "OnShutdown", args = 0)]
    pub fn on_shutdown(self) -> ();

    #[method(name = "JumpTo", args = 1)]
    pub fn jump_to(self, label: crate::app::relaysequence::RelaySequence_Label) -> ();

    #[method(name = "Backup", args = 0)]
    pub fn backup(self) -> ();

    #[method(name = "RestoreFromBackup", args = 0)]
    pub fn restore_from_backup(self) -> ();

    #[method(name = "RestoreFromBackupForShutdown", args = 0)]
    pub fn restore_from_backup_for_shutdown(self) -> ();

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "OpenTitleBar", args = 0)]
    pub fn open_title_bar(self) -> ();

    #[method(name = "CloseTitleBar", args = 0)]
    pub fn close_title_bar(self) -> ();

    #[method(name = "CreateBg", args = 0)]
    pub fn create_bg(self) -> ();

    #[method(name = "DestroyBg", args = 0)]
    pub fn destroy_bg(self) -> ();

    #[method(name = "UpdateProfileCard", args = 0)]
    pub fn update_profile_card(self) -> ();

    #[method(name = "ModeMenu", args = 0)]
    pub fn mode_menu(self) -> ();

    #[method(name = "SelectMode", args = 1)]
    pub fn select_mode(self, mode: crate::app::relay::Relay_Modes) -> ();

    #[method(name = "DisposPlayerCounts", args = 0)]
    pub fn dispos_player_counts(self) -> ();

    #[method(name = "NewMapSelectMenu", args = 0)]
    pub fn new_map_select_menu(self) -> ();

    #[method(name = "SelectMap", args = 1)]
    pub fn select_map(self, cid: ::unity2::Il2CppString) -> ();

    #[method(name = "NewSetup", args = 0)]
    pub fn new_setup(self) -> ();

    #[method(name = "TakeOverModeMenu", args = 0)]
    pub fn take_over_mode_menu(self) -> ();

    #[method(name = "SelectTakeOverMode", args = 1)]
    pub fn select_take_over_mode(self, mode: crate::app::relay::Relay_TakeOverModes) -> ();

    #[method(name = "TakeOverSearch", args = 0)]
    pub fn take_over_search(self) -> ();

    #[method(name = "TakeOverSearchImpl", args = 0)]
    pub fn take_over_search_impl(self) -> ();

    #[method(name = "TakeOverPostsearch", args = 0)]
    pub fn take_over_postsearch(self) -> ();

    #[method(name = "TakeOverPostsearchImpl", args = 0)]
    pub fn take_over_postsearch_impl(self) -> ();

    #[method(name = "TakeOverPostsearchSucceeded", args = 1)]
    pub fn take_over_postsearch_succeeded(self, principal_id: u64) -> ();

    #[method(name = "CanSelectTakeOver", args = 3)]
    pub fn can_select_take_over(
        self,
        meta_data_from_server: crate::app::relayservermetadata::RelayServerMetaData,
        principal_id: u64,
        result: crate::app::relay::Relay_CstoResult,
    ) -> bool;

    #[method(name = "CanSelectTakeOver", args = 4)]
    pub fn can_select_take_over_2(
        self,
        meta_data_from_server: crate::app::relayservermetadata::RelayServerMetaData,
        principal_id: u64,
        current_unix_time: i64,
        result: crate::app::relay::Relay_CstoResult,
    ) -> bool;

    #[method(name = "TakeOverPostsearchBranch", args = 0)]
    pub fn take_over_postsearch_branch(self) -> ();

    #[method(name = "TakeOverRandomSelect", args = 0)]
    pub fn take_over_random_select(self) -> ();

    #[method(name = "InputDataCode", args = 0)]
    pub fn input_data_code(self) -> ();

    #[method(name = "ConfirmSearchDataCode", args = 0)]
    pub fn confirm_search_data_code(self) -> ();

    #[method(name = "TakeOverDownloadMetaWithDataCode", args = 0)]
    pub fn take_over_download_meta_with_data_code(self) -> ();

    #[method(name = "TakeOverDownloadMetaWithDataCodeImpl", args = 0)]
    pub fn take_over_download_meta_with_data_code_impl(self) -> ();

    #[method(name = "TakeOverPostdownloadMetaWithDataCode", args = 0)]
    pub fn take_over_postdownload_meta_with_data_code(self) -> ();

    #[method(name = "TakeOverPostdownloadMetaWithDataCodeImpl", args = 0)]
    pub fn take_over_postdownload_meta_with_data_code_impl(self) -> ();

    #[method(name = "TakeOverShow", args = 0)]
    pub fn take_over_show(self) -> ();

    #[method(name = "TakeOverSetPlaying", args = 0)]
    pub fn take_over_set_playing(self) -> ();

    #[method(name = "TakeOverSetPlayingImpl", args = 0)]
    pub fn take_over_set_playing_impl(self) -> ();

    #[method(name = "TakeOverPostsetPlaying", args = 0)]
    pub fn take_over_postset_playing(self) -> ();

    #[method(name = "TakeOverPostsetPlayingImpl", args = 0)]
    pub fn take_over_postset_playing_impl(self) -> ();

    #[method(name = "ReplaySearch", args = 0)]
    pub fn replay_search(self) -> ();

    #[method(name = "ReplaySearchEnteredImpl", args = 0)]
    pub fn replay_search_entered_impl(self) -> ();

    #[method(name = "ReplaySearchAnyImpl", args = 0)]
    pub fn replay_search_any_impl(self) -> ();

    #[method(name = "ReplayPostsearch", args = 0)]
    pub fn replay_postsearch(self) -> ();

    #[method(name = "ReplayPostsearchImpl", args = 0)]
    pub fn replay_postsearch_impl(self) -> ();

    #[method(name = "ReplayPostsearchSucceeded", args = 1)]
    pub fn replay_postsearch_succeeded(self, is_cache_used: bool) -> ();

    #[method(name = "ReplayDataMenu", args = 0)]
    pub fn replay_data_menu(self) -> ();

    #[method(name = "DownloadTakeOver", args = 0)]
    pub fn download_take_over(self) -> ();

    #[method(name = "DownloadReplay", args = 0)]
    pub fn download_replay(self) -> ();

    #[method(name = "DownloadImpl", args = 1)]
    pub fn download_impl(self, with_meta_data: bool) -> ();

    #[method(name = "PostdownloadTakeOver", args = 0)]
    pub fn postdownload_take_over(self) -> ();

    #[method(name = "PostdownloadTakeOverImpl", args = 0)]
    pub fn postdownload_take_over_impl(self) -> ();

    #[method(name = "PostdownloadTakeOverSucceeded", args = 0)]
    pub fn postdownload_take_over_succeeded(self) -> ();

    #[method(name = "PostdownloadReplay", args = 0)]
    pub fn postdownload_replay(self) -> ();

    #[method(name = "PostdownloadReplayImpl", args = 0)]
    pub fn postdownload_replay_impl(self) -> ();

    #[method(name = "PostdownloadReplaySucceeded", args = 0)]
    pub fn postdownload_replay_succeeded(self) -> ();

    #[method(name = "PostdownloadSucceededImpl", args = 2)]
    pub fn postdownload_succeeded_impl(
        self,
        label_failed: crate::app::relaysequence::RelaySequence_Label,
        with_meta_data: bool,
    ) -> ();

    #[method(name = "MapSequence", args = 0)]
    pub fn map_sequence(self) -> ();

    #[method(name = "UploadShow", args = 0)]
    pub fn upload_show(self) -> ();

    #[method(name = "SetPublic", args = 1)]
    pub fn set_public(self, is_public: bool) -> ();

    #[method(name = "Upload", args = 0)]
    pub fn upload(self) -> ();

    #[method(name = "UploadImpl", args = 0)]
    pub fn upload_impl(self) -> ();

    #[method(name = "Postupload", args = 0)]
    pub fn postupload(self) -> ();

    #[method(name = "PostuploadImpl", args = 0)]
    pub fn postupload_impl(self) -> ();

    #[method(name = "ShowDataCode", args = 1)]
    pub fn show_data_code(self, code: ::unity2::Il2CppString) -> ();

    #[method(name = "UploadProfileCard", args = 0)]
    pub fn upload_profile_card(self) -> ();

    #[method(name = "UseTicket", args = 0)]
    pub fn use_ticket(self) -> ();

    #[method(name = "IsAward", args = 0)]
    pub fn is_award(self) -> bool;

    #[method(name = "Preaward", args = 0)]
    pub fn preaward(self) -> ();

    #[method(name = "DownloadProfileCards", args = 0)]
    pub fn download_profile_cards(self) -> ();

    #[method(name = "CloseWaitMessageSuccess", args = 0)]
    pub fn close_wait_message_success(self) -> ();

    #[method(name = "CloseWaitMessageFailure", args = 0)]
    pub fn close_wait_message_failure(self) -> ();

    #[method(name = "CloseWaitMessageImpl", args = 2)]
    pub fn close_wait_message_impl(
        self,
        super_: crate::app::procinst::ProcInst,
        is_success: bool,
    ) -> ();

    #[method(name = "NoTicket", args = 0)]
    pub fn no_ticket(self) -> ();

    #[method(name = "NoSelectableMap", args = 0)]
    pub fn no_selectable_map(self) -> ();

    #[method(name = "NotFoundMapForTakeOverRandom", args = 0)]
    pub fn not_found_map_for_take_over_random(self) -> ();

    #[method(name = "NotFoundMapForTakeOverDataCode", args = 0)]
    pub fn not_found_map_for_take_over_data_code(self) -> ();

    #[method(name = "CantTakeOverSelfCreate", args = 0)]
    pub fn cant_take_over_self_create(self) -> ();

    #[method(name = "CantTakeOverEnd", args = 0)]
    pub fn cant_take_over_end(self) -> ();

    #[method(name = "CantTakeOverOtherPlaying", args = 0)]
    pub fn cant_take_over_other_playing(self) -> ();

    #[method(name = "CantTakeOverAlreadyPlayed", args = 0)]
    pub fn cant_take_over_already_played(self) -> ();

    #[method(name = "CantTakeOverLackOfUnit", args = 0)]
    pub fn cant_take_over_lack_of_unit(self) -> ();

    #[method(name = "CantTakeOverUnknown", args = 0)]
    pub fn cant_take_over_unknown(self) -> ();

    #[method(name = "NotFoundMapForReplay", args = 0)]
    pub fn not_found_map_for_replay(self) -> ();

    #[method(name = "NoEnteredMap", args = 0)]
    pub fn no_entered_map(self) -> ();

    #[method(name = "CantGetByDeleting", args = 0)]
    pub fn cant_get_by_deleting(self) -> ();

    #[method(name = "InvalidDownloadData", args = 0)]
    pub fn invalid_download_data(self) -> ();

    #[method(name = "DeletedHalfwayBranchForReplay", args = 0)]
    pub fn deleted_halfway_branch_for_replay(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relaysequence")]
impl<T0: ::unity2::ClassIdentity> RelaySequence_RelaySequenceBase_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelaySequence_RelaySequenceBase_1),
                ::core::stringify!(new),
            )
        });
        <Self as IRelaySequence_RelaySequenceBase_1Methods<T0>>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaysequence/RelaySequence.md")))]
#[::unity2::class(namespace = "App", name = "RelaySequence")]
#[parent(crate::system::object::Object)]
pub struct RelaySequence {}

#[cfg(feature = "app-relaysequence")]
#[::unity2::methods]
impl RelaySequence {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "SelectMode", args = 1)]
    pub fn select_mode(mode: crate::app::relay::Relay_Modes) -> ();

    #[method(name = "SelectTakeOverMode", args = 1)]
    pub fn select_take_over_mode(mode: crate::app::relay::Relay_TakeOverModes) -> ();

    #[method(name = "SelectMap", args = 1)]
    pub fn select_map(cid: ::unity2::Il2CppString) -> ();

    #[method(name = "NoSelectableMap", args = 0)]
    pub fn no_selectable_map() -> ();

    #[method(name = "JumpToTakeOverModeSelect", args = 0)]
    pub fn jump_to_take_over_mode_select() -> ();

    #[method(name = "JumpToTakeOverPredownloadMetaWithDataCode", args = 0)]
    pub fn jump_to_take_over_predownload_meta_with_data_code() -> ();

    #[method(name = "JumpToReplayDownload", args = 0)]
    pub fn jump_to_replay_download() -> ();

    #[method(name = "JumpToAward", args = 0)]
    pub fn jump_to_award() -> ();

    #[method(name = "SetPublic", args = 1)]
    pub fn set_public(is_public: bool) -> ();

    #[method(name = "GetSequence", args = 0)]
    pub fn get_sequence() -> crate::app::relaysequence::RelaySequence_IRelaySequenceBase;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaysequence/RelaySequence_RelaySequenceLocal.md")))]
#[::unity2::class(namespace = "App", name = "RelaySequence.RelaySequenceLocal")]
# [parent (crate :: app :: relaysequence :: RelaySequence_RelaySequenceBase_1 < crate :: app :: relaysequence :: RelaySequence_RelaySequenceLocal >)]
pub struct RelaySequence_RelaySequenceLocal {
    #[rename(name = "m_RootPath")]
    pub m_root_path: ::unity2::Il2CppString,
}

#[cfg(feature = "app-relaysequence")]
#[::unity2::methods]
impl RelaySequence_RelaySequenceLocal {
    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "InitializeLanRootPath", args = 0)]
    pub fn initialize_lan_root_path(self) -> ();

    #[method(name = "TakeOverSearchImpl", args = 0)]
    pub fn take_over_search_impl(self) -> ();

    #[method(name = "TakeOverPostsearchImpl", args = 0)]
    pub fn take_over_postsearch_impl(self) -> ();

    #[method(name = "TakeOverDownloadMetaWithDataCodeImpl", args = 0)]
    pub fn take_over_download_meta_with_data_code_impl(self) -> ();

    #[method(name = "TakeOverPostdownloadMetaWithDataCodeImpl", args = 0)]
    pub fn take_over_postdownload_meta_with_data_code_impl(self) -> ();

    #[method(name = "TakeOverSetPlayingImpl", args = 0)]
    pub fn take_over_set_playing_impl(self) -> ();

    #[method(name = "TakeOverPostsetPlayingImpl", args = 0)]
    pub fn take_over_postset_playing_impl(self) -> ();

    #[method(name = "ReplaySearchEnteredImpl", args = 0)]
    pub fn replay_search_entered_impl(self) -> ();

    #[method(name = "ReplaySearchAnyImpl", args = 0)]
    pub fn replay_search_any_impl(self) -> ();

    #[method(name = "ReplayPostsearchImpl", args = 0)]
    pub fn replay_postsearch_impl(self) -> ();

    #[method(name = "DownloadImpl", args = 1)]
    pub fn download_impl(self, with_meta_data: bool) -> ();

    #[method(name = "PostdownloadTakeOverImpl", args = 0)]
    pub fn postdownload_take_over_impl(self) -> ();

    #[method(name = "PostdownloadReplayImpl", args = 0)]
    pub fn postdownload_replay_impl(self) -> ();

    #[method(name = "UploadImpl", args = 0)]
    pub fn upload_impl(self) -> ();

    #[method(name = "PostuploadImpl", args = 0)]
    pub fn postupload_impl(self) -> ();

    #[method(name = "GetDataFileName", args = 1)]
    pub fn get_data_file_name(
        meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetMetaDataFileName", args = 1)]
    pub fn get_meta_data_file_name(
        meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetPseudoPrincipalId", args = 1)]
    pub fn get_pseudo_principal_id(player_name: ::unity2::Il2CppString) -> u64;

    #[method(name = "GetPseudoDataId", args = 1)]
    pub fn get_pseudo_data_id(meta_path: ::unity2::Il2CppString) -> u64;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relaysequence")]
impl RelaySequence_RelaySequenceLocal {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelaySequence_RelaySequenceLocal),
                ::core::stringify!(new),
            )
        });
        <Self as IRelaySequence_RelaySequenceLocalMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaysequence/RelaySequence_IRelaySequenceBase.md")))]
#[::unity2::class(namespace = "App", name = "RelaySequence.IRelaySequenceBase")]
pub struct RelaySequence_IRelaySequenceBase {}

#[cfg(feature = "app-relaysequence")]
#[::unity2::methods]
impl RelaySequence_IRelaySequenceBase {
    #[method(name = "JumpTo", args = 1)]
    pub fn jump_to(self, label: crate::app::relaysequence::RelaySequence_Label) -> ();

    #[method(name = "SelectMode", args = 1)]
    pub fn select_mode(self, mode: crate::app::relay::Relay_Modes) -> ();

    #[method(name = "SelectMap", args = 1)]
    pub fn select_map(self, cid: ::unity2::Il2CppString) -> ();

    #[method(name = "SelectTakeOverMode", args = 1)]
    pub fn select_take_over_mode(self, mode: crate::app::relay::Relay_TakeOverModes) -> ();

    #[method(name = "SetPublic", args = 1)]
    pub fn set_public(self, enable: bool) -> ();
}
