
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentrustscore/SortieEntrustScore_Vulnerary.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrustScore.Vulnerary")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrustScore_Vulnerary {}

#[cfg(feature = "app-sortieentrustscore")]
#[::unity2::methods]
impl SortieEntrustScore_Vulnerary {
    #[method(name = "IsExclude", args = 2)]
    pub fn is_exclude(
        unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData,
        stock_data: crate::app::sortieentruststockdata::SortieEntrustStockData,
    ) -> bool;

    #[method(name = "Calc", args = 3)]
    pub fn calc(
        result: crate::app::sortieentrustscore::SortieEntrustScore_Result,
        unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData,
        stock_data: crate::app::sortieentruststockdata::SortieEntrustStockData,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortieentrustscore")]
impl SortieEntrustScore_Vulnerary {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrustScore_Vulnerary),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrustScore_VulneraryMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentrustscore/SortieEntrustScore_Rod.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrustScore.Rod")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrustScore_Rod {}

#[cfg(feature = "app-sortieentrustscore")]
#[::unity2::methods]
impl SortieEntrustScore_Rod {
    #[method(name = "IsExclude", args = 2)]
    pub fn is_exclude(
        unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData,
        stock_data: crate::app::sortieentruststockdata::SortieEntrustStockData,
    ) -> bool;

    #[method(name = "Calc", args = 3)]
    pub fn calc(
        result: crate::app::sortieentrustscore::SortieEntrustScore_Result,
        unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData,
        stock_data: crate::app::sortieentruststockdata::SortieEntrustStockData,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortieentrustscore")]
impl SortieEntrustScore_Rod {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrustScore_Rod),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrustScore_RodMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentrustscore/SortieEntrustScore_EnhancePerson.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrustScore.EnhancePerson")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrustScore_EnhancePerson {}

#[cfg(feature = "app-sortieentrustscore")]
#[::unity2::methods]
impl SortieEntrustScore_EnhancePerson {
    #[method(name = "IsExclude", args = 2)]
    pub fn is_exclude(
        unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData,
        stock_data: crate::app::sortieentruststockdata::SortieEntrustStockData,
    ) -> bool;

    #[method(name = "Calc", args = 3)]
    pub fn calc(
        result: crate::app::sortieentrustscore::SortieEntrustScore_Result,
        unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData,
        stock_data: crate::app::sortieentruststockdata::SortieEntrustStockData,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortieentrustscore")]
impl SortieEntrustScore_EnhancePerson {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrustScore_EnhancePerson),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrustScore_EnhancePersonMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentrustscore/SortieEntrustScore.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrustScore")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrustScore {
    #[static_field]
    #[rename(name = "MaxRangeForRangeCovered")]
    pub max_range_for_range_covered: i32,
    #[rename(name = "m_StockData")]
    pub m_stock_data: crate::app::sortieentruststockdata::SortieEntrustStockData,
    #[rename(name = "m_Result")]
    pub m_result: crate::app::sortieentrustscore::SortieEntrustScore_Result,
}

#[cfg(feature = "app-sortieentrustscore")]
#[::unity2::methods]
impl SortieEntrustScore {
    #[method(name = "GetItemKindMask", args = 1)]
    pub fn get_item_kind_mask(
        unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData,
    ) -> u32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Initialize", args = 1)]
    pub fn initialize(
        self,
        stock_data: crate::app::sortieentruststockdata::SortieEntrustStockData,
    ) -> ();

    #[method(name = "Calc", args = 1)]
    pub fn calc(self, unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData) -> ();

    #[method(name = "SelectHigh", args = 1)]
    pub fn select_high(self, other: crate::app::sortieentrustscore::SortieEntrustScore) -> ();

    #[method(name = "get_StockData", args = 0)]
    pub fn get_stock_data(self) -> crate::app::sortieentruststockdata::SortieEntrustStockData;

    #[method(name = "get_IsValidScore", args = 0)]
    pub fn get_is_valid_score(self) -> bool;

    #[method(name = "get_Score", args = 0)]
    pub fn get_score(self) -> i32;

    #[method(name = "IsExclude", args = 1)]
    pub fn is_exclude(
        self,
        unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData,
    ) -> bool;

    #[method(name = "IsRangeNormal", args = 1)]
    pub fn is_range_normal(item_data: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "IsRangeSpecial", args = 1)]
    pub fn is_range_special(item_data: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "IsRangeCovered", args = 2)]
    pub fn is_range_covered(
        unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData,
        stock_item_data: crate::app::itemdata::ItemData,
    ) -> bool;

    #[method(name = "IsPersonalUse", args = 1)]
    pub fn is_personal_use(item_data: crate::app::itemdata::ItemData) -> bool;
}

#[cfg(feature = "app-sortieentrustscore")]
impl SortieEntrustScore {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrustScore),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrustScoreMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentrustscore/SortieEntrustScore_Weapon.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrustScore.Weapon")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrustScore_Weapon {
    #[static_field]
    #[rename(name = "PowerFactor")]
    pub power_factor: i32,
    #[static_field]
    #[rename(name = "HitLow")]
    pub hit_low: i32,
    #[static_field]
    #[rename(name = "HitHigh")]
    pub hit_high: i32,
}

#[cfg(feature = "app-sortieentrustscore")]
#[::unity2::methods]
impl SortieEntrustScore_Weapon {
    #[method(name = "IsExclude", args = 2)]
    pub fn is_exclude(
        unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData,
        stock_data: crate::app::sortieentruststockdata::SortieEntrustStockData,
    ) -> bool;

    #[method(name = "Calc", args = 3)]
    pub fn calc(
        result: crate::app::sortieentrustscore::SortieEntrustScore_Result,
        unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData,
        stock_data: crate::app::sortieentruststockdata::SortieEntrustStockData,
    ) -> ();

    #[method(name = "GetEfficacyPowerOffset", args = 2)]
    pub fn get_efficacy_power_offset(
        stock_unit_item: crate::app::unititem::UnitItem,
        stock_item_data: crate::app::itemdata::ItemData,
    ) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortieentrustscore")]
impl SortieEntrustScore_Weapon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrustScore_Weapon),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrustScore_WeaponMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentrustscore/SortieEntrustScore_Enhance.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrustScore.Enhance")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrustScore_Enhance {}

#[cfg(feature = "app-sortieentrustscore")]
#[::unity2::methods]
impl SortieEntrustScore_Enhance {
    #[method(name = "IsExclude", args = 2)]
    pub fn is_exclude(
        unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData,
        stock_data: crate::app::sortieentruststockdata::SortieEntrustStockData,
    ) -> bool;

    #[method(name = "Calc", args = 3)]
    pub fn calc(
        result: crate::app::sortieentrustscore::SortieEntrustScore_Result,
        unit_data: crate::app::sortieentrustunitdata::SortieEntrustUnitData,
        stock_data: crate::app::sortieentruststockdata::SortieEntrustStockData,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortieentrustscore")]
impl SortieEntrustScore_Enhance {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrustScore_Enhance),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrustScore_EnhanceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortieentrustscore/SortieEntrustScore_Result.md")))]
#[::unity2::class(namespace = "App", name = "SortieEntrustScore.Result")]
#[parent(crate::system::object::Object)]
pub struct SortieEntrustScore_Result {
    #[rename(name = "m_IsValid")]
    pub m_is_valid: bool,
    #[rename(name = "m_Score")]
    pub m_score: i32,
}

#[cfg(feature = "app-sortieentrustscore")]
#[::unity2::methods]
impl SortieEntrustScore_Result {
    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "Set", args = 1)]
    pub fn set(self, score: i32) -> ();

    #[method(name = "get_IsValid", args = 0)]
    pub fn get_is_valid(self) -> bool;

    #[method(name = "get_Score", args = 0)]
    pub fn get_score(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortieentrustscore")]
impl SortieEntrustScore_Result {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieEntrustScore_Result),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieEntrustScore_ResultMethods>::ctor(this);
        this
    }
}
