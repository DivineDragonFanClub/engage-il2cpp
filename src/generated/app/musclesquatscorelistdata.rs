
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/musclesquatscorelistdata/MuscleSquatScoreListData.md")))]
#[::unity2::class(namespace = "App", name = "MuscleSquatScoreListData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: musclesquatscorelistdata :: MuscleSquatScoreListData >)]
pub struct MuscleSquatScoreListData {}

#[cfg(feature = "app-musclesquatscorelistdata")]
#[::unity2::methods]
impl MuscleSquatScoreListData {
    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Level", args = 0)]
    pub fn get_level(self) -> i32;

    #[method(name = "set_Level", args = 1)]
    pub fn set_level(self, value: i32) -> ();

    #[method(name = "get_UseCount", args = 0)]
    pub fn get_use_count(self) -> i32;

    #[method(name = "set_UseCount", args = 1)]
    pub fn set_use_count(self, value: i32) -> ();

    #[method(name = "get_IsDoubleChoice", args = 0)]
    pub fn get_is_double_choice(self) -> bool;

    #[method(name = "set_IsDoubleChoice", args = 1)]
    pub fn set_is_double_choice(self, value: bool) -> ();

    #[method(name = "get_Speed", args = 0)]
    pub fn get_speed(self) -> f32;

    #[method(name = "set_Speed", args = 1)]
    pub fn set_speed(self, value: f32) -> ();

    #[method(name = "get_Length", args = 0)]
    pub fn get_length(self) -> f32;

    #[method(name = "set_Length", args = 1)]
    pub fn set_length(self, value: f32) -> ();

    #[method(name = "get_Type_1", args = 0)]
    pub fn get_type_1(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Type_1", args = 1)]
    pub fn set_type_1(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LotteryParam_1", args = 0)]
    pub fn get_lottery_param_1(self) -> f32;

    #[method(name = "set_LotteryParam_1", args = 1)]
    pub fn set_lottery_param_1(self, value: f32) -> ();

    #[method(name = "get_Type_2", args = 0)]
    pub fn get_type_2(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Type_2", args = 1)]
    pub fn set_type_2(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LotteryParam_2", args = 0)]
    pub fn get_lottery_param_2(self) -> f32;

    #[method(name = "set_LotteryParam_2", args = 1)]
    pub fn set_lottery_param_2(self, value: f32) -> ();

    #[method(name = "get_Type_3", args = 0)]
    pub fn get_type_3(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Type_3", args = 1)]
    pub fn set_type_3(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LotteryParam_3", args = 0)]
    pub fn get_lottery_param_3(self) -> f32;

    #[method(name = "set_LotteryParam_3", args = 1)]
    pub fn set_lottery_param_3(self, value: f32) -> ();

    #[method(name = "get_Type_4", args = 0)]
    pub fn get_type_4(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Type_4", args = 1)]
    pub fn set_type_4(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LotteryParam_4", args = 0)]
    pub fn get_lottery_param_4(self) -> f32;

    #[method(name = "set_LotteryParam_4", args = 1)]
    pub fn set_lottery_param_4(self, value: f32) -> ();

    #[method(name = "get_Type_5", args = 0)]
    pub fn get_type_5(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Type_5", args = 1)]
    pub fn set_type_5(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LotteryParam_5", args = 0)]
    pub fn get_lottery_param_5(self) -> f32;

    #[method(name = "set_LotteryParam_5", args = 1)]
    pub fn set_lottery_param_5(self, value: f32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();
}

#[cfg(feature = "app-musclesquatscorelistdata")]
impl MuscleSquatScoreListData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MuscleSquatScoreListData),
                ::core::stringify!(new),
            )
        });
        <Self as IMuscleSquatScoreListDataMethods>::ctor(this);
        this
    }
}
