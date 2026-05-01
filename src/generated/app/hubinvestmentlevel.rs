
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubinvestmentlevel/HubInvestmentLevel.md")))]
#[::unity2::class(namespace = "App", name = "HubInvestmentLevel")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: hubinvestmentlevel :: HubInvestmentLevel >)]
pub struct HubInvestmentLevel {}

#[cfg(feature = "app-hubinvestmentlevel")]
#[::unity2::methods]
impl HubInvestmentLevel {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Cost", args = 0)]
    pub fn get_cost(self) -> i32;

    #[method(name = "set_Cost", args = 1)]
    pub fn set_cost(self, value: i32) -> ();

    #[method(name = "get_BonusName", args = 0)]
    pub fn get_bonus_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BonusName", args = 1)]
    pub fn set_bonus_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_BonusItem", args = 0)]
    pub fn get_bonus_item(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BonusItem", args = 1)]
    pub fn set_bonus_item(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_BonusFood", args = 0)]
    pub fn get_bonus_food(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BonusFood", args = 1)]
    pub fn set_bonus_food(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_BonusAnimal", args = 0)]
    pub fn get_bonus_animal(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BonusAnimal", args = 1)]
    pub fn set_bonus_animal(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_BonusAccessoryAid", args = 0)]
    pub fn get_bonus_accessory_aid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_BonusAccessoryAid", args = 1)]
    pub fn set_bonus_accessory_aid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_BonusIron", args = 0)]
    pub fn get_bonus_iron(self) -> i32;

    #[method(name = "set_BonusIron", args = 1)]
    pub fn set_bonus_iron(self, value: i32) -> ();

    #[method(name = "get_BonusSteel", args = 0)]
    pub fn get_bonus_steel(self) -> i32;

    #[method(name = "set_BonusSteel", args = 1)]
    pub fn set_bonus_steel(self, value: i32) -> ();

    #[method(name = "get_BonusSilver", args = 0)]
    pub fn get_bonus_silver(self) -> i32;

    #[method(name = "set_BonusSilver", args = 1)]
    pub fn set_bonus_silver(self, value: i32) -> ();

    #[method(name = "get_BonusPieceOfBond", args = 0)]
    pub fn get_bonus_piece_of_bond(self) -> i32;

    #[method(name = "set_BonusPieceOfBond", args = 1)]
    pub fn set_bonus_piece_of_bond(self, value: i32) -> ();

    #[method(name = "get_GoldEnemyRate", args = 0)]
    pub fn get_gold_enemy_rate(self) -> i8;

    #[method(name = "set_GoldEnemyRate", args = 1)]
    pub fn set_gold_enemy_rate(self, value: i8) -> ();

    #[method(name = "get_ExpEnemyRate", args = 0)]
    pub fn get_exp_enemy_rate(self) -> i8;

    #[method(name = "set_ExpEnemyRate", args = 1)]
    pub fn set_exp_enemy_rate(self, value: i8) -> ();

    #[method(name = "get_Iron", args = 0)]
    pub fn get_iron(self) -> u8;

    #[method(name = "set_Iron", args = 1)]
    pub fn set_iron(self, value: u8) -> ();

    #[method(name = "get_Steel", args = 0)]
    pub fn get_steel(self) -> u8;

    #[method(name = "set_Steel", args = 1)]
    pub fn set_steel(self, value: u8) -> ();

    #[method(name = "get_Silver", args = 0)]
    pub fn get_silver(self) -> u8;

    #[method(name = "set_Silver", args = 1)]
    pub fn set_silver(self, value: u8) -> ();

    #[method(name = "get_PieceOfBond", args = 0)]
    pub fn get_piece_of_bond(self) -> u8;

    #[method(name = "set_PieceOfBond", args = 1)]
    pub fn set_piece_of_bond(self, value: u8) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubinvestmentlevel")]
impl HubInvestmentLevel {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubInvestmentLevel),
                ::core::stringify!(new),
            )
        });
        <Self as IHubInvestmentLevelMethods>::ctor(this);
        this
    }
}
