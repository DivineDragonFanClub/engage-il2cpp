
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
use crate::app::pool::IPool_List_1;
use crate::app::pool::IPool_Node;
use crate::app::pool::Pool_List_1;
use crate::app::pool::Pool_Node;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator_HitSkill.md")))]
#[::unity2::class(namespace = "App", name = "BattleCalculator.HitSkill")]
#[parent(crate::app::pool::Pool_Node)]
pub struct BattleCalculator_HitSkill {
    #[rename(name = "Side")]
    pub side: crate::app::battleinfoside::BattleInfoSide,
    #[rename(name = "Action")]
    pub action: crate::app::skilldata::SkillData_Actions,
    #[rename(name = "Skill")]
    pub skill: crate::app::skilldata::SkillData,
}

#[cfg(feature = "app-battlecalculator")]
#[::unity2::methods]
impl BattleCalculator_HitSkill {
    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnExit", args = 0)]
    pub fn on_exit(self) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "get_SortKey", args = 0)]
    pub fn get_sort_key(self) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-battlecalculator")]
impl BattleCalculator_HitSkill {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleCalculator_HitSkill),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleCalculator_HitSkillMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator_FuncExp1.md")))]
#[::unity2::class(namespace = "App", name = "BattleCalculator.FuncExp1")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct BattleCalculator_FuncExp1 {}

#[cfg(feature = "app-battlecalculator")]
#[::unity2::methods]
impl BattleCalculator_FuncExp1 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(self, current: crate::app::battleinfoside::BattleInfoSide) -> i32;
}

#[cfg(feature = "app-battlecalculator")]
impl BattleCalculator_FuncExp1 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleCalculator_FuncExp1),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleCalculator_FuncExp1Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator_Attributes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct BattleCalculator_Attributes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for BattleCalculator_Attributes {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BattleCalculator.Attributes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BattleCalculator_Attributes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl BattleCalculator_Attributes {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn physical() -> Self {
        Self { value: 1 }
    }

    pub fn magic() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator_FuncExp2.md")))]
#[::unity2::class(namespace = "App", name = "BattleCalculator.FuncExp2")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct BattleCalculator_FuncExp2 {}

#[cfg(feature = "app-battlecalculator")]
#[::unity2::methods]
impl BattleCalculator_FuncExp2 {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
    ) -> i32;
}

#[cfg(feature = "app-battlecalculator")]
impl BattleCalculator_FuncExp2 {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleCalculator_FuncExp2),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleCalculator_FuncExp2Methods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator_Order.md")))]
#[::unity2::class(namespace = "App", name = "BattleCalculator.Order")]
#[parent(crate::app::pool::Pool_Node)]
pub struct BattleCalculator_Order {
    #[rename(name = "Side")]
    pub side: crate::app::battleside::BattleSide_Type,
}

#[cfg(feature = "app-battlecalculator")]
#[::unity2::methods]
impl BattleCalculator_Order {
    #[method(name = "OnEnter", args = 0)]
    pub fn on_enter(self) -> ();

    #[method(name = "OnExit", args = 0)]
    pub fn on_exit(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-battlecalculator")]
impl BattleCalculator_Order {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleCalculator_Order),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleCalculator_OrderMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator.md")))]
#[::unity2::class(namespace = "App", name = "BattleCalculator")]
#[parent(crate::system::object::Object)]
pub struct BattleCalculator {
    #[rename(name = "m_Mode")]
    pub m_mode: crate::app::battlecalculator::BattleCalculator_Mode,
    #[rename(name = "m_Info")]
    pub m_info: crate::app::battleinfo::BattleInfo,
    #[rename(name = "m_Flag")]
    pub m_flag: crate::app::battlecalculator::BattleCalculator_FlagField,
    #[rename(name = "m_SceneList")]
    pub m_scene_list: crate::app::battlescenelist::BattleSceneList,
    #[rename(name = "m_Orders")]
    pub m_orders: crate::app::battlecalculator::BattleCalculator_OrderList,
    #[rename(name = "m_NextOrderIndex")]
    pub m_next_order_index: i32,
    #[rename(name = "m_EquipSkill")]
    pub m_equip_skill: crate::app::skilldata::SkillData,
    #[rename(name = "m_ChainOffenses")]
    pub m_chain_offenses: crate::system::collections::generic::list_1::List_1<
        crate::app::battleinfoside::BattleInfoSide,
    >,
    #[rename(name = "m_ChainDefenses")]
    pub m_chain_defenses: crate::system::collections::generic::list_1::List_1<
        crate::app::battleinfoside::BattleInfoSide,
    >,
    #[rename(name = "m_HitSkillPool")]
    pub m_hit_skill_pool: crate::app::battlecalculator::BattleCalculator_HitSkillPool,
    #[rename(name = "m_CommitSkillUnits")]
    pub m_commit_skill_units: crate::system::collections::generic::hashset_1::HashSet_1<i32>,
    #[static_field]
    #[rename(name = "CannonConditions")]
    pub cannon_conditions: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "DanceConditions")]
    pub dance_conditions: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "EngageSummon3")]
    pub engage_summon3: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "EngageSummon5")]
    pub engage_summon5: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-battlecalculator")]
#[::unity2::methods]
impl BattleCalculator {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, info: crate::app::battleinfo::BattleInfo) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "GetMode", args = 0)]
    pub fn get_mode(self) -> crate::app::battlecalculator::BattleCalculator_Mode;

    #[method(name = "CalcBattle", args = 0)]
    pub fn calc_battle(self) -> crate::app::battlecalculator::BattleCalculator;

    #[method(name = "Dump", args = 0)]
    pub fn dump(self) -> ();

    #[method(name = "TryAddSkillScene", args = 3)]
    pub fn try_add_skill_scene(
        self,
        kind: crate::app::battlescene::BattleScene_Kind,
        side: crate::app::battleside::BattleSide_Type,
        skill: crate::app::skilldata::SkillData,
    ) -> bool;

    #[method(name = "CalcEngageStart", args = 1)]
    pub fn calc_engage_start(self, side: crate::app::battleside::BattleSide_Type) -> ();

    #[method(name = "CalcInvokeSkill", args = 1)]
    pub fn calc_invoke_skill(self, side: crate::app::battleside::BattleSide_Type) -> ();

    #[method(name = "CalcEngageEnd", args = 1)]
    pub fn calc_engage_end(self, side: crate::app::battleside::BattleSide_Type) -> ();

    #[method(name = "CalcDetailSkill", args = 0)]
    pub fn calc_detail_skill(self) -> ();

    #[method(name = "CalcBranch", args = 0)]
    pub fn calc_branch(self) -> ();

    #[method(name = "IsLateOrder", args = 0)]
    pub fn is_late_order(self) -> bool;

    #[method(name = "SeparatorTotalOrder", args = 0)]
    pub fn separator_total_order(self) -> ();

    #[method(name = "CalcNormalBattle", args = 0)]
    pub fn calc_normal_battle(self) -> ();

    #[method(name = "CalcDestroy", args = 0)]
    pub fn calc_destroy(self) -> ();

    #[method(name = "AddSkillSceneResult", args = 1)]
    pub fn add_skill_scene_result(self, result: crate::app::battlescene::BattleScene_Result) -> ();

    #[method(name = "CalcDestroyAttack", args = 1)]
    pub fn calc_destroy_attack(self, side: crate::app::battleside::BattleSide_Type) -> bool;

    #[method(name = "CalcWarmup", args = 0)]
    pub fn calc_warmup(self) -> crate::app::battlecalculator::BattleCalculator;

    #[method(name = "CalcSimulation", args = 0)]
    pub fn calc_simulation(self) -> crate::app::battlecalculator::BattleCalculator;

    #[method(name = "CalcJobIntro", args = 0)]
    pub fn calc_job_intro(self) -> crate::app::battlecalculator::BattleCalculator;

    #[method(name = "CalcClassChange", args = 0)]
    pub fn calc_class_change(self) -> crate::app::battlecalculator::BattleCalculator;

    #[method(name = "GetHp", args = 1)]
    pub fn get_hp(self, side: crate::app::battleside::BattleSide_Type) -> i32;

    #[method(name = "GetStatus", args = 1)]
    pub fn get_status(
        self,
        side: crate::app::battleside::BattleSide_Type,
    ) -> crate::app::battleinfoside::BattleInfoSide_BitFieldStatus;

    #[method(name = "get_Info", args = 0)]
    pub fn get_info(self) -> crate::app::battleinfo::BattleInfo;

    #[method(name = "get_SceneList", args = 0)]
    pub fn get_scene_list(self) -> crate::app::battlescenelist::BattleSceneList;

    #[method(name = "CalcActiveSkill", args = 2)]
    pub fn calc_active_skill(
        self,
        timing: crate::app::skilldata::SkillData_Timings,
        side: crate::app::battleside::BattleSide_Type,
    ) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "CanAttack", args = 1)]
    pub fn can_attack(self, side: crate::app::battleinfoside::BattleInfoSide) -> bool;

    #[method(name = "AddSkillScene", args = 3)]
    pub fn add_skill_scene(
        self,
        skill: crate::app::skilldata::SkillData,
        action: crate::app::skilldata::SkillData_Actions,
        current: crate::app::battleinfoside::BattleInfoSide,
    ) -> ();

    #[method(name = "GetBreaked", args = 1)]
    pub fn get_breaked(
        self,
        target: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "AddGiveScene", args = 4)]
    pub fn add_give_scene(
        self,
        give: crate::app::skilldata::SkillData,
        current: crate::app::battleinfoside::BattleInfoSide,
        target: crate::app::battleinfoside::BattleInfoSide,
        kind: crate::app::battlescene::BattleScene_Kind,
    ) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "FindIgnoreSkill", args = 2)]
    pub fn find_ignore_skill(
        target: crate::app::battleinfoside::BattleInfoSide,
        skill: crate::app::skilldata::SkillData,
    ) -> crate::app::skilldata::SkillData;

    #[method(name = "AddGiveScene", args = 3)]
    pub fn add_give_scene_2(
        self,
        give: crate::app::skilldata::SkillData,
        current: crate::app::battleinfoside::BattleInfoSide,
        target: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "AddGivesScene", args = 3)]
    pub fn add_gives_scene(
        self,
        skill: crate::app::skilldata::SkillData,
        current: crate::app::battleinfoside::BattleInfoSide,
        target: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "AddGivesScene", args = 3)]
    pub fn add_gives_scene_2(
        self,
        skill: crate::app::skilldata::SkillData,
        action: crate::app::skilldata::SkillData_Actions,
        current: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "CalcHitSkill", args = 2)]
    pub fn calc_hit_skill(
        self,
        current: crate::app::battleinfoside::BattleInfoSide,
        unit_item: crate::app::unititem::UnitItem,
    ) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "CalcActiveSkill", args = 3)]
    pub fn calc_active_skill_2(
        self,
        timing: crate::app::skilldata::SkillData_Timings,
        action: crate::app::skilldata::SkillData_Actions,
        current: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "CanOrederSkill", args = 2)]
    pub fn can_oreder_skill(
        self,
        skill: crate::app::skilldata::SkillData,
        current: crate::app::battleinfoside::BattleInfoSide,
    ) -> bool;

    #[method(name = "CalcActiveSkill", args = 3)]
    pub fn calc_active_skill_3(
        self,
        skill: crate::app::skilldata::SkillData,
        action: crate::app::skilldata::SkillData_Actions,
        current: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "CalcHitSkill", args = 2)]
    pub fn calc_hit_skill_2(
        self,
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "CalcHitSkill", args = 3)]
    pub fn calc_hit_skill_3(
        self,
        current: crate::app::battleinfoside::BattleInfoSide,
        action: crate::app::skilldata::SkillData_Actions,
        skill: crate::app::skilldata::SkillData,
    ) -> ();

    #[method(name = "CalcResultSkill", args = 1)]
    pub fn calc_result_skill(self, current: crate::app::battleinfoside::BattleInfoSide) -> ();

    #[method(name = "CalcResultSkill", args = 0)]
    pub fn calc_result_skill_2(self) -> ();

    #[method(name = "AddDebugIndent", args = 1)]
    pub fn add_debug_indent(name: ::unity2::Il2CppString) -> ();

    #[method(name = "DecDebugIndent", args = 0)]
    pub fn dec_debug_indent() -> ();

    #[method(name = "CalcChainAttackSides", args = 1)]
    pub fn calc_chain_attack_sides(
        self,
        current: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::battleinfoside::BattleInfoSide,
    >;

    #[method(name = "CalcChainGuardSide", args = 1)]
    pub fn calc_chain_guard_side(
        self,
        reverse: crate::app::battleinfoside::BattleInfoSide,
    ) -> crate::app::battleside::BattleSide_Type;

    #[method(name = "CanChainAttack", args = 1)]
    pub fn can_chain_attack(self, side: crate::app::battleside::BattleSide_Type) -> bool;

    #[method(name = "CalcChainAttack", args = 1)]
    pub fn calc_chain_attack(self, side: crate::app::battleside::BattleSide_Type) -> bool;

    #[method(name = "CalcOrders", args = 1)]
    pub fn calc_orders(
        self,
        orders: crate::app::battlecalculator::BattleCalculator_OrderList,
    ) -> bool;

    #[method(name = "CalcInterruptOrder", args = 1)]
    pub fn calc_interrupt_order(self, side: crate::app::battleside::BattleSide_Type) -> bool;

    #[method(name = "CalcInterrupt", args = 1)]
    pub fn calc_interrupt(self, side: crate::app::battleside::BattleSide_Type) -> bool;

    #[method(name = "AddEngageCount", args = 1)]
    pub fn add_engage_count(self, current: crate::app::battleinfoside::BattleInfoSide) -> ();

    #[method(name = "AddEngageCount", args = 1)]
    pub fn add_engage_count_2(self, side: crate::app::battleside::BattleSide_Type) -> ();

    #[method(name = "CalcOrders", args = 1)]
    pub fn calc_orders_2(self, side: crate::app::battleside::BattleSide_Type) -> bool;

    #[method(name = "CalcAssistSkill", args = 1)]
    pub fn calc_assist_skill(self, side: crate::app::battleside::BattleSide_Type) -> ();

    #[method(name = "CalcOrder", args = 1)]
    pub fn calc_order(self, side: crate::app::battleside::BattleSide_Type) -> bool;

    #[method(name = "CalcAction", args = 1)]
    pub fn calc_action(self, side: crate::app::battleside::BattleSide_Type) -> bool;

    #[method(name = "GetCommandValue", args = 3)]
    pub fn get_command_value(
        key: ::unity2::Il2CppString,
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
    ) -> i32;

    #[method(name = "GetChainGuardDamage", args = 2)]
    pub fn get_chain_guard_damage(
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
    ) -> i32;

    #[method(name = "RandomCheckHit", args = 1)]
    pub fn random_check_hit(self, ratio: i32) -> bool;

    #[method(name = "CalcAttack", args = 1)]
    pub fn calc_attack(self, side_type: crate::app::battleside::BattleSide_Type) -> bool;

    #[method(name = "CanEnter", args = 3)]
    pub fn can_enter(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
        x: i32,
        z: i32,
    ) -> bool;

    #[method(name = "CanBlow", args = 2)]
    pub fn can_blow(
        self,
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
    ) -> bool;

    #[method(name = "CalcAttackHit", args = 3)]
    pub fn calc_attack_hit(
        self,
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
        critical: i32,
    ) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "TryAddDeadScene", args = 2)]
    pub fn try_add_dead_scene(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
        result: crate::app::battlescene::BattleScene_Result,
    ) -> bool;

    #[method(name = "CalcAttackPost", args = 3)]
    pub fn calc_attack_post(
        self,
        side_type: crate::app::battleside::BattleSide_Type,
        damage: i32,
        result: crate::app::battlescene::BattleScene_Result,
    ) -> bool;

    #[method(name = "get_SceneResult", args = 0)]
    pub fn get_scene_result(self) -> crate::app::battlescene::BattleScene_Result;

    #[method(name = "set_SceneResult", args = 1)]
    pub fn set_scene_result(self, value: crate::app::battlescene::BattleScene_Result) -> ();

    #[method(name = "CalcRod", args = 0)]
    pub fn calc_rod(self) -> ();

    #[method(name = "CalcRodOrder", args = 1)]
    pub fn calc_rod_order(self, side: crate::app::battleside::BattleSide_Type) -> ();

    #[method(name = "CalcRodAction", args = 1)]
    pub fn calc_rod_action(self, side: crate::app::battleside::BattleSide_Type) -> ();

    #[method(name = "CalcRodAttack", args = 1)]
    pub fn calc_rod_attack(self, side: crate::app::battleside::BattleSide_Type) -> ();

    #[method(name = "CalcDance", args = 0)]
    pub fn calc_dance(self) -> ();

    #[method(name = "CalcDanceOrder", args = 0)]
    pub fn calc_dance_order(self) -> ();

    #[method(name = "CalcDanceAction", args = 0)]
    pub fn calc_dance_action(self) -> ();

    #[method(name = "CalcDanceAttack", args = 0)]
    pub fn calc_dance_attack(self) -> ();

    #[method(name = "CalcEnchant", args = 0)]
    pub fn calc_enchant(self) -> ();

    #[method(name = "CalcRodHit", args = 4)]
    pub fn calc_rod_hit(
        self,
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
        unit_item: crate::app::unititem::UnitItem,
        item: crate::app::itemdata::ItemData,
    ) -> ();

    #[method(name = "CalcResult", args = 0)]
    pub fn calc_result(self) -> ();

    #[method(name = "CanBattleRecord", args = 1)]
    pub fn can_battle_record(self, side: crate::app::battleinfoside::BattleInfoSide) -> bool;

    #[method(name = "CalcRecord", args = 0)]
    pub fn calc_record(self) -> ();

    #[method(name = "GetSide", args = 1)]
    pub fn get_side(
        self,
        force: crate::app::force::Force_Type,
    ) -> crate::app::battleinfoside::BattleInfoSide;

    #[method(name = "CalcAchieve", args = 0)]
    pub fn calc_achieve(self) -> ();

    #[method(name = "CalcExpendCount", args = 0)]
    pub fn calc_expend_count(self) -> ();

    #[method(name = "IsChainAttackDead", args = 0)]
    pub fn is_chain_attack_dead(self) -> bool;

    #[method(name = "CalcExp", args = 2)]
    pub fn calc_exp(
        self,
        func: crate::app::battlecalculator::BattleCalculator_FuncExp1,
        current: crate::app::battleinfoside::BattleInfoSide,
    ) -> ();

    #[method(name = "CalcExp", args = 3)]
    pub fn calc_exp_2(
        self,
        func: crate::app::battlecalculator::BattleCalculator_FuncExp2,
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
    ) -> ();

    #[method(name = "CalcResultGain", args = 0)]
    pub fn calc_result_gain(self) -> ();

    #[method(name = "CalcExpCount", args = 0)]
    pub fn calc_exp_count(self) -> ();

    #[method(name = "CommitHP", args = 2)]
    pub fn commit_hp(self, side: crate::app::battleinfoside::BattleInfoSide, hp: i32) -> ();

    #[method(name = "CommitEngageCount", args = 2)]
    pub fn commit_engage_count(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
        count: i32,
    ) -> ();

    #[method(name = "UpdateScene", args = 0)]
    pub fn update_scene(self) -> ();

    #[method(name = "UpdateScene", args = 2)]
    pub fn update_scene_2(self, index: i32, is_engae_count: bool) -> ();

    #[method(name = "UpdateScene", args = 2)]
    pub fn update_scene_3(
        self,
        scene: crate::app::battlescene::BattleScene,
        is_engae_count: bool,
    ) -> ();

    #[method(name = "CommitUnit", args = 0)]
    pub fn commit_unit(self) -> ();

    #[method(name = "CommitSkill", args = 0)]
    pub fn commit_skill(self) -> ();

    #[method(name = "CommitDestroy", args = 0)]
    pub fn commit_destroy(self) -> ();

    #[method(name = "CommitBattle", args = 0)]
    pub fn commit_battle(self) -> ();

    #[method(name = "MapHistoryCommitSkill", args = 1)]
    pub fn map_history_commit_skill(self, unit: crate::app::unit::Unit) -> ();

    #[method(name = "IsTraining", args = 0)]
    pub fn is_training(self) -> bool;

    #[method(name = "IsRod", args = 0)]
    pub fn is_rod(self) -> bool;

    #[method(name = "GetUnit", args = 1)]
    pub fn get_unit(self, index: i32) -> crate::app::unit::Unit;

    #[method(name = "GetUnit", args = 1)]
    pub fn get_unit_2(
        self,
        side: crate::app::battleside::BattleSide_Type,
    ) -> crate::app::unit::Unit;

    #[method(name = "GetSide", args = 1)]
    pub fn get_side_2(self, index: i32) -> crate::app::battleinfoside::BattleInfoSide;

    #[method(name = "GetSide", args = 1)]
    pub fn get_side_3(
        self,
        side: crate::app::battleside::BattleSide_Type,
    ) -> crate::app::battleinfoside::BattleInfoSide;

    #[method(name = "GetDeadSide", args = 0)]
    pub fn get_dead_side(self) -> crate::app::battleside::BattleSide_Type;

    #[method(name = "GetDeadUnit", args = 0)]
    pub fn get_dead_unit(self) -> crate::app::unit::Unit;

    #[method(name = "GetExpendCount", args = 1)]
    pub fn get_expend_count(self, side: crate::app::battleinfoside::BattleInfoSide) -> i32;

    #[method(name = "CanSkyBattle", args = 0)]
    pub fn can_sky_battle(self) -> bool;

    #[method(name = "IsMultiBattle", args = 0)]
    pub fn is_multi_battle(self) -> bool;

    #[method(name = "IsCannonBattle", args = 0)]
    pub fn is_cannon_battle(self) -> bool;

    #[method(name = "IsFireCannon", args = 0)]
    pub fn is_fire_cannon(self) -> bool;

    #[method(name = "IsFullBullet", args = 0)]
    pub fn is_full_bullet(self) -> bool;

    #[method(name = "IsLastBossDie", args = 0)]
    pub fn is_last_boss_die(self) -> bool;

    #[method(name = "IsBattleGrow", args = 0)]
    pub fn is_battle_grow(self) -> bool;

    #[method(name = "IsCombatDie", args = 0)]
    pub fn is_combat_die(self) -> bool;

    #[method(name = "IsCombatGrow", args = 0)]
    pub fn is_combat_grow(self) -> bool;

    #[method(name = "GetAssetConditions", args = 1)]
    pub fn get_asset_conditions(
        self,
        side_type: crate::app::battleside::BattleSide_Type,
    ) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-battlecalculator")]
impl BattleCalculator {
    pub fn new(info: crate::app::battleinfo::BattleInfo) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleCalculator),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleCalculatorMethods>::ctor(this, info);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct BattleCalculator_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for BattleCalculator_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BattleCalculator.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BattleCalculator_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl BattleCalculator_Flags {
    pub fn interrupt_offense() -> Self {
        Self { value: 1 }
    }

    pub fn interrupt_defense() -> Self {
        Self { value: 2 }
    }

    pub fn interrupting() -> Self {
        Self { value: 4 }
    }

    pub fn continue_battle() -> Self {
        Self { value: 8 }
    }

    pub fn swap_order() -> Self {
        Self { value: 16 }
    }

    pub fn dead() -> Self {
        Self { value: 32 }
    }

    pub fn chain_attacked() -> Self {
        Self { value: 64 }
    }

    pub fn commited() -> Self {
        Self { value: 128 }
    }

    pub fn mask_interrupt() -> Self {
        Self { value: 7 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator_TargetScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct BattleCalculator_TargetScope {
    pub m_current: crate::app::battleinfoside::BattleInfoSide,
    pub m_reverse: crate::app::battleinfoside::BattleInfoSide,
    pub m_is_dump: bool,
}

impl ::unity2::ClassIdentity for BattleCalculator_TargetScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BattleCalculator.TargetScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BattleCalculator_TargetScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-battlecalculator")]
#[::unity2::methods(value)]
impl BattleCalculator_TargetScope {
    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        current: crate::app::battleinfoside::BattleInfoSide,
        reverse: crate::app::battleinfoside::BattleInfoSide,
        is_dump: bool,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator_FlagField.md")))]
#[::unity2::class(namespace = "App", name = "BattleCalculator.FlagField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: battlecalculator :: BattleCalculator_Flags >)]
pub struct BattleCalculator_FlagField {}

#[cfg(feature = "app-battlecalculator")]
#[::unity2::methods]
impl BattleCalculator_FlagField {
    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::battlecalculator::BattleCalculator_Flags) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-battlecalculator")]
impl BattleCalculator_FlagField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleCalculator_FlagField),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleCalculator_FlagFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator_OrderList.md")))]
#[::unity2::class(namespace = "App", name = "BattleCalculator.OrderList")]
# [parent (crate :: app :: pool :: Pool_List_1 < crate :: app :: battlecalculator :: BattleCalculator_Order >)]
pub struct BattleCalculator_OrderList {
    #[static_field]
    #[rename(name = "MaxOrder")]
    pub max_order: i32,
}

#[cfg(feature = "app-battlecalculator")]
#[::unity2::methods]
impl BattleCalculator_OrderList {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "TryAdd", args = 2)]
    pub fn try_add(
        self,
        info: crate::app::battleinfo::BattleInfo,
        side: crate::app::battleside::BattleSide_Type,
    ) -> ();

    #[method(name = "CanSwap", args = 2)]
    pub fn can_swap(self, side: crate::app::battleside::BattleSide_Type, index: i32) -> bool;
}

#[cfg(feature = "app-battlecalculator")]
impl BattleCalculator_OrderList {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleCalculator_OrderList),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleCalculator_OrderListMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator_Mode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct BattleCalculator_Mode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for BattleCalculator_Mode {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BattleCalculator.Mode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BattleCalculator_Mode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl BattleCalculator_Mode {
    pub fn battle() -> Self {
        Self { value: 0 }
    }

    pub fn job_intro() -> Self {
        Self { value: 1 }
    }

    pub fn class_change() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator_TrainingResult.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct BattleCalculator_TrainingResult {
    pub value: i32,
}

impl ::unity2::ClassIdentity for BattleCalculator_TrainingResult {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BattleCalculator.TrainingResult";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BattleCalculator_TrainingResult {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl BattleCalculator_TrainingResult {
    pub fn win() -> Self {
        Self { value: 0 }
    }

    pub fn lose() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator_DetailScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct BattleCalculator_DetailScope {
    pub m_info: crate::app::battleinfo::BattleInfo,
}

impl ::unity2::ClassIdentity for BattleCalculator_DetailScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BattleCalculator.DetailScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BattleCalculator_DetailScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-battlecalculator")]
#[::unity2::methods(value)]
impl BattleCalculator_DetailScope {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, info: crate::app::battleinfo::BattleInfo) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator_SeparatorScope.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct BattleCalculator_SeparatorScope {
    pub m_calc: crate::app::battlecalculator::BattleCalculator,
    pub m_push: crate::app::battlescene::BattleScene_Kind,
    pub m_pop: crate::app::battlescene::BattleScene_Kind,
    pub m_side: crate::app::battleside::BattleSide_Type,
    pub m_equip_skill: crate::app::skilldata::SkillData,
    pub m_is_dump: bool,
}

impl ::unity2::ClassIdentity for BattleCalculator_SeparatorScope {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "BattleCalculator.SeparatorScope";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for BattleCalculator_SeparatorScope {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "app-battlecalculator")]
#[::unity2::methods(value)]
impl BattleCalculator_SeparatorScope {
    #[method(name = ".ctor", args = 5)]
    pub fn ctor(
        self,
        calc: crate::app::battlecalculator::BattleCalculator,
        side: crate::app::battleside::BattleSide_Type,
        push: crate::app::battlescene::BattleScene_Kind,
        pop: crate::app::battlescene::BattleScene_Kind,
        is_dump: bool,
    ) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = "GetSide", args = 0)]
    pub fn get_side(self) -> crate::app::battleinfoside::BattleInfoSide;

    #[method(name = "GetSide", args = 1)]
    pub fn get_side_2(
        self,
        side: crate::app::battleside::BattleSide_Type,
    ) -> crate::app::battleinfoside::BattleInfoSide;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/battlecalculator/BattleCalculator_HitSkillPool.md")))]
#[::unity2::class(namespace = "App", name = "BattleCalculator.HitSkillPool")]
# [parent (crate :: app :: pool :: Pool_List_1 < crate :: app :: battlecalculator :: BattleCalculator_HitSkill >)]
pub struct BattleCalculator_HitSkillPool {}

#[cfg(feature = "app-battlecalculator")]
#[::unity2::methods]
impl BattleCalculator_HitSkillPool {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Add", args = 3)]
    pub fn add(
        self,
        side: crate::app::battleinfoside::BattleInfoSide,
        timing: crate::app::skilldata::SkillData_Timings,
        action: crate::app::skilldata::SkillData_Actions,
    ) -> ();
}

#[cfg(feature = "app-battlecalculator")]
impl BattleCalculator_HitSkillPool {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BattleCalculator_HitSkillPool),
                ::core::stringify!(new),
            )
        });
        <Self as IBattleCalculator_HitSkillPoolMethods>::ctor(this);
        this
    }
}
