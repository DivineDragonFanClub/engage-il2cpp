
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopcore/ShopCore.md")))]
#[::unity2::class(namespace = "App", name = "ShopCore")]
#[parent(crate::system::object::Object)]
pub struct ShopCore {}

#[cfg(feature = "app-shopcore")]
#[::unity2::methods]
impl ShopCore {
    #[method(name = "BuyOnWeaponShop", args = 2)]
    pub fn buy_on_weapon_shop(
        unit: crate::app::unit::Unit,
        item_data: crate::app::itemdata::ItemData,
    ) -> bool;

    #[method(name = "BuyOnItemShop", args = 2)]
    pub fn buy_on_item_shop(
        unit: crate::app::unit::Unit,
        item_data: crate::app::itemdata::ItemData,
    ) -> bool;

    #[method(name = "BuyOnFleaMarket", args = 2)]
    pub fn buy_on_flea_market(
        unit: crate::app::unit::Unit,
        item_data: crate::app::itemdata::ItemData,
    ) -> bool;

    #[method(name = "BuyOnAccessoryShop", args = 1)]
    pub fn buy_on_accessory_shop(accessory_data: crate::app::accessorydata::AccessoryData) -> ();

    #[method(name = "Sell", args = 3)]
    pub fn sell(unit: crate::app::unit::Unit, item_index: i32, closeup: bool) -> ();

    #[method(name = "Refine", args = 3)]
    pub fn refine(
        unit: crate::app::unit::Unit,
        item_index: i32,
        refine_level: i32,
    ) -> crate::app::unititem::UnitItem;

    #[method(name = "GetNeededIronToRefine", args = 2)]
    pub fn get_needed_iron_to_refine(
        unit_item: crate::app::unititem::UnitItem,
        new_refine_level: i32,
    ) -> i32;

    #[method(name = "GetNeededSteelToRefine", args = 2)]
    pub fn get_needed_steel_to_refine(
        unit_item: crate::app::unititem::UnitItem,
        new_refine_level: i32,
    ) -> i32;

    #[method(name = "GetNeededSilverToRefine", args = 2)]
    pub fn get_needed_silver_to_refine(
        unit_item: crate::app::unititem::UnitItem,
        new_refine_level: i32,
    ) -> i32;

    #[method(name = "GetNeededMoneyToRefine", args = 2)]
    pub fn get_needed_money_to_refine(
        unit_item: crate::app::unititem::UnitItem,
        new_refine_level: i32,
    ) -> i32;

    #[method(name = "Evolve", args = 3)]
    pub fn evolve(
        unit: crate::app::unit::Unit,
        item_index: i32,
        evolve_data_index: i32,
    ) -> crate::app::unititem::UnitItem;

    #[method(name = "Engrave", args = 3)]
    pub fn engrave(
        unit: crate::app::unit::Unit,
        item_index: i32,
        god_data: crate::app::goddata::GodData,
    ) -> crate::app::unititem::UnitItem;

    #[method(name = "GetEngraveCost", args = 1)]
    pub fn get_engrave_cost(item_data: crate::app::itemdata::ItemData) -> i32;

    #[method(name = "Exchange", args = 3)]
    pub fn exchange(
        source_material_data: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
        target_material_data: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
        source_material_count: i32,
    ) -> i32;

    #[method(name = "GetUnitItemEmptyCount", args = 1)]
    pub fn get_unit_item_empty_count(unit: crate::app::unit::Unit) -> i32;

    #[method(name = "GetUnitItem", args = 2)]
    pub fn get_unit_item(
        unit: crate::app::unit::Unit,
        item_index: i32,
    ) -> crate::app::unititem::UnitItem;

    #[method(name = "IsPriceDown", args = 0)]
    pub fn is_price_down() -> bool;

    #[method(name = "GetPrice", args = 2)]
    pub fn get_price(item_data: crate::app::itemdata::ItemData, is_discountable_shop: bool) -> i32;

    #[method(name = "CanAdd", args = 2)]
    pub fn can_add(unit: crate::app::unit::Unit, item_data: crate::app::itemdata::ItemData)
        -> bool;

    #[method(name = "CalcAdd", args = 2)]
    pub fn calc_add(
        unit: crate::app::unit::Unit,
        item_data: crate::app::itemdata::ItemData,
    ) -> crate::app::shopcore::ShopCore_Result;

    #[method(name = "IsInventoryMax", args = 1)]
    pub fn is_inventory_max(item_data: crate::app::itemdata::ItemData) -> bool;

    #[method(name = "AddItem", args = 2)]
    pub fn add_item(
        unit: crate::app::unit::Unit,
        item_data: crate::app::itemdata::ItemData,
    ) -> crate::app::shopcore::ShopCore_Result;

    #[method(name = "GetRefineMaterialCount", args = 1)]
    pub fn get_refine_material_count(
        material_data: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
    ) -> i32;

    #[method(name = "GetRefineMaterialMax", args = 1)]
    pub fn get_refine_material_max(
        material_data: crate::app::itemrefineexchangedata::ItemRefineExchangeData,
    ) -> i32;

    #[method(name = "AddAchievementOnBuy", args = 2)]
    pub fn add_achievement_on_buy(item_data: crate::app::itemdata::ItemData, count: i32) -> ();

    #[method(name = "AddAchievementOnSell", args = 2)]
    pub fn add_achievement_on_sell(item_data: crate::app::itemdata::ItemData, count: i32) -> ();

    #[method(name = "AddAchievementOnRefine", args = 2)]
    pub fn add_achievement_on_refine(item_data: crate::app::itemdata::ItemData, count: i32) -> ();

    #[method(name = "AddAchievementOnBuyAccessory", args = 0)]
    pub fn add_achievement_on_buy_accessory() -> ();

    #[method(name = "GetUnknownName", args = 1)]
    pub fn get_unknown_name(unititem: crate::app::unititem::UnitItem) -> ::unity2::Il2CppString;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-shopcore")]
impl ShopCore {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShopCore),
                ::core::stringify!(new),
            )
        });
        <Self as IShopCoreMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/shopcore/ShopCore_Result.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ShopCore_Result {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ShopCore_Result {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ShopCore.Result";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ShopCore_Result {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ShopCore_Result {
    pub fn failed() -> Self {
        Self { value: 0 }
    }

    pub fn unit() -> Self {
        Self { value: 1 }
    }

    pub fn transporter() -> Self {
        Self { value: 2 }
    }

    pub fn inventory() -> Self {
        Self { value: 3 }
    }
}
