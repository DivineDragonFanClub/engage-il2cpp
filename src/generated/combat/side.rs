
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/side/Side.md")))]
#[::unity2::class(namespace = "Combat", name = "Side")]
#[parent(crate::system::object::Object)]
pub struct Side {
    #[static_field]
    #[rename(name = "Invalid")]
    pub invalid: i32,
    #[static_field]
    #[rename(name = "Player")]
    pub player: i32,
    #[static_field]
    #[rename(name = "Enemy")]
    pub enemy: i32,
    #[static_field]
    #[rename(name = "PGrandew")]
    pub p_grandew: i32,
    #[static_field]
    #[rename(name = "EGrandew")]
    pub e_grandew: i32,
    #[static_field]
    #[rename(name = "PChainAtk")]
    pub p_chain_atk: i32,
    #[static_field]
    #[rename(name = "EChainAtk")]
    pub e_chain_atk: i32,
    #[static_field]
    #[rename(name = "PChainGrd1")]
    pub p_chain_grd1: i32,
    #[static_field]
    #[rename(name = "EChainGrd1")]
    pub e_chain_grd1: i32,
    #[static_field]
    #[rename(name = "PChainGrd2")]
    pub p_chain_grd2: i32,
    #[static_field]
    #[rename(name = "EChainGrd2")]
    pub e_chain_grd2: i32,
    #[static_field]
    #[rename(name = "PChainGrd3")]
    pub p_chain_grd3: i32,
    #[static_field]
    #[rename(name = "EChainGrd3")]
    pub e_chain_grd3: i32,
    #[static_field]
    #[rename(name = "PChainGrd4")]
    pub p_chain_grd4: i32,
    #[static_field]
    #[rename(name = "EChainGrd4")]
    pub e_chain_grd4: i32,
    #[static_field]
    #[rename(name = "Max")]
    pub max: i32,
    #[static_field]
    #[rename(name = "Regular")]
    pub regular: i32,
}

#[cfg(feature = "combat-side")]
#[::unity2::methods]
impl Side {
    #[method(name = "ToName", args = 1)]
    pub fn to_name(i: i32) -> ::unity2::Il2CppString;

    #[method(name = "FromName", args = 1)]
    pub fn from_name(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "IsMaster", args = 1)]
    pub fn is_master(i: i32) -> bool;

    #[method(name = "IsPairedGrandew", args = 1)]
    pub fn is_paired_grandew(i: i32) -> bool;

    #[method(name = "IsChain", args = 1)]
    pub fn is_chain(i: i32) -> bool;

    #[method(name = "IsChainAtk", args = 1)]
    pub fn is_chain_atk(i: i32) -> bool;

    #[method(name = "IsChainGrd", args = 1)]
    pub fn is_chain_grd(i: i32) -> bool;

    #[method(name = "IsPlayerSide", args = 1)]
    pub fn is_player_side(i: i32) -> bool;

    #[method(name = "IsEnemySide", args = 1)]
    pub fn is_enemy_side(i: i32) -> bool;

    #[method(name = "ToEnemy", args = 1)]
    pub fn to_enemy(i: i32) -> i32;

    #[method(name = "ToEnemyGrandew", args = 1)]
    pub fn to_enemy_grandew(i: i32) -> i32;

    #[method(name = "ToGrandew", args = 1)]
    pub fn to_grandew(i: i32) -> i32;

    #[method(name = "ToMaster", args = 1)]
    pub fn to_master(i: i32) -> i32;

    #[method(name = "ToPartner", args = 1)]
    pub fn to_partner(i: i32) -> i32;

    #[method(name = "ToMirrorSide", args = 1)]
    pub fn to_mirror_side(i: i32) -> i32;

    #[method(name = "ToEnemyChr", args = 1)]
    pub fn to_enemy_chr(i: i32) -> crate::combat::character::Character;

    #[method(name = "ToGrandewChr", args = 1)]
    pub fn to_grandew_chr(i: i32) -> crate::combat::character::Character;

    #[method(name = "ToMasterChr", args = 1)]
    pub fn to_master_chr(i: i32) -> crate::combat::character::Character;

    #[method(name = "ToPartnerChr", args = 1)]
    pub fn to_partner_chr(i: i32) -> crate::combat::character::Character;

    #[method(name = "ToEnemyGrandewChr", args = 1)]
    pub fn to_enemy_grandew_chr(i: i32) -> crate::combat::character::Character;

    #[method(name = "ConvertFrom", args = 2)]
    pub fn convert_from(
        side_type: crate::app::battleside::BattleSide_Type,
        is_reversed: bool,
    ) -> i32;
}
