
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/rewarddata/RewardData.md")))]
#[::unity2::class(namespace = "App", name = "RewardData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: rewarddata :: RewardData >)]
pub struct RewardData {}

#[cfg(feature = "app-rewarddata")]
#[::unity2::methods]
impl RewardData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Iid", args = 0)]
    pub fn get_iid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Iid", args = 1)]
    pub fn set_iid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Ratio", args = 0)]
    pub fn get_ratio(self) -> f32;

    #[method(name = "set_Ratio", args = 1)]
    pub fn set_ratio(self, value: f32) -> ();

    #[method(name = "get_Factor", args = 0)]
    pub fn get_factor(self) -> f32;

    #[method(name = "set_Factor", args = 1)]
    pub fn set_factor(self, value: f32) -> ();

    #[method(name = "get_Min", args = 0)]
    pub fn get_min(self) -> f32;

    #[method(name = "set_Min", args = 1)]
    pub fn set_min(self, value: f32) -> ();

    #[method(name = "get_Max", args = 0)]
    pub fn get_max(self) -> f32;

    #[method(name = "set_Max", args = 1)]
    pub fn set_max(self, value: f32) -> ();

    #[method(name = "get_IsShow", args = 0)]
    pub fn get_is_show(self) -> bool;

    #[method(name = "set_IsShow", args = 1)]
    pub fn set_is_show(self, value: bool) -> ();

    #[method(name = "get_Condition", args = 0)]
    pub fn get_condition(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Condition", args = 1)]
    pub fn set_condition(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetItem", args = 0)]
    pub fn get_item(self) -> crate::app::itemdata::ItemData;

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "IsCondition", args = 1)]
    pub fn is_condition(condition: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetPercent", args = 1)]
    pub fn get_percent(self, level: i32) -> f32;

    #[method(name = "GetExpFormChallenge", args = 2)]
    pub fn get_exp_form_challenge(sortie_count: i32, difficulty_diff: i32) -> i32;

    #[method(name = "CalcRewardsImpl", args = 4)]
    pub fn calc_rewards_impl(
        array: crate::app::structdataarraylist_1::StructDataArrayList_1<
            crate::app::rewarddata::RewardData,
        >,
        random: crate::app::random_2::Random_2,
        level: i32,
        is_dump: bool,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::rewarddata::RewardData>;

    #[method(name = "CalcRewards", args = 4)]
    pub fn calc_rewards(
        name: ::unity2::Il2CppString,
        random: crate::app::random_2::Random_2,
        level: i32,
        is_dump: bool,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>;

    #[method(name = "CalcRewards", args = 1)]
    pub fn calc_rewards_2(
        name: ::unity2::Il2CppString,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>;

    #[method(name = "CalcGameClearRewards", args = 1)]
    pub fn calc_game_clear_rewards(
        difficulty: crate::app::difficulty::Difficulty,
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>;

    #[method(name = "CalcEvilClearRewards", args = 0)]
    pub fn calc_evil_clear_rewards(
    ) -> crate::system::collections::generic::list_1::List_1<crate::app::itemdata::ItemData>;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-rewarddata")]
impl RewardData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RewardData),
                ::core::stringify!(new),
            )
        });
        <Self as IRewardDataMethods>::ctor(this);
        this
    }
}
