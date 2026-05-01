
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingfishdata/FishingFishData.md")))]
#[::unity2::class(namespace = "App", name = "FishingFishData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: fishingfishdata :: FishingFishData >)]
pub struct FishingFishData {}

#[cfg(feature = "app-fishingfishdata")]
#[::unity2::methods]
impl FishingFishData {
    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_FishName", args = 0)]
    pub fn get_fish_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FishName", args = 1)]
    pub fn set_fish_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_LargeType", args = 0)]
    pub fn get_large_type(self) -> i32;

    #[method(name = "set_LargeType", args = 1)]
    pub fn set_large_type(self, value: i32) -> ();

    #[method(name = "get_ShadowSize", args = 0)]
    pub fn get_shadow_size(self) -> i32;

    #[method(name = "set_ShadowSize", args = 1)]
    pub fn set_shadow_size(self, value: i32) -> ();

    #[method(name = "get_RadarSizeMult", args = 0)]
    pub fn get_radar_size_mult(self) -> f32;

    #[method(name = "set_RadarSizeMult", args = 1)]
    pub fn set_radar_size_mult(self, value: f32) -> ();

    #[method(name = "get_FoodType", args = 0)]
    pub fn get_food_type(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FoodType", args = 1)]
    pub fn set_food_type(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PieceCount", args = 0)]
    pub fn get_piece_count(self) -> i32;

    #[method(name = "set_PieceCount", args = 1)]
    pub fn set_piece_count(self, value: i32) -> ();

    #[method(name = "get_CounterTime", args = 0)]
    pub fn get_counter_time(self) -> f32;

    #[method(name = "set_CounterTime", args = 1)]
    pub fn set_counter_time(self, value: f32) -> ();

    #[method(name = "get_TurnCounterTime", args = 0)]
    pub fn get_turn_counter_time(self) -> f32;

    #[method(name = "set_TurnCounterTime", args = 1)]
    pub fn set_turn_counter_time(self, value: f32) -> ();

    #[method(name = "get_TurnAngleMin", args = 0)]
    pub fn get_turn_angle_min(self) -> f32;

    #[method(name = "set_TurnAngleMin", args = 1)]
    pub fn set_turn_angle_min(self, value: f32) -> ();

    #[method(name = "get_TurnAngleMax", args = 0)]
    pub fn get_turn_angle_max(self) -> f32;

    #[method(name = "set_TurnAngleMax", args = 1)]
    pub fn set_turn_angle_max(self, value: f32) -> ();

    #[method(name = "get_EscapeSpeed", args = 0)]
    pub fn get_escape_speed(self) -> f32;

    #[method(name = "set_EscapeSpeed", args = 1)]
    pub fn set_escape_speed(self, value: f32) -> ();

    #[method(name = "get_CounterSpeedH", args = 0)]
    pub fn get_counter_speed_h(self) -> f32;

    #[method(name = "set_CounterSpeedH", args = 1)]
    pub fn set_counter_speed_h(self, value: f32) -> ();

    #[method(name = "get_CounterSpeedM", args = 0)]
    pub fn get_counter_speed_m(self) -> f32;

    #[method(name = "set_CounterSpeedM", args = 1)]
    pub fn set_counter_speed_m(self, value: f32) -> ();

    #[method(name = "get_CounterSpeedL", args = 0)]
    pub fn get_counter_speed_l(self) -> f32;

    #[method(name = "set_CounterSpeedL", args = 1)]
    pub fn set_counter_speed_l(self, value: f32) -> ();

    #[method(name = "get_CatchTime", args = 0)]
    pub fn get_catch_time(self) -> f32;

    #[method(name = "set_CatchTime", args = 1)]
    pub fn set_catch_time(self, value: f32) -> ();

    #[method(name = "get_CatchTimeRandomAdd", args = 0)]
    pub fn get_catch_time_random_add(self) -> f32;

    #[method(name = "set_CatchTimeRandomAdd", args = 1)]
    pub fn set_catch_time_random_add(self, value: f32) -> ();

    #[method(name = "get_EscapeTime", args = 0)]
    pub fn get_escape_time(self) -> f32;

    #[method(name = "set_EscapeTime", args = 1)]
    pub fn set_escape_time(self, value: f32) -> ();

    #[method(name = "get_HP", args = 0)]
    pub fn get_hp(self) -> f32;

    #[method(name = "set_HP", args = 1)]
    pub fn set_hp(self, value: f32) -> ();

    #[method(name = "get_LethalHP", args = 0)]
    pub fn get_lethal_hp(self) -> f32;

    #[method(name = "set_LethalHP", args = 1)]
    pub fn set_lethal_hp(self, value: f32) -> ();

    #[method(name = "get_RegenaratePerSec", args = 0)]
    pub fn get_regenarate_per_sec(self) -> f32;

    #[method(name = "set_RegenaratePerSec", args = 1)]
    pub fn set_regenarate_per_sec(self, value: f32) -> ();

    #[method(name = "get_BaseSize", args = 0)]
    pub fn get_base_size(self) -> f32;

    #[method(name = "set_BaseSize", args = 1)]
    pub fn set_base_size(self, value: f32) -> ();

    #[method(name = "get_NameLabel", args = 0)]
    pub fn get_name_label(self) -> ::unity2::Il2CppString;

    #[method(name = "set_NameLabel", args = 1)]
    pub fn set_name_label(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MessageLabel", args = 0)]
    pub fn get_message_label(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MessageLabel", args = 1)]
    pub fn set_message_label(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_TimeFlagMorning", args = 0)]
    pub fn get_time_flag_morning(self) -> bool;

    #[method(name = "set_TimeFlagMorning", args = 1)]
    pub fn set_time_flag_morning(self, value: bool) -> ();

    #[method(name = "get_TimeFlagDay", args = 0)]
    pub fn get_time_flag_day(self) -> bool;

    #[method(name = "set_TimeFlagDay", args = 1)]
    pub fn set_time_flag_day(self, value: bool) -> ();

    #[method(name = "get_TimeFlagNight", args = 0)]
    pub fn get_time_flag_night(self) -> bool;

    #[method(name = "set_TimeFlagNight", args = 1)]
    pub fn set_time_flag_night(self, value: bool) -> ();

    #[method(name = "get_BestRodType", args = 0)]
    pub fn get_best_rod_type(self) -> i32;

    #[method(name = "set_BestRodType", args = 1)]
    pub fn set_best_rod_type(self, value: i32) -> ();

    #[method(name = "get_TextureID", args = 0)]
    pub fn get_texture_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_TextureID", args = 1)]
    pub fn set_texture_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "RegistGlobalFlags", args = 0)]
    pub fn regist_global_flags() -> ();

    #[method(name = "GetFishedFishCount", args = 0)]
    pub fn get_fished_fish_count() -> i32;
}

#[cfg(feature = "app-fishingfishdata")]
impl FishingFishData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingFishData),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingFishDataMethods>::ctor(this);
        this
    }
}
