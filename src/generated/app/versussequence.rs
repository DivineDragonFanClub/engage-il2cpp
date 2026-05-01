
use crate::app::basicdialog::BasicDialog;
use crate::app::basicdialog::IBasicDialog;
use crate::app::basicdialogitem::BasicDialogItem;
use crate::app::basicdialogitem::IBasicDialogItem;
use crate::app::basicdialogitemno::BasicDialogItemNo;
use crate::app::basicdialogitemno::IBasicDialogItemNo;
use crate::app::basicdialogitemyes::BasicDialogItemYes;
use crate::app::basicdialogitemyes::IBasicDialogItemYes;
use crate::app::basicmenu::BasicMenu;
use crate::app::basicmenu::IBasicMenu;
use crate::app::basicmenuitem::BasicMenuItem;
use crate::app::basicmenuitem::IBasicMenuItem;
use crate::app::debugmenu::DebugMenu;
use crate::app::debugmenu::IDebugMenu;
use crate::app::menuitem::IMenuItem;
use crate::app::menuitem::MenuItem;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::app::yesnodialog::IYesNoDialog;
use crate::app::yesnodialog::YesNoDialog;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusMatchingMenu_CodeMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusMatchingMenu.CodeMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusSequence_VersusMatchingMenu_CodeMenuItem {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusMatchingMenu_CodeMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusMatchingMenu_CodeMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusMatchingMenu_CodeMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusMatchingMenu_CodeMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusSequenceLocal.SelectReplayMetaMenuItem"
)]
#[parent(crate::app::menuitem::MenuItem)]
pub struct VersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem {
# [rename (name = "m_MetaData")] pub m_meta_data : crate :: app :: versusserverreplaymetadata :: VersusServerReplayMetaData ,
# [rename (name = "m_MetaPath")] pub m_meta_path : :: unity2 :: Il2CppString ,
# [rename (name = "m_SlotId")] pub m_slot_id : u16 ,
# [rename (name = "m_ACallback")] pub m_a_callback : crate :: app :: versussequence :: VersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem_ACallback ,
# [rename (name = "m_IsUpload")] pub m_is_upload : bool ,
# [rename (name = "m_IsPlay")] pub m_is_play : bool ,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem {
    #[method(name = ".ctor", args = 6)]
    pub fn ctor(
        self,
        path: ::unity2::Il2CppString,
        meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
        slot_id: u16,
        is_upload: bool,
        is_play: bool,
        callback : crate :: app :: versussequence :: VersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem_ACallback,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem {
    pub fn new(
        path: ::unity2::Il2CppString,
        meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
        slot_id: u16,
        is_upload: bool,
        is_play: bool,
        callback : crate :: app :: versussequence :: VersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem_ACallback,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItemMethods>::ctor(
            this, path, meta_data, slot_id, is_upload, is_play, callback,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusFriendMenu.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.VersusFriendMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct VersusSequence_VersusFriendMenu {
    #[static_field]
    #[rename(name = "c_FriendShowMax")]
    pub c_friend_show_max: i32,
    #[static_field]
    #[rename(name = "m_InitialSelected")]
    pub m_initial_selected: i32,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusFriendMenu {
    #[method(name = "InitializedSelected", args = 0)]
    pub fn initialized_selected() -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        meta_data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::versusservercasualmetadata::VersusServerCasualMetaData,
        >,
    ) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind_2(
        super_: crate::app::procinst::ProcInst,
        meta_data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::versusserverrankedmetadata::VersusServerRankedMetaData,
        >,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::versusfriendmenucontent::VersusFriendMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusFriendMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::versusfriendmenucontent::VersusFriendMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusFriendMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusFriendMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceBase_1.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.VersusSequenceBase`1")]
pub struct VersusSequence_VersusSequenceBase_1<T0: ::unity2::ClassIdentity> {
    #[rename(name = "m_IsStartedCasual")]
    pub m_is_started_casual: bool,
    #[rename(name = "m_SearchCasualResults")]
    pub m_search_casual_results: crate::system::collections::generic::list_1::List_1<
        crate::app::versusservercasualmetadata::VersusServerCasualMetaData,
    >,
    #[rename(name = "m_SearchFriendCasualResults")]
    pub m_search_friend_casual_results: crate::system::collections::generic::list_1::List_1<
        crate::app::versusservercasualmetadata::VersusServerCasualMetaData,
    >,
    #[rename(name = "m_ProfileCasualResults")]
    pub m_profile_casual_results: crate::app::nexversus::NexVersus_TargetSlotList,
    #[rename(name = "m_IsStartedRanked")]
    pub m_is_started_ranked: bool,
    #[rename(name = "m_SearchRankedResults")]
    pub m_search_ranked_results: crate::system::collections::generic::list_1::List_1<
        crate::app::versusserverrankedmetadata::VersusServerRankedMetaData,
    >,
    #[rename(name = "m_SearchFriendRankedResults")]
    pub m_search_friend_ranked_results: crate::system::collections::generic::list_1::List_1<
        crate::app::versusserverrankedmetadata::VersusServerRankedMetaData,
    >,
    #[rename(name = "m_ProfileRankedResults")]
    pub m_profile_ranked_results: crate::app::nexversus::NexVersus_TargetSlotList,
    #[rename(name = "m_MyEditDataId")]
    pub m_my_edit_data_id: u64,
    #[rename(name = "m_IsReported")]
    pub m_is_reported: bool,
    #[rename(name = "m_IsShowReportedWindow")]
    pub m_is_show_reported_window: bool,
    #[static_field]
    #[rename(name = "s_RateDataType")]
    pub s_rate_data_type: u16,
    #[static_field]
    #[rename(name = "m_MyCasualDataCode")]
    pub m_my_casual_data_code: ::unity2::Il2CppString,
    #[rename(name = "m_SelectedReplayMetaData")]
    pub m_selected_replay_meta_data:
        crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
    #[rename(name = "m_ReplayResult")]
    pub m_replay_result: crate::app::versus::Versus_MapResult,
    #[rename(name = "m_Bg")]
    pub m_bg: crate::app::menubg::MenuBg,
    #[rename(name = "m_MyInfo")]
    pub m_my_info: crate::app::versusmyinfocontent::VersusMyInfoContent,
    #[rename(name = "m_IsInitialized")]
    pub m_is_initialized: bool,
    #[rename(name = "m_TargetDataCode")]
    pub m_target_data_code: ::unity2::Il2CppString,
    #[rename(name = "m_ProfileTarget")]
    pub m_profile_target: crate::app::profilecard::ProfileCard,
    #[static_field]
    #[rename(name = "MaxGettableFriendCount")]
    pub max_gettable_friend_count: i32,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl<T0: ::unity2::ClassIdentity> VersusSequence_VersusSequenceBase_1<T0> {
    #[method(name = "get_RateDataTypeName", args = 0)]
    pub fn get_rate_data_type_name() -> ::unity2::Il2CppString;

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "JumpTo", args = 1)]
    pub fn jump_to(self, label: crate::app::versussequence::VersusSequence_Label) -> ();

    #[method(name = "GetMyEditDataId", args = 0)]
    pub fn get_my_edit_data_id(self) -> u64;

    #[method(name = "SetProfileTarget", args = 1)]
    pub fn set_profile_target(self, profile: crate::app::profilecard::ProfileCard) -> ();

    #[method(name = "GetReported", args = 0)]
    pub fn get_reported(self) -> bool;

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "MenuInitialized", args = 0)]
    pub fn menu_initialized(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "InitImpl", args = 0)]
    pub fn init_impl(self) -> ();

    #[method(name = "Final", args = 0)]
    pub fn r#final(self) -> ();

    #[method(name = "FinalImpl", args = 0)]
    pub fn final_impl(self) -> ();

    #[method(name = "IsFailedJumpToTopImpl", args = 0)]
    pub fn is_failed_jump_to_top_impl(self) -> bool;

    #[method(name = "IsReportedEditMapImpl", args = 0)]
    pub fn is_reported_edit_map_impl(self) -> bool;

    #[method(name = "UploadReportDataImpl", args = 0)]
    pub fn upload_report_data_impl(self) -> ();

    #[method(name = "StartSequence", args = 0)]
    pub fn start_sequence(self) -> ();

    #[method(name = "EndSequence", args = 0)]
    pub fn end_sequence(self) -> ();

    #[method(name = "OpenTopMenu", args = 0)]
    pub fn open_top_menu(self) -> ();

    #[method(name = "OpenCasualMenu", args = 0)]
    pub fn open_casual_menu(self) -> ();

    #[method(name = "CopyEditDataFromServer", args = 0)]
    pub fn copy_edit_data_from_server(self)
        -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "OpenRankedMenu", args = 0)]
    pub fn open_ranked_menu(self) -> ();

    #[method(name = "CheckOldEditData", args = 0)]
    pub fn check_old_edit_data(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "OpenMatchingMenu", args = 0)]
    pub fn open_matching_menu(self) -> ();

    #[method(name = "BranchMatching", args = 0)]
    pub fn branch_matching(self) -> ();

    #[method(name = "CloseTitleBarAndBg", args = 0)]
    pub fn close_title_bar_and_bg(self) -> ();

    #[method(name = "ResetCheckData", args = 1)]
    pub fn reset_check_data(self, check_data_type: crate::app::versus::Versus_CheckDataType) -> ();

    #[method(name = "GetMyCasualData", args = 0)]
    pub fn get_my_casual_data(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "GetMyCasualMetaDataImpl", args = 0)]
    pub fn get_my_casual_meta_data_impl(self) -> ();

    #[method(name = "PostGetMyCasualMetaDataImpl", args = 0)]
    pub fn post_get_my_casual_meta_data_impl(self) -> ();

    #[method(name = "CheckParentalControl", args = 0)]
    pub fn check_parental_control(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "GetMyEditData", args = 0)]
    pub fn get_my_edit_data(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "GetEditMetaDataImpl", args = 0)]
    pub fn get_edit_meta_data_impl(self) -> ();

    #[method(name = "PostGetEditMetaDataImpl", args = 0)]
    pub fn post_get_edit_meta_data_impl(self) -> ();

    #[method(name = "UpdateRateImpl", args = 0)]
    pub fn update_rate_impl(self) -> ();

    #[method(name = "SetRankedInfoImpl", args = 0)]
    pub fn set_ranked_info_impl(self) -> ();

    #[method(name = "SearchSameRate", args = 0)]
    pub fn search_same_rate(self) -> ();

    #[method(name = "SearchSameRateImpl", args = 0)]
    pub fn search_same_rate_impl(self) -> ();

    #[method(name = "SearchRankedFriend", args = 0)]
    pub fn search_ranked_friend(self) -> ();

    #[method(name = "SearchRankedFriendImpl", args = 0)]
    pub fn search_ranked_friend_impl(self) -> ();

    #[method(name = "PostSearchRankedFriend", args = 0)]
    pub fn post_search_ranked_friend(self) -> ();

    #[method(name = "PostSearchSameRate", args = 0)]
    pub fn post_search_same_rate(self) -> ();

    #[method(name = "ExcludeMySearchRankedData", args = 0)]
    pub fn exclude_my_search_ranked_data(self) -> ();

    #[method(name = "SearchCasual", args = 0)]
    pub fn search_casual(self) -> ();

    #[method(name = "SearchCasualImpl", args = 0)]
    pub fn search_casual_impl(self) -> ();

    #[method(name = "SearchCasualFriend", args = 0)]
    pub fn search_casual_friend(self) -> ();

    #[method(name = "SearchCasualFriendImpl", args = 0)]
    pub fn search_casual_friend_impl(self) -> ();

    #[method(name = "PostSearchCasual", args = 0)]
    pub fn post_search_casual(self) -> ();

    #[method(name = "ExcludeMySearchCasualData", args = 1)]
    pub fn exclude_my_search_casual_data(
        self,
        data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::versusservercasualmetadata::VersusServerCasualMetaData,
        >,
    ) -> ();

    #[method(name = "PostSearchCasualFriend", args = 0)]
    pub fn post_search_casual_friend(self) -> ();

    #[method(name = "SelectFriendCasual", args = 0)]
    pub fn select_friend_casual(self) -> ();

    #[method(name = "InputDataCode", args = 0)]
    pub fn input_data_code(self) -> ();

    #[method(name = "SearchDataCode", args = 0)]
    pub fn search_data_code(self) -> ();

    #[method(name = "DownloadMetaCasualFromDataCode", args = 0)]
    pub fn download_meta_casual_from_data_code(self) -> ();

    #[method(name = "DownloadMetaCasualFromDataCodeImpl", args = 0)]
    pub fn download_meta_casual_from_data_code_impl(self) -> ();

    #[method(name = "DownloadMetaRankedFromDataCode", args = 0)]
    pub fn download_meta_ranked_from_data_code(self) -> ();

    #[method(name = "DownloadMetaRankedFromDataCodeImpl", args = 0)]
    pub fn download_meta_ranked_from_data_code_impl(self) -> ();

    #[method(name = "SelectFriendRanked", args = 0)]
    pub fn select_friend_ranked(self) -> ();

    #[method(name = "StartBattleDialogCasualDataCode", args = 0)]
    pub fn start_battle_dialog_casual_data_code(self) -> ();

    #[method(name = "StartBattleDialogRankedDataCode", args = 0)]
    pub fn start_battle_dialog_ranked_data_code(self) -> ();

    #[method(name = "CheckProfileCasual", args = 0)]
    pub fn check_profile_casual(self) -> ();

    #[method(name = "CheckAccessibleProfileImpl", args = 2)]
    pub fn check_accessible_profile_impl(
        self,
        mode: crate::app::versus::Versus_Mode,
        target_slot_list: crate::app::nexversus::NexVersus_TargetSlotList,
    ) -> ();

    #[method(name = "SelectProfileCasual", args = 0)]
    pub fn select_profile_casual(self) -> ();

    #[method(name = "DownloadMetaCasualProfile", args = 0)]
    pub fn download_meta_casual_profile(self) -> ();

    #[method(name = "DownloadMetaProfileImpl", args = 2)]
    pub fn download_meta_profile_impl(
        self,
        mode: crate::app::versus::Versus_Mode,
        profile: crate::app::profilecard::ProfileCard,
    ) -> ();

    #[method(name = "CheckProfileRanked", args = 0)]
    pub fn check_profile_ranked(self) -> ();

    #[method(name = "SelectProfileRanked", args = 0)]
    pub fn select_profile_ranked(self) -> ();

    #[method(name = "DownloadMetaRankedProfile", args = 0)]
    pub fn download_meta_ranked_profile(self) -> ();

    #[method(name = "Prepare", args = 0)]
    pub fn prepare(self) -> ();

    #[method(name = "ClearReliance", args = 0)]
    pub fn clear_reliance(self) -> ();

    #[method(name = "ClearEnhance", args = 0)]
    pub fn clear_enhance(self) -> ();

    #[method(name = "DeleteGodLueur", args = 0)]
    pub fn delete_god_lueur(self) -> ();

    #[method(name = "PrepareCasual", args = 0)]
    pub fn prepare_casual(self) -> ();

    #[method(name = "PrepareRanked", args = 0)]
    pub fn prepare_ranked(self) -> ();

    #[method(name = "PrepareMapEdit", args = 0)]
    pub fn prepare_map_edit(self) -> ();

    #[method(name = "SelectReplay", args = 0)]
    pub fn select_replay(self) -> ();

    #[method(name = "SelectReplayImpl", args = 0)]
    pub fn select_replay_impl(self) -> ();

    #[method(name = "OpenReplayDialog", args = 0)]
    pub fn open_replay_dialog(self) -> ();

    #[method(name = "DownloadReplay", args = 0)]
    pub fn download_replay(self) -> ();

    #[method(name = "DownloadReplayImpl", args = 0)]
    pub fn download_replay_impl(self) -> ();

    #[method(name = "PostDownloadReplay", args = 0)]
    pub fn post_download_replay(self) -> ();

    #[method(name = "CheckMockBattleData", args = 0)]
    pub fn check_mock_battle_data(self) -> ();

    #[method(name = "PrepareReplay", args = 0)]
    pub fn prepare_replay(self) -> ();

    #[method(name = "DisableReplayData", args = 0)]
    pub fn disable_replay_data(self) -> ();

    #[method(name = "ChangeReplayMetaImpl", args = 0)]
    pub fn change_replay_meta_impl(self) -> ();

    #[method(name = "ReceiveReward", args = 0)]
    pub fn receive_reward(self) -> ();

    #[method(name = "GetReward", args = 3)]
    pub fn get_reward(
        self,
        reward_iid: ::unity2::Il2CppString,
        count: i32,
        result_mess: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "DownloadCasual", args = 0)]
    pub fn download_casual(self) -> ();

    #[method(name = "DownloadCasualImpl", args = 0)]
    pub fn download_casual_impl(self) -> ();

    #[method(name = "PostDownloadCasual", args = 0)]
    pub fn post_download_casual(self) -> ();

    #[method(name = "DownloadRanked", args = 0)]
    pub fn download_ranked(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "DownloadImpl", args = 0)]
    pub fn download_impl(self) -> ();

    #[method(name = "DownloadMyEditData", args = 0)]
    pub fn download_my_edit_data(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "DownloadMyEditImpl", args = 0)]
    pub fn download_my_edit_impl(self) -> ();

    #[method(name = "DownloadMyEditForEditMode", args = 0)]
    pub fn download_my_edit_for_edit_mode(
        self,
    ) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "UploadReplay", args = 0)]
    pub fn upload_replay(self) -> ();

    #[method(name = "UploadReplayImpl", args = 0)]
    pub fn upload_replay_impl(self) -> ();

    #[method(name = "PostUploadReplay", args = 0)]
    pub fn post_upload_replay(self) -> ();

    #[method(name = "SerializeCasualDataInit", args = 0)]
    pub fn serialize_casual_data_init(self) -> ();

    #[method(name = "BranchEndCasual", args = 0)]
    pub fn branch_end_casual(self) -> ();

    #[method(name = "BranchEndRanked", args = 0)]
    pub fn branch_end_ranked(self) -> ();

    #[method(name = "UploadCasual", args = 0)]
    pub fn upload_casual(self) -> ();

    #[method(name = "PostUploadCasual", args = 0)]
    pub fn post_upload_casual(self) -> ();

    #[method(name = "ChangeCasualOpponent", args = 0)]
    pub fn change_casual_opponent(self) -> ();

    #[method(name = "UploadCasualImpl", args = 0)]
    pub fn upload_casual_impl(self) -> ();

    #[method(name = "ChangeCasualImpl", args = 0)]
    pub fn change_casual_impl(self) -> ();

    #[method(name = "ChangeCasualOpponentImpl", args = 0)]
    pub fn change_casual_opponent_impl(self) -> ();

    #[method(name = "CalculateRate", args = 0)]
    pub fn calculate_rate(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "GetDownloadedEditMetaDataImpl", args = 0)]
    pub fn get_downloaded_edit_meta_data_impl(
        self,
    ) -> crate::app::versusserverrankedmetadata::VersusServerRankedMetaData;

    #[method(name = "CheckRateDataType", args = 3)]
    pub fn check_rate_data_type(self, old_rate: i32, new_rate: i32, data_id: u64) -> ();

    #[method(name = "ChangeRankedMetaDataImpl", args = 1)]
    pub fn change_ranked_meta_data_impl(
        self,
        meta_data: crate::app::versusserverrankedmetadata::VersusServerRankedMetaData,
    ) -> ();

    #[method(name = "ChangeDataTypeImpl", args = 2)]
    pub fn change_data_type_impl(self, data_id: u64, data_type: u16) -> ();

    #[method(name = "ShowCasualResult", args = 0)]
    pub fn show_casual_result(self) -> ();

    #[method(name = "ShowRankedResult", args = 0)]
    pub fn show_ranked_result(self) -> ();

    #[method(name = "DownloadProfile", args = 0)]
    pub fn download_profile(self) -> ();

    #[method(name = "DownloadProfileForReplay", args = 0)]
    pub fn download_profile_for_replay(self) -> ();

    #[method(name = "UploadProfile", args = 0)]
    pub fn upload_profile(self) -> ();

    #[method(name = "UpdateProfile", args = 0)]
    pub fn update_profile(self) -> ();

    #[method(name = "ResetCachePostDownload", args = 0)]
    pub fn reset_cache_post_download(self) -> ();

    #[method(name = "StopBgm_Hub", args = 0)]
    pub fn stop_bgm_hub() -> ();

    #[method(name = "Save", args = 0)]
    pub fn save(self) -> ();

    #[method(name = "Restore", args = 0)]
    pub fn restore(self) -> ();

    #[method(name = "OpenInvalidDataDialog", args = 0)]
    pub fn open_invalid_data_dialog(self) -> ();

    #[method(name = "AddCounters", args = 0)]
    pub fn add_counters(self) -> ();

    #[method(name = "AddCounterDefense", args = 0)]
    pub fn add_counter_defense(self) -> ();

    #[method(name = "AddPlayReportStartCount", args = 0)]
    pub fn add_play_report_start_count(self) -> ();

    #[method(name = "GetFriendList", args = 3)]
    pub fn get_friend_list(
        self,
        is_filter: bool,
        is_sort: bool,
        max_count: i32,
    ) -> crate::system::collections::generic::list_1::List_1<u64>;

    #[method(name = "CheckParentalControlImpl", args = 0)]
    pub fn check_parental_control_impl(self) -> ();

    #[method(name = "IsParentalControlAvailableImpl", args = 0)]
    pub fn is_parental_control_available_impl(self) -> bool;

    #[method(name = "EndParentalControlImpl", args = 0)]
    pub fn end_parental_control_impl(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "GetDataTypeName", args = 1)]
    pub fn get_data_type_name(data_type: u16) -> ::unity2::Il2CppString;

    #[method(
        name = "App.VersusSequence.IVersusSequenceBase.GetNowRateName",
        args = 0
    )]
    pub fn app_versus_sequence_i_versus_sequence_base_get_now_rate_name(
        self,
    ) -> ::unity2::Il2CppString;

    #[method(name = "InitEditData", args = 0)]
    pub fn init_edit_data(self) -> ();

    #[method(name = "InitEditDataImpl", args = 0)]
    pub fn init_edit_data_impl(self) -> ();

    #[method(name = "PrepareEditData", args = 0)]
    pub fn prepare_edit_data(self) -> ();

    #[method(name = "UploadEditData", args = 0)]
    pub fn upload_edit_data(self) -> ();

    #[method(name = "UploadEditDataImpl", args = 0)]
    pub fn upload_edit_data_impl(self) -> ();

    #[method(name = "PostUploadEditData", args = 0)]
    pub fn post_upload_edit_data(self) -> ();

    #[method(name = "PostUploadEditDataImpl", args = 0)]
    pub fn post_upload_edit_data_impl(self) -> ();

    #[method(name = "GetLastUploadedResultImpl", args = 0)]
    pub fn get_last_uploaded_result_impl(self) -> bool;

    #[method(name = "CreateBindUploadEditData", args = 1)]
    pub fn create_bind_upload_edit_data(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindUploadCasualData", args = 1)]
    pub fn create_bind_upload_casual_data(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-versussequence")]
impl<T0: ::unity2::ClassIdentity> VersusSequence_VersusSequenceBase_1<T0> {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusSequenceBase_1),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusSequenceBase_1Methods<T0>>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusRankedMenu_Result2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct VersusSequence_VersusRankedMenu_Result2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for VersusSequence_VersusRankedMenu_Result2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "VersusSequence.VersusRankedMenu.Result2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for VersusSequence_VersusRankedMenu_Result2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl VersusSequence_VersusRankedMenu_Result2 {
    pub fn start() -> Self {
        Self { value: 0 }
    }

    pub fn map_edit() -> Self {
        Self { value: 1 }
    }

    pub fn result() -> Self {
        Self { value: 2 }
    }

    pub fn mock_battle() -> Self {
        Self { value: 3 }
    }

    pub fn end() -> Self {
        Self { value: 4 }
    }

    pub fn top() -> Self {
        Self { value: 0 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceNet_UploadEditMapSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct VersusSequence_VersusSequenceNet_UploadEditMapSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for VersusSequence_VersusSequenceNet_UploadEditMapSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "VersusSequence.VersusSequenceNet.UploadEditMapSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for VersusSequence_VersusSequenceNet_UploadEditMapSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl VersusSequence_VersusSequenceNet_UploadEditMapSequence_Label {
    pub fn error() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceNet_SelectReplaySequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusSequenceNet.SelectReplaySequence"
)]
#[parent(crate::app::procinst::ProcInst)]
pub struct VersusSequence_VersusSequenceNet_SelectReplaySequence {
    #[rename(name = "m_Callback")]
    pub m_callback:
        crate::app::versussequence::VersusSequence_VersusSequenceNet_SelectReplaySequence_Callback,
    #[rename(name = "m_ResultData")]
    pub m_result_data: crate::app::nexversus::NexVersus_ReplaySlotMetaResultData,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusSequenceNet_SelectReplaySequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        callback : crate :: app :: versussequence :: VersusSequence_VersusSequenceNet_SelectReplaySequence_Callback,
    ) -> ();

    #[method(name = "GetReplaySlots", args = 0)]
    pub fn get_replay_slots(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "SortMetaData", args = 0)]
    pub fn sort_meta_data(self) -> ();

    #[method(name = "Error", args = 0)]
    pub fn error(self) -> ();

    #[method(name = "OpenSelectMenu", args = 0)]
    pub fn open_select_menu(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        callback : crate :: app :: versussequence :: VersusSequence_VersusSequenceNet_SelectReplaySequence_Callback,
    ) -> ();

    #[method(name = "CreateSelectBind", args = 2)]
    pub fn create_select_bind(
        super_: crate::app::procinst::ProcInst,
        callback : crate :: app :: versussequence :: VersusSequence_VersusSequenceNet_SelectReplaySequence_Callback,
    ) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusSequenceNet_SelectReplaySequence {
    pub fn new(
        callback : crate :: app :: versussequence :: VersusSequence_VersusSequenceNet_SelectReplaySequence_Callback,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusSequenceNet_SelectReplaySequence),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusSequenceNet_SelectReplaySequenceMethods>::ctor(
            this, callback,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusFriendMenu_FriendCasualMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusFriendMenu.FriendCasualMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusSequence_VersusFriendMenu_FriendCasualMenuItem {
    #[rename(name = "m_MetaData")]
    pub m_meta_data: crate::app::versusservercasualmetadata::VersusServerCasualMetaData,
    #[rename(name = "m_FriendNumber")]
    pub m_friend_number: i32,
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusFriendMenu_FriendCasualMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        meta_data: crate::app::versusservercasualmetadata::VersusServerCasualMetaData,
        num: i32,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "StartBattleDialog", args = 0)]
    pub fn start_battle_dialog(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusFriendMenu_FriendCasualMenuItem {
    pub fn new(
        meta_data: crate::app::versusservercasualmetadata::VersusServerCasualMetaData,
        num: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusFriendMenu_FriendCasualMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusFriendMenu_FriendCasualMenuItemMethods>::ctor(
            this, meta_data, num,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusRankedMenu_MockBattleMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusRankedMenu.MockBattleMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusSequence_VersusRankedMenu_MockBattleMenuItem {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusRankedMenu_MockBattleMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusRankedMenu_MockBattleMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusRankedMenu_MockBattleMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusRankedMenu_MockBattleMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusMatchingMenu_FriendMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusMatchingMenu.FriendMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusSequence_VersusMatchingMenu_FriendMenuItem {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusMatchingMenu_FriendMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusMatchingMenu_FriendMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusMatchingMenu_FriendMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusMatchingMenu_FriendMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceNet_UploadEditMapSequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusSequenceNet.UploadEditMapSequence"
)]
#[parent(crate::app::procinst::ProcInst)]
pub struct VersusSequence_VersusSequenceNet_UploadEditMapSequence {
    #[rename(name = "m_IsNewUpload")]
    pub m_is_new_upload: bool,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusSequenceNet_UploadEditMapSequence {
    #[method(name = "get_Result", args = 0)]
    pub fn get_result() -> crate::app::nexversus::NexVersus_Results;

    #[method(name = "set_Result", args = 1)]
    pub fn set_result(value: crate::app::nexversus::NexVersus_Results) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, is_new_upload: bool) -> ();

    #[method(name = "UploadEditMap", args = 0)]
    pub fn upload_edit_map(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "PostUploadEditMap", args = 0)]
    pub fn post_upload_edit_map(self) -> ();

    #[method(name = "UploadReplayDatas", args = 0)]
    pub fn upload_replay_datas(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "UploadReplay", args = 1)]
    pub fn upload_replay(self, slot_id: u16) -> ();

    #[method(name = "PostUploadReplay", args = 0)]
    pub fn post_upload_replay(self) -> ();

    #[method(name = "SetResult", args = 0)]
    pub fn set_result_2(self) -> ();

    #[method(name = "PrintError", args = 0)]
    pub fn print_error(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, is_new_upload: bool) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusSequenceNet_UploadEditMapSequence {
    pub fn new(is_new_upload: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusSequenceNet_UploadEditMapSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusSequenceNet_UploadEditMapSequenceMethods>::ctor(
            this,
            is_new_upload,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceNet_UploadReplaySequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusSequenceNet.UploadReplaySequence"
)]
#[parent(crate::app::procinst::ProcInst)]
pub struct VersusSequence_VersusSequenceNet_UploadReplaySequence {
    #[rename(name = "m_MetaData")]
    pub m_meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
    #[rename(name = "m_Data")]
    pub m_data: crate::app::versusserverreplaydata::VersusServerReplayData,
    #[rename(name = "m_OpponentPrincipalId")]
    pub m_opponent_principal_id: u64,
    #[rename(name = "m_OpponentMetaData")]
    pub m_opponent_meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
    #[rename(name = "m_TargetMetaData")]
    pub m_target_meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusSequenceNet_UploadReplaySequence {
    #[method(name = "get_Result", args = 0)]
    pub fn get_result() -> crate::app::nexversus::NexVersus_Results;

    #[method(name = "set_Result", args = 1)]
    pub fn set_result(value: crate::app::nexversus::NexVersus_Results) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        opponent_principal_id: u64,
        meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
        data: crate::app::versusserverreplaydata::VersusServerReplayData,
    ) -> ();

    #[method(name = "GetReplayMetaDatas", args = 0)]
    pub fn get_replay_meta_datas(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ChangeMetaData", args = 0)]
    pub fn change_meta_data(self) -> ();

    #[method(name = "PostChangeMetaData", args = 0)]
    pub fn post_change_meta_data(self) -> ();

    #[method(name = "ChangeReplayData", args = 0)]
    pub fn change_replay_data(self) -> ();

    #[method(name = "PostChangeReplayData", args = 0)]
    pub fn post_change_replay_data(self) -> ();

    #[method(name = "PrintError", args = 0)]
    pub fn print_error(self) -> ();

    #[method(name = "SetResult", args = 0)]
    pub fn set_result_2(self) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        opponent_principal_id: u64,
        meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
        data: crate::app::versusserverreplaydata::VersusServerReplayData,
    ) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusSequenceNet_UploadReplaySequence {
    pub fn new(
        opponent_principal_id: u64,
        meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
        data: crate::app::versusserverreplaydata::VersusServerReplayData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusSequenceNet_UploadReplaySequence),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusSequenceNet_UploadReplaySequenceMethods>::ctor(
            this,
            opponent_principal_id,
            meta_data,
            data,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusMatchingMenu_RandomMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusMatchingMenu.RandomMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusSequence_VersusMatchingMenu_RandomMenuItem {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusMatchingMenu_RandomMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusMatchingMenu_RandomMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusMatchingMenu_RandomMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusMatchingMenu_RandomMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusRankedMenu.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.VersusRankedMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct VersusSequence_VersusRankedMenu {
    #[static_field]
    #[rename(name = "m_MenuContent")]
    pub m_menu_content: crate::app::versustopmenucontent::VersusTopMenuContent,
    #[static_field]
    #[rename(name = "m_InitialSelected")]
    pub m_initial_selected: i32,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusRankedMenu {
    #[method(name = "InitializedSelected", args = 0)]
    pub fn initialized_selected() -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::versustopmenucontent::VersusTopMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTutorial", args = 0)]
    pub fn get_tutorial(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusRankedMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::versustopmenucontent::VersusTopMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusRankedMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusRankedMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_ResultDialog_AssistItem.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.ResultDialog.AssistItem")]
#[parent(crate::app::basicdialogitem::BasicDialogItem)]
pub struct VersusSequence_ResultDialog_AssistItem {
    #[rename(name = "m_SelectId")]
    pub m_select_id: crate::app::versussequence::VersusSequence_ResultDialog_SelectMenu,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_ResultDialog_AssistItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        select_id: crate::app::versussequence::VersusSequence_ResultDialog_SelectMenu,
        messege: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_ResultDialog_AssistItem {
    pub fn new(
        select_id: crate::app::versussequence::VersusSequence_ResultDialog_SelectMenu,
        messege: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_ResultDialog_AssistItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_ResultDialog_AssistItemMethods>::ctor(this, select_id, messege);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceNet_SelectReplaySequence_SelectReplayMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusSequenceNet.SelectReplaySequence.SelectReplayMenuItem"
)]
#[parent(crate::app::menuitem::MenuItem)]
pub struct VersusSequence_VersusSequenceNet_SelectReplaySequence_SelectReplayMenuItem {
    #[rename(name = "m_MetaData")]
    pub m_meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
    #[rename(name = "m_Callback")]
    pub m_callback:
        crate::app::versussequence::VersusSequence_VersusSequenceNet_SelectReplaySequence_Callback,
    #[rename(name = "m_Index")]
    pub m_index: i32,
    #[rename(name = "m_IsUpload")]
    pub m_is_upload: bool,
    #[rename(name = "m_IsPlay")]
    pub m_is_play: bool,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusSequenceNet_SelectReplaySequence_SelectReplayMenuItem {
    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
        index: i32,
        is_upload: bool,
        is_play: bool,
        callback : crate :: app :: versussequence :: VersusSequence_VersusSequenceNet_SelectReplaySequence_Callback,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsEnable", args = 0)]
    pub fn is_enable(self) -> bool;

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::menuitem::MenuItem_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::menuitem::MenuItem_Result;
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusSequenceNet_SelectReplaySequence_SelectReplayMenuItem {
    pub fn new(
        meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
        index: i32,
        is_upload: bool,
        is_play: bool,
        callback : crate :: app :: versussequence :: VersusSequence_VersusSequenceNet_SelectReplaySequence_Callback,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    VersusSequence_VersusSequenceNet_SelectReplaySequence_SelectReplayMenuItem
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IVersusSequence_VersusSequenceNet_SelectReplaySequence_SelectReplayMenuItemMethods > :: ctor (this , meta_data , index , is_upload , is_play , callback) ;
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusCasualMenu_SelectMapMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusCasualMenu.SelectMapMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusSequence_VersusCasualMenu_SelectMapMenuItem {
    #[rename(name = "m_MapNumber")]
    pub m_map_number: i32,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusCasualMenu_SelectMapMenuItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, map_number: i32) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusCasualMenu_SelectMapMenuItem {
    pub fn new(map_number: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusCasualMenu_SelectMapMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusCasualMenu_SelectMapMenuItemMethods>::ctor(this, map_number);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusRankedMenu_ResultMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusRankedMenu.ResultMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusSequence_VersusRankedMenu_ResultMenuItem {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusRankedMenu_ResultMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusRankedMenu_ResultMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusRankedMenu_ResultMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusRankedMenu_ResultMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusYesNoDialog.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.VersusYesNoDialog")]
#[parent(crate::app::yesnodialog::YesNoDialog)]
pub struct VersusSequence_VersusYesNoDialog {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusYesNoDialog {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> ();

    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        message: ::unity2::Il2CppString,
        yes_name: ::unity2::Il2CppString,
        no_name: ::unity2::Il2CppString,
        decide_callback: crate::system::action::Action,
        cancel_callback: crate::system::action::Action,
    ) -> crate::app::versussequence::VersusSequence_VersusYesNoDialog;

    #[method(name = "CreateBind", args = 6)]
    pub fn create_bind_2(
        super_: crate::app::procinst::ProcInst,
        message: ::unity2::Il2CppString,
        yes_name: ::unity2::Il2CppString,
        no_name: ::unity2::Il2CppString,
        decide_callback: crate::system::func_1::Func_1<crate::app::basicmenu::BasicMenu_Result>,
        cancel_callback: crate::system::func_1::Func_1<crate::app::basicmenu::BasicMenu_Result>,
    ) -> crate::app::versussequence::VersusSequence_VersusYesNoDialog;

    #[method(name = "CreateBindImpl", args = 6)]
    pub fn create_bind_impl(
        super_: crate::app::procinst::ProcInst,
        message: ::unity2::Il2CppString,
        yes_name: ::unity2::Il2CppString,
        no_name: ::unity2::Il2CppString,
        yes_item: crate::app::versussequence::VersusSequence_VersusYesNoDialog_YesItem,
        no_item: crate::app::versussequence::VersusSequence_VersusYesNoDialog_NoItem,
    ) -> crate::app::versussequence::VersusSequence_VersusYesNoDialog;
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusYesNoDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusYesNoDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusYesNoDialogMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceLocal_MetaDataPack.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusSequenceLocal.MetaDataPack"
)]
#[parent(crate::system::object::Object)]
pub struct VersusSequence_VersusSequenceLocal_MetaDataPack {
    #[rename(name = "path")]
    pub path: ::unity2::Il2CppString,
    #[rename(name = "data")]
    pub data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusSequenceLocal_MetaDataPack {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
        path: ::unity2::Il2CppString,
    ) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusSequenceLocal_MetaDataPack {
    pub fn new(
        data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
        path: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusSequenceLocal_MetaDataPack),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusSequenceLocal_MetaDataPackMethods>::ctor(this, data, path);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_ProfileUploadSequence.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.ProfileUploadSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct VersusSequence_ProfileUploadSequence {
    #[rename(name = "m_Profile")]
    pub m_profile: crate::app::profilecard::ProfileCard,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_ProfileUploadSequence {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Sanitize", args = 0)]
    pub fn sanitize(self) -> ();

    #[method(name = "Upload", args = 0)]
    pub fn upload(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_ProfileUploadSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_ProfileUploadSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_ProfileUploadSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_ProfileDownloadSequence_DownloadYesNoDialog_YesItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.ProfileDownloadSequence.DownloadYesNoDialog.YesItem"
)]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct VersusSequence_ProfileDownloadSequence_DownloadYesNoDialog_YesItem {
    #[rename(name = "m_DecideCallback")]
    pub m_decide_callback: crate::system::action::Action,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_ProfileDownloadSequence_DownloadYesNoDialog_YesItem {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, decide_callback: crate::system::action::Action) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_ProfileDownloadSequence_DownloadYesNoDialog_YesItem {
    pub fn new(decide_callback: crate::system::action::Action) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    VersusSequence_ProfileDownloadSequence_DownloadYesNoDialog_YesItem
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_ProfileDownloadSequence_DownloadYesNoDialog_YesItemMethods>::ctor(
            this,
            decide_callback,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceNet_UploadReplaySequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct VersusSequence_VersusSequenceNet_UploadReplaySequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for VersusSequence_VersusSequenceNet_UploadReplaySequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "VersusSequence.VersusSequenceNet.UploadReplaySequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for VersusSequence_VersusSequenceNet_UploadReplaySequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl VersusSequence_VersusSequenceNet_UploadReplaySequence_Label {
    pub fn error() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_ProfileDownloadSequence_DownloadYesNoDialog.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.ProfileDownloadSequence.DownloadYesNoDialog"
)]
#[parent(crate::system::object::Object)]
pub struct VersusSequence_ProfileDownloadSequence_DownloadYesNoDialog {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_ProfileDownloadSequence_DownloadYesNoDialog {
    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        mess: ::unity2::Il2CppString,
        decide_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_ProfileDownloadSequence_DownloadYesNoDialog {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_ProfileDownloadSequence_DownloadYesNoDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_ProfileDownloadSequence_DownloadYesNoDialogMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceNet.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.VersusSequenceNet")]
# [parent (crate :: app :: versussequence :: VersusSequence_VersusSequenceBase_1 < crate :: app :: versussequence :: VersusSequence_VersusSequenceNet >)]
pub struct VersusSequence_VersusSequenceNet {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusSequenceNet {
    #[method(name = "InitImpl", args = 0)]
    pub fn init_impl(self) -> ();

    #[method(name = "FinalImpl", args = 0)]
    pub fn final_impl(self) -> ();

    #[method(name = "IsFailedJumpToTopImpl", args = 0)]
    pub fn is_failed_jump_to_top_impl(self) -> bool;

    #[method(name = "IsReportedEditMapImpl", args = 0)]
    pub fn is_reported_edit_map_impl(self) -> bool;

    #[method(name = "UploadReportDataImpl", args = 0)]
    pub fn upload_report_data_impl(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindUploadEditData", args = 1)]
    pub fn create_bind_upload_edit_data(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindUploadCasualData", args = 1)]
    pub fn create_bind_upload_casual_data(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "GetMyCasualMetaDataImpl", args = 0)]
    pub fn get_my_casual_meta_data_impl(self) -> ();

    #[method(name = "PostGetMyCasualMetaDataImpl", args = 0)]
    pub fn post_get_my_casual_meta_data_impl(self) -> ();

    #[method(name = "GetEditMetaDataImpl", args = 0)]
    pub fn get_edit_meta_data_impl(self) -> ();

    #[method(name = "PostGetEditMetaDataImpl", args = 0)]
    pub fn post_get_edit_meta_data_impl(self) -> ();

    #[method(name = "UpdateRateImpl", args = 0)]
    pub fn update_rate_impl(self) -> ();

    #[method(name = "SetRankedInfoImpl", args = 0)]
    pub fn set_ranked_info_impl(self) -> ();

    #[method(name = "UploadEditDataImpl", args = 0)]
    pub fn upload_edit_data_impl(self) -> ();

    #[method(name = "PostUploadEditDataImpl", args = 0)]
    pub fn post_upload_edit_data_impl(self) -> ();

    #[method(name = "UploadReplayImpl", args = 0)]
    pub fn upload_replay_impl(self) -> ();

    #[method(name = "UploadCasualImpl", args = 0)]
    pub fn upload_casual_impl(self) -> ();

    #[method(name = "ChangeCasualImpl", args = 0)]
    pub fn change_casual_impl(self) -> ();

    #[method(name = "ChangeCasualOpponentImpl", args = 0)]
    pub fn change_casual_opponent_impl(self) -> ();

    #[method(name = "SearchSameRateImpl", args = 0)]
    pub fn search_same_rate_impl(self) -> ();

    #[method(name = "SearchRankedFriendImpl", args = 0)]
    pub fn search_ranked_friend_impl(self) -> ();

    #[method(name = "ExcludeMySearchRankedData", args = 0)]
    pub fn exclude_my_search_ranked_data(self) -> ();

    #[method(name = "SearchCasualImpl", args = 0)]
    pub fn search_casual_impl(self) -> ();

    #[method(name = "SearchCasualFriendImpl", args = 0)]
    pub fn search_casual_friend_impl(self) -> ();

    #[method(name = "ExcludeMySearchCasualData", args = 1)]
    pub fn exclude_my_search_casual_data(
        self,
        data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::versusservercasualmetadata::VersusServerCasualMetaData,
        >,
    ) -> ();

    #[method(name = "DownloadImpl", args = 0)]
    pub fn download_impl(self) -> ();

    #[method(name = "DownloadMyEditImpl", args = 0)]
    pub fn download_my_edit_impl(self) -> ();

    #[method(name = "DownloadMetaCasualFromDataCodeImpl", args = 0)]
    pub fn download_meta_casual_from_data_code_impl(self) -> ();

    #[method(name = "DownloadMetaRankedFromDataCodeImpl", args = 0)]
    pub fn download_meta_ranked_from_data_code_impl(self) -> ();

    #[method(name = "CheckAccessibleProfileImpl", args = 2)]
    pub fn check_accessible_profile_impl(
        self,
        mode: crate::app::versus::Versus_Mode,
        target_slot_list: crate::app::nexversus::NexVersus_TargetSlotList,
    ) -> ();

    #[method(name = "DownloadMetaProfileImpl", args = 2)]
    pub fn download_meta_profile_impl(
        self,
        mode: crate::app::versus::Versus_Mode,
        profile: crate::app::profilecard::ProfileCard,
    ) -> ();

    #[method(name = "DownloadCasualImpl", args = 0)]
    pub fn download_casual_impl(self) -> ();

    #[method(name = "SelectReplayImpl", args = 0)]
    pub fn select_replay_impl(self) -> ();

    #[method(name = "DownloadReplayImpl", args = 0)]
    pub fn download_replay_impl(self) -> ();

    #[method(name = "ChangeReplayMetaImpl", args = 0)]
    pub fn change_replay_meta_impl(self) -> ();

    #[method(name = "GetLastUploadedResultImpl", args = 0)]
    pub fn get_last_uploaded_result_impl(self) -> bool;

    #[method(name = "GetDownloadedEditMetaDataImpl", args = 0)]
    pub fn get_downloaded_edit_meta_data_impl(
        self,
    ) -> crate::app::versusserverrankedmetadata::VersusServerRankedMetaData;

    #[method(name = "ChangeRankedMetaDataImpl", args = 1)]
    pub fn change_ranked_meta_data_impl(
        self,
        meta_data: crate::app::versusserverrankedmetadata::VersusServerRankedMetaData,
    ) -> ();

    #[method(name = "ChangeDataTypeImpl", args = 2)]
    pub fn change_data_type_impl(self, data_id: u64, data_type: u16) -> ();

    #[method(name = "InitEditDataImpl", args = 0)]
    pub fn init_edit_data_impl(self) -> ();

    #[method(name = "CheckParentalControlImpl", args = 0)]
    pub fn check_parental_control_impl(self) -> ();

    #[method(name = "IsParentalControlAvailableImpl", args = 0)]
    pub fn is_parental_control_available_impl(self) -> bool;

    #[method(name = "EndParentalControlImpl", args = 0)]
    pub fn end_parental_control_impl(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusSequenceNet {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusSequenceNet),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusSequenceNetMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequence_DataType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct VersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequence_DataType {
    pub value: i32,
}

impl ::unity2::ClassIdentity
    for VersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequence_DataType
{
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str =
        "VersusSequence.VersusSequenceNet.DownloadMetaFromDataCodeSequence.DataType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType
    for VersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequence_DataType
{
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl VersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequence_DataType {
    pub fn casual() -> Self {
        Self { value: 0 }
    }

    pub fn ranked() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_DebugPersistentMenu.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.DebugPersistentMenu")]
#[parent(crate::app::debugmenu::DebugMenu)]
pub struct VersusSequence_DebugPersistentMenu {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_DebugPersistentMenu {
    #[method(name = "get_IsInitCasual", args = 0)]
    pub fn get_is_init_casual() -> bool;

    #[method(name = "set_IsInitCasual", args = 1)]
    pub fn set_is_init_casual(value: bool) -> ();

    #[method(name = "get_IsSelectReplay", args = 0)]
    pub fn get_is_select_replay() -> bool;

    #[method(name = "set_IsSelectReplay", args = 1)]
    pub fn set_is_select_replay(value: bool) -> ();

    #[method(name = "get_IsNotDeleteReplay", args = 0)]
    pub fn get_is_not_delete_replay() -> bool;

    #[method(name = "set_IsNotDeleteReplay", args = 1)]
    pub fn set_is_not_delete_replay(value: bool) -> ();

    #[method(name = "get_IsLocalAirportMode", args = 0)]
    pub fn get_is_local_airport_mode() -> bool;

    #[method(name = "set_IsLocalAirportMode", args = 1)]
    pub fn set_is_local_airport_mode(value: bool) -> ();

    #[method(name = "get_IsReportedEdit", args = 0)]
    pub fn get_is_reported_edit() -> bool;

    #[method(name = "set_IsReportedEdit", args = 1)]
    pub fn set_is_reported_edit(value: bool) -> ();

    #[method(name = "get_IsDuplicate", args = 0)]
    pub fn get_is_duplicate() -> bool;

    #[method(name = "set_IsDuplicate", args = 1)]
    pub fn set_is_duplicate(value: bool) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_DebugPersistentMenu {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_DebugPersistentMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_DebugPersistentMenuMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusRankedMenu_StartMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusRankedMenu.StartMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusSequence_VersusRankedMenu_StartMenuItem {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusRankedMenu_StartMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusRankedMenu_StartMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusRankedMenu_StartMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusRankedMenu_StartMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusMatchingMenu_ProfileMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusMatchingMenu.ProfileMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusSequence_VersusMatchingMenu_ProfileMenuItem {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusMatchingMenu_ProfileMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusMatchingMenu_ProfileMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusMatchingMenu_ProfileMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusMatchingMenu_ProfileMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceNet_SelectReplaySequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct VersusSequence_VersusSequenceNet_SelectReplaySequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for VersusSequence_VersusSequenceNet_SelectReplaySequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "VersusSequence.VersusSequenceNet.SelectReplaySequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for VersusSequence_VersusSequenceNet_SelectReplaySequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl VersusSequence_VersusSequenceNet_SelectReplaySequence_Label {
    pub fn error() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceNet_SelectReplaySequence_Callback.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusSequenceNet.SelectReplaySequence.Callback"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct VersusSequence_VersusSequenceNet_SelectReplaySequence_Callback {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusSequenceNet_SelectReplaySequence_Callback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        result: crate::app::nexversus::NexVersus_Results,
        meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
    ) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusSequenceNet_SelectReplaySequence_Callback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusSequenceNet_SelectReplaySequence_Callback),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusSequenceNet_SelectReplaySequence_CallbackMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceLocal_LocalRate.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusSequenceLocal.LocalRate"
)]
#[parent(crate::system::object::Object)]
pub struct VersusSequence_VersusSequenceLocal_LocalRate {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusSequenceLocal_LocalRate {
    #[method(name = "ParseToNumber", args = 1)]
    pub fn parse_to_number(rate_data_type: u16) -> i32;

    #[method(name = "ParseToDataType", args = 1)]
    pub fn parse_to_data_type(rate_number: i32) -> u16;

    #[method(name = "ParseToSlotId", args = 1)]
    pub fn parse_to_slot_id(index: i32) -> u16;

    #[method(name = "PerseToSlotIndex", args = 1)]
    pub fn perse_to_slot_index(slot_id: u16) -> i32;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusProfileMenu.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.VersusProfileMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct VersusSequence_VersusProfileMenu {
    #[static_field]
    #[rename(name = "c_FriendShowMax")]
    pub c_friend_show_max: i32,
    #[static_field]
    #[rename(name = "s_InitialSelected")]
    pub s_initial_selected: i32,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusProfileMenu {
    #[method(name = "InitializedSelected", args = 0)]
    pub fn initialized_selected() -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        target_list: crate::app::nexversus::NexVersus_TargetSlotList,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::versusprofilemenucontent::VersusProfileMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusProfileMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::versusprofilemenucontent::VersusProfileMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusProfileMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusProfileMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusYesNoDialog_YesItem.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.VersusYesNoDialog.YesItem")]
#[parent(crate::app::basicdialogitemyes::BasicDialogItemYes)]
pub struct VersusSequence_VersusYesNoDialog_YesItem {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_DecideCallback")]
    pub m_decide_callback: crate::system::func_1::Func_1<crate::app::basicmenu::BasicMenu_Result>,
    #[rename(name = "m_CancelCallback")]
    pub m_cancel_callback: crate::system::func_1::Func_1<crate::app::basicmenu::BasicMenu_Result>,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusYesNoDialog_YesItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        name: ::unity2::Il2CppString,
        decide_callback: crate::system::action::Action,
        cancel_callback: crate::system::action::Action,
    ) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor_2(
        self,
        name: ::unity2::Il2CppString,
        decide_callback: crate::system::func_1::Func_1<crate::app::basicmenu::BasicMenu_Result>,
        cancel_callback: crate::system::func_1::Func_1<crate::app::basicmenu::BasicMenu_Result>,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusYesNoDialog_YesItem {
    pub fn new(
        name: ::unity2::Il2CppString,
        decide_callback: crate::system::action::Action,
        cancel_callback: crate::system::action::Action,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusYesNoDialog_YesItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusYesNoDialog_YesItemMethods>::ctor(
            this,
            name,
            decide_callback,
            cancel_callback,
        );
        this
    }

    pub fn new_2(
        name: ::unity2::Il2CppString,
        decide_callback: crate::system::func_1::Func_1<crate::app::basicmenu::BasicMenu_Result>,
        cancel_callback: crate::system::func_1::Func_1<crate::app::basicmenu::BasicMenu_Result>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusYesNoDialog_YesItem),
                ::core::stringify!(new_2),
            )
        });
        <Self as IVersusSequence_VersusYesNoDialog_YesItemMethods>::ctor_2(
            this,
            name,
            decide_callback,
            cancel_callback,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusYesNoDialog_NoItem.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.VersusYesNoDialog.NoItem")]
#[parent(crate::app::basicdialogitemno::BasicDialogItemNo)]
pub struct VersusSequence_VersusYesNoDialog_NoItem {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_CancelCallback")]
    pub m_cancel_callback: crate::system::func_1::Func_1<crate::app::basicmenu::BasicMenu_Result>,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusYesNoDialog_NoItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, name: ::unity2::Il2CppString, callback: crate::system::action::Action) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(
        self,
        name: ::unity2::Il2CppString,
        callback: crate::system::func_1::Func_1<crate::app::basicmenu::BasicMenu_Result>,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusYesNoDialog_NoItem {
    pub fn new(name: ::unity2::Il2CppString, callback: crate::system::action::Action) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusYesNoDialog_NoItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusYesNoDialog_NoItemMethods>::ctor(this, name, callback);
        this
    }

    pub fn new_2(
        name: ::unity2::Il2CppString,
        callback: crate::system::func_1::Func_1<crate::app::basicmenu::BasicMenu_Result>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusYesNoDialog_NoItem),
                ::core::stringify!(new_2),
            )
        });
        <Self as IVersusSequence_VersusYesNoDialog_NoItemMethods>::ctor_2(this, name, callback);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusRankedMenu_MapEditMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusRankedMenu.MapEditMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusSequence_VersusRankedMenu_MapEditMenuItem {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusRankedMenu_MapEditMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusRankedMenu_MapEditMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusRankedMenu_MapEditMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusRankedMenu_MapEditMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceLocal.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.VersusSequenceLocal")]
# [parent (crate :: app :: versussequence :: VersusSequence_VersusSequenceBase_1 < crate :: app :: versussequence :: VersusSequence_VersusSequenceLocal >)]
pub struct VersusSequence_VersusSequenceLocal {
    #[rename(name = "m_RootPath")]
    pub m_root_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "RateTop")]
    pub rate_top: i32,
    #[static_field]
    #[rename(name = "RateBottom")]
    pub rate_bottom: i32,
    #[rename(name = "m_MyCasualMetaDataPath")]
    pub m_my_casual_meta_data_path: ::unity2::Il2CppString,
    #[rename(name = "m_MyEditDataPath")]
    pub m_my_edit_data_path: ::unity2::Il2CppString,
    #[rename(name = "m_SelectedReplayMetaPath")]
    pub m_selected_replay_meta_path: ::unity2::Il2CppString,
    #[rename(name = "m_IsSucceededLastUpload")]
    pub m_is_succeeded_last_upload: bool,
    #[rename(name = "m_LastReplayGetSlot")]
    pub m_last_replay_get_slot: u16,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusSequenceLocal {
    #[method(name = "InitImpl", args = 0)]
    pub fn init_impl(self) -> ();

    #[method(name = "FinalImpl", args = 0)]
    pub fn final_impl(self) -> ();

    #[method(name = "IsFailedJumpToTopImpl", args = 0)]
    pub fn is_failed_jump_to_top_impl(self) -> bool;

    #[method(name = "IsReportedEditMapImpl", args = 0)]
    pub fn is_reported_edit_map_impl(self) -> bool;

    #[method(name = "UploadReportDataImpl", args = 0)]
    pub fn upload_report_data_impl(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindUploadEditData", args = 1)]
    pub fn create_bind_upload_edit_data(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindUploadCasualData", args = 1)]
    pub fn create_bind_upload_casual_data(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "InitializeLanRootPath", args = 0)]
    pub fn initialize_lan_root_path(self) -> ();

    #[method(name = "GetRateDirectoriesPaths", args = 0)]
    pub fn get_rate_directories_paths(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetRateDirPath", args = 1)]
    pub fn get_rate_dir_path(self, rate: i32) -> ::unity2::Il2CppString;

    #[method(name = "AppendRate", args = 1)]
    pub fn append_rate(rate: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetDataFileName", args = 2)]
    pub fn get_data_file_name(
        meta_data: crate::app::versusserverrankedmetadata::VersusServerRankedMetaData,
        rate: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetDataFileName", args = 2)]
    pub fn get_data_file_name_2(
        player_name: ::unity2::Il2CppString,
        rate: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetDataFileName", args = 1)]
    pub fn get_data_file_name_3(
        meta_data: crate::app::versusserverrankedmetadata::VersusServerRankedMetaData,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetDataFileName", args = 1)]
    pub fn get_data_file_name_4(owner_name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetMetaDataFileName", args = 2)]
    pub fn get_meta_data_file_name(
        meta_data: crate::app::versusserverrankedmetadata::VersusServerRankedMetaData,
        rate: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetMetaDataFileName", args = 2)]
    pub fn get_meta_data_file_name_2(
        owner_name: ::unity2::Il2CppString,
        rate: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "MoveFiles", args = 3)]
    pub fn move_files(
        self,
        player_name: ::unity2::Il2CppString,
        from_rate: i32,
        to_rate: i32,
    ) -> bool;

    #[method(name = "GetReplayDataPath", args = 2)]
    pub fn get_replay_data_path(
        self,
        player_name: ::unity2::Il2CppString,
        slot_id: u16,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetReplayMetaPath", args = 2)]
    pub fn get_replay_meta_path(
        self,
        player_name: ::unity2::Il2CppString,
        slot_id: u16,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetReplayMetaPaths", args = 1)]
    pub fn get_replay_meta_paths(
        self,
        player_name: ::unity2::Il2CppString,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "GetCasualDirectoryPath", args = 0)]
    pub fn get_casual_directory_path(self) -> ::unity2::Il2CppString;

    #[method(name = "GetCasualDataPath", args = 1)]
    pub fn get_casual_data_path(
        self,
        player_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetCasualMetaPath", args = 1)]
    pub fn get_casual_meta_path(
        self,
        player_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "FindCasualMetaPath", args = 1)]
    pub fn find_casual_meta_path(self, principal_id: u64) -> ::unity2::Il2CppString;

    #[method(name = "GetPath", args = 1)]
    pub fn get_path(
        self,
        meta_data: crate::app::versusserverrankedmetadata::VersusServerRankedMetaData,
    ) -> ::unity2::Il2CppString;

    #[method(name = "FindRankedMetaPath", args = 1)]
    pub fn find_ranked_meta_path(self, principal_id: u64) -> ::unity2::Il2CppString;

    #[method(name = "GetPlayerName", args = 1)]
    pub fn get_player_name(self, path: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "TryGetPrincipalId", args = 1)]
    pub fn try_get_principal_id(self, principal_id: u64) -> bool;

    #[method(name = "GetLocalNickName", args = 0)]
    pub fn get_local_nick_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetMyCasualMetaDataImpl", args = 0)]
    pub fn get_my_casual_meta_data_impl(self) -> ();

    #[method(name = "PostGetMyCasualMetaDataImpl", args = 0)]
    pub fn post_get_my_casual_meta_data_impl(self) -> ();

    #[method(name = "GetEditMetaDataImpl", args = 0)]
    pub fn get_edit_meta_data_impl(self) -> ();

    #[method(name = "PostGetEditMetaDataImpl", args = 0)]
    pub fn post_get_edit_meta_data_impl(self) -> ();

    #[method(name = "UpdateRateImpl", args = 0)]
    pub fn update_rate_impl(self) -> ();

    #[method(name = "SetRankedInfoImpl", args = 0)]
    pub fn set_ranked_info_impl(self) -> ();

    #[method(name = "UploadEditDataImpl", args = 0)]
    pub fn upload_edit_data_impl(self) -> ();

    #[method(name = "LocalUpload", args = 4)]
    pub fn local_upload(
        self,
        player_name: ::unity2::Il2CppString,
        meta_data: crate::app::versusserverrankedmetadata::VersusServerRankedMetaData,
        data: crate::app::versusserverrankeddata::VersusServerRankedData,
        rate_data_type: u16,
    ) -> ();

    #[method(name = "PostUploadEditDataImpl", args = 0)]
    pub fn post_upload_edit_data_impl(self) -> ();

    #[method(name = "DownloadMetaCasualFromDataCodeImpl", args = 0)]
    pub fn download_meta_casual_from_data_code_impl(self) -> ();

    #[method(name = "DownloadMetaRankedFromDataCodeImpl", args = 0)]
    pub fn download_meta_ranked_from_data_code_impl(self) -> ();

    #[method(name = "CheckAccessibleProfileImpl", args = 2)]
    pub fn check_accessible_profile_impl(
        self,
        mode: crate::app::versus::Versus_Mode,
        target_slot_list: crate::app::nexversus::NexVersus_TargetSlotList,
    ) -> ();

    #[method(name = "DownloadMetaProfileImpl", args = 2)]
    pub fn download_meta_profile_impl(
        self,
        mode: crate::app::versus::Versus_Mode,
        profile: crate::app::profilecard::ProfileCard,
    ) -> ();

    #[method(name = "DownloadCasualImpl", args = 0)]
    pub fn download_casual_impl(self) -> ();

    #[method(name = "UploadReplayImpl", args = 0)]
    pub fn upload_replay_impl(self) -> ();

    #[method(name = "GetUploadReplaySlot", args = 1)]
    pub fn get_upload_replay_slot(self, opponent_name: ::unity2::Il2CppString) -> u16;

    #[method(name = "UploadCasualImpl", args = 0)]
    pub fn upload_casual_impl(self) -> ();

    #[method(name = "ChangeCasualImpl", args = 0)]
    pub fn change_casual_impl(self) -> ();

    #[method(name = "ChangeCasualOpponentImpl", args = 0)]
    pub fn change_casual_opponent_impl(self) -> ();

    #[method(name = "SearchSameRateImpl", args = 0)]
    pub fn search_same_rate_impl(self) -> ();

    #[method(name = "ExcludeMySearchRankedData", args = 0)]
    pub fn exclude_my_search_ranked_data(self) -> ();

    #[method(name = "SearchCasualImpl", args = 0)]
    pub fn search_casual_impl(self) -> ();

    #[method(name = "SearchCasualLocal", args = 2)]
    pub fn search_casual_local(
        self,
        result_list: crate::system::collections::generic::list_1::List_1<
            crate::app::versusservercasualmetadata::VersusServerCasualMetaData,
        >,
        is_myself: bool,
    ) -> ();

    #[method(name = "SearchCasualFriendImpl", args = 0)]
    pub fn search_casual_friend_impl(self) -> ();

    #[method(name = "SearchRankedFriendImpl", args = 0)]
    pub fn search_ranked_friend_impl(self) -> ();

    #[method(name = "ExcludeMySearchCasualData", args = 1)]
    pub fn exclude_my_search_casual_data(
        self,
        data_list: crate::system::collections::generic::list_1::List_1<
            crate::app::versusservercasualmetadata::VersusServerCasualMetaData,
        >,
    ) -> ();

    #[method(name = "DownloadImpl", args = 0)]
    pub fn download_impl(self) -> ();

    #[method(name = "DownloadMyEditImpl", args = 0)]
    pub fn download_my_edit_impl(self) -> ();

    #[method(name = "SelectReplayImpl", args = 0)]
    pub fn select_replay_impl(self) -> ();

    #[method(name = "MostOldestReplayMetaData", args = 3)]
    pub fn most_oldest_replay_meta_data(
        self,
        paths: ::unity2::Array<::unity2::Il2CppString>,
        meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
        meta_path: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "DownloadReplayImpl", args = 0)]
    pub fn download_replay_impl(self) -> ();

    #[method(name = "ChangeReplayMetaImpl", args = 0)]
    pub fn change_replay_meta_impl(self) -> ();

    #[method(name = "SearchMyDataToDataId", args = 0)]
    pub fn search_my_data_to_data_id(self) -> u64;

    #[method(name = "GetLastUploadedResultImpl", args = 0)]
    pub fn get_last_uploaded_result_impl(self) -> bool;

    #[method(name = "GetDownloadedEditMetaDataImpl", args = 0)]
    pub fn get_downloaded_edit_meta_data_impl(
        self,
    ) -> crate::app::versusserverrankedmetadata::VersusServerRankedMetaData;

    #[method(name = "ChangeRankedMetaDataImpl", args = 1)]
    pub fn change_ranked_meta_data_impl(
        self,
        meta_data: crate::app::versusserverrankedmetadata::VersusServerRankedMetaData,
    ) -> ();

    #[method(name = "ChangeDataTypeImpl", args = 2)]
    pub fn change_data_type_impl(self, data_id: u64, data_type: u16) -> ();

    #[method(name = "InitEditDataImpl", args = 0)]
    pub fn init_edit_data_impl(self) -> ();

    #[method(name = "CheckParentalControlImpl", args = 0)]
    pub fn check_parental_control_impl(self) -> ();

    #[method(name = "IsParentalControlAvailableImpl", args = 0)]
    pub fn is_parental_control_available_impl(self) -> bool;

    #[method(name = "EndParentalControlImpl", args = 0)]
    pub fn end_parental_control_impl(self) -> ();

    #[method(name = "CreateSelectReplayMetaMenu", args = 2)]
    pub fn create_select_replay_meta_menu(
        self,
        meta_paths: ::unity2::Array<::unity2::Il2CppString>,
        callback : crate :: app :: versussequence :: VersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem_ACallback,
    ) -> ();

    #[method(name = "Deserialize", args = 1)]
    pub fn deserialize(
        self,
        path: ::unity2::Il2CppString,
    ) -> crate::app::versusserverreplaymetadata::VersusServerReplayMetaData;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusSequenceLocal {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusSequenceLocal),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusSequenceLocalMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusMatchingMenu_Result2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct VersusSequence_VersusMatchingMenu_Result2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for VersusSequence_VersusMatchingMenu_Result2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "VersusSequence.VersusMatchingMenu.Result2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for VersusSequence_VersusMatchingMenu_Result2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl VersusSequence_VersusMatchingMenu_Result2 {
    pub fn random() -> Self {
        Self { value: 0 }
    }

    pub fn friend() -> Self {
        Self { value: 1 }
    }

    pub fn code() -> Self {
        Self { value: 2 }
    }

    pub fn profile() -> Self {
        Self { value: 3 }
    }

    pub fn end() -> Self {
        Self { value: 4 }
    }

    pub fn top() -> Self {
        Self { value: 0 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct VersusSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for VersusSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "VersusSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for VersusSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl VersusSequence_Label {
    pub fn top() -> Self {
        Self { value: 0 }
    }

    pub fn casual() -> Self {
        Self { value: 1 }
    }

    pub fn ranked() -> Self {
        Self { value: 2 }
    }

    pub fn matching() -> Self {
        Self { value: 3 }
    }

    pub fn matching_start() -> Self {
        Self { value: 4 }
    }

    pub fn net_casual_random() -> Self {
        Self { value: 5 }
    }

    pub fn net_casual_friend() -> Self {
        Self { value: 6 }
    }

    pub fn net_casual_friend_select() -> Self {
        Self { value: 7 }
    }

    pub fn net_casual_data_code() -> Self {
        Self { value: 8 }
    }

    pub fn net_casual_profile() -> Self {
        Self { value: 9 }
    }

    pub fn net_casual_profile_select() -> Self {
        Self { value: 10 }
    }

    pub fn net_casual_upload() -> Self {
        Self { value: 11 }
    }

    pub fn net_ranked() -> Self {
        Self { value: 12 }
    }

    pub fn net_ranked_random() -> Self {
        Self { value: 13 }
    }

    pub fn net_ranked_friend() -> Self {
        Self { value: 14 }
    }

    pub fn net_ranked_friend_select() -> Self {
        Self { value: 15 }
    }

    pub fn net_ranked_data_code() -> Self {
        Self { value: 16 }
    }

    pub fn net_ranked_profile() -> Self {
        Self { value: 17 }
    }

    pub fn map_edit() -> Self {
        Self { value: 18 }
    }

    pub fn replay() -> Self {
        Self { value: 19 }
    }

    pub fn reward() -> Self {
        Self { value: 20 }
    }

    pub fn mock_battle() -> Self {
        Self { value: 21 }
    }

    pub fn net_download_casual() -> Self {
        Self { value: 22 }
    }

    pub fn net_download_ranked() -> Self {
        Self { value: 23 }
    }

    pub fn map_sequence() -> Self {
        Self { value: 24 }
    }

    pub fn fade_out_to_top() -> Self {
        Self { value: 25 }
    }

    pub fn fade_out_to_casual() -> Self {
        Self { value: 26 }
    }

    pub fn fade_out_to_ranked() -> Self {
        Self { value: 27 }
    }

    pub fn end_casual() -> Self {
        Self { value: 28 }
    }

    pub fn end() -> Self {
        Self { value: 29 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_ProfileDownloadSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct VersusSequence_ProfileDownloadSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for VersusSequence_ProfileDownloadSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "VersusSequence.ProfileDownloadSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for VersusSequence_ProfileDownloadSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl VersusSequence_ProfileDownloadSequence_Label {
    pub fn sanitize() -> Self {
        Self { value: 0 }
    }

    pub fn save() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusMatchingMenu.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.VersusMatchingMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct VersusSequence_VersusMatchingMenu {
    #[static_field]
    #[rename(name = "m_MenuContent")]
    pub m_menu_content: crate::app::versustopmenucontent::VersusTopMenuContent,
    #[static_field]
    #[rename(name = "m_InitialSelected")]
    pub m_initial_selected: i32,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusMatchingMenu {
    #[method(name = "InitializedSelected", args = 0)]
    pub fn initialized_selected() -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::versustopmenucontent::VersusTopMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTutorial", args = 0)]
    pub fn get_tutorial(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusMatchingMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::versustopmenucontent::VersusTopMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusMatchingMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusMatchingMenuMethods>::ctor(
            this,
            menu_item_list,
            menu_content,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_IVersusSequenceBase.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.IVersusSequenceBase")]
pub struct VersusSequence_IVersusSequenceBase {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_IVersusSequenceBase {
    #[method(name = "JumpTo", args = 1)]
    pub fn jump_to(self, label: crate::app::versussequence::VersusSequence_Label) -> ();

    #[method(name = "GetMyEditDataId", args = 0)]
    pub fn get_my_edit_data_id(self) -> u64;

    #[method(name = "GetNowRateName", args = 0)]
    pub fn get_now_rate_name(self) -> ::unity2::Il2CppString;

    #[method(name = "SetProfileTarget", args = 1)]
    pub fn set_profile_target(self, profile: crate::app::profilecard::ProfileCard) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem_ACallback.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusSequenceLocal.SelectReplayMetaMenuItem.ACallback"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct VersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem_ACallback {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem_ACallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(
        self,
        meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
        meta_path: ::unity2::Il2CppString,
        slot_id: u16,
    ) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem_ACallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    VersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem_ACallback
                ),
                ::core::stringify!(new),
            )
        });
        < Self as IVersusSequence_VersusSequenceLocal_SelectReplayMetaMenuItem_ACallbackMethods > :: ctor (this , object , method) ;
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence")]
#[parent(crate::system::object::Object)]
pub struct VersusSequence {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindUploadEditData", args = 1)]
    pub fn create_bind_upload_edit_data(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindUploadCasualData", args = 1)]
    pub fn create_bind_upload_casual_data(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "ShowDisableNetworkMessage", args = 1)]
    pub fn show_disable_network_message(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "JumpToTop", args = 0)]
    pub fn jump_to_top() -> ();

    #[method(name = "JumpToCasual", args = 0)]
    pub fn jump_to_casual() -> ();

    #[method(name = "JumpToRanked", args = 0)]
    pub fn jump_to_ranked() -> ();

    #[method(name = "JumpToMatching", args = 0)]
    pub fn jump_to_matching() -> ();

    #[method(name = "JumpToMatchingStart", args = 0)]
    pub fn jump_to_matching_start() -> ();

    #[method(name = "JumpToNetCasual", args = 0)]
    pub fn jump_to_net_casual() -> ();

    #[method(name = "JumpToMapEdit", args = 0)]
    pub fn jump_to_map_edit() -> ();

    #[method(name = "JumpToReplay", args = 0)]
    pub fn jump_to_replay() -> ();

    #[method(name = "JumpToMockBattle", args = 0)]
    pub fn jump_to_mock_battle() -> ();

    #[method(name = "JumpToReward", args = 0)]
    pub fn jump_to_reward() -> ();

    #[method(name = "JumpToMapSequence", args = 0)]
    pub fn jump_to_map_sequence() -> ();

    #[method(name = "JumpToEnd", args = 0)]
    pub fn jump_to_end() -> ();

    #[method(name = "GetSequence", args = 0)]
    pub fn get_sequence() -> crate::app::versussequence::VersusSequence_IVersusSequenceBase;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusTopMenu_Result2.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct VersusSequence_VersusTopMenu_Result2 {
    pub value: i32,
}

impl ::unity2::ClassIdentity for VersusSequence_VersusTopMenu_Result2 {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "VersusSequence.VersusTopMenu.Result2";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for VersusSequence_VersusTopMenu_Result2 {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl VersusSequence_VersusTopMenu_Result2 {
    pub fn casual() -> Self {
        Self { value: 0 }
    }

    pub fn ranked() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }

    pub fn top() -> Self {
        Self { value: 0 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_ResultDialog.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.ResultDialog")]
#[parent(crate::app::basicdialog::BasicDialog)]
pub struct VersusSequence_ResultDialog {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_ResultDialog {
    #[method(name = "get_EventHandler", args = 0)]
    pub fn get_event_handler(
    ) -> crate::app::versussequence::VersusSequence_ResultDialog_DecideEventHandler;

    #[method(name = "set_EventHandler", args = 1)]
    pub fn set_event_handler(
        value: crate::app::versussequence::VersusSequence_ResultDialog_DecideEventHandler,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        event_handler: crate::app::versussequence::VersusSequence_ResultDialog_DecideEventHandler,
    ) -> crate::app::versussequence::VersusSequence_ResultDialog;

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_ResultDialog {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::basicdialogcontent::BasicDialogContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_ResultDialog),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_ResultDialogMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_ProfileDownloadSequence.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.ProfileDownloadSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct VersusSequence_ProfileDownloadSequence {
    #[rename(name = "m_ReplayMetaData")]
    pub m_replay_meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_ProfileDownloadSequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        replay_meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
    ) -> ();

    #[method(name = "TryGetOpponentInfo", args = 2)]
    pub fn try_get_opponent_info(self, name: ::unity2::Il2CppString, principal_id: u64) -> bool;

    #[method(name = "Download", args = 0)]
    pub fn download(self) -> ();

    #[method(name = "PostDownload", args = 0)]
    pub fn post_download(self) -> ();

    #[method(name = "Sanitize", args = 0)]
    pub fn sanitize(self) -> ();

    #[method(name = "OpenDialog", args = 0)]
    pub fn open_dialog(self) -> ();

    #[method(name = "Save", args = 0)]
    pub fn save(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        replay_meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
    ) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_ProfileDownloadSequence {
    pub fn new(
        replay_meta_data: crate::app::versusserverreplaymetadata::VersusServerReplayMetaData,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_ProfileDownloadSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_ProfileDownloadSequenceMethods>::ctor(this, replay_meta_data);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusTopMenu_CasualMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusTopMenu.CasualMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusSequence_VersusTopMenu_CasualMenuItem {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusTopMenu_CasualMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusTopMenu_CasualMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusTopMenu_CasualMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusTopMenu_CasualMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusCasualMenu.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.VersusCasualMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct VersusSequence_VersusCasualMenu {
    #[static_field]
    #[rename(name = "c_MapMax")]
    pub c_map_max: i32,
    #[static_field]
    #[rename(name = "m_MenuContent")]
    pub m_menu_content: crate::app::versusmapmenucontent::VersusMapMenuContent,
    #[static_field]
    #[rename(name = "m_InitialSelected")]
    pub m_initial_selected: i32,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusCasualMenu {
    #[method(name = "InitializedSelected", args = 0)]
    pub fn initialized_selected() -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::versusmapmenucontent::VersusMapMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTutorial", args = 0)]
    pub fn get_tutorial(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusCasualMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::versusmapmenucontent::VersusMapMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusCasualMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusCasualMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusProfileMenu_VersusProfileMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusProfileMenu.VersusProfileMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusSequence_VersusProfileMenu_VersusProfileMenuItem {
    #[rename(name = "m_Index")]
    pub m_index: i32,
    #[rename(name = "m_Profile")]
    pub m_profile: crate::app::profilecard::ProfileCard,
    #[rename(name = "m_IsEnable")]
    pub m_is_enable: bool,
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_PrincipalID")]
    pub m_principal_id: u64,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusProfileMenu_VersusProfileMenuItem {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        index: i32,
        profile: crate::app::profilecard::ProfileCard,
        is_enable: bool,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "BuildAttribute", args = 0)]
    pub fn build_attribute(self) -> crate::app::basicmenuitem::BasicMenuItem_Attribute;

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "OpenDialog", args = 0)]
    pub fn open_dialog(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusProfileMenu_VersusProfileMenuItem {
    pub fn new(index: i32, profile: crate::app::profilecard::ProfileCard, is_enable: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusProfileMenu_VersusProfileMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusProfileMenu_VersusProfileMenuItemMethods>::ctor(
            this, index, profile, is_enable,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusTopMenu.md")))]
#[::unity2::class(namespace = "App", name = "VersusSequence.VersusTopMenu")]
#[parent(crate::app::basicmenu::BasicMenu)]
pub struct VersusSequence_VersusTopMenu {
    #[static_field]
    #[rename(name = "m_MenuContent")]
    pub m_menu_content: crate::app::versustopmenucontent::VersusTopMenuContent,
    #[static_field]
    #[rename(name = "m_InitialSelected")]
    pub m_initial_selected: i32,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusTopMenu {
    #[method(name = "InitializedSelected", args = 0)]
    pub fn initialized_selected() -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::versustopmenucontent::VersusTopMenuContent,
    ) -> ();

    #[method(name = "GetName", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetTutorial", args = 0)]
    pub fn get_tutorial(self) -> ::unity2::Il2CppString;

    #[method(name = "BCall", args = 0)]
    pub fn b_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusTopMenu {
    pub fn new(
        menu_item_list: crate::system::collections::generic::list_1::List_1<
            crate::app::basicmenuitem::BasicMenuItem,
        >,
        menu_content: crate::app::versustopmenucontent::VersusTopMenuContent,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusTopMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusTopMenuMethods>::ctor(this, menu_item_list, menu_content);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_ResultDialog_DecideEventHandler.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.ResultDialog.DecideEventHandler"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct VersusSequence_ResultDialog_DecideEventHandler {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_ResultDialog_DecideEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        set: crate::app::versussequence::VersusSequence_ResultDialog_SelectMenu,
    ) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_ResultDialog_DecideEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_ResultDialog_DecideEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_ResultDialog_DecideEventHandlerMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequence.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusSequenceNet.DownloadMetaFromDataCodeSequence"
)]
#[parent(crate::app::procinst::ProcInst)]
pub struct VersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequence {
# [rename (name = "m_DataId")] pub m_data_id : u64 ,
# [rename (name = "m_DataType")] pub m_data_type : crate :: app :: versussequence :: VersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequence_DataType ,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequence {
    #[method(name = "get_IsSucceed", args = 0)]
    pub fn get_is_succeed() -> bool;

    #[method(name = "set_IsSucceed", args = 1)]
    pub fn set_is_succeed(value: bool) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        data_id: u64,
        data_type : crate :: app :: versussequence :: VersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequence_DataType,
    ) -> ();

    #[method(name = "Download", args = 0)]
    pub fn download(self) -> ();

    #[method(name = "SetVersus", args = 0)]
    pub fn set_versus(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        data_id: u64,
        data_type : crate :: app :: versussequence :: VersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequence_DataType,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequence {
    pub fn new(
        data_id: u64,
        data_type : crate :: app :: versussequence :: VersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequence_DataType,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(
                    VersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequence
                ),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusSequenceNet_DownloadMetaFromDataCodeSequenceMethods>::ctor(
            this, data_id, data_type,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_ResultDialog_SelectMenu.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct VersusSequence_ResultDialog_SelectMenu {
    pub value: i32,
}

impl ::unity2::ClassIdentity for VersusSequence_ResultDialog_SelectMenu {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "VersusSequence.ResultDialog.SelectMenu";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for VersusSequence_ResultDialog_SelectMenu {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl VersusSequence_ResultDialog_SelectMenu {
    pub fn replay() -> Self {
        Self { value: 0 }
    }

    pub fn result() -> Self {
        Self { value: 1 }
    }

    pub fn back() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusTopMenu_RankedMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusTopMenu.RankedMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusSequence_VersusTopMenu_RankedMenuItem {}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusTopMenu_RankedMenuItem {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;

    #[method(name = "OnSelect", args = 0)]
    pub fn on_select(self) -> ();
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusTopMenu_RankedMenuItem {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusTopMenu_RankedMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusTopMenu_RankedMenuItemMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versussequence/VersusSequence_VersusFriendMenu_FriendRankedMenuItem.md")))]
#[::unity2::class(
    namespace = "App",
    name = "VersusSequence.VersusFriendMenu.FriendRankedMenuItem"
)]
#[parent(crate::app::basicmenuitem::BasicMenuItem)]
pub struct VersusSequence_VersusFriendMenu_FriendRankedMenuItem {
    #[rename(name = "m_MetaData")]
    pub m_meta_data: crate::app::versusserverrankedmetadata::VersusServerRankedMetaData,
    #[rename(name = "m_FriendNumber")]
    pub m_friend_number: i32,
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-versussequence")]
#[::unity2::methods]
impl VersusSequence_VersusFriendMenu_FriendRankedMenuItem {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        meta_data: crate::app::versusserverrankedmetadata::VersusServerRankedMetaData,
        num: i32,
    ) -> ();

    #[method(name = "ACall", args = 0)]
    pub fn a_call(self) -> crate::app::basicmenu::BasicMenu_Result;

    #[method(name = "OnBuildMenuItemContent", args = 0)]
    pub fn on_build_menu_item_content(self) -> ();

    #[method(name = "StartBattleDialog", args = 0)]
    pub fn start_battle_dialog(self) -> crate::app::basicmenu::BasicMenu_Result;
}

#[cfg(feature = "app-versussequence")]
impl VersusSequence_VersusFriendMenu_FriendRankedMenuItem {
    pub fn new(
        meta_data: crate::app::versusserverrankedmetadata::VersusServerRankedMetaData,
        num: i32,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusSequence_VersusFriendMenu_FriendRankedMenuItem),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusSequence_VersusFriendMenu_FriendRankedMenuItemMethods>::ctor(
            this, meta_data, num,
        );
        this
    }
}
