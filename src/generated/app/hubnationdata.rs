
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubnationdata/HubNationData.md")))]
#[::unity2::class(namespace = "App", name = "HubNationData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: hubnationdata :: HubNationData >)]
pub struct HubNationData {}

#[cfg(feature = "app-hubnationdata")]
#[::unity2::methods]
impl HubNationData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Chapter", args = 0)]
    pub fn get_chapter(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Chapter", args = 1)]
    pub fn set_chapter(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_SymbolTexture", args = 0)]
    pub fn get_symbol_texture(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SymbolTexture", args = 1)]
    pub fn set_symbol_texture(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LevelInfo", args = 0)]
    pub fn get_level_info(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LevelInfo", args = 1)]
    pub fn set_level_info(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_FoodstuffInfo", args = 0)]
    pub fn get_foodstuff_info(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FoodstuffInfo", args = 1)]
    pub fn set_foodstuff_info(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AnimalInfo", args = 0)]
    pub fn get_animal_info(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AnimalInfo", args = 1)]
    pub fn set_animal_info(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsNotLevel", args = 0)]
    pub fn get_is_not_level(self) -> bool;

    #[method(name = "set_IsNotLevel", args = 1)]
    pub fn set_is_not_level(self, value: bool) -> ();

    #[method(name = "IsInvestmentEnable", args = 0)]
    pub fn is_investment_enable(self) -> bool;

    #[method(name = "GetInvestmentTotal", args = 0)]
    pub fn get_investment_total(self) -> i32;

    #[method(name = "AddInvestmentTotal", args = 1)]
    pub fn add_investment_total(self, add: i32) -> ();

    #[method(name = "GetLevelAverage", args = 0)]
    pub fn get_level_average(self) -> i32;

    #[method(name = "GetLevelData", args = 1)]
    pub fn get_level_data(self, level: i32) -> crate::app::hubinvestmentlevel::HubInvestmentLevel;

    #[method(name = "GetCurrentLevelData", args = 0)]
    pub fn get_current_level_data(self) -> crate::app::hubinvestmentlevel::HubInvestmentLevel;

    #[method(name = "GetCurrentLevelData", args = 1)]
    pub fn get_current_level_data_2(
        self,
        total_use_cost: i32,
    ) -> crate::app::hubinvestmentlevel::HubInvestmentLevel;

    #[method(name = "GetNextLevelData", args = 0)]
    pub fn get_next_level_data(self) -> crate::app::hubinvestmentlevel::HubInvestmentLevel;

    #[method(name = "GetNextLevelData", args = 1)]
    pub fn get_next_level_data_2(
        self,
        total_use_cost: i32,
    ) -> crate::app::hubinvestmentlevel::HubInvestmentLevel;

    #[method(name = "GetNextCost", args = 0)]
    pub fn get_next_cost(self) -> i32;

    #[method(name = "GetNextCost", args = 1)]
    pub fn get_next_cost_2(self, total_use_cost: i32) -> i32;

    #[method(name = "GetCurrentLevel", args = 0)]
    pub fn get_current_level(self) -> i32;

    #[method(name = "GetCurrentLevel", args = 1)]
    pub fn get_current_level_2(self, total_use_cost: i32) -> i32;

    #[method(name = "IsLevelMax", args = 0)]
    pub fn is_level_max(self) -> bool;

    #[method(name = "IsLevelMax", args = 1)]
    pub fn is_level_max_2(self, total_use_cost: i32) -> bool;

    #[method(name = "GetFoodstuffNum", args = 1)]
    pub fn get_foodstuff_num(self, iid: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetAnimalAppearRate", args = 1)]
    pub fn get_animal_appear_rate(self, anid: ::unity2::Il2CppString) -> u8;

    #[method(name = "IsCaptureAnimal", args = 1)]
    pub fn is_capture_animal(self, anid: ::unity2::Il2CppString) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubnationdata")]
impl HubNationData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubNationData),
                ::core::stringify!(new),
            )
        });
        <Self as IHubNationDataMethods>::ctor(this);
        this
    }
}
