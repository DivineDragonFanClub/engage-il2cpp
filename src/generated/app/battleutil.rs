
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battleutil/BattleUtil.md")))]
#[::unity2::class(namespace = "App", name = "BattleUtil")]
#[parent(crate::system::object::Object)]
pub struct BattleUtil {}

#[cfg(feature = "app-battleutil")]
#[::unity2::methods]
impl BattleUtil {
    #[method(name = "GetDamageLevel", args = 3)]
    pub fn get_damage_level(
        damage: i32,
        critical: bool,
        efficacy: bool,
    ) -> crate::app::damagelevel::DamageLevel;

    #[method(name = "GetDamageLevel", args = 2)]
    pub fn get_damage_level_2(
        scene: crate::app::battlescene::BattleScene,
        side: crate::app::battleside::BattleSide_Type,
    ) -> crate::app::damagelevel::DamageLevel;

    #[method(name = "CanGainExp", args = 1)]
    pub fn can_gain_exp(current: crate::app::battleinfoside::BattleInfoSide) -> bool;

    #[method(name = "CanGainExp", args = 2)]
    pub fn can_gain_exp_2(
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
    ) -> bool;

    #[method(name = "CanRodExp", args = 1)]
    pub fn can_rod_exp(current: crate::app::battleinfoside::BattleInfoSide) -> bool;

    #[method(name = "GetBattleExp", args = 3)]
    pub fn get_battle_exp(
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
        command: ::unity2::Il2CppString,
    ) -> i32;

    #[method(name = "GetRodExp", args = 1)]
    pub fn get_rod_exp(current: crate::app::battleinfoside::BattleInfoSide) -> i32;

    #[method(name = "GetDanceExp", args = 1)]
    pub fn get_dance_exp(current: crate::app::battleinfoside::BattleInfoSide) -> i32;

    #[method(name = "GetSummonExp", args = 1)]
    pub fn get_summon_exp(current: crate::app::battleinfoside::BattleInfoSide) -> i32;

    #[method(name = "GetGuardExp", args = 1)]
    pub fn get_guard_exp(current: crate::app::battleinfoside::BattleInfoSide) -> i32;

    #[method(name = "GetBattleExp", args = 2)]
    pub fn get_battle_exp_2(
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
    ) -> i32;

    #[method(name = "GetEnchantExp", args = 2)]
    pub fn get_enchant_exp(unit: crate::app::unit::Unit, target: crate::app::unit::Unit) -> i32;

    #[method(name = "IsAmbush", args = 2)]
    pub fn is_ambush(
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
    ) -> bool;

    #[method(name = "IsIncessantAttack", args = 2)]
    pub fn is_incessant_attack(
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
    ) -> bool;

    #[method(name = "IsSwapOrder", args = 2)]
    pub fn is_swap_order(
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
    ) -> bool;

    #[method(name = "HasPlayerUnit", args = 1)]
    pub fn has_player_unit(info: crate::app::battleinfo::BattleInfo) -> bool;

    #[method(name = "IsDetailBattle", args = 1)]
    pub fn is_detail_battle(info: crate::app::battleinfo::BattleInfo) -> bool;

    #[method(name = "IsGrowTalk", args = 2)]
    pub fn is_grow_talk(
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
    ) -> bool;

    #[method(name = "CanGainSituation", args = 0)]
    pub fn can_gain_situation() -> bool;

    #[method(name = "PlayLastBattleDieSound", args = 0)]
    pub fn play_last_battle_die_sound() -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-battleutil")]
impl BattleUtil {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleUtil),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleUtilMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battleutil/BattleUtil_CalcScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct BattleUtil_CalcScope {
    pub info: crate::app::battleinfo::BattleInfo,
    pub calc: crate::app::battlecalculator::BattleCalculator,
}

impl ::unity2::ClassIdentity for BattleUtil_CalcScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BattleUtil.CalcScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BattleUtil_CalcScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-battleutil")]
#[::unity2::methods(value)]
impl BattleUtil_CalcScope {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        unit: crate::app::unit::Unit,
        flags: crate::app::battleinfo::BattleInfo_Flags,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "GetSide", args = 1)]
    pub fn get_side(
        self,
        side: crate::app::battleside::BattleSide_Type,
    ) -> crate::app::battleinfoside::BattleInfoSide;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}
