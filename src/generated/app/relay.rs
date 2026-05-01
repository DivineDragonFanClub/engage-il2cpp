
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relay/Relay_CstoResult.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Relay_CstoResult {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Relay_CstoResult {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Relay.CstoResult";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Relay_CstoResult {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Relay_CstoResult {
    pub fn ok() -> Self {
        Self { value: 0 }
    }

    pub fn ng_end() -> Self {
        Self { value: 1 }
    }

    pub fn ng_other_playing() -> Self {
        Self { value: 2 }
    }

    pub fn ng_already_played() -> Self {
        Self { value: 3 }
    }

    pub fn ng_lack_of_unit() -> Self {
        Self { value: 4 }
    }

    pub fn ng_unknown() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relay/Relay_DisposPlayerCountsSequence.md")))]
#[::unity2::class(namespace = "App", name = "Relay.DisposPlayerCountsSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct Relay_DisposPlayerCountsSequence {
    #[rename(name = "m_Index")]
    pub m_index: i32,
    #[rename(name = "m_Cid")]
    pub m_cid: ::unity2::Il2CppString,
}

#[cfg(feature = "app-relay")]
#[::unity2::methods]
impl Relay_DisposPlayerCountsSequence {
    #[method(name = "LoadFirst", args = 0)]
    pub fn load_first(self) -> ();

    #[method(name = "LoadDone", args = 0)]
    pub fn load_done(self) -> ();

    #[method(name = "GetChapter", args = 0)]
    pub fn get_chapter(self) -> crate::app::chapterdata::ChapterData;

    #[method(name = "Load", args = 0)]
    pub fn load(self) -> bool;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relay")]
impl Relay_DisposPlayerCountsSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Relay_DisposPlayerCountsSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IRelay_DisposPlayerCountsSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relay/Relay.md")))]
#[::unity2::class(namespace = "App", name = "Relay")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: relay :: Relay >)]
pub struct Relay {
    #[rename(name = "m_MetaData")]
    pub m_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    #[rename(name = "m_Data")]
    pub m_data: crate::app::relayserverdata::RelayServerData,
    #[rename(name = "m_SelectedMetaData")]
    pub m_selected_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    #[rename(name = "m_BackupDataForReplay")]
    pub m_backup_data_for_replay: crate::app::relaybackupdata::RelayBackupData,
    #[rename(name = "m_BackupDataForAll")]
    pub m_backup_data_for_all: crate::app::relaybackupdata::RelayBackupData,
    #[rename(name = "m_CopiedUserData")]
    pub m_copied_user_data: crate::app::relayuserdata::RelayUserData,
    #[rename(name = "m_Mode")]
    pub m_mode: crate::app::relay::Relay_Modes,
    #[rename(name = "m_Flag")]
    pub m_flag: crate::app::relay::Relay_FlagsField,
    #[rename(name = "m_DisposPlayerCounts")]
    pub m_dispos_player_counts: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        i32,
    >,
    #[rename(name = "m_Sortie")]
    pub m_sortie: crate::app::relaysortie::RelaySortie,
    #[rename(name = "m_AppearanceUnits")]
    pub m_appearance_units:
        crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
    #[rename(name = "m_LeavingUnits")]
    pub m_leaving_units:
        crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>,
    #[rename(name = "m_MaxTurn")]
    pub m_max_turn: i32,
    #[rename(name = "m_LimitTurn")]
    pub m_limit_turn: i32,
    #[rename(name = "m_CurrentPlayerIndex")]
    pub m_current_player_index: i32,
    #[rename(name = "m_CurrentBattle")]
    pub m_current_battle: crate::app::relayuserdata::RelayUserData_EnteredBattle,
    #[rename(name = "m_ReplayPlayerIndex")]
    pub m_replay_player_index: i32,
    #[rename(name = "m_PlayingTermHolder")]
    pub m_playing_term_holder: crate::app::gameparam::GameParam_Holder,
    #[rename(name = "m_PoolUnits")]
    pub m_pool_units: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "m_PoolGods")]
    pub m_pool_gods: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
}

#[cfg(feature = "app-relay")]
#[::unity2::methods]
impl Relay {
    #[method(name = "IsValid", args = 0)]
    pub fn is_valid() -> bool;

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "DisposPlayerCounts", args = 1)]
    pub fn dispos_player_counts(self, super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CanSelectNew", args = 1)]
    pub fn can_select_new(self, cid: ::unity2::Il2CppString) -> bool;

    #[method(name = "CanSelectTakeOver", args = 4)]
    pub fn can_select_take_over(
        self,
        meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        principal_id: u64,
        current_unix_time: i64,
        result: crate::app::relay::Relay_CstoResult,
    ) -> bool;

    #[method(name = "InitializeForNew", args = 1)]
    pub fn initialize_for_new(self, cid: ::unity2::Il2CppString) -> ();

    #[method(name = "InitializeForTakeOver", args = 1)]
    pub fn initialize_for_take_over(
        self,
        meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> ();

    #[method(name = "InitializeForReplay", args = 2)]
    pub fn initialize_for_replay(
        self,
        meta_data: crate::app::relayservermetadata::RelayServerMetaData,
        reinit_meta: bool,
    ) -> ();

    #[method(name = "TryReinitializeForReplay", args = 1)]
    pub fn try_reinitialize_for_replay(
        self,
        new_meta_data: crate::app::relayservermetadata::RelayServerMetaData,
    ) -> crate::app::relay::Relay_TrfrResult;

    #[method(name = "SetPlayingDone", args = 0)]
    pub fn set_playing_done(self) -> ();

    #[method(name = "DeleteGodLink", args = 0)]
    pub fn delete_god_link(self) -> ();

    #[method(name = "InitializeForSortie", args = 0)]
    pub fn initialize_for_sortie(self) -> ();

    #[method(name = "SetupForSortie", args = 0)]
    pub fn setup_for_sortie(self) -> ();

    #[method(name = "GetSortieCount", args = 2)]
    pub fn get_sortie_count(self, max_total_count: i32, req_player_count: i32) -> ();

    #[method(name = "GetSortieCount", args = 7)]
    pub fn get_sortie_count_2(
        self,
        mode: crate::app::relay::Relay_Modes,
        cid: ::unity2::Il2CppString,
        pids: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
        max_total_count: i32,
        req_player_count: i32,
        player_count: i32,
        player_no_sortie_count: i32,
    ) -> ();

    #[method(name = "GetSortieCountFromData", args = 4)]
    pub fn get_sortie_count_from_data(
        self,
        mode: crate::app::relay::Relay_Modes,
        cid: ::unity2::Il2CppString,
        max_total_count: i32,
        req_player_count: i32,
    ) -> ();

    #[method(name = "GetPlayerUnitCount", args = 4)]
    pub fn get_player_unit_count(
        self,
        mode: crate::app::relay::Relay_Modes,
        pids: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
        unit_count: i32,
        no_sortie_unit_count: i32,
    ) -> ();

    #[method(name = "DecideToBattle", args = 0)]
    pub fn decide_to_battle(self) -> ();

    #[method(name = "MapBegin", args = 0)]
    pub fn map_begin(self) -> ();

    #[method(name = "MapEnd", args = 0)]
    pub fn map_end(self) -> ();

    #[method(name = "Complete", args = 0)]
    pub fn complete(self) -> ();

    #[method(name = "GameOver", args = 0)]
    pub fn game_over(self) -> ();

    #[method(name = "AddEnteredBattle", args = 1)]
    pub fn add_entered_battle(
        self,
        new_battle: crate::app::relayuserdata::RelayUserData_EnteredBattle,
    ) -> ();

    #[method(name = "GetEnteredBattle", args = 1)]
    pub fn get_entered_battle(
        self,
        data_id: u64,
    ) -> crate::app::relayuserdata::RelayUserData_EnteredBattle;

    #[method(name = "ClearCurrentBattle", args = 0)]
    pub fn clear_current_battle(self) -> ();

    #[method(name = "GetPlayerName", args = 1)]
    pub fn get_player_name(self, player_index: i32) -> ::unity2::Il2CppString;

    #[method(name = "GetLastPlayerName", args = 0)]
    pub fn get_last_player_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetReplayPlayerName", args = 0)]
    pub fn get_replay_player_name(self) -> ::unity2::Il2CppString;

    #[method(name = "NextReplayPlayer", args = 0)]
    pub fn next_replay_player(self) -> ();

    #[method(name = "SetReplayPlayerForSkip", args = 1)]
    pub fn set_replay_player_for_skip(self, player_index: i32) -> ();

    #[method(name = "DbgFakeAwardee", args = 0)]
    pub fn dbg_fake_awardee(self) -> ();

    #[method(name = "IsMyUnit", args = 1)]
    pub fn is_my_unit(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsOthersUnit", args = 1)]
    pub fn is_others_unit(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "IsNoCountUnit", args = 1)]
    pub fn is_no_count_unit(unit: crate::app::unit::Unit) -> bool;

    #[method(name = "GetNickname", args = 0)]
    pub fn get_nickname() -> ::unity2::Il2CppString;

    #[method(name = "get_MetaData", args = 0)]
    pub fn get_meta_data(self) -> crate::app::relayservermetadata::RelayServerMetaData;

    #[method(name = "get_Data", args = 0)]
    pub fn get_data(self) -> crate::app::relayserverdata::RelayServerData;

    #[method(name = "get_SelectedMetaData", args = 0)]
    pub fn get_selected_meta_data(self) -> crate::app::relayservermetadata::RelayServerMetaData;

    #[method(name = "get_BackupDataForReplay", args = 0)]
    pub fn get_backup_data_for_replay(self) -> crate::app::relaybackupdata::RelayBackupData;

    #[method(name = "get_BackupDataForAll", args = 0)]
    pub fn get_backup_data_for_all(self) -> crate::app::relaybackupdata::RelayBackupData;

    #[method(name = "get_Flag", args = 0)]
    pub fn get_flag(self) -> crate::app::relay::Relay_FlagsField;

    #[method(name = "get_IsNew", args = 0)]
    pub fn get_is_new(self) -> bool;

    #[method(name = "get_IsTakeOver", args = 0)]
    pub fn get_is_take_over(self) -> bool;

    #[method(name = "get_IsReplay", args = 0)]
    pub fn get_is_replay(self) -> bool;

    #[method(name = "get_MaxTurn", args = 0)]
    pub fn get_max_turn(self) -> i32;

    #[method(name = "get_LimitTurn", args = 0)]
    pub fn get_limit_turn(self) -> i32;

    #[method(name = "get_Sortie", args = 0)]
    pub fn get_sortie(self) -> crate::app::relaysortie::RelaySortie;

    #[method(name = "get_AppearanceUnits", args = 0)]
    pub fn get_appearance_units(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>;

    #[method(name = "get_LeavingUnits", args = 0)]
    pub fn get_leaving_units(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::unit::Unit>;

    #[method(name = "get_CurrentPlayerIndex", args = 0)]
    pub fn get_current_player_index(self) -> i32;

    #[method(name = "set_CurrentPlayerIndex", args = 1)]
    pub fn set_current_player_index(self, value: i32) -> ();

    #[method(name = "get_CurrentBattle", args = 0)]
    pub fn get_current_battle(self) -> crate::app::relayuserdata::RelayUserData_EnteredBattle;

    #[method(name = "get_PoolUnits", args = 0)]
    pub fn get_pool_units(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "get_PoolGods", args = 0)]
    pub fn get_pool_gods(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "ConvertToKinds", args = 1)]
    pub fn convert_to_kinds(
        country: crate::app::persondata::PersonData_Country,
    ) -> crate::app::relaystampdata::RelayStampData_Kinds;

    #[method(name = "DeleteUnneededUnits", args = 0)]
    pub fn delete_unneeded_units(self) -> ();

    #[method(name = "DeleteUnneededGodUnitsAndBonds", args = 0)]
    pub fn delete_unneeded_god_units_and_bonds(self) -> ();

    #[method(name = "DeleteUnneededRings", args = 0)]
    pub fn delete_unneeded_rings(self) -> ();

    #[method(name = "ChooseAwardee", args = 0)]
    pub fn choose_awardee(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "SaveTempPoolUnit", args = 0)]
    pub fn save_temp_pool_unit(self) -> ();

    #[method(name = "GetMaxTurnFromRelayData", args = 1)]
    pub fn get_max_turn_from_relay_data(cid: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetPlayTurnFromRelayData", args = 2)]
    pub fn get_play_turn_from_relay_data(cid: ::unity2::Il2CppString, is_new: bool) -> i32;

    #[method(name = "DbgLogBegin", args = 0)]
    pub fn dbg_log_begin(self) -> ();

    #[method(name = "DbgLogEnd", args = 0)]
    pub fn dbg_log_end(self) -> ();

    #[method(name = "DbgLogAppendLine", args = 1)]
    pub fn dbg_log_append_line(self, str: ::unity2::Il2CppString) -> ();

    #[method(name = "DbgLogBeginImpl", args = 0)]
    pub fn dbg_log_begin_impl(self) -> ();

    #[method(name = "DbgLogEndImpl", args = 0)]
    pub fn dbg_log_end_impl(self) -> ();

    #[method(name = "DbgLogAppendLineImpl", args = 1)]
    pub fn dbg_log_append_line_impl(self, str: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relay")]
impl Relay {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Relay),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relay/Relay_ChooseAwardeeData.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct Relay_ChooseAwardeeData {
    pub unit: crate::app::unit::Unit,
    pub leaving_unit_data: crate::app::relayleavingunitdata::RelayLeavingUnitData,
    pub count: i32,
}

impl ::unity2::ClassIdentity for Relay_ChooseAwardeeData {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Relay.ChooseAwardeeData";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Relay_ChooseAwardeeData {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-relay")]
#[::unity2::methods(value)]
impl Relay_ChooseAwardeeData {
    #[method(name = "CompareAndSwap", args = 3)]
    pub fn compare_and_swap(
        self,
        unit: crate::app::unit::Unit,
        count: i32,
        op: crate::app::relayawarddata::RelayAwardData_CompareOp,
    ) -> ();

    #[method(name = "CompareAndSwap", args = 3)]
    pub fn compare_and_swap_2(
        self,
        leaving_unit_data: crate::app::relayleavingunitdata::RelayLeavingUnitData,
        count: i32,
        op: crate::app::relayawarddata::RelayAwardData_CompareOp,
    ) -> ();

    #[method(name = "Compare", args = 2)]
    pub fn compare(
        self,
        count: i32,
        op: crate::app::relayawarddata::RelayAwardData_CompareOp,
    ) -> bool;

    #[method(name = "CompareGreater", args = 1)]
    pub fn compare_greater(self, count: i32) -> bool;

    #[method(name = "CompareLess", args = 1)]
    pub fn compare_less(self, count: i32) -> bool;

    #[method(name = "CompareZero", args = 1)]
    pub fn compare_zero(self, count: i32) -> bool;

    #[method(name = "IsRandom", args = 0)]
    pub fn is_random(self) -> bool;

    #[method(name = "IsValid", args = 0)]
    pub fn is_valid(self) -> bool;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relay/Relay_TakeOverModes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Relay_TakeOverModes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Relay_TakeOverModes {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Relay.TakeOverModes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Relay_TakeOverModes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Relay_TakeOverModes {
    pub fn random() -> Self {
        Self { value: 0 }
    }

    pub fn data_code() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relay/Relay_DisposPlayerCountsSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Relay_DisposPlayerCountsSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Relay_DisposPlayerCountsSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Relay.DisposPlayerCountsSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Relay_DisposPlayerCountsSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Relay_DisposPlayerCountsSequence_Label {
    pub fn loading() -> Self {
        Self { value: 0 }
    }

    pub fn end() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relay/Relay_FlagsField.md")))]
#[::unity2::class(namespace = "App", name = "Relay.FlagsField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: relay :: Relay_Flags >)]
pub struct Relay_FlagsField {}

#[cfg(feature = "app-relay")]
#[::unity2::methods]
impl Relay_FlagsField {
    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::relay::Relay_Flags) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relay")]
impl Relay_FlagsField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Relay_FlagsField),
                ::core::stringify!(new),
            )
        });
        <Self as IRelay_FlagsFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relay/Relay_TrfrResult.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Relay_TrfrResult {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Relay_TrfrResult {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Relay.TrfrResult";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Relay_TrfrResult {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Relay_TrfrResult {
    pub fn no_changed() -> Self {
        Self { value: 0 }
    }

    pub fn done() -> Self {
        Self { value: 1 }
    }

    pub fn failed() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relay/Relay_Modes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Relay_Modes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Relay_Modes {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Relay.Modes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Relay_Modes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Relay_Modes {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn take_over() -> Self {
        Self { value: 1 }
    }

    pub fn replay() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relay/Relay_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Relay_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Relay_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Relay.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Relay_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Relay_Flags {
    pub fn need_to_upload() -> Self {
        Self { value: 1 }
    }

    pub fn uploaded() -> Self {
        Self { value: 2 }
    }

    pub fn global_saved() -> Self {
        Self { value: 4 }
    }

    pub fn show_win_rule() -> Self {
        Self { value: 8 }
    }
}
