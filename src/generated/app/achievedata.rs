
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/achievedata/AchieveData_Status.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AchieveData_Status {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AchieveData_Status {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AchieveData.Status";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AchieveData_Status {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AchieveData_Status {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn cleared() -> Self {
        Self { value: 1 }
    }

    pub fn showed() -> Self {
        Self { value: 2 }
    }

    pub fn completed() -> Self {
        Self { value: 3 }
    }

    pub fn num() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/achievedata/AchieveData_Kinds.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AchieveData_Kinds {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AchieveData_Kinds {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AchieveData.Kinds";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AchieveData_Kinds {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AchieveData_Kinds {
    pub fn _unnamed() -> Self {
        Self { value: 0 }
    }

    pub fn p_______() -> Self {
        Self { value: 90 }
    }

    pub fn p_________() -> Self {
        Self { value: 91 }
    }

    pub fn p________() -> Self {
        Self { value: 92 }
    }

    pub fn p___________() -> Self {
        Self { value: 93 }
    }

    pub fn p______() -> Self {
        Self { value: 95 }
    }

    pub fn p_____() -> Self {
        Self { value: 96 }
    }

    pub fn p_____________() -> Self {
        Self { value: 103 }
    }

    pub fn p____________() -> Self {
        Self { value: 104 }
    }

    pub fn p_______________() -> Self {
        Self { value: 105 }
    }

    pub fn p__________() -> Self {
        Self { value: 122 }
    }

    pub fn num() -> Self {
        Self { value: 137 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/achievedata/AchieveData_ArgType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AchieveData_ArgType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AchieveData_ArgType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AchieveData.ArgType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AchieveData_ArgType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AchieveData_ArgType {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn message() -> Self {
        Self { value: 1 }
    }

    pub fn chapter() -> Self {
        Self { value: 2 }
    }

    pub fn chapter_side() -> Self {
        Self { value: 3 }
    }

    pub fn person() -> Self {
        Self { value: 4 }
    }

    pub fn num() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/achievedata/AchieveData_Categories.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AchieveData_Categories {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AchieveData_Categories {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AchieveData.Categories";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AchieveData_Categories {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AchieveData_Categories {
    pub fn unit() -> Self {
        Self { value: 0 }
    }

    pub fn battle() -> Self {
        Self { value: 1 }
    }

    pub fn solanel() -> Self {
        Self { value: 2 }
    }

    pub fn shop() -> Self {
        Self { value: 3 }
    }

    pub fn system() -> Self {
        Self { value: 4 }
    }

    pub fn play_report() -> Self {
        Self { value: 5 }
    }

    pub fn num() -> Self {
        Self { value: 5 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/achievedata/AchieveData.md")))]
#[::unity2::class(namespace = "App", name = "AchieveData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: achievedata :: AchieveData >)]
pub struct AchieveData {
    #[rename(name = "m_FlagName")]
    pub m_flag_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "s_KindDictionary")]
    pub s_kind_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        i32,
        crate::system::collections::generic::list_1::List_1<crate::app::achievedata::AchieveData>,
    >,
    #[static_field]
    #[rename(name = "s_ShowQueue")]
    pub s_show_queue:
        crate::system::collections::generic::queue_1::Queue_1<crate::app::achievedata::AchieveData>,
}

#[cfg(feature = "app-achievedata")]
#[::unity2::methods]
impl AchieveData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "IsGrouping", args = 1)]
    pub fn is_grouping(kind: crate::app::achievedata::AchieveData_Kinds) -> bool;

    #[method(name = "IsCount", args = 1)]
    pub fn is_count(kind: crate::app::achievedata::AchieveData_Kinds) -> bool;

    #[method(name = "get_Aid", args = 0)]
    pub fn get_aid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Aid", args = 1)]
    pub fn set_aid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Category", args = 0)]
    pub fn get_category(self) -> crate::app::achievedata::AchieveData_Categories;

    #[method(name = "set_Category", args = 1)]
    pub fn set_category(self, value: crate::app::achievedata::AchieveData_Categories) -> ();

    #[method(name = "get_Kind", args = 0)]
    pub fn get_kind(self) -> crate::app::achievedata::AchieveData_Kinds;

    #[method(name = "set_Kind", args = 1)]
    pub fn set_kind(self, value: crate::app::achievedata::AchieveData_Kinds) -> ();

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "set_Count", args = 1)]
    pub fn set_count(self, value: i32) -> ();

    #[method(name = "get_Arg", args = 0)]
    pub fn get_arg(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Arg", args = 1)]
    pub fn set_arg(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_CountUnit", args = 0)]
    pub fn get_count_unit(self) -> ::unity2::Il2CppString;

    #[method(name = "set_CountUnit", args = 1)]
    pub fn set_count_unit(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_KizunaReward", args = 0)]
    pub fn get_kizuna_reward(self) -> i32;

    #[method(name = "set_KizunaReward", args = 1)]
    pub fn set_kizuna_reward(self, value: i32) -> ();

    #[method(name = "get_Chapter", args = 0)]
    pub fn get_chapter(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Chapter", args = 1)]
    pub fn set_chapter(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetArgType", args = 0)]
    pub fn get_arg_type(self) -> crate::app::achievedata::AchieveData_ArgType;

    #[method(name = "GetPerson", args = 0)]
    pub fn get_person(self) -> crate::app::persondata::PersonData;

    #[method(name = "GetGod", args = 0)]
    pub fn get_god(self) -> crate::app::goddata::GodData;

    #[method(name = "GetStatus", args = 0)]
    pub fn get_status(self) -> crate::app::achievedata::AchieveData_Status;

    #[method(name = "SetCleared", args = 0)]
    pub fn set_cleared(self) -> bool;

    #[method(name = "SetShowed", args = 0)]
    pub fn set_showed(self) -> bool;

    #[method(name = "SetComplete", args = 0)]
    pub fn set_complete(self) -> bool;

    #[method(name = "IsCanGet", args = 0)]
    pub fn is_can_get(self) -> bool;

    #[method(name = "SetStatus", args = 1)]
    pub fn set_status(self, status: crate::app::achievedata::AchieveData_Status) -> ();

    #[method(name = "GetValue", args = 0)]
    pub fn get_value(self) -> i32;

    #[method(name = "RegistGlobalFlags", args = 0)]
    pub fn regist_global_flags() -> ();

    #[method(name = "GetFlagName", args = 1)]
    pub fn get_flag_name(
        kind: crate::app::achievedata::AchieveData_Kinds,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetFlagName", args = 2)]
    pub fn get_flag_name_2(
        kind: crate::app::achievedata::AchieveData_Kinds,
        footer: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetFlagName", args = 1)]
    pub fn get_flag_name_3(kind: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "GetCountCurrent", args = 1)]
    pub fn get_count_current(self, footer: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetKindCount", args = 1)]
    pub fn get_kind_count(kind: ::unity2::Il2CppString) -> i32;

    #[method(name = "SetClearReliance", args = 1)]
    pub fn set_clear_reliance(person: crate::app::persondata::PersonData) -> ();

    #[method(name = "AddCountRelianceB", args = 0)]
    pub fn add_count_reliance_b() -> ();

    #[method(name = "AddCountRelianceA", args = 0)]
    pub fn add_count_reliance_a() -> ();

    #[method(name = "AddCountRelianceS", args = 0)]
    pub fn add_count_reliance_s() -> ();

    #[method(name = "AddCountSortie", args = 1)]
    pub fn add_count_sortie(person: crate::app::persondata::PersonData) -> ();

    #[method(name = "AddCountEncount", args = 0)]
    pub fn add_count_encount() -> ();

    #[method(name = "AddCountBattle", args = 0)]
    pub fn add_count_battle() -> ();

    #[method(name = "AddCountDefeat", args = 1)]
    pub fn add_count_defeat(person: crate::app::persondata::PersonData) -> ();

    #[method(name = "AddCountRod", args = 0)]
    pub fn add_count_rod() -> ();

    #[method(name = "AddCountCritical", args = 0)]
    pub fn add_count_critical() -> ();

    #[method(name = "AddCountEngage", args = 0)]
    pub fn add_count_engage() -> ();

    #[method(name = "AddCountEngageAttack", args = 0)]
    pub fn add_count_engage_attack() -> ();

    #[method(name = "AddCountAvoidance", args = 0)]
    pub fn add_count_avoidance() -> ();

    #[method(name = "AddCountChainGuard", args = 0)]
    pub fn add_count_chain_guard() -> ();

    #[method(name = "AddCountChainAttack", args = 0)]
    pub fn add_count_chain_attack() -> ();

    #[method(name = "SetValueChainUnit", args = 1)]
    pub fn set_value_chain_unit(unit_num: i32) -> ();

    #[method(name = "AddCountBreak", args = 0)]
    pub fn add_count_break() -> ();

    #[method(name = "AddCountSmash", args = 0)]
    pub fn add_count_smash() -> ();

    #[method(name = "AddCountMiniGame", args = 0)]
    pub fn add_count_mini_game() -> ();

    #[method(name = "AddCountRingForm", args = 1)]
    pub fn add_count_ring_form(add_value: i32) -> ();

    #[method(name = "AddCountRingMix", args = 0)]
    pub fn add_count_ring_mix() -> ();

    #[method(name = "AddCountBondsRingC", args = 1)]
    pub fn add_count_bonds_ring_c(add_value: i32) -> ();

    #[method(name = "AddCountBondsRingB", args = 1)]
    pub fn add_count_bonds_ring_b(add_value: i32) -> ();

    #[method(name = "AddCountBondsRingA", args = 1)]
    pub fn add_count_bonds_ring_a(add_value: i32) -> ();

    #[method(name = "AddCountBondsRingS", args = 1)]
    pub fn add_count_bonds_ring_s(add_value: i32) -> ();

    #[method(name = "AddCountRingCleaning", args = 0)]
    pub fn add_count_ring_cleaning() -> ();

    #[method(name = "SetValueInvestmentFilene", args = 1)]
    pub fn set_value_investment_filene(level: i32) -> ();

    #[method(name = "SetValueInvestmentBrodia", args = 1)]
    pub fn set_value_investment_brodia(level: i32) -> ();

    #[method(name = "SetValueInvestmentIrcion", args = 1)]
    pub fn set_value_investment_ircion(level: i32) -> ();

    #[method(name = "SetValueInvestmentSolum", args = 1)]
    pub fn set_value_investment_solum(level: i32) -> ();

    #[method(name = "SetClearInvestmentAll", args = 0)]
    pub fn set_clear_investment_all() -> ();

    #[method(name = "AddCountInvestmentMoney", args = 1)]
    pub fn add_count_investment_money(money: i32) -> ();

    #[method(name = "AddCountCookAll", args = 0)]
    pub fn add_count_cook_all() -> ();

    #[method(name = "AddCountCookG", args = 0)]
    pub fn add_count_cook_g() -> ();

    #[method(name = "AddCountCookF", args = 0)]
    pub fn add_count_cook_f() -> ();

    #[method(name = "AddCountCookE", args = 0)]
    pub fn add_count_cook_e() -> ();

    #[method(name = "AddCountCookD", args = 0)]
    pub fn add_count_cook_d() -> ();

    #[method(name = "AddCountCookC", args = 0)]
    pub fn add_count_cook_c() -> ();

    #[method(name = "AddCountCookB", args = 0)]
    pub fn add_count_cook_b() -> ();

    #[method(name = "AddCountCookA", args = 0)]
    pub fn add_count_cook_a() -> ();

    #[method(name = "AddCountCookS", args = 0)]
    pub fn add_count_cook_s() -> ();

    #[method(name = "AddCountCookSS", args = 0)]
    pub fn add_count_cook_ss() -> ();

    #[method(name = "AddCountSleep", args = 0)]
    pub fn add_count_sleep() -> ();

    #[method(name = "AddCountWakeUpC", args = 0)]
    pub fn add_count_wake_up_c() -> ();

    #[method(name = "AddCountWakeUpB", args = 0)]
    pub fn add_count_wake_up_b() -> ();

    #[method(name = "AddCountWakeUpA", args = 0)]
    pub fn add_count_wake_up_a() -> ();

    #[method(name = "AddCountWakeUpS", args = 0)]
    pub fn add_count_wake_up_s() -> ();

    #[method(name = "AddCountUnitBattle", args = 0)]
    pub fn add_count_unit_battle() -> ();

    #[method(name = "AddCountUnitBattleWin", args = 0)]
    pub fn add_count_unit_battle_win() -> ();

    #[method(name = "AddCountGodBattle", args = 0)]
    pub fn add_count_god_battle() -> ();

    #[method(name = "AddCountGodBattleWin", args = 0)]
    pub fn add_count_god_battle_win() -> ();

    #[method(name = "AddCountBuyWeapon", args = 0)]
    pub fn add_count_buy_weapon() -> ();

    #[method(name = "AddCountBuySword", args = 0)]
    pub fn add_count_buy_sword() -> ();

    #[method(name = "AddCountBuyLance", args = 0)]
    pub fn add_count_buy_lance() -> ();

    #[method(name = "AddCountBuyAxe", args = 0)]
    pub fn add_count_buy_axe() -> ();

    #[method(name = "AddCountBuyBow", args = 0)]
    pub fn add_count_buy_bow() -> ();

    #[method(name = "AddCountBuyKnife", args = 0)]
    pub fn add_count_buy_knife() -> ();

    #[method(name = "AddCountBuyMagic", args = 0)]
    pub fn add_count_buy_magic() -> ();

    #[method(name = "AddCountBuyFist", args = 0)]
    pub fn add_count_buy_fist() -> ();

    #[method(name = "AddCountSellWeapon", args = 1)]
    pub fn add_count_sell_weapon(add_value: i32) -> ();

    #[method(name = "AddCountBuyItem", args = 0)]
    pub fn add_count_buy_item() -> ();

    #[method(name = "AddCountSellItem", args = 1)]
    pub fn add_count_sell_item(add_value: i32) -> ();

    #[method(name = "AddCountBuyRod", args = 0)]
    pub fn add_count_buy_rod() -> ();

    #[method(name = "AddCountBuyAccessories", args = 0)]
    pub fn add_count_buy_accessories() -> ();

    #[method(name = "AddCountChangeAccessories", args = 0)]
    pub fn add_count_change_accessories() -> ();

    #[method(name = "AddCountForging", args = 0)]
    pub fn add_count_forging() -> ();

    #[method(name = "AddCountForgingSword", args = 0)]
    pub fn add_count_forging_sword() -> ();

    #[method(name = "AddCountForgingLance", args = 0)]
    pub fn add_count_forging_lance() -> ();

    #[method(name = "AddCountForgingAxe", args = 0)]
    pub fn add_count_forging_axe() -> ();

    #[method(name = "AddCountForgingBow", args = 0)]
    pub fn add_count_forging_bow() -> ();

    #[method(name = "AddCountForgingKnife", args = 0)]
    pub fn add_count_forging_knife() -> ();

    #[method(name = "AddCountForgingMagic", args = 0)]
    pub fn add_count_forging_magic() -> ();

    #[method(name = "AddCountForgingFist", args = 0)]
    pub fn add_count_forging_fist() -> ();

    #[method(name = "AddCountEngrave", args = 0)]
    pub fn add_count_engrave() -> ();

    #[method(name = "SetValuePlayTime", args = 1)]
    pub fn set_value_play_time(time: f32) -> ();

    #[method(name = "SetClearChapter", args = 1)]
    pub fn set_clear_chapter(chapter: crate::app::chapterdata::ChapterData) -> ();

    #[method(name = "AddCountNetMatch", args = 0)]
    pub fn add_count_net_match() -> ();

    #[method(name = "AddCountNetMatchWin", args = 0)]
    pub fn add_count_net_match_win() -> ();

    #[method(name = "AddCountRelayBattle", args = 0)]
    pub fn add_count_relay_battle() -> ();

    #[method(name = "AddCountRelayBattleWin", args = 0)]
    pub fn add_count_relay_battle_win() -> ();

    #[method(name = "SetChallengeRank", args = 2)]
    pub fn set_challenge_rank(route: i32, level: i32) -> ();

    #[method(name = "PlayReportAddCountClassChange", args = 0)]
    pub fn play_report_add_count_class_change() -> ();

    #[method(name = "PlayReportAddCountPushUpsNormal", args = 0)]
    pub fn play_report_add_count_push_ups_normal() -> ();

    #[method(name = "PlayReportAddCountPushUpsHard", args = 0)]
    pub fn play_report_add_count_push_ups_hard() -> ();

    #[method(name = "PlayReportAddCountPushUpsExpert", args = 0)]
    pub fn play_report_add_count_push_ups_expert() -> ();

    #[method(name = "PlayReportAddCountPushUpsMuscle", args = 0)]
    pub fn play_report_add_count_push_ups_muscle() -> ();

    #[method(name = "PlayReportAddCountAdsNormal", args = 0)]
    pub fn play_report_add_count_ads_normal() -> ();

    #[method(name = "PlayReportAddCountAdsHard", args = 0)]
    pub fn play_report_add_count_ads_hard() -> ();

    #[method(name = "PlayReportAddCountAdsExpert", args = 0)]
    pub fn play_report_add_count_ads_expert() -> ();

    #[method(name = "PlayReportAddCountAdsMuscle", args = 0)]
    pub fn play_report_add_count_ads_muscle() -> ();

    #[method(name = "PlayReportAddCountSquatNormal", args = 0)]
    pub fn play_report_add_count_squat_normal() -> ();

    #[method(name = "PlayReportAddCountSquatHard", args = 0)]
    pub fn play_report_add_count_squat_hard() -> ();

    #[method(name = "PlayReportAddCountSquatExpert", args = 0)]
    pub fn play_report_add_count_squat_expert() -> ();

    #[method(name = "PlayReportAddCountSquatMuscle", args = 0)]
    pub fn play_report_add_count_squat_muscle() -> ();

    #[method(name = "PlayReportAddCountDragonRideNormal", args = 0)]
    pub fn play_report_add_count_dragon_ride_normal() -> ();

    #[method(name = "PlayReportAddCountDragonRideHard", args = 0)]
    pub fn play_report_add_count_dragon_ride_hard() -> ();

    #[method(name = "PlayReportAddCountDragonRideExpert", args = 0)]
    pub fn play_report_add_count_dragon_ride_expert() -> ();

    #[method(name = "PlayReportAddCountFishingRodSmoll", args = 0)]
    pub fn play_report_add_count_fishing_rod_smoll() -> ();

    #[method(name = "PlayReportAddCountFishingRodNormal", args = 0)]
    pub fn play_report_add_count_fishing_rod_normal() -> ();

    #[method(name = "PlayReportAddCountFishingRodAllPurpose", args = 0)]
    pub fn play_report_add_count_fishing_rod_all_purpose() -> ();

    #[method(name = "PlayReportAddCountCleaningFormGod", args = 1)]
    pub fn play_report_add_count_cleaning_form_god(gid: ::unity2::Il2CppString) -> ();

    #[method(name = "PlayReportAddCountChallenge", args = 1)]
    pub fn play_report_add_count_challenge(route: i32) -> ();

    #[method(name = "PlayReportAddCountChallengeClear", args = 1)]
    pub fn play_report_add_count_challenge_clear(route: i32) -> ();

    #[method(name = "PlayReportAddCountRelayBattle", args = 1)]
    pub fn play_report_add_count_relay_battle(route: i32) -> ();

    #[method(name = "PlayReportAddCountRelayBattleInherit", args = 0)]
    pub fn play_report_add_count_relay_battle_inherit() -> ();

    #[method(name = "PlayReportAddCountVersusCasual", args = 0)]
    pub fn play_report_add_count_versus_casual() -> ();

    #[method(name = "PlayReportAddCountVersusCasualWin", args = 0)]
    pub fn play_report_add_count_versus_casual_win() -> ();

    #[method(name = "PlayReportAddCountVersusCasualLose", args = 0)]
    pub fn play_report_add_count_versus_casual_lose() -> ();

    #[method(name = "PlayReportAddCountVersusRanked", args = 0)]
    pub fn play_report_add_count_versus_ranked() -> ();

    #[method(name = "PlayReportAddCountVersusRankedWin", args = 0)]
    pub fn play_report_add_count_versus_ranked_win() -> ();

    #[method(name = "PlayReportAddCountVersusRankedLose", args = 0)]
    pub fn play_report_add_count_versus_ranked_lose() -> ();

    #[method(name = "PlayReportAddCountVersusRankedDefenseWin", args = 0)]
    pub fn play_report_add_count_versus_ranked_defense_win() -> ();

    #[method(name = "PlayReportAddCountVersusRankedDefenseLose", args = 0)]
    pub fn play_report_add_count_versus_ranked_defense_lose() -> ();

    #[method(name = "PlayReportGetCount", args = 1)]
    pub fn play_report_get_count(kinds: crate::app::achievedata::AchieveData_Kinds) -> i32;

    #[method(name = "PlayReportGetCountCleaningFormGod", args = 1)]
    pub fn play_report_get_count_cleaning_form_god(gid: ::unity2::Il2CppString) -> i32;

    #[method(name = "TrySetCleard", args = 1)]
    pub fn try_set_cleard(data: crate::app::achievedata::AchieveData) -> bool;

    #[method(name = "IsProhibited", args = 0)]
    pub fn is_prohibited() -> bool;

    #[method(name = "IsValid", args = 1)]
    pub fn is_valid(flag_name: ::unity2::Il2CppString) -> bool;

    #[method(name = "AddCount", args = 2)]
    pub fn add_count(kind: crate::app::achievedata::AchieveData_Kinds, add_value: i32) -> ();

    #[method(name = "CommitValue", args = 2)]
    pub fn commit_value(kind: crate::app::achievedata::AchieveData_Kinds, value: i32) -> ();

    #[method(name = "GetKindList", args = 1)]
    pub fn get_kind_list(
        kind: crate::app::achievedata::AchieveData_Kinds,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::achievedata::AchieveData>;

    #[method(name = "UpdateShowQueue", args = 0)]
    pub fn update_show_queue() -> ();

    #[method(name = "ClearShowQueue", args = 0)]
    pub fn clear_show_queue() -> ();

    #[method(name = "DequeueShowData", args = 0)]
    pub fn dequeue_show_data() -> crate::app::achievedata::AchieveData;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-achievedata")]
impl AchieveData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AchieveData),
                ::core::stringify!(new),
            )
        });
        <Self as IAchieveDataMethods>::ctor(this);
        this
    }
}
