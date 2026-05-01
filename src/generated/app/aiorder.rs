
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aiorder/AIOrder.md")))]
#[::unity2::class(namespace = "App", name = "AIOrder")]
#[parent(crate::system::object::Object)]
pub struct AIOrder {
    #[static_field]
    #[rename(name = "aFunc")]
    pub a_func:
        crate::system::collections::object_model::readonlycollection_1::ReadOnlyCollection_1<
            crate::app::aiorder::AIOrder_Func,
        >,
    #[static_field]
    #[rename(name = "aFuncEntrust")]
    pub a_func_entrust:
        crate::system::collections::object_model::readonlycollection_1::ReadOnlyCollection_1<
            crate::app::aiorder::AIOrder_Func,
        >,
    #[static_field]
    #[rename(name = "aFuncUncontroll")]
    pub a_func_uncontroll:
        crate::system::collections::object_model::readonlycollection_1::ReadOnlyCollection_1<
            crate::app::aiorder::AIOrder_Func,
        >,
    #[static_field]
    #[rename(name = "UnitMax")]
    pub unit_max: i32,
    #[rename(name = "m_aUnitPriority")]
    pub m_a_unit_priority: ::unity2::Array<crate::app::aiorder::AIOrder_UnitPriority>,
    #[rename(name = "m_Num")]
    pub m_num: i32,
    #[rename(name = "m_Seq")]
    pub m_seq: i32,
    #[rename(name = "m_Current")]
    pub m_current: i32,
    #[rename(name = "m_EngageOrGodChange")]
    pub m_engage_or_god_change: i32,
    #[rename(name = "m_Remove")]
    pub m_remove: i32,
    #[rename(name = "m_Remagic")]
    pub m_remagic: i32,
    #[rename(name = "m_Rerewarp")]
    pub m_rerewarp: i32,
    #[rename(name = "m_MoveOver")]
    pub m_move_over: i32,
    #[rename(name = "m_IsAllowIdle")]
    pub m_is_allow_idle: bool,
}

#[cfg(feature = "app-aiorder")]
#[::unity2::methods]
impl AIOrder {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "SetEngageOrGodChange", args = 1)]
    pub fn set_engage_or_god_change(self, unit_index: i32) -> ();

    #[method(name = "IsEngageOrGodChange", args = 0)]
    pub fn is_engage_or_god_change(self) -> bool;

    #[method(name = "SetRemove", args = 1)]
    pub fn set_remove(self, unit_index: i32) -> ();

    #[method(name = "IsRemove", args = 0)]
    pub fn is_remove(self) -> bool;

    #[method(name = "SetRemagic", args = 1)]
    pub fn set_remagic(self, unit_index: i32) -> ();

    #[method(name = "IsRemagic", args = 0)]
    pub fn is_remagic(self) -> bool;

    #[method(name = "SetRerewarp", args = 1)]
    pub fn set_rerewarp(self, unit_index: i32) -> ();

    #[method(name = "IsRerewarp", args = 0)]
    pub fn is_rerewarp(self) -> bool;

    #[method(name = "SetMoveOver", args = 1)]
    pub fn set_move_over(self, unit_index: i32) -> ();

    #[method(name = "IsMoveOver", args = 0)]
    pub fn is_move_over(self) -> bool;

    #[method(name = "IsAllowIdle", args = 0)]
    pub fn is_allow_idle(self) -> bool;

    #[method(name = "Processing", args = 0)]
    pub fn processing(self) -> ();

    #[method(name = "GetUnit", args = 0)]
    pub fn get_unit(self) -> crate::app::unit::Unit;

    #[method(name = "Next", args = 0)]
    pub fn next(self) -> ();

    #[method(name = "StaticUpdateTarget", args = 1)]
    pub fn static_update_target(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticAllowIdle", args = 1)]
    pub fn static_allow_idle(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticPriority", args = 1)]
    pub fn static_priority(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticCause", args = 1)]
    pub fn static_cause(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticMind", args = 1)]
    pub fn static_mind(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticAttackCrossfire", args = 1)]
    pub fn static_attack_crossfire(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticAttackLongRange", args = 1)]
    pub fn static_attack_long_range(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticAttackHigh", args = 1)]
    pub fn static_attack_high(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticAttackMiddle", args = 1)]
    pub fn static_attack_middle(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticAttackLow", args = 1)]
    pub fn static_attack_low(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticMove", args = 1)]
    pub fn static_move(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticTurnEnd", args = 1)]
    pub fn static_turn_end(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticEntrustUpdateIdle", args = 1)]
    pub fn static_entrust_update_idle(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticEntrustHeal", args = 1)]
    pub fn static_entrust_heal(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticEntrustUpdateCannon", args = 1)]
    pub fn static_entrust_update_cannon(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticEntrustAttack", args = 1)]
    pub fn static_entrust_attack(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticEntrustHeroRushMove", args = 1)]
    pub fn static_entrust_hero_rush_move(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticEntrustMove", args = 1)]
    pub fn static_entrust_move(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticEntrustFixed", args = 1)]
    pub fn static_entrust_fixed(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticUncontrollUpdateIdle", args = 1)]
    pub fn static_uncontroll_update_idle(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticUncontrollHeal", args = 1)]
    pub fn static_uncontroll_heal(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticUncontrollAttack", args = 1)]
    pub fn static_uncontroll_attack(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticUncontrollMove", args = 1)]
    pub fn static_uncontroll_move(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "StaticUncontrollFixed", args = 1)]
    pub fn static_uncontroll_fixed(order: crate::app::aiorder::AIOrder) -> ();

    #[method(name = "GetDistanceFromEnemy", args = 1)]
    pub fn get_distance_from_enemy(self, actor: crate::app::unit::Unit) -> i32;

    #[method(name = "HasLongRangeWeapon", args = 1)]
    pub fn has_long_range_weapon(self, unit: crate::app::unit::Unit) -> bool;

    #[method(name = "GetEnchantPriority", args = 2)]
    pub fn get_enchant_priority(self, unit: crate::app::unit::Unit, is_entrust: bool) -> i32;

    #[method(name = "AllowIdle", args = 0)]
    pub fn allow_idle(self) -> ();

    #[method(name = "SortDescend", args = 0)]
    pub fn sort_descend(self) -> ();

    #[method(name = "SortAscend", args = 0)]
    pub fn sort_ascend(self) -> ();

    #[method(name = "EnumerateAll", args = 0)]
    pub fn enumerate_all(self) -> ();

    #[method(name = "EnumeratePriority", args = 0)]
    pub fn enumerate_priority(self) -> ();

    #[method(name = "EnumerateAttackLongRange", args = 0)]
    pub fn enumerate_attack_long_range(self) -> ();

    #[method(name = "EnumerateAttack", args = 0)]
    pub fn enumerate_attack(self) -> ();

    #[method(name = "EnumerateMove", args = 0)]
    pub fn enumerate_move(self) -> ();

    #[method(name = "EnumerateEntrust", args = 0)]
    pub fn enumerate_entrust(self) -> ();

    #[method(name = "EnumerateUncontroll", args = 0)]
    pub fn enumerate_uncontroll(self) -> ();

    #[method(name = "UpdateAskHeal", args = 0)]
    pub fn update_ask_heal(self) -> ();

    #[method(name = "UpdateTarget", args = 0)]
    pub fn update_target(self) -> ();

    #[method(name = "Priority", args = 0)]
    pub fn priority(self) -> ();

    #[method(name = "Cause", args = 0)]
    pub fn cause(self) -> ();

    #[method(name = "Mind", args = 0)]
    pub fn mind(self) -> ();

    #[method(name = "AttackCrossfire", args = 0)]
    pub fn attack_crossfire(self) -> ();

    #[method(name = "AttackLongRange", args = 0)]
    pub fn attack_long_range(self) -> ();

    #[method(name = "AttackHigh", args = 0)]
    pub fn attack_high(self) -> ();

    #[method(name = "AttackMiddle", args = 0)]
    pub fn attack_middle(self) -> ();

    #[method(name = "AttackLow", args = 0)]
    pub fn attack_low(self) -> ();

    #[method(name = "Move", args = 0)]
    pub fn r#move(self) -> ();

    #[method(name = "TurnEnd", args = 0)]
    pub fn turn_end(self) -> ();

    #[method(name = "EntrustUpdateIdle", args = 0)]
    pub fn entrust_update_idle(self) -> ();

    #[method(name = "EntrustHeal", args = 0)]
    pub fn entrust_heal(self) -> ();

    #[method(name = "EntrustUpdateCannon", args = 0)]
    pub fn entrust_update_cannon(self) -> ();

    #[method(name = "EntrustAttack", args = 0)]
    pub fn entrust_attack(self) -> ();

    #[method(name = "EntrustMove", args = 0)]
    pub fn entrust_move(self) -> ();

    #[method(name = "EntrustHeroRushMove", args = 0)]
    pub fn entrust_hero_rush_move(self) -> ();

    #[method(name = "EntrustFixed", args = 0)]
    pub fn entrust_fixed(self) -> ();

    #[method(name = "UncontrollUpdateIdle", args = 0)]
    pub fn uncontroll_update_idle(self) -> ();

    #[method(name = "UncontrollHeal", args = 0)]
    pub fn uncontroll_heal(self) -> ();

    #[method(name = "UncontrollAttack", args = 0)]
    pub fn uncontroll_attack(self) -> ();

    #[method(name = "UncontrollMove", args = 0)]
    pub fn uncontroll_move(self) -> ();

    #[method(name = "UncontrollFixed", args = 0)]
    pub fn uncontroll_fixed(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-aiorder")]
impl AIOrder {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AIOrder),
                ::core::stringify!(new),
            )
        });
        <Self as IAIOrderMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aiorder/AIOrder_Func.md")))]
#[::unity2::class(namespace = "App", name = "AIOrder.Func")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct AIOrder_Func {}

#[cfg(feature = "app-aiorder")]
#[::unity2::methods]
impl AIOrder_Func {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, order: crate::app::aiorder::AIOrder) -> ();
}

#[cfg(feature = "app-aiorder")]
impl AIOrder_Func {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AIOrder_Func),
                ::core::stringify!(new),
            )
        });
        <Self as IAIOrder_FuncMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/aiorder/AIOrder_UnitPriority.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct AIOrder_UnitPriority {
    pub number: i32,
    pub priority: u32,
}

impl ::unity2::ClassIdentity for AIOrder_UnitPriority {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AIOrder.UnitPriority";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AIOrder_UnitPriority {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-aiorder")]
#[::unity2::methods(value)]
impl AIOrder_UnitPriority {
    #[method(name = "op_LessThanOrEqual", args = 2)]
    pub fn op_less_than_or_equal(
        a: crate::app::aiorder::AIOrder_UnitPriority,
        b: crate::app::aiorder::AIOrder_UnitPriority,
    ) -> bool;

    #[method(name = "op_GreaterThanOrEqual", args = 2)]
    pub fn op_greater_than_or_equal(
        a: crate::app::aiorder::AIOrder_UnitPriority,
        b: crate::app::aiorder::AIOrder_UnitPriority,
    ) -> bool;
}
