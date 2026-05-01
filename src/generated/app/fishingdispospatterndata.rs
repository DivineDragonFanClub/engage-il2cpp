
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/fishingdispospatterndata/FishingDisposPatternData.md")))]
#[::unity2::class(namespace = "App", name = "FishingDisposPatternData")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: fishingdispospatterndata :: FishingDisposPatternData >)]
pub struct FishingDisposPatternData {}

#[cfg(feature = "app-fishingdispospatterndata")]
#[::unity2::methods]
impl FishingDisposPatternData {
    #[method(name = "get_ID", args = 0)]
    pub fn get_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ID", args = 1)]
    pub fn set_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_StickType", args = 0)]
    pub fn get_stick_type(self) -> ::unity2::Il2CppString;

    #[method(name = "set_StickType", args = 1)]
    pub fn set_stick_type(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Time", args = 0)]
    pub fn get_time(self) -> i32;

    #[method(name = "set_Time", args = 1)]
    pub fn set_time(self, value: i32) -> ();

    #[method(name = "get_PositionNum", args = 0)]
    pub fn get_position_num(self) -> i32;

    #[method(name = "set_PositionNum", args = 1)]
    pub fn set_position_num(self, value: i32) -> ();

    #[method(name = "get_LotteryParam", args = 0)]
    pub fn get_lottery_param(self) -> i32;

    #[method(name = "set_LotteryParam", args = 1)]
    pub fn set_lottery_param(self, value: i32) -> ();

    #[method(name = "get_FishID", args = 0)]
    pub fn get_fish_id(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FishID", args = 1)]
    pub fn set_fish_id(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetForecastList", args = 3)]
    pub fn get_forecast_list(
        r#type: crate::app::fishing::sticktype::StickType,
        time: i32,
        out_list: crate::system::collections::generic::list_1::List_1<
            crate::app::fishing::forecastfishdata::ForecastFishData,
        >,
    ) -> ();

    #[method(name = "GetForecastListBestSix", args = 3)]
    pub fn get_forecast_list_best_six(
        r#type: crate::app::fishing::sticktype::StickType,
        time: i32,
        out_list: crate::system::collections::generic::list_1::List_1<
            crate::app::fishing::forecastfishdata::ForecastFishData,
        >,
    ) -> ();

    #[method(name = "GetForecastListByPosition", args = 4)]
    pub fn get_forecast_list_by_position(
        r#type: crate::app::fishing::sticktype::StickType,
        time: i32,
        pos_num: i32,
        out_list: crate::system::collections::generic::list_1::List_1<
            crate::app::fishing::forecastfishdata::ForecastFishData,
        >,
    ) -> ();

    #[method(name = "GetRipplesList", args = 4)]
    pub fn get_ripples_list(
        r#type: crate::app::fishing::sticktype::StickType,
        time: i32,
        pos_num: i32,
        out_list: crate::system::collections::generic::list_1::List_1<i32>,
    ) -> ();
}

#[cfg(feature = "app-fishingdispospatterndata")]
impl FishingDisposPatternData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FishingDisposPatternData),
                ::core::stringify!(new),
            )
        });
        <Self as IFishingDisposPatternDataMethods>::ctor(this);
        this
    }
}
