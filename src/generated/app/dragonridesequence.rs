
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::procscenesequence_1::IProcSceneSequence_1;
use crate::app::procscenesequence_1::ProcSceneSequence_1;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridesequence/DragonRideSequence.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideSequence")]
# [parent (crate :: app :: procscenesequence_1 :: ProcSceneSequence_1 < crate :: app :: hubsequence :: HubSequence >)]
pub struct DragonRideSequence {
    #[rename(name = "m_IsLoadMenuContent")]
    pub m_is_load_menu_content: bool,
    #[rename(name = "m_PrizeBondData")]
    pub m_prize_bond_data: crate::app::dragonridesequence::DragonRideSequence_PrizeData,
    #[rename(name = "m_PrizeItemDataList")]
    pub m_prize_item_data_list: crate::system::collections::generic::list_1::List_1<
        crate::app::dragonridesequence::DragonRideSequence_PrizeData,
    >,
    #[rename(name = "m_PrizeItemMax")]
    pub m_prize_item_max: i32,
    #[rename(name = "m_PrizeItemCount")]
    pub m_prize_item_count: i32,
    #[rename(name = "m_AnnouceDifficult")]
    pub m_annouce_difficult:
        crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "m_IsAnnounceVoiceOnce")]
    pub m_is_announce_voice_once: bool,
    #[rename(name = "m_Talker")]
    pub m_talker: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_TalkerChara")]
    pub m_talker_chara: crate::combat::character::Character,
}

#[cfg(feature = "app-dragonridesequence")]
#[::unity2::methods]
impl DragonRideSequence {
    #[method(name = "get_FromDebugMenu", args = 0)]
    pub fn get_from_debug_menu(self) -> bool;

    #[method(name = "set_FromDebugMenu", args = 1)]
    pub fn set_from_debug_menu(self, value: bool) -> ();

    #[method(name = "_CommonDebugMenu", args = 0)]
    pub fn common_debug_menu(self) -> ();

    #[method(name = "OnShutdown", args = 0)]
    pub fn on_shutdown(self) -> ();

    #[method(name = "CancelFade", args = 0)]
    pub fn cancel_fade(self) -> ();

    #[method(name = "CheckTestExecute", args = 0)]
    pub fn check_test_execute(self) -> ();

    #[method(name = "LoadResource", args = 0)]
    pub fn load_resource(self) -> ();

    #[method(name = "IsLoadingResource", args = 0)]
    pub fn is_loading_resource(self) -> bool;

    #[method(name = "UnloadResouce", args = 0)]
    pub fn unload_resouce(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "CheckNewDifficult", args = 0)]
    pub fn check_new_difficult(self) -> ();

    #[method(name = "AnnounceNewDifficult", args = 0)]
    pub fn announce_new_difficult(self) -> ();

    #[method(name = "CheckAnnounceDifficult", args = 0)]
    pub fn check_announce_difficult(self) -> bool;

    #[method(name = "CreateTitleBar", args = 0)]
    pub fn create_title_bar(self) -> ();

    #[method(name = "CloseTitleBar", args = 0)]
    pub fn close_title_bar(self) -> ();

    #[method(name = "CreateSelectMenu", args = 0)]
    pub fn create_select_menu(self) -> ();

    #[method(name = "Talk_CourseSelect", args = 0)]
    pub fn talk_course_select(self) -> ();

    #[method(name = "ConfirmAssist", args = 0)]
    pub fn confirm_assist(self) -> ();

    #[method(name = "CreateGameSequence", args = 0)]
    pub fn create_game_sequence(self) -> ();

    #[method(name = "CheckRetireMinigame", args = 0)]
    pub fn check_retire_minigame(self) -> bool;

    #[method(name = "CheckGetablePrize", args = 0)]
    pub fn check_getable_prize(self) -> bool;

    #[method(name = "SetPrizeFlag", args = 0)]
    pub fn set_prize_flag(self) -> ();

    #[method(name = "SetPrizeList", args = 0)]
    pub fn set_prize_list(self) -> ();

    #[method(name = "GetPrizeBond", args = 0)]
    pub fn get_prize_bond(self) -> ();

    #[method(name = "GetPrizeItem", args = 0)]
    pub fn get_prize_item(self) -> ();

    #[method(name = "isGetAllPrizeItem", args = 0)]
    pub fn is_get_all_prize_item(self) -> bool;

    #[method(name = "VoiceAnnounceNewDifficult", args = 0)]
    pub fn voice_announce_new_difficult(self) -> ();

    #[method(name = "StopVoiceNewDifficult", args = 0)]
    pub fn stop_voice_new_difficult(self) -> ();

    #[method(name = "VoiceSelectDifficult", args = 0)]
    pub fn voice_select_difficult(self) -> ();

    #[method(name = "VoiceStart", args = 0)]
    pub fn voice_start(self) -> ();

    #[method(name = "VoiceEnd", args = 0)]
    pub fn voice_end(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, from_debug_menu: bool) -> ();

    #[method(name = "RegistFlag", args = 0)]
    pub fn regist_flag() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-dragonridesequence")]
impl DragonRideSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridesequence/DragonRideSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct DragonRideSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for DragonRideSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "DragonRideSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for DragonRideSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl DragonRideSequence_Label {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn init() -> Self {
        Self { value: 1 }
    }

    pub fn check_test() -> Self {
        Self { value: 2 }
    }

    pub fn ready_menu() -> Self {
        Self { value: 3 }
    }

    pub fn play_select() -> Self {
        Self { value: 4 }
    }

    pub fn check_new_difficult() -> Self {
        Self { value: 5 }
    }

    pub fn announce_new_difficult() -> Self {
        Self { value: 6 }
    }

    pub fn difficult_select() -> Self {
        Self { value: 7 }
    }

    pub fn ready_game() -> Self {
        Self { value: 8 }
    }

    pub fn execute_game() -> Self {
        Self { value: 9 }
    }

    pub fn prize_talk() -> Self {
        Self { value: 10 }
    }

    pub fn prize_bond() -> Self {
        Self { value: 11 }
    }

    pub fn prize_item() -> Self {
        Self { value: 12 }
    }

    pub fn exit() -> Self {
        Self { value: 13 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/dragonridesequence/DragonRideSequence_PrizeData.md")))]
#[::unity2::class(namespace = "App", name = "DragonRideSequence.PrizeData")]
#[parent(crate::system::object::Object)]
pub struct DragonRideSequence_PrizeData {}

#[cfg(feature = "app-dragonridesequence")]
#[::unity2::methods]
impl DragonRideSequence_PrizeData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor_2(self, set_item: crate::app::itemdata::ItemData, set_num: i32) -> ();

    #[method(name = "AddNum", args = 1)]
    pub fn add_num(self, add: i32) -> ();

    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_item", args = 0)]
    pub fn get_item(self) -> crate::app::itemdata::ItemData;

    #[method(name = "set_item", args = 1)]
    pub fn set_item(self, value: crate::app::itemdata::ItemData) -> ();

    #[method(name = "get_num", args = 0)]
    pub fn get_num(self) -> i32;

    #[method(name = "set_num", args = 1)]
    pub fn set_num(self, value: i32) -> ();
}

#[cfg(feature = "app-dragonridesequence")]
impl DragonRideSequence_PrizeData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideSequence_PrizeData),
                ::core::stringify!(new),
            )
        });
        <Self as IDragonRideSequence_PrizeDataMethods>::ctor(this);
        this
    }

    pub fn new_2(set_item: crate::app::itemdata::ItemData, set_num: i32) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(DragonRideSequence_PrizeData),
                ::core::stringify!(new_2),
            )
        });
        <Self as IDragonRideSequence_PrizeDataMethods>::ctor_2(this, set_item, set_num);
        this
    }
}
